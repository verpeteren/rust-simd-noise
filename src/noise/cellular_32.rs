//! Low-level cellular noise primitives
//!
//! Useful for writing your own SIMD-generic code for use cases not covered by the higher level
//! interfaces.

use std::f32;

use simdeez::Simd;

pub const BIT_10_MASK_32: i32 = 1023;
pub const BIT_10_MASK_64: i64 = 1023;
pub const HASH_2_FLOAT_32: f32 = 1.0 / 2147483648.0;
pub const HASH_2_FLOAT_64: f64 = 1.0 / 2147483648.0;
pub const X_PRIME_32: i32 = 1619;
pub const X_PRIME_64: i64 = 1619;
pub const Y_PRIME_32: i32 = 31337;
pub const Y_PRIME_64: i64 = 31337;
pub const Z_PRIME_32: i32 = 6971;
pub const Z_PRIME_64: i64 = 6971;

#[inline(always)]
pub unsafe fn hash_2d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32) -> S::Vi32 {
    let mut hash = S::xor_epi32(x, S::set1_epi32(seed));
    hash = S::xor_epi32(y, hash);
    S::mullo_epi32(
        S::mullo_epi32(S::mullo_epi32(hash, hash), S::set1_epi32(60493)),
        hash,
    )
}

#[inline(always)]
pub unsafe fn hash_3d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32, z: S::Vi32) -> S::Vi32 {
    let mut hash = S::xor_epi32(x, S::set1_epi32(seed));
    hash = S::xor_epi32(y, hash);
    hash = S::xor_epi32(z, hash);
    S::mullo_epi32(
        S::mullo_epi32(S::mullo_epi32(hash, hash), S::set1_epi32(60493)),
        hash,
    )
}
