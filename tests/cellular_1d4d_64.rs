//! f64 golden-fixture tests for **1D and 4D** cellular / cellular2 noise
//! (x86 engines). Companion to `intrinsics_cellular_64.rs`, which covers 2D/3D.
//!
//! The 2D/3D file spells out one `#[test]` per case; for the much larger 1D+4D
//! matrix we loop over the (distance, return) combos inside one test per engine
//! instead. The committed `.bin` fixtures and their names are identical in form
//! to the 2D/3D ones (`intrinsics_{cellular|cellular2}_64_{dim}_{eng}_{combo}_{arch}.bin`),
//! so this is the same golden matrix, just generated compactly.
//!
//! Regenerate the fixtures with:
//!     SIMDNOISE_REGEN=1 cargo test --test cellular_1d4d_64
//!
//! NEON f64 cellular is intentionally excluded (approximate `rsqrt` → NaN), so
//! this file is x86-only, matching `intrinsics_cellular_64.rs`.
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use simdnoise::intrinsics::{avx2, scalar, sse2, sse41};
use simdnoise::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, Cellular2Settings, CellularSettings,
    NoiseDimensions, Settings,
};

mod helpers;
use helpers::{read_from_file_f64, save_to_file_f64, ARCH, BIN_PATH};

// (dim, dim_str, width, height, depth, time). 4D uses small extents (width a
// multiple of 16 for full AVX-512 vectors) to keep fixtures compact.
const DIMS: [(usize, &str, usize, usize, usize, usize); 2] =
    [(1, "1d", 64, 1, 1, 1), (4, "4d", 16, 8, 4, 4)];

const CELL_DF: [(CellDistanceFunction, &str); 3] = [
    (CellDistanceFunction::Euclidean, "euclidean"),
    (CellDistanceFunction::Manhattan, "manhattan"),
    (CellDistanceFunction::Natural, "natural"),
];
const CELL_RT: [(CellReturnType, &str); 2] = [
    (CellReturnType::Distance, "distance"),
    (CellReturnType::CellValue, "cellvalue"),
];
const CELL2_RT: [(Cell2ReturnType, &str); 5] = [
    (Cell2ReturnType::Distance2, "distance2"),
    (Cell2ReturnType::Distance2Add, "distance2add"),
    (Cell2ReturnType::Distance2Sub, "distance2sub"),
    (Cell2ReturnType::Distance2Mul, "distance2mul"),
    (Cell2ReturnType::Distance2Div, "distance2div"),
];

fn dims_of(dim: usize, w: usize, h: usize, d: usize, t: usize) -> NoiseDimensions {
    let mut nd = NoiseDimensions::default(dim);
    nd.width = w;
    nd.height = h;
    nd.depth = d;
    nd.time = t;
    nd
}

fn check(file_name: String, noise: &[f64]) {
    if std::env::var_os("SIMDNOISE_REGEN").is_some() {
        save_to_file_f64(&file_name, noise).unwrap();
    } else {
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, *noise, "mismatch: {}", file_name);
    }
}

macro_rules! cell_engine {
    ($name:ident, $m:ident, $eng:ty, $engname:literal $(, $tf:literal)?) => {
        #[test]
        fn $name() {
            $(#[target_feature(enable = $tf)])?
            unsafe fn run() {
                for (dim, dim_str, w, h, d, t) in DIMS {
                    for (df, dfn) in CELL_DF {
                        for (rt, rtn) in CELL_RT {
                            let nt = CellularSettings::default(dims_of(dim, w, h, d, t))
                                .with_seed(1337)
                                .with_distance_function(df)
                                .with_return_type(rt)
                                .wrap();
                            let noise = if dim == 1 {
                                $m::get_1d_noise_64::<$eng>(&nt).0
                            } else {
                                $m::get_4d_noise_64::<$eng>(&nt).0
                            };
                            let file_name = format!(
                                "{}/intrinsics_cellular_64_{}_{}_{}_{}_{}.bin",
                                BIN_PATH, dim_str, $engname, dfn, rtn, ARCH
                            );
                            check(file_name, &noise);
                        }
                    }
                    for (rt, rtn) in CELL2_RT {
                        for (df, dfn) in CELL_DF {
                            let nt = Cellular2Settings::default(dims_of(dim, w, h, d, t))
                                .with_seed(1337)
                                .with_distance_function(df)
                                .with_return_type(rt)
                                .wrap();
                            let noise = if dim == 1 {
                                $m::get_1d_noise_64::<$eng>(&nt).0
                            } else {
                                $m::get_4d_noise_64::<$eng>(&nt).0
                            };
                            let file_name = format!(
                                "{}/intrinsics_cellular2_64_{}_{}_{}_{}_{}.bin",
                                BIN_PATH, dim_str, $engname, dfn, rtn, ARCH
                            );
                            check(file_name, &noise);
                        }
                    }
                }
            }
            unsafe { run() }
        }
    };
}

cell_engine!(
    cellular_1d4d_64_scalar,
    scalar,
    simdeez::scalar::Scalar,
    "scalar"
);
cell_engine!(
    cellular_1d4d_64_sse2,
    sse2,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "sse2"
);
cell_engine!(
    cellular_1d4d_64_sse41,
    sse41,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "sse4.1"
);
cell_engine!(
    cellular_1d4d_64_avx2,
    avx2,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "avx2"
);

/// Correctness sanity on the scalar reference engine: every 1D/4D cellular
/// `Distance` value must be finite and non-negative. This catches gross kernel
/// bugs (NaN/inf, wrong metric) that a golden fixture would otherwise enshrine.
#[test]
fn cellular_1d4d_64_scalar_sane() {
    for (dim, _s, w, h, d, t) in DIMS {
        for (df, _) in CELL_DF {
            let nt = CellularSettings::default(dims_of(dim, w, h, d, t))
                .with_seed(1337)
                .with_distance_function(df)
                .with_return_type(CellReturnType::Distance)
                .wrap();
            let noise = unsafe {
                if dim == 1 {
                    scalar::get_1d_noise_64::<simdeez::scalar::Scalar>(&nt).0
                } else {
                    scalar::get_4d_noise_64::<simdeez::scalar::Scalar>(&nt).0
                }
            };
            assert!(noise.iter().all(|v| v.is_finite()), "finite dim={}", dim);
            assert!(noise.iter().all(|v| *v >= 0.0), "distance>=0 dim={}", dim);
        }
    }
}
