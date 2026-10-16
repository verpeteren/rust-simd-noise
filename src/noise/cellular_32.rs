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

// 4th-axis prime for 4D (and higher) cellular noise. There is no reference
// implementation to match for 4D cellular, so this is simply a distinct prime.
pub const W_PRIME_32: i32 = 1013;
pub const W_PRIME_64: i64 = 1013;

// 5th- and 6th-axis primes for 5D/6D cellular noise (distinct primes, same
// rationale as `W_PRIME`).
pub const U_PRIME_32: i32 = 2053;
pub const U_PRIME_64: i64 = 2053;

pub const V_PRIME_32: i32 = 3079;
pub const V_PRIME_64: i64 = 3079;

#[inline(always)]
pub fn hash_2d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32) -> S::Vi32 {
    let mut hash = x ^ S::Vi32::set1(seed);
    hash = y ^ hash;
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}

#[inline(always)]
pub fn hash_3d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32, z: S::Vi32) -> S::Vi32 {
    let mut hash = x ^ S::Vi32::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}

#[inline(always)]
pub fn hash_1d<S: Simd>(seed: i32, x: S::Vi32) -> S::Vi32 {
    let hash = x ^ S::Vi32::set1(seed);
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}

#[inline(always)]
pub fn hash_4d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32, z: S::Vi32, w: S::Vi32) -> S::Vi32 {
    let mut hash = x ^ S::Vi32::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash = w ^ hash;
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn hash_5d<S: Simd>(
    seed: i32,
    x: S::Vi32,
    y: S::Vi32,
    z: S::Vi32,
    w: S::Vi32,
    u: S::Vi32,
) -> S::Vi32 {
    let mut hash = x ^ S::Vi32::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash = w ^ hash;
    hash = u ^ hash;
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn hash_6d<S: Simd>(
    seed: i32,
    x: S::Vi32,
    y: S::Vi32,
    z: S::Vi32,
    w: S::Vi32,
    u: S::Vi32,
    v: S::Vi32,
) -> S::Vi32 {
    let mut hash = x ^ S::Vi32::set1(seed);
    hash = y ^ hash;
    hash = z ^ hash;
    hash = w ^ hash;
    hash = u ^ hash;
    hash = v ^ hash;
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}

/// A second avalanche round, used to draw fresh jitter bits for the 4th (and
/// higher) axis: a 32-bit hash only has room for three 10-bit jitter fields
/// (bits 0-9/10-19/20-29), so axes beyond the third re-hash instead of shifting.
#[inline(always)]
pub fn rehash<S: Simd>(hash: S::Vi32) -> S::Vi32 {
    ((hash * hash) * S::Vi32::set1(60493)) * hash
}
