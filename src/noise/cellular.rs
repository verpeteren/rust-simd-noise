//! Low-level cellular noise primitives
//!
//! Useful for writing your own SIMD-generic code for use cases not covered by the higher level
//! interfaces.

use std::f32;

use simdeez::prelude::*;

pub const BIT_10_MASK_32: i32 = 1023;
pub const BIT_10_MASK_64: i64 = 1023;
pub const HASH_2_FLOAT_32: f32 = 1.0 / 2147483648.0;
pub const HASH_2_FLOAT_64: f64 = 1.0 / 2147483648.0;

pub const X_PRIME_32: i32 = 1619;
pub const X_PRIME_64: i64 = 1619;

pub const Y_PRIME_32: i32 = 31337;
pub const Y_PRIME_64: i64 = 31337;

// It seems that b47708e459 had a typo and set this value to 6791.
// That seems to be a type (not 100 % sure though), because it is also a prime.
// This went unnoticed untill cce07cdc.
pub const Z_PRIME_32: i32 = 6971;
pub const Z_PRIME_64: i64 = 6971;

#[inline(always)]
pub unsafe fn hash_2d<S: Simd, I: SimdInt<Engine = S>>(seed: I::Scalar, x: I, y: I) -> I {
    let mut hash = x ^ I::set1(seed);
    hash = y ^ hash;
    hash * hash * hash * I::from_i64(60493)
}

#[inline(always)]
pub unsafe fn hash_3d<S: Simd, I: SimdInt<Engine = S>>(seed: I::Scalar, x: I, y: I, z: I) -> I {
    let mut hash = x ^ I::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash * hash * hash * I::from_i64(60493)
}
