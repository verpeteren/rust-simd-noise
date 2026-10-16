//! Low-level simplex noise primitives
//!
//! Useful for writing your own SIMD-generic code for use cases not covered by the higher level
//! interfaces.

use crate::noise::cellular_32::{X_PRIME_32, Y_PRIME_32, Z_PRIME_32};
use crate::noise::gradient_32::{grad1, grad2, grad3d, grad3d_dot, grad4, grad5};
use crate::noise::ops::gather_32;

use simdeez::prelude::*;

use std::f32;
use std::f64;

/// Skew factor for 2D simplex noise
const F2_32: f32 = 0.366_025_42;
pub const F2_64: f64 = 0.36602540378;
/// Skew factor for 3D simplex noise
const F3_32: f32 = 1.0 / 3.0;
pub const F3_64: f64 = 1.0 / 3.0;
/// Skew factor for 4D simplex noise
const F4_32: f32 = 0.309_017;
pub const F4_64: f64 = 0.309016994;
/// Unskew factor for 2D simplex noise
const G2_32: f32 = 0.211_324_87;
pub const G2_64: f64 = 0.2113248654;
const G22_32: f32 = G2_32 * 2.0;
pub const G22_64: f64 = G2_64 * 2.0;
/// Unskew factor for 3D simplex noise
const G3_32: f32 = 1.0 / 6.0;
pub const G3_64: f64 = 1.0 / 6.0;
const G33_32: f32 = 3.0 / 6.0 - 1.0;
pub const G33_64: f64 = 3.0 / 6.0 - 1.0;
/// Unskew factor for 4D simplex noise
const G4_32: f32 = 0.138_196_6;
pub const G4_64: f64 = 0.138196601;
const G24_32: f32 = 2.0 * G4_32;
pub const G24_64: f64 = 2.0 * G4_64;
const G34_32: f32 = 3.0 * G4_32;
pub const G34_64: f64 = 3.0 * G4_64;
const G44_32: f32 = 4.0 * G4_32;
pub const G44_64: f64 = 4.0 * G4_64;
/// Skew factor for 5D simplex noise: `(sqrt(6) - 1) / 5`.
const F5_32: f32 = 0.289_897_95;
pub const F5_64: f64 = 0.2898979485566356;
/// Unskew factor for 5D simplex noise: `(1 - 1/sqrt(6)) / 5`.
const G5_32: f32 = 0.118_350_34;
pub const G5_64: f64 = 0.11835034190722738;
const G25_32: f32 = 2.0 * G5_32;
pub const G25_64: f64 = 2.0 * G5_64;
const G35_32: f32 = 3.0 * G5_32;
pub const G35_64: f64 = 3.0 * G5_64;
const G45_32: f32 = 4.0 * G5_32;
pub const G45_64: f64 = 4.0 * G5_64;
const G55_32: f32 = 5.0 * G5_32;
pub const G55_64: f64 = 5.0 * G5_64;

static PERM: [i32; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

#[inline(always)]
fn assert_in_perm_range<S: Simd>(values: S::Vi32) {
    debug_assert!(values
        .cmp_lt(S::Vi32::set1(PERM.len() as i32))
        .iter()
        .all(|is_less_than| is_less_than != 0));
}

/// Like `simplex_1d`, but also computes the derivative
#[inline(always)]
pub fn simplex_1d_deriv<S: Simd>(x: S::Vf32, seed: i32) -> (S::Vf32, S::Vf32) {
    // Gradients are selected deterministically based on the whole part of `x`
    let ips = x.fast_floor();
    let mut i0 = ips.cast_i32();
    let i1 = (i0 + S::Vi32::set1(1)) & S::Vi32::set1(0xff);

    // the fractional part of x, i.e. the distance to the left gradient node. 0 ≤ x0 < 1.
    let x0 = x - ips;
    // signed distance to the right gradient node
    let x1 = x0 - S::Vf32::set1(1.0);

    i0 &= S::Vi32::set1(0xff);
    let (gi0, gi1) = unsafe {
        // Safety: We just masked i0 and i1 with 0xff, so they're in 0..255.
        let gi0 = gather_32::<S>(&PERM, i0);
        let gi1 = gather_32::<S>(&PERM, i1);
        (gi0, gi1)
    };

    // Compute the contribution from the first gradient
    let x20 = x0 * x0; // x^2_0
    let t0 = S::Vf32::set1(1.0) - x20; // t_0
    let t20 = t0 * t0; // t^2_0
    let t40 = t20 * t20; // t^4_0
    let gx0 = grad1::<S>(seed, gi0);
    let n0 = t40 * gx0 * x0;
    // n0 = (1 - x0^2)^4 * x0 * grad

    // Compute the contribution from the second gradient
    let x21 = x1 * x1; // x^2_1
    let t1 = S::Vf32::set1(1.0) - x21; // t_1
    let t21 = t1 * t1; // t^2_1
    let t41 = t21 * t21; // t^4_1
    let gx1 = grad1::<S>(seed, gi1);
    let n1 = t41 * gx1 * x1;

    // n0 + n1 =
    //    grad0 * x0 * (1 - x0^2)^4
    //  + grad1 * (x0 - 1) * (1 - (x0 - 1)^2)^4
    //
    // Assuming worst-case values for grad0 and grad1, we therefore need only determine the maximum of
    //
    // |x0 * (1 - x0^2)^4| + |(x0 - 1) * (1 - (x0 - 1)^2)^4|
    //
    // for 0 ≤ x0 < 1. This can be done by root-finding on the derivative, obtaining 81 / 256 when
    // x0 = 0.5, which we finally multiply by the maximum gradient to get the maximum value,
    // allowing us to scale into [-1, 1]
    const SCALE: f32 = 256.0 / (81.0 * 7.0);

    let value = (n0 + n1) * S::Vf32::set1(SCALE);
    let derivative = ((t20 * t0 * gx0 * x20 + t21 * t1 * gx1 * x21) * S::Vf32::set1(-8.0)
        + t40 * gx0
        + t41 * gx1)
        * S::Vf32::set1(SCALE);
    (value, derivative)
}

/// Samples 1-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub fn simplex_1d<S: Simd>(x: S::Vf32, seed: i32) -> S::Vf32 {
    simplex_1d_deriv::<S>(x, seed).0
}

/// Samples 2-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub fn simplex_2d<S: Simd>(x: S::Vf32, y: S::Vf32, seed: i32) -> S::Vf32 {
    simplex_2d_deriv::<S>(x, y, seed).0
}

/// Like `simplex_2d`, but also computes the derivative
// Not `#[inline(always)]`: like the 3D/4D paths (issue #38), force-inlining this
// large function builds one oversized stack frame that overflows small stacks
// under AVX2. `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
pub fn simplex_2d_deriv<S: Simd>(x: S::Vf32, y: S::Vf32, seed: i32) -> (S::Vf32, [S::Vf32; 2]) {
    // Skew to distort simplexes with side length sqrt(2)/sqrt(3) until they make up
    // squares
    let s = S::Vf32::set1(F2_32) * (x + y);
    let ips = (x + s).floor();
    let jps = (y + s).floor();

    // Integer coordinates for the base vertex of the triangle
    let i = ips.cast_i32();
    let j = jps.cast_i32();

    let t = (i + j).cast_f32() * S::Vf32::set1(G2_32);

    // Unskewed distances to the first point of the enclosing simplex
    let x0 = x - (ips - t);
    let y0 = y - (jps - t);

    let i1 = (x0.cmp_gte(y0)).bitcast_i32();

    let j1 = (y0.cmp_gt(x0)).bitcast_i32();

    // Distances to the second and third points of the enclosing simplex
    let x1 = (x0 + i1.cast_f32()) + S::Vf32::set1(G2_32);
    let y1 = (y0 + j1.cast_f32()) + S::Vf32::set1(G2_32);
    let x2 = (x0 + S::Vf32::set1(-1.0)) + S::Vf32::set1(G22_32);
    let y2 = (y0 + S::Vf32::set1(-1.0)) + S::Vf32::set1(G22_32);

    let ii = i & S::Vi32::set1(0xff);
    let jj = j & S::Vi32::set1(0xff);

    let (gi0, gi1, gi2) = unsafe {
        assert_in_perm_range::<S>(ii);
        assert_in_perm_range::<S>(jj);
        assert_in_perm_range::<S>(ii - i1);
        assert_in_perm_range::<S>(jj - j1);
        assert_in_perm_range::<S>(ii + 1);
        assert_in_perm_range::<S>(jj + 1);

        let gi0 = gather_32::<S>(&PERM, ii + gather_32::<S>(&PERM, jj));
        let gi1 = gather_32::<S>(&PERM, (ii - i1) + gather_32::<S>(&PERM, jj - j1));
        let gi2 = gather_32::<S>(
            &PERM,
            (ii - S::Vi32::set1(-1)) + gather_32::<S>(&PERM, jj - S::Vi32::set1(-1)),
        );

        (gi0, gi1, gi2)
    };

    // Weights associated with the gradients at each corner
    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let mut t0 = S::Vf32::neg_mul_add(y0, y0, S::Vf32::neg_mul_add(x0, x0, S::Vf32::set1(0.5)));
    let mut t1 = S::Vf32::neg_mul_add(y1, y1, S::Vf32::neg_mul_add(x1, x1, S::Vf32::set1(0.5)));
    let mut t2 = S::Vf32::neg_mul_add(y2, y2, S::Vf32::neg_mul_add(x2, x2, S::Vf32::set1(0.5)));

    // Zero out negative weights
    t0 &= t0.cmp_gte(S::Vf32::zeroes());
    t1 &= t1.cmp_gte(S::Vf32::zeroes());
    t2 &= t2.cmp_gte(S::Vf32::zeroes());

    let t20 = t0 * t0;
    let t40 = t20 * t20;
    let t21 = t1 * t1;
    let t41 = t21 * t21;
    let t22 = t2 * t2;
    let t42 = t22 * t22;

    let [gx0, gy0] = grad2::<S>(seed, gi0);
    let g0 = gx0 * x0 + gy0 * y0;
    let n0 = t40 * g0;
    let [gx1, gy1] = grad2::<S>(seed, gi1);
    let g1 = gx1 * x1 + gy1 * y1;
    let n1 = t41 * g1;
    let [gx2, gy2] = grad2::<S>(seed, gi2);
    let g2 = gx2 * x2 + gy2 * y2;
    let n2 = t42 * g2;

    // Scaling factor found by numerical approximation
    let scale = S::Vf32::set1(45.264_507);
    let value = (n0 + (n1 + n2)) * scale;
    let derivative = {
        let temp0 = t20 * t0 * g0;
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let temp1 = t21 * t1 * g1;
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        let temp2 = t22 * t2 * g2;
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dx *= S::Vf32::set1(-8.0);
        dnoise_dy *= S::Vf32::set1(-8.0);
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2;
        dnoise_dx *= scale;
        dnoise_dy *= scale;
        [dnoise_dx, dnoise_dy]
    };
    (value, derivative)
}

/// Samples 3-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub fn simplex_3d<S: Simd>(x: S::Vf32, y: S::Vf32, z: S::Vf32, seed: i32) -> S::Vf32 {
    simplex_3d_deriv::<S>(x, y, z, seed).0
}

/// Like `simplex_3d`, but also computes the derivative
// Not `#[inline(always)]`: force-inlining this large function into the noise
// generation loop builds one huge stack frame that overflows small stacks under
// AVX2 (issue #38). `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
pub fn simplex_3d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 3]) {
    // ImproveXY domain rotation (issue #15): rotate the input so that world-z
    // lands on the lattice (1,1,1) diagonal. This removes the simplex skew's
    // in-plane projection for constant-z slices, so XY/z-slices become
    // isotropic cross-sections (the anisotropy moves to the XZ/YZ planes, which
    // is the desired trade-off for z-slice / animation use). Pure rotation, so
    // the noise statistics are unchanged.
    let xy = x + y;
    let s2 = xy * S::Vf32::set1(-0.21132487);
    let zz = z * S::Vf32::set1(0.57735026);
    let x = x + s2 + zz;
    let y = y + s2 + zz;
    let z = xy * S::Vf32::set1(-0.57735026) + zz;

    // Find skewed simplex grid coordinates associated with the input coordinates
    let f = S::Vf32::set1(F3_32) * ((x + y) + z);
    let mut x0 = (x + f).fast_floor();
    let mut y0 = (y + f).fast_floor();
    let mut z0 = (z + f).fast_floor();

    // Integer grid coordinates
    let i = x0.cast_i32() * S::Vi32::set1(X_PRIME_32);
    let j = y0.cast_i32() * S::Vi32::set1(Y_PRIME_32);
    let k = z0.cast_i32() * S::Vi32::set1(Z_PRIME_32);

    // Compute distance from first simplex vertex to input coordinates
    let g = S::Vf32::set1(G3_32) * ((x0 + y0) + z0);
    x0 = x - (x0 - g);
    y0 = y - (y0 - g);
    z0 = z - (z0 - g);

    let x0_ge_y0 = x0.cmp_gte(y0);
    let y0_ge_z0 = y0.cmp_gte(z0);
    let x0_ge_z0 = x0.cmp_gte(z0);

    let i1 = x0_ge_y0 & x0_ge_z0;
    let j1 = y0_ge_z0.and_not(x0_ge_y0);
    let k1 = (!y0_ge_z0).and_not(x0_ge_z0);

    let i2 = x0_ge_y0 | x0_ge_z0;
    let j2 = (!x0_ge_y0) | y0_ge_z0;
    let k2 = !(x0_ge_z0 & y0_ge_z0);

    // Compute distances from remaining simplex vertices to input coordinates
    let x1 = x0 - (i1 & S::Vf32::set1(1.0)) + S::Vf32::set1(G3_32);
    let y1 = y0 - (j1 & S::Vf32::set1(1.0)) + S::Vf32::set1(G3_32);
    let z1 = z0 - (k1 & S::Vf32::set1(1.0)) + S::Vf32::set1(G3_32);

    let x2 = x0 - (i2 & S::Vf32::set1(1.0)) + S::Vf32::set1(F3_32);
    let y2 = y0 - (j2 & S::Vf32::set1(1.0)) + S::Vf32::set1(F3_32);
    let z2 = z0 - (k2 & S::Vf32::set1(1.0)) + S::Vf32::set1(F3_32);

    let x3 = x0 + S::Vf32::set1(G33_32);
    let y3 = y0 + S::Vf32::set1(G33_32);
    let z3 = z0 + S::Vf32::set1(G33_32);

    // Compute base weight factors associated with each vertex, `0.6 - v . v` where v is the
    // distance to the vertex. Strictly the constant should be 0.5, but 0.6 is thought by Gustavson
    // to give visually better results at the cost of subtle discontinuities.
    //#define SIMDf_NMUL_ADD(a,b,c) = SIMDf_SUB(c, SIMDf_MUL(a,b)
    let mut t0 = S::Vf32::set1(0.6) - (x0 * x0) - (y0 * y0) - (z0 * z0);
    let mut t1 = S::Vf32::set1(0.6) - (x1 * x1) - (y1 * y1) - (z1 * z1);
    let mut t2 = S::Vf32::set1(0.6) - (x2 * x2) - (y2 * y2) - (z2 * z2);
    let mut t3 = S::Vf32::set1(0.6) - (x3 * x3) - (y3 * y3) - (z3 * z3);

    // Zero out negative weights
    t0 &= t0.cmp_gte(S::Vf32::zeroes());
    t1 &= t1.cmp_gte(S::Vf32::zeroes());
    t2 &= t2.cmp_gte(S::Vf32::zeroes());
    t3 &= t3.cmp_gte(S::Vf32::zeroes());

    // Square each weight
    let t20 = t0 * t0;
    let t21 = t1 * t1;
    let t22 = t2 * t2;
    let t23 = t3 * t3;

    // ...twice!
    let t40 = t20 * t20;
    let t41 = t21 * t21;
    let t42 = t22 * t22;
    let t43 = t23 * t23;

    //#define SIMDf_MASK_ADD(m,a,b) SIMDf_ADD(a,SIMDf_AND(SIMDf_CAST_TO_FLOAT(m),b))

    // Compute contribution from each vertex
    let g0 = grad3d_dot::<S>(seed, i, j, k, x0, y0, z0);
    let v0 = t40 * g0;

    let v1x = i + (i1.bitcast_i32() & S::Vi32::set1(X_PRIME_32));
    let v1y = j + (j1.bitcast_i32() & S::Vi32::set1(Y_PRIME_32));
    let v1z = k + (k1.bitcast_i32() & S::Vi32::set1(Z_PRIME_32));
    let g1 = grad3d_dot::<S>(seed, v1x, v1y, v1z, x1, y1, z1);
    let v1 = t41 * g1;

    let v2x = i + (i2.bitcast_i32() & S::Vi32::set1(X_PRIME_32));
    let v2y = j + (j2.bitcast_i32() & S::Vi32::set1(Y_PRIME_32));
    let v2z = k + (k2.bitcast_i32() & S::Vi32::set1(Z_PRIME_32));
    let g2 = grad3d_dot::<S>(seed, v2x, v2y, v2z, x2, y2, z2);
    let v2 = t42 * g2;

    //SIMDf v3 = SIMDf_MASK(n3, SIMDf_MUL(SIMDf_MUL(t3, t3), FUNC(GradCoord)(seed, SIMDi_ADD(i, SIMDi_NUM(xPrime)), SIMDi_ADD(j, SIMDi_NUM(yPrime)), SIMDi_ADD(k, SIMDi_NUM(zPrime)), x3, y3, z3)));
    let v3x = i + S::Vi32::set1(X_PRIME_32);
    let v3y = j + S::Vi32::set1(Y_PRIME_32);
    let v3z = k + S::Vi32::set1(Z_PRIME_32);
    //define SIMDf_MASK(m,a) SIMDf_AND(SIMDf_CAST_TO_FLOAT(m),a)
    let g3 = grad3d_dot::<S>(seed, v3x, v3y, v3z, x3, y3, z3);
    let v3 = t43 * g3;

    let p1 = v3 + v2;
    let p2 = p1 + v1;

    // Scaling factor found by numerical approximation
    let scale = S::Vf32::set1(32.695_873);
    let result = (p2 + v0) * scale;
    let derivative = {
        let temp0 = t20 * t0 * g0;
        let mut dnoise_dx = temp0 * x0;
        let mut dnoise_dy = temp0 * y0;
        let mut dnoise_dz = temp0 * z0;
        let temp1 = t21 * t1 * g1;
        dnoise_dx += temp1 * x1;
        dnoise_dy += temp1 * y1;
        dnoise_dz += temp1 * z1;
        let temp2 = t22 * t2 * g2;
        dnoise_dx += temp2 * x2;
        dnoise_dy += temp2 * y2;
        dnoise_dz += temp2 * z2;
        let temp3 = t23 * t3 * g3;
        dnoise_dx += temp3 * x3;
        dnoise_dy += temp3 * y3;
        dnoise_dz += temp3 * z3;
        dnoise_dx *= S::Vf32::set1(-8.0);
        dnoise_dy *= S::Vf32::set1(-8.0);
        dnoise_dz *= S::Vf32::set1(-8.0);
        let [gx0, gy0, gz0] = grad3d::<S>(seed, i, j, k);
        let [gx1, gy1, gz1] = grad3d::<S>(seed, v1x, v1y, v1z);
        let [gx2, gy2, gz2] = grad3d::<S>(seed, v2x, v2y, v2z);
        let [gx3, gy3, gz3] = grad3d::<S>(seed, v3x, v3y, v3z);
        dnoise_dx += t40 * gx0 + t41 * gx1 + t42 * gx2 + t43 * gx3;
        dnoise_dy += t40 * gy0 + t41 * gy1 + t42 * gy2 + t43 * gy3;
        dnoise_dz += t40 * gz0 + t41 * gz1 + t42 * gz2 + t43 * gz3;
        // Scale into range
        dnoise_dx *= scale;
        dnoise_dy *= scale;
        dnoise_dz *= scale;
        // The derivative above is with respect to the ImproveXY-rotated coords
        // (issue #15). Rotate it back to the original input coords with the
        // rotation's transpose so the returned gradient matches the value's
        // dependence on the caller's (x, y, z).
        let a = S::Vf32::set1(-0.21132487);
        let b = S::Vf32::set1(0.57735026);
        let c = S::Vf32::set1(-0.57735026);
        let one_a = S::Vf32::set1(0.7886751); // 1 + a
        [
            one_a * dnoise_dx + a * dnoise_dy + c * dnoise_dz,
            a * dnoise_dx + one_a * dnoise_dy + c * dnoise_dz,
            b * dnoise_dx + b * dnoise_dy + b * dnoise_dz,
        ]
    };
    (result, derivative)
}

/// Samples 4-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
// Not `#[inline(always)]`: see `simplex_3d_deriv` (issue #38).
#[inline]
pub fn simplex_4d<S: Simd>(x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, seed: i32) -> S::Vf32 {
    // ImproveXYZ domain rotation (issue #15): a pure orthonormal rotation that
    // maps world-w onto the lattice (1,1,1,1) diagonal, with world-x/y/z spanning
    // the orthogonal 3-space. Constant-w slices (and the xy/xz/yz/xyz slices
    // within them) then become isotropic cross-sections, so the diagonal streak
    // disappears from them; the anisotropy lives only along w. Same trade-off as
    // the 3D ImproveXY rotation, which is correct for slice / animation use.
    let xr = S::Vf32::set1(0.707_106_77) * x
        + S::Vf32::set1(0.408_248_3) * y
        + S::Vf32::set1(0.288_675_14) * z
        + S::Vf32::set1(0.5) * w;
    let yr = S::Vf32::set1(-0.707_106_77) * x
        + S::Vf32::set1(0.408_248_3) * y
        + S::Vf32::set1(0.288_675_14) * z
        + S::Vf32::set1(0.5) * w;
    let zr =
        S::Vf32::set1(-0.816_496_6) * y + S::Vf32::set1(0.288_675_14) * z + S::Vf32::set1(0.5) * w;
    let wr = S::Vf32::set1(-0.866_025_4) * z + S::Vf32::set1(0.5) * w;
    let (x, y, z, w) = (xr, yr, zr, wr);
    //
    // Determine which simplex these points lie in, and compute the distance along each axis to each
    // vertex of the simplex
    //

    let s = S::Vf32::set1(F4_32) * (x + y + z + w);

    let ips = (x + s).floor();
    let jps = (y + s).floor();
    let kps = (z + s).floor();
    let lps = (w + s).floor();

    let i = ips.cast_i32();
    let j = jps.cast_i32();
    let k = kps.cast_i32();
    let l = lps.cast_i32();

    let t = (i + j + k + l).cast_f32() * S::Vf32::set1(G4_32);
    let x0 = x - (ips - t);
    let y0 = y - (jps - t);
    let z0 = z - (kps - t);
    let w0 = w - (lps - t);

    let mut rank_x = S::Vi32::zeroes();
    let mut rank_y = S::Vi32::zeroes();
    let mut rank_z = S::Vi32::zeroes();
    let mut rank_w = S::Vi32::zeroes();

    let cond = (x0.cmp_gt(y0)).bitcast_i32();
    rank_x += cond & S::Vi32::set1(1);
    rank_y += S::Vi32::set1(1).and_not(cond);
    let cond = (x0.cmp_gt(z0)).bitcast_i32();
    rank_x += cond & S::Vi32::set1(1);
    rank_z += S::Vi32::set1(1).and_not(cond);
    let cond = (x0.cmp_gt(w0)).bitcast_i32();
    rank_x += cond & S::Vi32::set1(1);
    rank_w += S::Vi32::set1(1).and_not(cond);
    let cond = (y0.cmp_gt(z0)).bitcast_i32();
    rank_y += cond & S::Vi32::set1(1);
    rank_z += S::Vi32::set1(1).and_not(cond);
    let cond = (y0.cmp_gt(w0)).bitcast_i32();
    rank_y += cond & S::Vi32::set1(1);
    rank_w += S::Vi32::set1(1).and_not(cond);
    let cond = (z0.cmp_gt(w0)).bitcast_i32();
    rank_z += cond & S::Vi32::set1(1);
    rank_w += S::Vi32::set1(1).and_not(cond);

    let cond = rank_x.cmp_gt(S::Vi32::set1(2));
    let i1 = S::Vi32::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::Vi32::set1(2));
    let j1 = S::Vi32::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::Vi32::set1(2));
    let k1 = S::Vi32::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::Vi32::set1(2));
    let l1 = S::Vi32::set1(1) & cond;

    let cond = rank_x.cmp_gt(S::Vi32::set1(1));
    let i2 = S::Vi32::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::Vi32::set1(1));
    let j2 = S::Vi32::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::Vi32::set1(1));
    let k2 = S::Vi32::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::Vi32::set1(1));
    let l2 = S::Vi32::set1(1) & cond;

    let cond = rank_x.cmp_gt(S::Vi32::zeroes());
    let i3 = S::Vi32::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::Vi32::zeroes());
    let j3 = S::Vi32::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::Vi32::zeroes());
    let k3 = S::Vi32::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::Vi32::zeroes());
    let l3 = S::Vi32::set1(1) & cond;

    let x1 = x0 - i1.cast_f32() + S::Vf32::set1(G4_32);
    let y1 = y0 - j1.cast_f32() + S::Vf32::set1(G4_32);
    let z1 = z0 - k1.cast_f32() + S::Vf32::set1(G4_32);
    let w1 = w0 - l1.cast_f32() + S::Vf32::set1(G4_32);
    let x2 = x0 - i2.cast_f32() + S::Vf32::set1(G24_32);
    let y2 = y0 - j2.cast_f32() + S::Vf32::set1(G24_32);
    let z2 = z0 - k2.cast_f32() + S::Vf32::set1(G24_32);
    let w2 = w0 - l2.cast_f32() + S::Vf32::set1(G24_32);
    let x3 = x0 - i3.cast_f32() + S::Vf32::set1(G34_32);
    let y3 = y0 - j3.cast_f32() + S::Vf32::set1(G34_32);
    let z3 = z0 - k3.cast_f32() + S::Vf32::set1(G34_32);
    let w3 = w0 - l3.cast_f32() + S::Vf32::set1(G34_32);
    let x4 = x0 - S::Vf32::set1(1.0) + S::Vf32::set1(G44_32);
    let y4 = y0 - S::Vf32::set1(1.0) + S::Vf32::set1(G44_32);
    let z4 = z0 - S::Vf32::set1(1.0) + S::Vf32::set1(G44_32);
    let w4 = w0 - S::Vf32::set1(1.0) + S::Vf32::set1(G44_32);

    let ii = i & S::Vi32::set1(0xff);
    let jj = j & S::Vi32::set1(0xff);
    let kk = k & S::Vi32::set1(0xff);
    let ll = l & S::Vi32::set1(0xff);

    let (gi0, gi1, gi2, gi3, gi4) = unsafe {
        // Safety: ii, jj, kk, and ll are all 0..255. All other temporary variables were fetched from PERM, which only
        // contains elements in the range 0..255.
        let lp = gather_32::<S>(&PERM, ll);
        let kp = gather_32::<S>(&PERM, kk + lp);
        let jp = gather_32::<S>(&PERM, jj + kp);
        let gi0 = gather_32::<S>(&PERM, ii + jp);

        let lp = gather_32::<S>(&PERM, ll + l1);
        let kp = gather_32::<S>(&PERM, kk + k1 + lp);
        let jp = gather_32::<S>(&PERM, jj + j1 + kp);
        let gi1 = gather_32::<S>(&PERM, ii + i1 + jp);

        let lp = gather_32::<S>(&PERM, ll + l2);
        let kp = gather_32::<S>(&PERM, kk + k2 + lp);
        let jp = gather_32::<S>(&PERM, jj + j2 + kp);
        let gi2 = gather_32::<S>(&PERM, ii + i2 + jp);

        let lp = gather_32::<S>(&PERM, ll + l3);
        let kp = gather_32::<S>(&PERM, kk + k3 + lp);
        let jp = gather_32::<S>(&PERM, jj + j3 + kp);
        let gi3 = gather_32::<S>(&PERM, ii + i3 + jp);

        let lp = gather_32::<S>(&PERM, ll + S::Vi32::set1(1));
        let kp = gather_32::<S>(&PERM, kk + S::Vi32::set1(1) + lp);
        let jp = gather_32::<S>(&PERM, jj + S::Vi32::set1(1) + kp);
        let gi4 = gather_32::<S>(&PERM, ii + S::Vi32::set1(1) + jp);
        (gi0, gi1, gi2, gi3, gi4)
    };

    //
    // Compute base weight factors associated with each vertex
    //

    let t0 = S::Vf32::set1(0.5) - (x0 * x0) - (y0 * y0) - (z0 * z0) - (w0 * w0);
    let t1 = S::Vf32::set1(0.5) - (x1 * x1) - (y1 * y1) - (z1 * z1) - (w1 * w1);
    let t2 = S::Vf32::set1(0.5) - (x2 * x2) - (y2 * y2) - (z2 * z2) - (w2 * w2);
    let t3 = S::Vf32::set1(0.5) - (x3 * x3) - (y3 * y3) - (z3 * z3) - (w3 * w3);
    let t4 = S::Vf32::set1(0.5) - (x4 * x4) - (y4 * y4) - (z4 * z4) - (w4 * w4);
    // Cube each weight
    let mut t0q = t0 * t0;
    t0q = t0q * t0q;
    let mut t1q = t1 * t1;
    t1q = t1q * t1q;
    let mut t2q = t2 * t2;
    t2q = t2q * t2q;
    let mut t3q = t3 * t3;
    t3q = t3q * t3q;
    let mut t4q = t4 * t4;
    t4q = t4q * t4q;

    let mut n0 = t0q * grad4::<S>(seed, gi0, x0, y0, z0, w0);
    let mut n1 = t1q * grad4::<S>(seed, gi1, x1, y1, z1, w1);
    let mut n2 = t2q * grad4::<S>(seed, gi2, x2, y2, z2, w2);
    let mut n3 = t3q * grad4::<S>(seed, gi3, x3, y3, z3, w3);
    let mut n4 = t4q * grad4::<S>(seed, gi4, x4, y4, z4, w4);

    // Discard contributions whose base weight factors are negative
    let mut cond = t0.cmp_lt(S::Vf32::zeroes());
    n0 = n0.and_not(cond);
    cond = t1.cmp_lt(S::Vf32::zeroes());
    n1 = n1.and_not(cond);
    cond = t2.cmp_lt(S::Vf32::zeroes());
    n2 = n2.and_not(cond);
    cond = t3.cmp_lt(S::Vf32::zeroes());
    n3 = n3.and_not(cond);
    cond = t4.cmp_lt(S::Vf32::zeroes());
    n4 = n4.and_not(cond);

    // Scaling factor found by numerical approximation
    (n0 + n1 + n2 + n3 + n4) * S::Vf32::set1(62.777_72)
}

/// Empirical normalisation for 5D simplex (calibrated so output lands in
/// `[-1, 1]`; there is no reference implementation to match).
const SIMPLEX_5D_SCALE_32: f32 = 54.5;

/// 5D simplex noise. A from-scratch generalisation of `simplex_4d` (no reference
/// implementation exists for 5D). Plain simplex — the ImproveXYZ-style domain
/// rotation that 2D/3D/4D apply is intentionally omitted (a future improvement
/// could add an N-D rotation for slice isotropy). Uses skew/unskew `F5`/`G5`, a
/// 10-comparison rank ordering selecting the 6 simplex corners, the 80-gradient
/// `grad5`, and the `r² = 0.5` kernel (as 4D).
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn simplex_5d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    u: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let s = S::Vf32::set1(F5_32) * (x + y + z + w + u);
    let ips = (x + s).floor();
    let jps = (y + s).floor();
    let kps = (z + s).floor();
    let lps = (w + s).floor();
    let mps = (u + s).floor();

    let i = ips.cast_i32();
    let j = jps.cast_i32();
    let k = kps.cast_i32();
    let l = lps.cast_i32();
    let m = mps.cast_i32();

    let t = (i + j + k + l + m).cast_f32() * S::Vf32::set1(G5_32);
    let x0 = x - (ips - t);
    let y0 = y - (jps - t);
    let z0 = z - (kps - t);
    let w0 = w - (lps - t);
    let v0 = u - (mps - t);

    // Rank each coordinate by how many of the others it exceeds (0..=4), via the
    // 10 pairwise comparisons.
    let mut rank_x = S::Vi32::zeroes();
    let mut rank_y = S::Vi32::zeroes();
    let mut rank_z = S::Vi32::zeroes();
    let mut rank_w = S::Vi32::zeroes();
    let mut rank_v = S::Vi32::zeroes();
    let one = S::Vi32::set1(1);
    macro_rules! cmp_rank {
        ($a0:expr, $ra:ident, $b0:expr, $rb:ident) => {{
            let cond = ($a0.cmp_gt($b0)).bitcast_i32();
            $ra += cond & one;
            $rb += one.and_not(cond);
        }};
    }
    cmp_rank!(x0, rank_x, y0, rank_y);
    cmp_rank!(x0, rank_x, z0, rank_z);
    cmp_rank!(x0, rank_x, w0, rank_w);
    cmp_rank!(x0, rank_x, v0, rank_v);
    cmp_rank!(y0, rank_y, z0, rank_z);
    cmp_rank!(y0, rank_y, w0, rank_w);
    cmp_rank!(y0, rank_y, v0, rank_v);
    cmp_rank!(z0, rank_z, w0, rank_w);
    cmp_rank!(z0, rank_z, v0, rank_v);
    cmp_rank!(w0, rank_w, v0, rank_v);

    // Corner c (1..=4) increments every axis whose rank exceeds `5-1-c`.
    let offs = |rank: S::Vi32, thresh: i32| -> S::Vi32 { one & rank.cmp_gt(S::Vi32::set1(thresh)) };
    let (i1, j1, k1, l1, m1) = (
        offs(rank_x, 3),
        offs(rank_y, 3),
        offs(rank_z, 3),
        offs(rank_w, 3),
        offs(rank_v, 3),
    );
    let (i2, j2, k2, l2, m2) = (
        offs(rank_x, 2),
        offs(rank_y, 2),
        offs(rank_z, 2),
        offs(rank_w, 2),
        offs(rank_v, 2),
    );
    let (i3, j3, k3, l3, m3) = (
        offs(rank_x, 1),
        offs(rank_y, 1),
        offs(rank_z, 1),
        offs(rank_w, 1),
        offs(rank_v, 1),
    );
    let (i4, j4, k4, l4, m4) = (
        offs(rank_x, 0),
        offs(rank_y, 0),
        offs(rank_z, 0),
        offs(rank_w, 0),
        offs(rank_v, 0),
    );

    let x1 = x0 - i1.cast_f32() + S::Vf32::set1(G5_32);
    let y1 = y0 - j1.cast_f32() + S::Vf32::set1(G5_32);
    let z1 = z0 - k1.cast_f32() + S::Vf32::set1(G5_32);
    let w1 = w0 - l1.cast_f32() + S::Vf32::set1(G5_32);
    let v1 = v0 - m1.cast_f32() + S::Vf32::set1(G5_32);
    let x2 = x0 - i2.cast_f32() + S::Vf32::set1(G25_32);
    let y2 = y0 - j2.cast_f32() + S::Vf32::set1(G25_32);
    let z2 = z0 - k2.cast_f32() + S::Vf32::set1(G25_32);
    let w2 = w0 - l2.cast_f32() + S::Vf32::set1(G25_32);
    let v2 = v0 - m2.cast_f32() + S::Vf32::set1(G25_32);
    let x3 = x0 - i3.cast_f32() + S::Vf32::set1(G35_32);
    let y3 = y0 - j3.cast_f32() + S::Vf32::set1(G35_32);
    let z3 = z0 - k3.cast_f32() + S::Vf32::set1(G35_32);
    let w3 = w0 - l3.cast_f32() + S::Vf32::set1(G35_32);
    let v3 = v0 - m3.cast_f32() + S::Vf32::set1(G35_32);
    let x4 = x0 - i4.cast_f32() + S::Vf32::set1(G45_32);
    let y4 = y0 - j4.cast_f32() + S::Vf32::set1(G45_32);
    let z4 = z0 - k4.cast_f32() + S::Vf32::set1(G45_32);
    let w4 = w0 - l4.cast_f32() + S::Vf32::set1(G45_32);
    let v4 = v0 - m4.cast_f32() + S::Vf32::set1(G45_32);
    let x5 = x0 - S::Vf32::set1(1.0) + S::Vf32::set1(G55_32);
    let y5 = y0 - S::Vf32::set1(1.0) + S::Vf32::set1(G55_32);
    let z5 = z0 - S::Vf32::set1(1.0) + S::Vf32::set1(G55_32);
    let w5 = w0 - S::Vf32::set1(1.0) + S::Vf32::set1(G55_32);
    let v5 = v0 - S::Vf32::set1(1.0) + S::Vf32::set1(G55_32);

    let ii = i & S::Vi32::set1(0xff);
    let jj = j & S::Vi32::set1(0xff);
    let kk = k & S::Vi32::set1(0xff);
    let ll = l & S::Vi32::set1(0xff);
    let mm = m & S::Vi32::set1(0xff);

    // Nested permutation gather over the five cell coordinates. Each PERM entry
    // is 0..255 and PERM has 512 slots, so `coord(0..255) + offset(0..1) +
    // prev(0..255)` stays in bounds.
    let (gi0, gi1, gi2, gi3, gi4, gi5) = unsafe {
        macro_rules! corner {
            ($i:expr, $j:expr, $k:expr, $l:expr, $m:expr) => {{
                let mp = gather_32::<S>(&PERM, mm + $m);
                let lp = gather_32::<S>(&PERM, ll + $l + mp);
                let kp = gather_32::<S>(&PERM, kk + $k + lp);
                let jp = gather_32::<S>(&PERM, jj + $j + kp);
                gather_32::<S>(&PERM, ii + $i + jp)
            }};
        }
        let z0i = S::Vi32::zeroes();
        let gi0 = corner!(z0i, z0i, z0i, z0i, z0i);
        let gi1 = corner!(i1, j1, k1, l1, m1);
        let gi2 = corner!(i2, j2, k2, l2, m2);
        let gi3 = corner!(i3, j3, k3, l3, m3);
        let gi4 = corner!(i4, j4, k4, l4, m4);
        let gi5 = corner!(one, one, one, one, one);
        (gi0, gi1, gi2, gi3, gi4, gi5)
    };

    let r2 = S::Vf32::set1(0.5);
    let t0 = r2 - (x0 * x0) - (y0 * y0) - (z0 * z0) - (w0 * w0) - (v0 * v0);
    let t1 = r2 - (x1 * x1) - (y1 * y1) - (z1 * z1) - (w1 * w1) - (v1 * v1);
    let t2 = r2 - (x2 * x2) - (y2 * y2) - (z2 * z2) - (w2 * w2) - (v2 * v2);
    let t3 = r2 - (x3 * x3) - (y3 * y3) - (z3 * z3) - (w3 * w3) - (v3 * v3);
    let t4 = r2 - (x4 * x4) - (y4 * y4) - (z4 * z4) - (w4 * w4) - (v4 * v4);
    let t5 = r2 - (x5 * x5) - (y5 * y5) - (z5 * z5) - (w5 * w5) - (v5 * v5);

    // Each weight to the 4th power.
    let q = |t: S::Vf32| -> S::Vf32 {
        let tt = t * t;
        tt * tt
    };

    let mut n0 = q(t0) * grad5::<S>(seed, gi0, x0, y0, z0, w0, v0);
    let mut n1 = q(t1) * grad5::<S>(seed, gi1, x1, y1, z1, w1, v1);
    let mut n2 = q(t2) * grad5::<S>(seed, gi2, x2, y2, z2, w2, v2);
    let mut n3 = q(t3) * grad5::<S>(seed, gi3, x3, y3, z3, w3, v3);
    let mut n4 = q(t4) * grad5::<S>(seed, gi4, x4, y4, z4, w4, v4);
    let mut n5 = q(t5) * grad5::<S>(seed, gi5, x5, y5, z5, w5, v5);

    // Discard contributions whose base weight is negative.
    let zf = S::Vf32::zeroes();
    n0 = n0.and_not(t0.cmp_lt(zf));
    n1 = n1.and_not(t1.cmp_lt(zf));
    n2 = n2.and_not(t2.cmp_lt(zf));
    n3 = n3.and_not(t3.cmp_lt(zf));
    n4 = n4.and_not(t4.cmp_lt(zf));
    n5 = n5.and_not(t5.cmp_lt(zf));

    (n0 + n1 + n2 + n3 + n4 + n5) * S::Vf32::set1(SIMPLEX_5D_SCALE_32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simdeez::scalar::{F32x1, Scalar};

    fn check_bounds(min: f32, max: f32) {
        assert!((-1.0..-0.75).contains(&min), "min out of range {}", min);
        assert!(max > 0.75 && max <= 1.0, "max out of range: {}", max);
    }

    #[test]
    fn test_noise_simplex32_1d_range() {
        // Aggregate across seeds: with properly mixed seeds (issue #42) a single
        // seed's samples along one line may not reach the extremes, but the noise
        // still spans [-1, 1] across the set. (2D+ hit the extremes per-seed.)
        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;
        for seed in 0..10 {
            for x in 0..1000 {
                let n = simplex_1d::<Scalar>(F32x1(x as f32 / 10.0), seed).0;
                min = min.min(n);
                max = max.max(n);
            }
        }
        check_bounds(min, max);
    }

    #[test]
    fn test_noise_simplex32_1d_deriv_sanity() {
        let mut avg_err = 0.0;
        const SEEDS: i32 = 10;
        const POINTS: i32 = 1000;
        for seed in 0..SEEDS {
            for x in 0..POINTS {
                // Offset a bit so we don't check derivative at lattice points, where it's always zero
                let center = x as f32 / 10.0 + 0.1234;
                const H: f32 = 0.01;
                let n0 = simplex_1d::<Scalar>(F32x1(center - H), seed).0;
                let (n1, d1) = simplex_1d_deriv::<Scalar>(F32x1(center), seed);
                let n2 = simplex_1d::<Scalar>(F32x1(center + H), seed).0;
                let (n1, d1) = (n1.0, d1.0);
                avg_err += ((n2 - (n1 + d1 * H)).abs() + (n0 - (n1 - d1 * H)).abs())
                    / (SEEDS * POINTS * 2) as f32;
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn test_noise_simplex32_2d_range() {
        for seed in 0..10 {
            let mut min = f32::INFINITY;
            let mut max = -f32::INFINITY;
            for y in 0..10 {
                for x in 0..100 {
                    let n =
                        simplex_2d::<Scalar>(F32x1(x as f32 / 10.0), F32x1(y as f32 / 10.0), seed)
                            .0;
                    min = min.min(n);
                    max = max.max(n);
                }
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn test_noise_simplex32_2d_deriv_sanity() {
        let mut avg_err = 0.0;
        const SEEDS: i32 = 10;
        const POINTS: i32 = 10;
        for seed in 0..SEEDS {
            for y in 0..POINTS {
                for x in 0..POINTS {
                    // Offset a bit so we don't check derivative at lattice points, where it's always zero
                    let center_x = x as f32 / 10.0 + 0.1234;
                    let center_y = y as f32 / 10.0 + 0.1234;
                    const H: f32 = 0.01;
                    let (value, d) =
                        simplex_2d_deriv::<Scalar>(F32x1(center_x), F32x1(center_y), seed);
                    let (value, d) = (value.0, [d[0].0, d[1].0]);
                    let left = simplex_2d::<Scalar>(F32x1(center_x - H), F32x1(center_y), seed).0;
                    let right = simplex_2d::<Scalar>(F32x1(center_x + H), F32x1(center_y), seed).0;
                    let down = simplex_2d::<Scalar>(F32x1(center_x), F32x1(center_y - H), seed).0;
                    let up = simplex_2d::<Scalar>(F32x1(center_x), F32x1(center_y + H), seed).0;
                    avg_err += ((left - (value - d[0] * H)).abs()
                        + (right - (value + d[0] * H)).abs()
                        + (down - (value - d[1] * H)).abs()
                        + (up - (value + d[1] * H)).abs())
                        / (SEEDS * POINTS * POINTS * 4) as f32;
                }
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn test_noise_simplex32_3d_range() {
        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;
        const SEED: i32 = 0;
        for z in 0..10 {
            for y in 0..10 {
                for x in 0..10000 {
                    let n = simplex_3d::<Scalar>(
                        F32x1(x as f32 / 10.0),
                        F32x1(y as f32 / 10.0),
                        F32x1(z as f32 / 10.0),
                        SEED,
                    )
                    .0;
                    min = min.min(n);
                    max = max.max(n);
                }
            }
        }
        check_bounds(min, max);
    }

    #[test]
    fn test_noise_simplex32_3d_deriv_sanity() {
        let mut avg_err = 0.0;
        const POINTS: i32 = 10;
        const SEED: i32 = 0;
        for z in 0..POINTS {
            for y in 0..POINTS {
                for x in 0..POINTS {
                    // Offset a bit so we don't check derivative at lattice points, where it's always zero
                    let center_x = x as f32 / 10.0 + 0.1234;
                    let center_y = y as f32 / 10.0 + 0.1234;
                    let center_z = z as f32 / 10.0 + 0.1234;
                    const H: f32 = 0.01;
                    let (value, d) = simplex_3d_deriv::<Scalar>(
                        F32x1(center_x),
                        F32x1(center_y),
                        F32x1(center_z),
                        SEED,
                    );
                    let (value, d) = (value.0, [d[0].0, d[1].0, d[2].0]);
                    let right = simplex_3d::<Scalar>(
                        F32x1(center_x + H),
                        F32x1(center_y),
                        F32x1(center_z),
                        SEED,
                    )
                    .0;
                    let up = simplex_3d::<Scalar>(
                        F32x1(center_x),
                        F32x1(center_y + H),
                        F32x1(center_z),
                        SEED,
                    )
                    .0;
                    let forward = simplex_3d::<Scalar>(
                        F32x1(center_x),
                        F32x1(center_y),
                        F32x1(center_z + H),
                        SEED,
                    )
                    .0;
                    avg_err += ((right - (value + d[0] * H)).abs()
                        + (up - (value + d[1] * H)).abs()
                        + (forward - (value + d[2] * H)).abs())
                        / (POINTS * POINTS * POINTS * 3) as f32;
                }
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn test_noise_simplex32_4d_range() {
        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;
        const SEED: i32 = 0;
        for w in 0..10 {
            for z in 0..10 {
                for y in 0..10 {
                    for x in 0..1000 {
                        let n = simplex_4d::<Scalar>(
                            F32x1(x as f32 / 10.0),
                            F32x1(y as f32 / 10.0),
                            F32x1(z as f32 / 10.0),
                            F32x1(w as f32 / 10.0),
                            SEED,
                        )
                        .0;
                        min = min.min(n);
                        max = max.max(n);
                    }
                }
            }
        }
        check_bounds(min, max);
    }
}
