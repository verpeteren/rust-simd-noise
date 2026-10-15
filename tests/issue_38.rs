//! Regression test for issue #38: stack overflow with 3D/4D noise under AVX2.
//!
//! The large 3D/4D simplex and cellular compute functions used to be
//! `#[inline(always)]`, which collapsed the whole computation into one stack
//! frame. Under AVX2 (32-byte vectors), in an unoptimized build, that single
//! frame exceeded a memory page / a small stack, tripping Windows' `__chkstk`
//! guard (and overflowing small stacks in general).
//!
//! On Linux `main` has an 8 MiB stack, so the crash does not show up there
//! directly. We reproduce the Windows (1 MiB main-thread) / Rust test-thread
//! (~2 MiB) condition by generating each noise inside a thread with a 1 MiB
//! stack. A stack overflow aborts the whole process (it cannot be caught), so a
//! regression turns these tests — and the test binary — red.
//!
//! Run with: `cargo test --test issue_38` (optionally `--release`).

use simdnoise::{NoiseBuilder, Settings};

/// Runs `f` on a 1 MiB stack (like the Windows main thread) and fails if it
/// cannot complete there. `black_box` keeps the optimizer from eliding the
/// otherwise-unused noise generation.
fn on_small_stack(f: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(1024 * 1024)
        .spawn(f)
        .expect("spawn thread")
        .join()
        .expect("noise generation overflowed a 1 MiB stack");
}

#[test]
fn gradient_3d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::gradient_3d(1, 1, 1).generate());
    });
}

#[test]
fn fbm_3d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::fbm_3d(1, 1, 1).generate());
    });
}

#[test]
fn ridge_3d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::ridge_3d(1, 1, 1).generate());
    });
}

#[test]
fn turbulence_3d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::turbulence_3d(1, 1, 1).generate());
    });
}

#[test]
fn cellular_3d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::cellular_3d(1, 1, 1).generate());
    });
}

#[test]
fn cellular2_3d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::cellular2_3d(1, 1, 1).generate());
    });
}

// 4D simplex is the largest frame of all.
#[test]
fn gradient_4d_fits_small_stack() {
    on_small_stack(|| {
        std::hint::black_box(NoiseBuilder::gradient_4d(1, 1, 1, 1).generate());
    });
}
