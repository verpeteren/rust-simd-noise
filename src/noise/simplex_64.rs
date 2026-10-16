use crate::noise::gradient_64::{grad1, grad2, grad3d, grad4, grad5, grad6};

use simdeez::prelude::*;

use crate::noise::cellular_32::{X_PRIME_64, Y_PRIME_64, Z_PRIME_64};
use crate::noise::gradient_64::grad3d_dot;
use crate::noise::ops::gather_64;
use crate::noise::simplex_32::{
    F2_64, F3_64, F4_64, F5_64, F6_64, G22_64, G24_64, G25_64, G26_64, G2_64, G33_64, G34_64,
    G35_64, G36_64, G3_64, G44_64, G45_64, G46_64, G4_64, G55_64, G56_64, G5_64, G66_64, G6_64,
};

static PERM64: [i64; 512] = [
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
fn assert_in_perm_range<S: Simd>(values: S::Vi64) {
    debug_assert!(values
        .cmp_lt(S::Vi64::set1(PERM64.len() as i64))
        .iter()
        .all(|is_less_than| is_less_than != 0));
}

/// Like `simplex_1d`, but also computes the derivative
#[inline(always)]
pub fn simplex_1d_deriv<S: Simd>(x: S::Vf64, seed: i64) -> (S::Vf64, S::Vf64) {
    // Gradients are selected deterministically based on the whole part of `x`
    let ips = x.fast_floor();
    let mut i0 = ips.cast_i64();
    let i1 = (i0 + S::Vi64::set1(1)) & S::Vi64::set1(0xff);

    // the fractional part of x, i.e. the distance to the left gradient node. 0 ≤ x0 < 1.
    let x0 = x - ips;
    // signed distance to the right gradient node
    let x1 = x0 - S::Vf64::set1(1.0);

    i0 &= S::Vi64::set1(0xff);
    let (gi0, gi1) = unsafe {
        // Safety: We just masked i0 and i1 with 0xff, so they're in 0..255.
        let gi0 = gather_64::<S>(&PERM64, i0);
        let gi1 = gather_64::<S>(&PERM64, i1);
        (gi0, gi1)
    };

    // Compute the contribution from the first gradient
    let x20 = x0 * x0; // x^2_0
    let t0 = S::Vf64::set1(1.0) - x20; // t_0
    let t20 = t0 * t0; // t^2_0
    let t40 = t20 * t20; // t^4_0
    let gx0 = grad1::<S>(seed, gi0);
    let n0 = t40 * gx0 * x0;
    // n0 = (1 - x0^2)^4 * x0 * grad

    // Compute the contribution from the second gradient
    let x21 = x1 * x1; // x^2_1
    let t1 = S::Vf64::set1(1.0) - x21; // t_1
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
    const SCALE: f64 = 256.0 / (81.0 * 7.0);

    let value = (n0 + n1) * S::Vf64::set1(SCALE);
    let derivative = ((t20 * t0 * gx0 * x20 + t21 * t1 * gx1 * x21) * S::Vf64::set1(-8.0)
        + t40 * gx0
        + t41 * gx1)
        * S::Vf64::set1(SCALE);
    (value, derivative)
}

/// Samples 1-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub fn simplex_1d<S: Simd>(x: S::Vf64, seed: i64) -> S::Vf64 {
    simplex_1d_deriv::<S>(x, seed).0
}

/// Samples 2-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub fn simplex_2d<S: Simd>(x: S::Vf64, y: S::Vf64, seed: i64) -> S::Vf64 {
    simplex_2d_deriv::<S>(x, y, seed).0
}

/// Like `simplex_2d`, but also computes the derivative
// Not `#[inline(always)]`: like the 3D/4D paths (issue #38), force-inlining this
// large function builds one oversized stack frame that overflows small stacks
// under AVX2. `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
pub fn simplex_2d_deriv<S: Simd>(x: S::Vf64, y: S::Vf64, seed: i64) -> (S::Vf64, [S::Vf64; 2]) {
    // Skew to distort simplexes with side length sqrt(2)/sqrt(3) until they make up
    // squares
    let s = S::Vf64::set1(F2_64) * (x + y);
    let ips = (x + s).floor();
    let jps = (y + s).floor();

    // Integer coordinates for the base vertex of the triangle
    let i = ips.cast_i64();
    let j = jps.cast_i64();

    let t = (i + j).cast_f64() * S::Vf64::set1(G2_64);

    // Unskewed distances to the first point of the enclosing simplex
    let x0 = x - (ips - t);
    let y0 = y - (jps - t);

    let i1 = (x0.cmp_gte(y0)).bitcast_i64();

    let j1 = (y0.cmp_gt(x0)).bitcast_i64();

    // Distances to the second and third points of the enclosing simplex
    let x1 = (x0 + i1.cast_f64()) + S::Vf64::set1(G2_64);
    let y1 = (y0 + j1.cast_f64()) + S::Vf64::set1(G2_64);
    let x2 = (x0 + S::Vf64::set1(-1.0)) + S::Vf64::set1(G22_64);
    let y2 = (y0 + S::Vf64::set1(-1.0)) + S::Vf64::set1(G22_64);

    let ii = i & S::Vi64::set1(0xff);
    let jj = j & S::Vi64::set1(0xff);

    let (gi0, gi1, gi2) = unsafe {
        assert_in_perm_range::<S>(ii);
        assert_in_perm_range::<S>(jj);
        assert_in_perm_range::<S>(ii - i1);
        assert_in_perm_range::<S>(jj - j1);
        assert_in_perm_range::<S>(ii + 1);
        assert_in_perm_range::<S>(jj + 1);

        let gi0 = gather_64::<S>(&PERM64, ii + gather_64::<S>(&PERM64, jj));
        let gi1 = gather_64::<S>(&PERM64, (ii - i1) + gather_64::<S>(&PERM64, jj - j1));
        let gi2 = gather_64::<S>(&PERM64, (ii - -1) + gather_64::<S>(&PERM64, jj - -1));

        (gi0, gi1, gi2)
    };

    // Weights associated with the gradients at each corner
    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let mut t0 = S::Vf64::neg_mul_add(y0, y0, S::Vf64::neg_mul_add(x0, x0, S::Vf64::set1(0.5)));
    let mut t1 = S::Vf64::neg_mul_add(y1, y1, S::Vf64::neg_mul_add(x1, x1, S::Vf64::set1(0.5)));
    let mut t2 = S::Vf64::neg_mul_add(y2, y2, S::Vf64::neg_mul_add(x2, x2, S::Vf64::set1(0.5)));

    // Zero out negative weights
    t0 &= t0.cmp_gte(S::Vf64::zeroes());
    t1 &= t1.cmp_gte(S::Vf64::zeroes());
    t2 &= t2.cmp_gte(S::Vf64::zeroes());

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
    let scale = S::Vf64::set1(45.264_507_749_855_62);
    let value = (n0 + n1 + n2) * scale;
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
        dnoise_dx *= S::Vf64::set1(-8.0);
        dnoise_dy *= S::Vf64::set1(-8.0);
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
pub fn simplex_3d<S: Simd>(x: S::Vf64, y: S::Vf64, z: S::Vf64, seed: i64) -> S::Vf64 {
    simplex_3d_deriv::<S>(x, y, z, seed).0
}

/// Like `simplex_3d`, but also computes the derivative
// Not `#[inline(always)]`: force-inlining this large function into the noise
// generation loop builds one huge stack frame that overflows small stacks under
// AVX2 (issue #38). `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
pub fn simplex_3d_deriv<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 3]) {
    // Find skewed simplex grid coordinates associated with the input coordinates
    let f = S::Vf64::set1(F3_64) * ((x + y) + z);
    let mut x0 = (x + f).fast_floor();
    let mut y0 = (y + f).fast_floor();
    let mut z0 = (z + f).fast_floor();

    // Integer grid coordinates
    let i = x0.cast_i64() * S::Vi64::set1(X_PRIME_64);
    let j = y0.cast_i64() * S::Vi64::set1(Y_PRIME_64);
    let k = z0.cast_i64() * S::Vi64::set1(Z_PRIME_64);

    // Compute distance from first simplex vertex to input coordinates
    let g = S::Vf64::set1(G3_64) * (x0 + y0 + z0);
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
    let x1 = x0 - (i1 & S::Vf64::set1(1.0)) + S::Vf64::set1(G3_64);
    let y1 = y0 - (j1 & S::Vf64::set1(1.0)) + S::Vf64::set1(G3_64);
    let z1 = z0 - (k1 & S::Vf64::set1(1.0)) + S::Vf64::set1(G3_64);

    let x2 = x0 - (i2 & S::Vf64::set1(1.0)) + S::Vf64::set1(F3_64);
    let y2 = y0 - (j2 & S::Vf64::set1(1.0)) + S::Vf64::set1(F3_64);
    let z2 = z0 - (k2 & S::Vf64::set1(1.0)) + S::Vf64::set1(F3_64);

    let x3 = x0 + S::Vf64::set1(G33_64);
    let y3 = y0 + S::Vf64::set1(G33_64);
    let z3 = z0 + S::Vf64::set1(G33_64);

    // Compute base weight factors associated with each vertex, `0.6 - v . v` where v is the
    // distance to the vertex. Strictly the constant should be 0.5, but 0.6 is thought by Gustavson
    // to give visually better results at the cost of subtle discontinuities.
    //#define SIMDf_NMUL_ADD(a,b,c) = SIMDf_SUB(c, SIMDf_MUL(a,b)
    let mut t0 = S::Vf64::set1(0.6) - (x0 * x0) - (y0 * y0) - (z0 * z0);
    let mut t1 = S::Vf64::set1(0.6) - (x1 * x1) - (y1 * y1) - (z1 * z1);
    let mut t2 = S::Vf64::set1(0.6) - (x2 * x2) - (y2 * y2) - (z2 * z2);
    let mut t3 = S::Vf64::set1(0.6) - (x3 * x3) - (y3 * y3) - (z3 * z3);

    // Zero out negative weights
    t0 &= t0.cmp_gte(S::Vf64::zeroes());
    t1 &= t1.cmp_gte(S::Vf64::zeroes());
    t2 &= t2.cmp_gte(S::Vf64::zeroes());
    t3 &= t3.cmp_gte(S::Vf64::zeroes());

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

    let v1x = i + (i1.bitcast_i64() & S::Vi64::set1(X_PRIME_64));
    let v1y = j + (j1.bitcast_i64() & S::Vi64::set1(Y_PRIME_64));
    let v1z = k + (k1.bitcast_i64() & S::Vi64::set1(Z_PRIME_64));
    let g1 = grad3d_dot::<S>(seed, v1x, v1y, v1z, x1, y1, z1);
    let v1 = t41 * g1;

    let v2x = i + (i2.bitcast_i64() & S::Vi64::set1(X_PRIME_64));
    let v2y = j + (j2.bitcast_i64() & S::Vi64::set1(Y_PRIME_64));
    let v2z = k + (k2.bitcast_i64() & S::Vi64::set1(Z_PRIME_64));
    let g2 = grad3d_dot::<S>(seed, v2x, v2y, v2z, x2, y2, z2);
    let v2 = t42 * g2;

    //SIMDf v3 = SIMDf_MASK(n3, SIMDf_MUL(SIMDf_MUL(t3, t3), FUNC(GradCoord)(seed, SIMDi_ADD(i, SIMDi_NUM(xPrime)), SIMDi_ADD(j, SIMDi_NUM(yPrime)), SIMDi_ADD(k, SIMDi_NUM(zPrime)), x3, y3, z3)));
    let v3x = i + S::Vi64::set1(X_PRIME_64);
    let v3y = j + S::Vi64::set1(Y_PRIME_64);
    let v3z = k + S::Vi64::set1(Z_PRIME_64);
    //define SIMDf_MASK(m,a) SIMDf_AND(SIMDf_CAST_TO_FLOAT(m),a)
    let g3 = grad3d_dot::<S>(seed, v3x, v3y, v3z, x3, y3, z3);
    let v3 = t43 * g3;

    let p1 = v3 + v2;
    let p2 = p1 + v1;

    // Scaling factor found by numerical approximation
    let scale = S::Vf64::set1(32.69587493801679);
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
        dnoise_dx *= S::Vf64::set1(-8.0);
        dnoise_dy *= S::Vf64::set1(-8.0);
        dnoise_dz *= S::Vf64::set1(-8.0);
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
        [dnoise_dx, dnoise_dy, dnoise_dz]
    };
    (result, derivative)
}

/// Samples 4-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
// Not `#[inline(always)]`: see `simplex_3d_deriv` (issue #38).
#[inline]
pub fn simplex_4d<S: Simd>(x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, seed: i64) -> S::Vf64 {
    // ImproveXYZ domain rotation (issue #15): see the f32 `simplex_4d`. Maps
    // world-w onto the lattice (1,1,1,1) diagonal so w-constant slices become
    // isotropic cross-sections, removing the diagonal streak from them.
    let xr = S::Vf64::set1(std::f64::consts::FRAC_1_SQRT_2) * x
        + S::Vf64::set1(0.4082482904638631) * y
        + S::Vf64::set1(0.28867513459481287) * z
        + S::Vf64::set1(0.5) * w;
    let yr = S::Vf64::set1(-std::f64::consts::FRAC_1_SQRT_2) * x
        + S::Vf64::set1(0.4082482904638631) * y
        + S::Vf64::set1(0.28867513459481287) * z
        + S::Vf64::set1(0.5) * w;
    let zr = S::Vf64::set1(-0.8164965809277261) * y
        + S::Vf64::set1(0.28867513459481287) * z
        + S::Vf64::set1(0.5) * w;
    let wr = S::Vf64::set1(-0.8660254037844386) * z + S::Vf64::set1(0.5) * w;
    let (x, y, z, w) = (xr, yr, zr, wr);
    let s = S::Vf64::set1(F4_64) * (x + y + z + w);

    let ipd = (x + s).floor();
    let jpd = (y + s).floor();
    let kpd = (z + s).floor();
    let lpd = (w + s).floor();

    let i = ipd.cast_i64();
    let j = jpd.cast_i64();
    let k = kpd.cast_i64();
    let l = lpd.cast_i64();

    let t = (i + j + k + l).cast_f64() * S::Vf64::set1(G4_64);
    let x0 = x - (ipd - t);
    let y0 = y - (jpd - t);
    let z0 = z - (kpd - t);
    let w0 = w - (lpd - t);

    let mut rank_x = S::Vi64::zeroes();
    let mut rank_y = S::Vi64::zeroes();
    let mut rank_z = S::Vi64::zeroes();
    let mut rank_w = S::Vi64::zeroes();

    let cond = (x0.cmp_gt(y0)).bitcast_i64();
    rank_x += cond & S::Vi64::set1(1);
    rank_y += S::Vi64::set1(1).and_not(cond);
    let cond = (x0.cmp_gt(z0)).bitcast_i64();
    rank_x += cond & S::Vi64::set1(1);
    rank_z += S::Vi64::set1(1).and_not(cond);
    let cond = (x0.cmp_gt(w0)).bitcast_i64();
    rank_x += cond & S::Vi64::set1(1);
    rank_w += S::Vi64::set1(1).and_not(cond);
    let cond = (y0.cmp_gt(z0)).bitcast_i64();
    rank_y += cond & S::Vi64::set1(1);
    rank_z += S::Vi64::set1(1).and_not(cond);
    let cond = (y0.cmp_gt(w0)).bitcast_i64();
    rank_y += cond & S::Vi64::set1(1);
    rank_w += S::Vi64::set1(1).and_not(cond);
    let cond = (z0.cmp_gt(w0)).bitcast_i64();
    rank_z += cond & S::Vi64::set1(1);
    rank_w += S::Vi64::set1(1).and_not(cond);

    let cond = rank_x.cmp_gt(S::Vi64::set1(2));
    let i1 = S::Vi64::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::Vi64::set1(2));
    let j1 = S::Vi64::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::Vi64::set1(2));
    let k1 = S::Vi64::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::Vi64::set1(2));
    let l1 = S::Vi64::set1(1) & cond;

    let cond = rank_x.cmp_gt(S::Vi64::set1(1));
    let i2 = S::Vi64::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::Vi64::set1(1));
    let j2 = S::Vi64::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::Vi64::set1(1));
    let k2 = S::Vi64::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::Vi64::set1(1));
    let l2 = S::Vi64::set1(1) & cond;

    let cond = rank_x.cmp_gt(S::Vi64::zeroes());
    let i3 = S::Vi64::set1(1) & cond;
    let cond = rank_y.cmp_gt(S::Vi64::zeroes());
    let j3 = S::Vi64::set1(1) & cond;
    let cond = rank_z.cmp_gt(S::Vi64::zeroes());
    let k3 = S::Vi64::set1(1) & cond;
    let cond = rank_w.cmp_gt(S::Vi64::zeroes());
    let l3 = S::Vi64::set1(1) & cond;

    let x1 = x0 - i1.cast_f64() + S::Vf64::set1(G4_64);
    let y1 = y0 - j1.cast_f64() + S::Vf64::set1(G4_64);
    let z1 = z0 - k1.cast_f64() + S::Vf64::set1(G4_64);
    let w1 = w0 - l1.cast_f64() + S::Vf64::set1(G4_64);
    let x2 = x0 - i2.cast_f64() + S::Vf64::set1(G24_64);
    let y2 = y0 - j2.cast_f64() + S::Vf64::set1(G24_64);
    let z2 = z0 - k2.cast_f64() + S::Vf64::set1(G24_64);
    let w2 = w0 - l2.cast_f64() + S::Vf64::set1(G24_64);
    let x3 = x0 - i3.cast_f64() + S::Vf64::set1(G34_64);
    let y3 = y0 - j3.cast_f64() + S::Vf64::set1(G34_64);
    let z3 = z0 - k3.cast_f64() + S::Vf64::set1(G34_64);
    let w3 = w0 - l3.cast_f64() + S::Vf64::set1(G34_64);
    let x4 = x0 - S::Vf64::set1(1.0) + S::Vf64::set1(G44_64);
    let y4 = y0 - S::Vf64::set1(1.0) + S::Vf64::set1(G44_64);
    let z4 = z0 - S::Vf64::set1(1.0) + S::Vf64::set1(G44_64);
    let w4 = w0 - S::Vf64::set1(1.0) + S::Vf64::set1(G44_64);

    let ii = i & S::Vi64::set1(0xff);
    let jj = j & S::Vi64::set1(0xff);
    let kk = k & S::Vi64::set1(0xff);
    let ll = l & S::Vi64::set1(0xff);

    let (gi0, gi1, gi2, gi3, gi4) = unsafe {
        // Safety: ii, jj, kk, and ll are all 0..255. All other temporary variables were fetched from PERM, which only
        // contains elements in the range 0..255.
        let lp = gather_64::<S>(&PERM64, ll);
        let kp = gather_64::<S>(&PERM64, kk + lp);
        let jp = gather_64::<S>(&PERM64, jj + kp);
        let gi0 = gather_64::<S>(&PERM64, ii + jp);

        let lp = gather_64::<S>(&PERM64, ll + l1);
        let kp = gather_64::<S>(&PERM64, kk + k1 + lp);
        let jp = gather_64::<S>(&PERM64, jj + j1 + kp);
        let gi1 = gather_64::<S>(&PERM64, ii + i1 + jp);

        let lp = gather_64::<S>(&PERM64, ll + l2);
        let kp = gather_64::<S>(&PERM64, kk + k2 + lp);
        let jp = gather_64::<S>(&PERM64, jj + j2 + kp);
        let gi2 = gather_64::<S>(&PERM64, ii + i2 + jp);

        let lp = gather_64::<S>(&PERM64, ll + l3);
        let kp = gather_64::<S>(&PERM64, kk + k3 + lp);
        let jp = gather_64::<S>(&PERM64, jj + j3 + kp);
        let gi3 = gather_64::<S>(&PERM64, ii + i3 + jp);

        let lp = gather_64::<S>(&PERM64, ll + S::Vi64::set1(1));
        let kp = gather_64::<S>(&PERM64, kk + S::Vi64::set1(1) + lp);
        let jp = gather_64::<S>(&PERM64, jj + S::Vi64::set1(1) + kp);
        let gi4 = gather_64::<S>(&PERM64, ii + S::Vi64::set1(1) + jp);
        (gi0, gi1, gi2, gi3, gi4)
    };

    let t0 = S::Vf64::set1(0.5) - (x0 * x0) - (y0 * y0) - (z0 * z0) - (w0 * w0);
    let t1 = S::Vf64::set1(0.5) - (x1 * x1) - (y1 * y1) - (z1 * z1) - (w1 * w1);
    let t2 = S::Vf64::set1(0.5) - (x2 * x2) - (y2 * y2) - (z2 * z2) - (w2 * w2);
    let t3 = S::Vf64::set1(0.5) - (x3 * x3) - (y3 * y3) - (z3 * z3) - (w3 * w3);
    let t4 = S::Vf64::set1(0.5) - (x4 * x4) - (y4 * y4) - (z4 * z4) - (w4 * w4);
    //ti*ti*ti*ti
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

    //if ti < 0 then 0 else ni
    let mut cond = t0.cmp_lt(S::Vf64::zeroes());
    n0 = n0.and_not(cond);
    cond = t1.cmp_lt(S::Vf64::zeroes());
    n1 = n1.and_not(cond);
    cond = t2.cmp_lt(S::Vf64::zeroes());
    n2 = n2.and_not(cond);
    cond = t3.cmp_lt(S::Vf64::zeroes());
    n3 = n3.and_not(cond);
    cond = t4.cmp_lt(S::Vf64::zeroes());
    n4 = n4.and_not(cond);

    (n0 + (n1 + (n2 + (n3 + n4)))) * S::Vf64::set1(62.77772078955791)
}

/// Empirical normalisation for 5D simplex (f64); see the f32
/// `simplex_32::SIMPLEX_5D_SCALE_32`.
const SIMPLEX_5D_SCALE_64: f64 = 54.5;

/// 5D simplex noise (f64). From-scratch generalisation of `simplex_4d`; see the
/// f32 `simplex_32::simplex_5d` for the design (plain simplex, no domain
/// rotation, `F5`/`G5`, 10-compare ranks → 6 corners, 80-gradient `grad5`,
/// `r² = 0.5`).
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn simplex_5d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    u: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let s = S::Vf64::set1(F5_64) * (x + y + z + w + u);
    let ipd = (x + s).floor();
    let jpd = (y + s).floor();
    let kpd = (z + s).floor();
    let lpd = (w + s).floor();
    let mpd = (u + s).floor();

    let i = ipd.cast_i64();
    let j = jpd.cast_i64();
    let k = kpd.cast_i64();
    let l = lpd.cast_i64();
    let m = mpd.cast_i64();

    let t = (i + j + k + l + m).cast_f64() * S::Vf64::set1(G5_64);
    let x0 = x - (ipd - t);
    let y0 = y - (jpd - t);
    let z0 = z - (kpd - t);
    let w0 = w - (lpd - t);
    let v0 = u - (mpd - t);

    let mut rank_x = S::Vi64::zeroes();
    let mut rank_y = S::Vi64::zeroes();
    let mut rank_z = S::Vi64::zeroes();
    let mut rank_w = S::Vi64::zeroes();
    let mut rank_v = S::Vi64::zeroes();
    let one = S::Vi64::set1(1);
    macro_rules! cmp_rank {
        ($a0:expr, $ra:ident, $b0:expr, $rb:ident) => {{
            let cond = ($a0.cmp_gt($b0)).bitcast_i64();
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

    let offs = |rank: S::Vi64, thresh: i64| -> S::Vi64 { one & rank.cmp_gt(S::Vi64::set1(thresh)) };
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

    let x1 = x0 - i1.cast_f64() + S::Vf64::set1(G5_64);
    let y1 = y0 - j1.cast_f64() + S::Vf64::set1(G5_64);
    let z1 = z0 - k1.cast_f64() + S::Vf64::set1(G5_64);
    let w1 = w0 - l1.cast_f64() + S::Vf64::set1(G5_64);
    let v1 = v0 - m1.cast_f64() + S::Vf64::set1(G5_64);
    let x2 = x0 - i2.cast_f64() + S::Vf64::set1(G25_64);
    let y2 = y0 - j2.cast_f64() + S::Vf64::set1(G25_64);
    let z2 = z0 - k2.cast_f64() + S::Vf64::set1(G25_64);
    let w2 = w0 - l2.cast_f64() + S::Vf64::set1(G25_64);
    let v2 = v0 - m2.cast_f64() + S::Vf64::set1(G25_64);
    let x3 = x0 - i3.cast_f64() + S::Vf64::set1(G35_64);
    let y3 = y0 - j3.cast_f64() + S::Vf64::set1(G35_64);
    let z3 = z0 - k3.cast_f64() + S::Vf64::set1(G35_64);
    let w3 = w0 - l3.cast_f64() + S::Vf64::set1(G35_64);
    let v3 = v0 - m3.cast_f64() + S::Vf64::set1(G35_64);
    let x4 = x0 - i4.cast_f64() + S::Vf64::set1(G45_64);
    let y4 = y0 - j4.cast_f64() + S::Vf64::set1(G45_64);
    let z4 = z0 - k4.cast_f64() + S::Vf64::set1(G45_64);
    let w4 = w0 - l4.cast_f64() + S::Vf64::set1(G45_64);
    let v4 = v0 - m4.cast_f64() + S::Vf64::set1(G45_64);
    let x5 = x0 - S::Vf64::set1(1.0) + S::Vf64::set1(G55_64);
    let y5 = y0 - S::Vf64::set1(1.0) + S::Vf64::set1(G55_64);
    let z5 = z0 - S::Vf64::set1(1.0) + S::Vf64::set1(G55_64);
    let w5 = w0 - S::Vf64::set1(1.0) + S::Vf64::set1(G55_64);
    let v5 = v0 - S::Vf64::set1(1.0) + S::Vf64::set1(G55_64);

    let ii = i & S::Vi64::set1(0xff);
    let jj = j & S::Vi64::set1(0xff);
    let kk = k & S::Vi64::set1(0xff);
    let ll = l & S::Vi64::set1(0xff);
    let mm = m & S::Vi64::set1(0xff);

    let (gi0, gi1, gi2, gi3, gi4, gi5) = unsafe {
        macro_rules! corner {
            ($i:expr, $j:expr, $k:expr, $l:expr, $m:expr) => {{
                let mp = gather_64::<S>(&PERM64, mm + $m);
                let lp = gather_64::<S>(&PERM64, ll + $l + mp);
                let kp = gather_64::<S>(&PERM64, kk + $k + lp);
                let jp = gather_64::<S>(&PERM64, jj + $j + kp);
                gather_64::<S>(&PERM64, ii + $i + jp)
            }};
        }
        let z0i = S::Vi64::zeroes();
        let gi0 = corner!(z0i, z0i, z0i, z0i, z0i);
        let gi1 = corner!(i1, j1, k1, l1, m1);
        let gi2 = corner!(i2, j2, k2, l2, m2);
        let gi3 = corner!(i3, j3, k3, l3, m3);
        let gi4 = corner!(i4, j4, k4, l4, m4);
        let gi5 = corner!(one, one, one, one, one);
        (gi0, gi1, gi2, gi3, gi4, gi5)
    };

    let r2 = S::Vf64::set1(0.5);
    let t0 = r2 - (x0 * x0) - (y0 * y0) - (z0 * z0) - (w0 * w0) - (v0 * v0);
    let t1 = r2 - (x1 * x1) - (y1 * y1) - (z1 * z1) - (w1 * w1) - (v1 * v1);
    let t2 = r2 - (x2 * x2) - (y2 * y2) - (z2 * z2) - (w2 * w2) - (v2 * v2);
    let t3 = r2 - (x3 * x3) - (y3 * y3) - (z3 * z3) - (w3 * w3) - (v3 * v3);
    let t4 = r2 - (x4 * x4) - (y4 * y4) - (z4 * z4) - (w4 * w4) - (v4 * v4);
    let t5 = r2 - (x5 * x5) - (y5 * y5) - (z5 * z5) - (w5 * w5) - (v5 * v5);

    let q = |t: S::Vf64| -> S::Vf64 {
        let tt = t * t;
        tt * tt
    };

    let mut n0 = q(t0) * grad5::<S>(seed, gi0, x0, y0, z0, w0, v0);
    let mut n1 = q(t1) * grad5::<S>(seed, gi1, x1, y1, z1, w1, v1);
    let mut n2 = q(t2) * grad5::<S>(seed, gi2, x2, y2, z2, w2, v2);
    let mut n3 = q(t3) * grad5::<S>(seed, gi3, x3, y3, z3, w3, v3);
    let mut n4 = q(t4) * grad5::<S>(seed, gi4, x4, y4, z4, w4, v4);
    let mut n5 = q(t5) * grad5::<S>(seed, gi5, x5, y5, z5, w5, v5);

    let zf = S::Vf64::zeroes();
    n0 = n0.and_not(t0.cmp_lt(zf));
    n1 = n1.and_not(t1.cmp_lt(zf));
    n2 = n2.and_not(t2.cmp_lt(zf));
    n3 = n3.and_not(t3.cmp_lt(zf));
    n4 = n4.and_not(t4.cmp_lt(zf));
    n5 = n5.and_not(t5.cmp_lt(zf));

    (n0 + n1 + n2 + n3 + n4 + n5) * S::Vf64::set1(SIMPLEX_5D_SCALE_64)
}

/// f64 6D normalization. Same value as `simplex_32::SIMPLEX_6D_SCALE_32`.
const SIMPLEX_6D_SCALE_64: f64 = 49.5;

/// 6D simplex noise (f64). Mirror of the f32 `simplex_32::simplex_6d`: `F6`/`G6`
/// skew, 15-compare ranks → 7 corners, 192-gradient `grad6`, base weight
/// `r^2 = 0.5`, scaled by `SIMPLEX_6D_SCALE_64`.
#[inline(always)]
pub fn simplex_6d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    u: S::Vf64,
    t: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let sk = S::Vf64::set1(F6_64) * (x + y + z + w + u + t);
    let ips = (x + sk).floor();
    let jps = (y + sk).floor();
    let kps = (z + sk).floor();
    let lps = (w + sk).floor();
    let mps = (u + sk).floor();
    let nps = (t + sk).floor();

    let i = ips.cast_i64();
    let j = jps.cast_i64();
    let k = kps.cast_i64();
    let l = lps.cast_i64();
    let m = mps.cast_i64();
    let n = nps.cast_i64();

    let uns = (i + j + k + l + m + n).cast_f64() * S::Vf64::set1(G6_64);
    let x0 = x - (ips - uns);
    let y0 = y - (jps - uns);
    let z0 = z - (kps - uns);
    let w0 = w - (lps - uns);
    let u0 = u - (mps - uns);
    let t0c = t - (nps - uns);

    let mut rank_x = S::Vi64::zeroes();
    let mut rank_y = S::Vi64::zeroes();
    let mut rank_z = S::Vi64::zeroes();
    let mut rank_w = S::Vi64::zeroes();
    let mut rank_u = S::Vi64::zeroes();
    let mut rank_t = S::Vi64::zeroes();
    let one = S::Vi64::set1(1);
    macro_rules! cmp_rank {
        ($a0:expr, $ra:ident, $b0:expr, $rb:ident) => {{
            let cond = ($a0.cmp_gt($b0)).bitcast_i64();
            $ra += cond & one;
            $rb += one.and_not(cond);
        }};
    }
    cmp_rank!(x0, rank_x, y0, rank_y);
    cmp_rank!(x0, rank_x, z0, rank_z);
    cmp_rank!(x0, rank_x, w0, rank_w);
    cmp_rank!(x0, rank_x, u0, rank_u);
    cmp_rank!(x0, rank_x, t0c, rank_t);
    cmp_rank!(y0, rank_y, z0, rank_z);
    cmp_rank!(y0, rank_y, w0, rank_w);
    cmp_rank!(y0, rank_y, u0, rank_u);
    cmp_rank!(y0, rank_y, t0c, rank_t);
    cmp_rank!(z0, rank_z, w0, rank_w);
    cmp_rank!(z0, rank_z, u0, rank_u);
    cmp_rank!(z0, rank_z, t0c, rank_t);
    cmp_rank!(w0, rank_w, u0, rank_u);
    cmp_rank!(w0, rank_w, t0c, rank_t);
    cmp_rank!(u0, rank_u, t0c, rank_t);

    let offs = |rank: S::Vi64, thresh: i64| -> S::Vi64 { one & rank.cmp_gt(S::Vi64::set1(thresh)) };
    let (i1, j1, k1, l1, m1, n1) = (
        offs(rank_x, 4),
        offs(rank_y, 4),
        offs(rank_z, 4),
        offs(rank_w, 4),
        offs(rank_u, 4),
        offs(rank_t, 4),
    );
    let (i2, j2, k2, l2, m2, n2) = (
        offs(rank_x, 3),
        offs(rank_y, 3),
        offs(rank_z, 3),
        offs(rank_w, 3),
        offs(rank_u, 3),
        offs(rank_t, 3),
    );
    let (i3, j3, k3, l3, m3, n3) = (
        offs(rank_x, 2),
        offs(rank_y, 2),
        offs(rank_z, 2),
        offs(rank_w, 2),
        offs(rank_u, 2),
        offs(rank_t, 2),
    );
    let (i4, j4, k4, l4, m4, n4) = (
        offs(rank_x, 1),
        offs(rank_y, 1),
        offs(rank_z, 1),
        offs(rank_w, 1),
        offs(rank_u, 1),
        offs(rank_t, 1),
    );
    let (i5, j5, k5, l5, m5, n5v) = (
        offs(rank_x, 0),
        offs(rank_y, 0),
        offs(rank_z, 0),
        offs(rank_w, 0),
        offs(rank_u, 0),
        offs(rank_t, 0),
    );

    let g1 = S::Vf64::set1(G6_64);
    let g2 = S::Vf64::set1(G26_64);
    let g3 = S::Vf64::set1(G36_64);
    let g4 = S::Vf64::set1(G46_64);
    let g5 = S::Vf64::set1(G56_64);
    let g6 = S::Vf64::set1(G66_64);
    let one_f = S::Vf64::set1(1.0);

    let x1 = x0 - i1.cast_f64() + g1;
    let y1 = y0 - j1.cast_f64() + g1;
    let z1 = z0 - k1.cast_f64() + g1;
    let w1 = w0 - l1.cast_f64() + g1;
    let u1 = u0 - m1.cast_f64() + g1;
    let t1 = t0c - n1.cast_f64() + g1;
    let x2 = x0 - i2.cast_f64() + g2;
    let y2 = y0 - j2.cast_f64() + g2;
    let z2 = z0 - k2.cast_f64() + g2;
    let w2 = w0 - l2.cast_f64() + g2;
    let u2 = u0 - m2.cast_f64() + g2;
    let t2 = t0c - n2.cast_f64() + g2;
    let x3 = x0 - i3.cast_f64() + g3;
    let y3 = y0 - j3.cast_f64() + g3;
    let z3 = z0 - k3.cast_f64() + g3;
    let w3 = w0 - l3.cast_f64() + g3;
    let u3 = u0 - m3.cast_f64() + g3;
    let t3 = t0c - n3.cast_f64() + g3;
    let x4 = x0 - i4.cast_f64() + g4;
    let y4 = y0 - j4.cast_f64() + g4;
    let z4 = z0 - k4.cast_f64() + g4;
    let w4 = w0 - l4.cast_f64() + g4;
    let u4 = u0 - m4.cast_f64() + g4;
    let t4 = t0c - n4.cast_f64() + g4;
    let x5 = x0 - i5.cast_f64() + g5;
    let y5 = y0 - j5.cast_f64() + g5;
    let z5 = z0 - k5.cast_f64() + g5;
    let w5 = w0 - l5.cast_f64() + g5;
    let u5 = u0 - m5.cast_f64() + g5;
    let t5 = t0c - n5v.cast_f64() + g5;
    let x6 = x0 - one_f + g6;
    let y6 = y0 - one_f + g6;
    let z6 = z0 - one_f + g6;
    let w6 = w0 - one_f + g6;
    let u6 = u0 - one_f + g6;
    let t6 = t0c - one_f + g6;

    let ii = i & S::Vi64::set1(0xff);
    let jj = j & S::Vi64::set1(0xff);
    let kk = k & S::Vi64::set1(0xff);
    let ll = l & S::Vi64::set1(0xff);
    let mm = m & S::Vi64::set1(0xff);
    let nn = n & S::Vi64::set1(0xff);

    let (gi0, gi1, gi2, gi3, gi4, gi5, gi6) = unsafe {
        macro_rules! corner {
            ($i:expr, $j:expr, $k:expr, $l:expr, $m:expr, $n:expr) => {{
                let np = gather_64::<S>(&PERM64, nn + $n);
                let mp = gather_64::<S>(&PERM64, mm + $m + np);
                let lp = gather_64::<S>(&PERM64, ll + $l + mp);
                let kp = gather_64::<S>(&PERM64, kk + $k + lp);
                let jp = gather_64::<S>(&PERM64, jj + $j + kp);
                gather_64::<S>(&PERM64, ii + $i + jp)
            }};
        }
        let z0i = S::Vi64::zeroes();
        let gi0 = corner!(z0i, z0i, z0i, z0i, z0i, z0i);
        let gi1 = corner!(i1, j1, k1, l1, m1, n1);
        let gi2 = corner!(i2, j2, k2, l2, m2, n2);
        let gi3 = corner!(i3, j3, k3, l3, m3, n3);
        let gi4 = corner!(i4, j4, k4, l4, m4, n4);
        let gi5 = corner!(i5, j5, k5, l5, m5, n5v);
        let gi6 = corner!(one, one, one, one, one, one);
        (gi0, gi1, gi2, gi3, gi4, gi5, gi6)
    };

    let r2 = S::Vf64::set1(0.5);
    let s0 = r2 - (x0 * x0) - (y0 * y0) - (z0 * z0) - (w0 * w0) - (u0 * u0) - (t0c * t0c);
    let s1 = r2 - (x1 * x1) - (y1 * y1) - (z1 * z1) - (w1 * w1) - (u1 * u1) - (t1 * t1);
    let s2 = r2 - (x2 * x2) - (y2 * y2) - (z2 * z2) - (w2 * w2) - (u2 * u2) - (t2 * t2);
    let s3 = r2 - (x3 * x3) - (y3 * y3) - (z3 * z3) - (w3 * w3) - (u3 * u3) - (t3 * t3);
    let s4 = r2 - (x4 * x4) - (y4 * y4) - (z4 * z4) - (w4 * w4) - (u4 * u4) - (t4 * t4);
    let s5 = r2 - (x5 * x5) - (y5 * y5) - (z5 * z5) - (w5 * w5) - (u5 * u5) - (t5 * t5);
    let s6 = r2 - (x6 * x6) - (y6 * y6) - (z6 * z6) - (w6 * w6) - (u6 * u6) - (t6 * t6);

    let q = |t: S::Vf64| -> S::Vf64 {
        let tt = t * t;
        tt * tt
    };

    let mut n0 = q(s0) * grad6::<S>(seed, gi0, x0, y0, z0, w0, u0, t0c);
    let mut na = q(s1) * grad6::<S>(seed, gi1, x1, y1, z1, w1, u1, t1);
    let mut nb = q(s2) * grad6::<S>(seed, gi2, x2, y2, z2, w2, u2, t2);
    let mut nc = q(s3) * grad6::<S>(seed, gi3, x3, y3, z3, w3, u3, t3);
    let mut nd = q(s4) * grad6::<S>(seed, gi4, x4, y4, z4, w4, u4, t4);
    let mut ne = q(s5) * grad6::<S>(seed, gi5, x5, y5, z5, w5, u5, t5);
    let mut nf = q(s6) * grad6::<S>(seed, gi6, x6, y6, z6, w6, u6, t6);

    let zf = S::Vf64::zeroes();
    n0 = n0.and_not(s0.cmp_lt(zf));
    na = na.and_not(s1.cmp_lt(zf));
    nb = nb.and_not(s2.cmp_lt(zf));
    nc = nc.and_not(s3.cmp_lt(zf));
    nd = nd.and_not(s4.cmp_lt(zf));
    ne = ne.and_not(s5.cmp_lt(zf));
    nf = nf.and_not(s6.cmp_lt(zf));

    (n0 + na + nb + nc + nd + ne + nf) * S::Vf64::set1(SIMPLEX_6D_SCALE_64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simdeez::scalar::{F64x1, Scalar};

    fn check_bounds(min: f64, max: f64) {
        assert!((-1.0..-0.75).contains(&min), "min out of range {}", min);
        assert!(max > 0.75 && max <= 1.0, "max out of range: {}", max);
    }

    #[test]
    fn test_noise_simplex64_1d_range() {
        // Aggregate across seeds: with properly mixed seeds (issue #42) a single
        // seed's samples along one line may not reach the extremes, but the noise
        // still spans [-1, 1] across the set. (2D+ hit the extremes per-seed.)
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        for seed in 0..10 {
            for x in 0..1000 {
                let n = simplex_1d::<Scalar>(F64x1(x as f64 / 10.0), seed).0;
                min = min.min(n);
                max = max.max(n);
            }
        }
        check_bounds(min, max);
    }

    #[test]
    fn test_noise_simplex64_1d_deriv_sanity() {
        let mut avg_err = 0.0;
        const SEEDS: i64 = 10;
        const POINTS: i64 = 1000;
        for seed in 0..SEEDS {
            for x in 0..POINTS {
                // Offset a bit so we don't check derivative at lattice points, where it's always zero
                let center = x as f64 / 10.0 + 0.1234;
                const H: f64 = 0.01;
                let n0 = simplex_1d::<Scalar>(F64x1(center - H), seed).0;
                let (n1, d1) = simplex_1d_deriv::<Scalar>(F64x1(center), seed);
                let n2 = simplex_1d::<Scalar>(F64x1(center + H), seed).0;
                let (n1, d1) = (n1.0, d1.0);
                avg_err += ((n2 - (n1 + d1 * H)).abs() + (n0 - (n1 - d1 * H)).abs())
                    / (SEEDS * POINTS * 2) as f64;
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn test_noise_simplex64_2d_range() {
        for seed in 0..10 {
            let mut min = f64::INFINITY;
            let mut max = -f64::INFINITY;
            for y in 0..10 {
                for x in 0..100 {
                    let n =
                        simplex_2d::<Scalar>(F64x1(x as f64 / 10.0), F64x1(y as f64 / 10.0), seed)
                            .0;
                    min = min.min(n);
                    max = max.max(n);
                }
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn test_noise_simplex64_2d_deriv_sanity() {
        let mut avg_err = 0.0;
        const SEEDS: i64 = 10;
        const POINTS: i64 = 10;
        for seed in 0..SEEDS {
            for y in 0..POINTS {
                for x in 0..POINTS {
                    // Offset a bit so we don't check derivative at lattice points, where it's always zero
                    let center_x = x as f64 / 10.0 + 0.1234;
                    let center_y = y as f64 / 10.0 + 0.1234;
                    const H: f64 = 0.01;
                    let (value, d) =
                        simplex_2d_deriv::<Scalar>(F64x1(center_x), F64x1(center_y), seed);
                    let (value, d) = (value.0, [d[0].0, d[1].0]);
                    let left = simplex_2d::<Scalar>(F64x1(center_x - H), F64x1(center_y), seed).0;
                    let right = simplex_2d::<Scalar>(F64x1(center_x + H), F64x1(center_y), seed).0;
                    let down = simplex_2d::<Scalar>(F64x1(center_x), F64x1(center_y - H), seed).0;
                    let up = simplex_2d::<Scalar>(F64x1(center_x), F64x1(center_y + H), seed).0;
                    avg_err += ((left - (value - d[0] * H)).abs()
                        + (right - (value + d[0] * H)).abs()
                        + (down - (value - d[1] * H)).abs()
                        + (up - (value + d[1] * H)).abs())
                        / (SEEDS * POINTS * POINTS * 4) as f64;
                }
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn test_noise_simplex64_3d_range() {
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        const SEED: i64 = 0;
        for z in 0..10 {
            for y in 0..10 {
                for x in 0..10000 {
                    let n = simplex_3d::<Scalar>(
                        F64x1(x as f64 / 10.0),
                        F64x1(y as f64 / 10.0),
                        F64x1(z as f64 / 10.0),
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
    fn test_noise_simplex64_3d_deriv_sanity() {
        let mut avg_err = 0.0;
        const POINTS: i64 = 10;
        const SEED: i64 = 0;
        for z in 0..POINTS {
            for y in 0..POINTS {
                for x in 0..POINTS {
                    // Offset a bit so we don't check derivative at lattice points, where it's always zero
                    let center_x = x as f64 / 10.0 + 0.1234;
                    let center_y = y as f64 / 10.0 + 0.1234;
                    let center_z = z as f64 / 10.0 + 0.1234;
                    const H: f64 = 0.01;
                    let (value, d) = simplex_3d_deriv::<Scalar>(
                        F64x1(center_x),
                        F64x1(center_y),
                        F64x1(center_z),
                        SEED,
                    );
                    let (value, d) = (value.0, [d[0].0, d[1].0, d[2].0]);
                    let right = simplex_3d::<Scalar>(
                        F64x1(center_x + H),
                        F64x1(center_y),
                        F64x1(center_z),
                        SEED,
                    )
                    .0;
                    let up = simplex_3d::<Scalar>(
                        F64x1(center_x),
                        F64x1(center_y + H),
                        F64x1(center_z),
                        SEED,
                    )
                    .0;
                    let forward = simplex_3d::<Scalar>(
                        F64x1(center_x),
                        F64x1(center_y),
                        F64x1(center_z + H),
                        SEED,
                    )
                    .0;
                    avg_err += ((right - (value + d[0] * H)).abs()
                        + (up - (value + d[1] * H)).abs()
                        + (forward - (value + d[2] * H)).abs())
                        / (POINTS * POINTS * POINTS * 3) as f64;
                }
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn test_noise_simplex64_4d_range() {
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        const SEED: i64 = 0;
        for w in 0..10 {
            for z in 0..10 {
                for y in 0..10 {
                    for x in 0..1000 {
                        let n = simplex_4d::<Scalar>(
                            F64x1(x as f64 / 10.0),
                            F64x1(y as f64 / 10.0),
                            F64x1(z as f64 / 10.0),
                            F64x1(w as f64 / 10.0),
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
