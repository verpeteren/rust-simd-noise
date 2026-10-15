use simdeez::prelude::*;

/// Raw avalanche-mixed 3D position hash. Used by the balanced 12-gradient LUT
/// (`gradient_32::grad3d_vec`) to pick a gradient index uniformly.
#[inline(always)]
pub fn hash3d_i32<S: Simd>(seed: i32, i: S::Vi32, j: S::Vi32, k: S::Vi32) -> S::Vi32 {
    let mut hash = i ^ S::Vi32::set1(seed);
    hash = j ^ hash;
    hash = k ^ hash;
    hash = ((hash * hash) * S::Vi32::set1(60493)) * hash;
    (hash >> 13) ^ hash
}
