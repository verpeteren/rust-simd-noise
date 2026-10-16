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

#[inline(always)]
pub fn hash_1d<S: Simd>(seed: i64, x: S::Vi64) -> S::Vi64 {
    let hash = x ^ S::Vi64::set1(seed);
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}

#[inline(always)]
pub fn hash_4d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64, z: S::Vi64, w: S::Vi64) -> S::Vi64 {
    let mut hash = x ^ S::Vi64::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash = w ^ hash;
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn hash_5d<S: Simd>(
    seed: i64,
    x: S::Vi64,
    y: S::Vi64,
    z: S::Vi64,
    w: S::Vi64,
    u: S::Vi64,
) -> S::Vi64 {
    let mut hash = x ^ S::Vi64::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash = w ^ hash;
    hash = u ^ hash;
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn hash_6d<S: Simd>(
    seed: i64,
    x: S::Vi64,
    y: S::Vi64,
    z: S::Vi64,
    w: S::Vi64,
    u: S::Vi64,
    v: S::Vi64,
) -> S::Vi64 {
    let mut hash = x ^ S::Vi64::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash = w ^ hash;
    hash = u ^ hash;
    hash = v ^ hash;
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}

/// A second avalanche round for the 4th (and higher) axis jitter bits. The f64
/// kernels have bits to spare, but re-hashing keeps the 32-bit and 64-bit 4D
/// kernels structurally identical (and generalises to 5D/6D).
#[inline(always)]
pub fn rehash<S: Simd>(hash: S::Vi64) -> S::Vi64 {
    ((hash * hash) * S::Vi64::set1(60493)) * hash
}
