//! White-box tests for the crate-internal noise kernels.
//!
//! These exercise the `pub(crate)` kernel modules under [`crate::noise`]
//! directly (raw per-precision kernels, not the public `intrinsics`
//! wrappers), so they must live inside the crate rather than in `tests/`.

mod cellular_5d6d;
mod fractal_5d6d;
mod fractional_octaves;
mod simplex_5d6d;
