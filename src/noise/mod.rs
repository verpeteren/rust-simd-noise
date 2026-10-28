// Noise kernels are crate-internal implementation detail. The low-level public
// entry point is `crate::intrinsics`; the public enum modules below are
// re-exported at the crate root.
pub(crate) mod cell2_32;
pub(crate) mod cell2_64;
pub mod cell2_return_type;
pub(crate) mod cell_32;
pub(crate) mod cell_64;
pub mod cell_distance_function;
pub mod cell_return_type;
pub mod cell_strategy;
pub(crate) mod cellular_32;
pub(crate) mod cellular_64;
pub(crate) mod fbm_32;
pub(crate) mod fbm_64;
pub(crate) mod gabor_32;
pub(crate) mod gradient_32;
pub(crate) mod gradient_64;
pub(crate) mod hash3d_32;
pub(crate) mod hash3d_64;
pub(crate) mod ops;
pub(crate) mod ridge_32;
pub(crate) mod ridge_64;
pub(crate) mod simplex_32;
pub(crate) mod simplex_64;
pub(crate) mod steerable_perlin_32;
pub(crate) mod turbulence_32;
pub(crate) mod turbulence_64;
