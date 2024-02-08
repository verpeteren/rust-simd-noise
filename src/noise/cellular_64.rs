use simdeez::prelude::*;

#[inline(always)]
pub fn hash_2d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64) -> S::Vi64 {
    let mut hash = x ^ S::Vi64::set1(seed);
    hash = y ^ hash;
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}

#[inline(always)]
pub fn hash_3d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64, z: S::Vi64) -> S::Vi64 {
    let mut hash = x ^ S::Vi64::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}
