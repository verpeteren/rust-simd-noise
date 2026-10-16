//! End-to-end tests for the 5D/6D public `NoiseBuilder` API (task P3.8/P3.9).
//!
//! These exercise the whole plumbing stack — builder -> settings -> helpers ->
//! runtime engine dispatch -> kernels — for every noise family at 5D and 6D,
//! for both float widths, checking block length, finiteness, and that the
//! preallocated `generate_into` path matches `generate`.
//!
//! Heavy 6D cellular (a 3^6 = 729-cell neighbourhood) builds a large stack frame
//! in unoptimized/test builds (no stack-slot reuse; see issue #38), so the work
//! runs on a thread with an enlarged stack, mirroring `tests/issue_38.rs`.

use simdnoise::{NoiseBuilder, Settings};

fn on_big_stack<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(f)
        .expect("spawn thread")
        .join()
        .expect("join thread");
}

fn check_block_f32(v: &[f32], expected_len: usize, label: &str) {
    assert_eq!(v.len(), expected_len, "{}: wrong length", label);
    assert!(
        v.iter().all(|x| x.is_finite()),
        "{}: non-finite value",
        label
    );
}

#[test]
fn builder_5d_all_families() {
    on_big_stack(|| {
        // 8 x 6 x 4 x 2 x 2 = 768
        let len = 8 * 6 * 4 * 2 * 2;
        macro_rules! gen5 {
            ($builder:expr, $label:literal) => {{
                let (v, mn, mx) = $builder.generate();
                check_block_f32(&v, len, $label);
                assert!(mn <= mx, concat!($label, ": min>max"));
                // generate_into must reproduce generate exactly.
                let mut buf = vec![0.0f32; len];
                let (mn2, mx2) = $builder.generate_into(&mut buf);
                assert_eq!(buf, v, concat!($label, ": generate_into != generate"));
                assert_eq!((mn, mx), (mn2, mx2));
                // f64 path is finite and correctly sized.
                let (v64, _, _) = $builder.generate_64();
                assert_eq!(v64.len(), len, concat!($label, ": f64 wrong length"));
                assert!(
                    v64.iter().all(|x| x.is_finite()),
                    concat!($label, ": f64 non-finite")
                );
            }};
        }
        gen5!(NoiseBuilder::gradient_5d(8, 6, 4, 2, 2), "gradient_5d");
        gen5!(NoiseBuilder::fbm_5d(8, 6, 4, 2, 2), "fbm_5d");
        gen5!(NoiseBuilder::ridge_5d(8, 6, 4, 2, 2), "ridge_5d");
        gen5!(NoiseBuilder::turbulence_5d(8, 6, 4, 2, 2), "turbulence_5d");
        gen5!(NoiseBuilder::cellular_5d(8, 6, 4, 2, 2), "cellular_5d");
        gen5!(NoiseBuilder::cellular2_5d(8, 6, 4, 2, 2), "cellular2_5d");
    });
}

#[test]
fn builder_6d_all_families() {
    on_big_stack(|| {
        // 8 x 4 x 4 x 2 x 2 x 2 = 1024
        let len = 8 * 4 * 4 * 2 * 2 * 2;
        macro_rules! gen6 {
            ($builder:expr, $label:literal) => {{
                let (v, mn, mx) = $builder.generate();
                check_block_f32(&v, len, $label);
                assert!(mn <= mx, concat!($label, ": min>max"));
                let mut buf = vec![0.0f32; len];
                let (mn2, mx2) = $builder.generate_into(&mut buf);
                assert_eq!(buf, v, concat!($label, ": generate_into != generate"));
                assert_eq!((mn, mx), (mn2, mx2));
                let (v64, _, _) = $builder.generate_64();
                assert_eq!(v64.len(), len, concat!($label, ": f64 wrong length"));
                assert!(
                    v64.iter().all(|x| x.is_finite()),
                    concat!($label, ": f64 non-finite")
                );
            }};
        }
        gen6!(NoiseBuilder::gradient_6d(8, 4, 4, 2, 2, 2), "gradient_6d");
        gen6!(NoiseBuilder::fbm_6d(8, 4, 4, 2, 2, 2), "fbm_6d");
        gen6!(NoiseBuilder::ridge_6d(8, 4, 4, 2, 2, 2), "ridge_6d");
        gen6!(
            NoiseBuilder::turbulence_6d(8, 4, 4, 2, 2, 2),
            "turbulence_6d"
        );
        gen6!(NoiseBuilder::cellular_6d(8, 4, 4, 2, 2, 2), "cellular_6d");
        gen6!(NoiseBuilder::cellular2_6d(8, 4, 4, 2, 2, 2), "cellular2_6d");
    });
}

#[test]
fn scaled_5d_6d_within_range() {
    on_big_stack(|| {
        // generate_scaled must keep every value inside the requested range.
        let v = NoiseBuilder::fbm_5d(8, 6, 4, 2, 2).generate_scaled(-1.0, 1.0);
        assert_eq!(v.len(), 8 * 6 * 4 * 2 * 2);
        assert!(v.iter().all(|&x| (-1.0..=1.0).contains(&x)));

        let v6 = NoiseBuilder::gradient_6d(8, 4, 4, 2, 2, 2).generate_scaled(0.0, 255.0);
        assert_eq!(v6.len(), 8 * 4 * 4 * 2 * 2 * 2);
        assert!(v6.iter().all(|&x| (0.0..=255.0).contains(&x)));
    });
}

// Block-level cross-engine agreement through the public per-engine API: every
// SIMD backend must reproduce the scalar engine's block (within the small
// tolerance the pre-existing `consistency_*` tests use). fbm keeps the stack
// frame modest, so no enlarged stack is needed here.
#[test]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn cross_engine_block_5d_6d() {
    on_big_stack(cross_engine_block_5d_6d_inner);
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn cross_engine_block_5d_6d_inner() {
    use simdnoise::intrinsics::{avx2, scalar, sse2, sse41};

    let settings_5d = NoiseBuilder::fbm_5d(8, 8, 6, 4, 4).wrap();
    let settings_6d = NoiseBuilder::fbm_6d(8, 6, 4, 4, 2, 2).wrap();

    for (settings, is_6d) in [(settings_5d, false), (settings_6d, true)] {
        let scalar_noise = unsafe {
            if is_6d {
                scalar::get_6d_scaled_noise::<simdeez::scalar::Scalar>(&settings)
            } else {
                scalar::get_5d_scaled_noise::<simdeez::scalar::Scalar>(&settings)
            }
        };
        let cmp = |other: &[f32]| {
            assert_eq!(other.len(), scalar_noise.len());
            for (a, b) in scalar_noise.iter().zip(other) {
                assert!(
                    (a - b).abs() < 0.1,
                    "engine disagrees with scalar: {} vs {}",
                    a,
                    b
                );
            }
        };
        unsafe {
            if is_x86_feature_detected!("sse2") {
                cmp(&if is_6d {
                    sse2::get_6d_scaled_noise::<simdeez::engines::sse2::Sse2>(&settings)
                } else {
                    sse2::get_5d_scaled_noise::<simdeez::engines::sse2::Sse2>(&settings)
                });
            }
            if is_x86_feature_detected!("sse4.1") {
                cmp(&if is_6d {
                    sse41::get_6d_scaled_noise::<simdeez::engines::sse41::Sse41>(&settings)
                } else {
                    sse41::get_5d_scaled_noise::<simdeez::engines::sse41::Sse41>(&settings)
                });
            }
            if is_x86_feature_detected!("avx2") {
                cmp(&if is_6d {
                    avx2::get_6d_scaled_noise::<simdeez::engines::avx2::Avx2>(&settings)
                } else {
                    avx2::get_5d_scaled_noise::<simdeez::engines::avx2::Avx2>(&settings)
                });
            }
        }
    }
}

#[test]
fn offset_builders_5d_6d() {
    on_big_stack(|| {
        // The `_offset` constructors should accept per-axis offsets and produce
        // the same-sized, finite blocks.
        let (v, _, _) =
            NoiseBuilder::fbm_5d_offset(1.0, 8, 2.0, 6, 3.0, 4, 4.0, 2, 5.0, 2).generate();
        check_block_f32(&v, 8 * 6 * 4 * 2 * 2, "fbm_5d_offset");

        let (v6, _, _) =
            NoiseBuilder::ridge_6d_offset(1.0, 8, 2.0, 4, 3.0, 4, 4.0, 2, 5.0, 2, 6.0, 2)
                .generate();
        check_block_f32(&v6, 8 * 4 * 4 * 2 * 2 * 2, "ridge_6d_offset");
    });
}
