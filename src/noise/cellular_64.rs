use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn hash_2d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64) -> S::Vi64 {
    let mut hash = x ^ S::set1_epi64(seed);
    hash = y ^ hash;
    S::mullo_epi64(
        S::mullo_epi64(S::mullo_epi64(hash, hash), S::set1_epi64(60493)),
        hash,
    )
}

#[inline(always)]
pub unsafe fn hash_3d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64, z: S::Vi64) -> S::Vi64 {
    let mut hash = x ^ S::set1_epi64(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    S::mullo_epi64(
        S::mullo_epi64(S::mullo_epi64(hash, hash), S::set1_epi64(60493)),
        hash,
    )
}
