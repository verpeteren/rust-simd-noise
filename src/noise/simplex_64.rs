use crate::noise::gradient_64::{grad1, grad2, grad3d, grad4};

use simdeez::prelude::*;

use crate::noise::cellular_32::{X_PRIME_64, Y_PRIME_64, Z_PRIME_64};
use crate::noise::gradient_64::grad3d_dot;
use crate::noise::simplex_32::{
    F2_64, F3_64, F4_64, G22_64, G24_64, G2_64, G33_64, G34_64, G3_64, G44_64, G4_64,
};

const PERM64: [i64; 512] = [
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

/// Like `simplex_1d`, but also computes the derivative
#[inline(always)]
pub unsafe fn simplex_1d_deriv<S: Simd>(x: S::Vf64, seed: i64) -> (S::Vf64, S::Vf64) {
    // Gradients are selected deterministically based on the whole part of `x`
    let ips = x.fast_floor();
    let mut i0 = ips.cast_i64();
    let i1 = (i0 + S::Vi64::set1(1)) & S::Vi64::set1(0xff);

    // the fractional part of x, i.e. the distance to the left gradient node. 0 ≤ x0 < 1.
    let x0 = x - ips;
    // signed distance to the right gradient node
    let x1 = x0 - S::Vf64::set1(1.0);

    i0 = i0 & S::Vi64::set1(0xff);
    let gi0 = S::i64gather_epi64(&PERM64, i0);
    let gi1 = S::i64gather_epi64(&PERM64, i1);

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
pub unsafe fn simplex_1d<S: Simd>(x: S::Vf64, seed: i64) -> S::Vf64 {
    simplex_1d_deriv::<S>(x, seed).0
}

/// Samples 2-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub unsafe fn simplex_2d<S: Simd>(x: S::Vf64, y: S::Vf64, seed: i64) -> S::Vf64 {
    simplex_2d_deriv::<S>(x, y, seed).0
}

/// Like `simplex_2d`, but also computes the derivative
#[inline(always)]
pub unsafe fn simplex_2d_deriv<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 2]) {
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
    let x0 = x - ips - t;
    let y0 = y - jps - t;

    let i1 = (x0.cmp_gte(y0)).cast_i64();

    let j1 = (y0.cmp_gt(x0)).cast_i64();

    // Distances to the second and third points of the enclosing simplex
    let x1 = (x0 + i1.cast_f64()) + S::Vf64::set1(G2_64);
    let y1 = (y0 + j1.cast_f64()) + S::Vf64::set1(G2_64);
    let x2 = (x0 + S::Vf64::set1(-1.0)) + S::Vf64::set1(G22_64);
    let y2 = (y0 + S::Vf64::set1(-1.0)) + S::Vf64::set1(G22_64);

    let ii = i & S::Vi64::set1(0xff);
    let jj = j & S::Vi64::set1(0xff);

    let gi0 = S::i64gather_epi64(&PERM64, ii + S::i64gather_epi64(&PERM64, jj));

    let gi1 = S::i64gather_epi64(
        &PERM64,
        (ii - i1) + S::i64gather_epi64(&PERM64, jj - j1),
    );

    let gi2 = S::i64gather_epi64(
        &PERM64,
        ii - S::Vi64::set1(-1) + S::i64gather_epi64(&PERM64, jj - S::Vi64::set1(-1)),
    );

    // Weights associated with the gradients at each corner
    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let mut t0 = S::fnmadd_pd(y0, y0, S::fnmadd_pd(x0, x0, S::Vf64::set1(0.5)));
    let mut t1 = S::fnmadd_pd(y1, y1, S::fnmadd_pd(x1, x1, S::Vf64::set1(0.5)));
    let mut t2 = S::fnmadd_pd(y2, y2, S::fnmadd_pd(x2, x2, S::Vf64::set1(0.5)));

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
    let scale = S::Vf64::set1(45.26450774985561631259);
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
pub unsafe fn simplex_3d<S: Simd>(x: S::Vf64, y: S::Vf64, z: S::Vf64, seed: i64) -> S::Vf64 {
    simplex_3d_deriv::<S>(x, y, z, seed).0
}

/// Like `simplex_3d`, but also computes the derivative
#[inline(always)]
pub unsafe fn simplex_3d_deriv<S: Simd>(
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
    x0 = x - x0 - g;
    y0 = y - y0 - g;
    z0 = z - z0 - g;

    let x0_ge_y0 = x0.cmp_gte(y0);
    let y0_ge_z0 = y0.cmp_gte(z0);
    let x0_ge_z0 = x0.cmp_gte(z0);

    let i1 = x0_ge_y0 & x0_ge_z0;
    let j1 = x0_ge_y0.and_not(y0_ge_z0);
    let k1 = x0_ge_z0.and_not(!y0_ge_z0);

    let i2 = x0_ge_y0 | x0_ge_z0;
    let j2 = (!x0_ge_y0) | y0_ge_z0;
    let k2 = !(x0_ge_z0 & y0_ge_z0);

    // Compute distances from remaining simplex vertices to input coordinates
    let x1 = x0 - i1 & S::Vf64::set1(1.0) + S::Vf64::set1(G3_64);
    let y1 = y0 - j1 & S::Vf64::set1(1.0) + S::Vf64::set1(G3_64);
    let z1 = z0 - k1 & S::Vf64::set1(1.0) + S::Vf64::set1(G3_64);

    let x2 = x0 - i2 & S::Vf64::set1(1.0) + S::Vf64::set1(F3_64);
    let y2 = y0 - j2 & S::Vf64::set1(1.0) + S::Vf64::set1(F3_64);
    let z2 = z0 - k2 & S::Vf64::set1(1.0) + S::Vf64::set1(F3_64);

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

    let v1x = i + (i1.cast_i64() & S::Vi64::set1(X_PRIME_64));
    let v1y = j + (j1.cast_i64() & S::Vi64::set1(Y_PRIME_64));
    let v1z = k + (k1.cast_i64() & S::Vi64::set1(Z_PRIME_64));
    let g1 = grad3d_dot::<S>(seed, v1x, v1y, v1z, x1, y1, z1);
    let v1 = t41 * g1;

    let v2x = i + (i2.cast_i64() & S::Vi64::set1(X_PRIME_64));
    let v2y = j + (j2.cast_i64() & S::Vi64::set1(Y_PRIME_64));
    let v2z = k + (k2.cast_i64() & S::Vi64::set1(Z_PRIME_64));
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
#[inline(always)]
pub unsafe fn simplex_4d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    seed: i64,
) -> S::Vf64 {
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
    let x0 = x - ipd - t;
    let y0 = y - jpd - t;
    let z0 = z - kpd - t;
    let w0 = w - lpd - t;

    let mut rank_x = S::Vi64::zeroes();
    let mut rank_y = S::Vi64::zeroes();
    let mut rank_z = S::Vi64::zeroes();
    let mut rank_w = S::Vi64::zeroes();

    let cond = (x0.cmp_gt(y0)).cast_i64();
    rank_x = rank_x + (cond & S::Vi64::set1(1));
    rank_y = rank_y + cond.and_not(S::Vi64::set1(1));
    let cond = (x0.cmp_gt(z0)).cast_i64();
    rank_x = rank_x + (cond & S::Vi64::set1(1));
    rank_z = rank_z + cond.and_not(S::Vi64::set1(1));
    let cond = (x0.cmp_gt(w0)).cast_i64();
    rank_x = rank_x + (cond & S::Vi64::set1(1));
    rank_w = rank_w + cond.and_not(S::Vi64::set1(1));
    let cond = (y0.cmp_gt(z0)).cast_i64();
    rank_y = rank_y + (cond & S::Vi64::set1(1));
    rank_z = rank_z + cond.and_not(S::Vi64::set1(1));
    let cond = (y0.cmp_gt(w0)).cast_i64();
    rank_y = rank_y + (cond & S::Vi64::set1(1));
    rank_w = rank_w + cond.and_not(S::Vi64::set1(1));
    let cond = (z0.cmp_gt(w0)).cast_i64();
    rank_z = rank_z + (cond & S::Vi64::set1(1));
    rank_w = rank_w + cond.and_not(S::Vi64::set1(1));

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

    let lp = S::i64gather_epi64(&PERM64, ll);
    let kp = S::i64gather_epi64(&PERM64, kk + lp);
    let jp = S::i64gather_epi64(&PERM64, jj + kp);
    let gi0 = S::i64gather_epi64(&PERM64, ii + jp);

    let lp = S::i64gather_epi64(&PERM64, ll + l1);
    let kp = S::i64gather_epi64(&PERM64, kk + k1 + lp);
    let jp = S::i64gather_epi64(&PERM64, jj + j1 + kp);
    let gi1 = S::i64gather_epi64(&PERM64,ii + i1 + jp);

    let lp = S::i64gather_epi64(&PERM64, ll + l2);
    let kp = S::i64gather_epi64(&PERM64, kk + k2 + lp);
    let jp = S::i64gather_epi64(&PERM64, jj + j2 + kp);
    let gi2 = S::i64gather_epi64(&PERM64, ii + i2 + jp);

    let lp = S::i64gather_epi64(&PERM64, ll + l3);
    let kp = S::i64gather_epi64(&PERM64, kk + k3 + lp);
    let jp = S::i64gather_epi64(&PERM64, jj + j3 + kp);
    let gi3 = S::i64gather_epi64(&PERM64, ii + i3 + jp);

    let lp = S::i64gather_epi64(&PERM64, ll + S::Vi64::set1(1));
    let kp = S::i64gather_epi64(&PERM64, kk + S::Vi64::set1(1) + lp);
    let jp = S::i64gather_epi64(&PERM64, jj + S::Vi64::set1(1) + kp);
    let gi4 = S::i64gather_epi64(&PERM64, ii + S::Vi64::set1(1) + jp);

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
    n0 = cond.and_not(n0);
    cond = t1.cmp_lt(S::Vf64::zeroes());
    n1 = cond.and_not(n1);
    cond = t2.cmp_lt(S::Vf64::zeroes());
    n2 = cond.and_not(n2);
    cond = t3.cmp_lt(S::Vf64::zeroes());
    n3 = cond.and_not(n3);
    cond = t4.cmp_lt(S::Vf64::zeroes());
    n4 = cond.and_not(n4);

    (n0 + (n1 + (n2 + (n3 + n4)))) * S::Vf64::set1(62.77772078955791)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simdeez::scalar::{F64x1, Scalar};

    fn check_bounds(min: f64, max: f64) {
        assert!(min < -0.75 && min >= -1.0, "min out of range {}", min);
        assert!(max > 0.75 && max <= 1.0, "max out of range: {}", max);
    }

    #[test]
    fn simplex_1d_range() {
        for seed in 0..10 {
            let mut min = f64::INFINITY;
            let mut max = -f64::INFINITY;
            for x in 0..1000 {
                let n = unsafe { simplex_1d::<Scalar>(F64x1(x as f64 / 10.0), seed).0 };
                min = min.min(n);
                max = max.max(n);
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn simplex_1d_deriv_sanity() {
        let mut avg_err = 0.0;
        const SEEDS: i64 = 10;
        const POINTS: i64 = 1000;
        for seed in 0..SEEDS {
            for x in 0..POINTS {
                // Offset a bit so we don't check derivative at lattice points, where it's always zero
                let center = x as f64 / 10.0 + 0.1234;
                const H: f64 = 0.01;
                let n0 = unsafe { simplex_1d::<Scalar>(F64x1(center - H), seed).0 };
                let (n1, d1) = unsafe { simplex_1d_deriv::<Scalar>(F64x1(center), seed) };
                let n2 = unsafe { simplex_1d::<Scalar>(F64x1(center + H), seed).0 };
                let (n1, d1) = (n1.0, d1.0);
                avg_err += ((n2 - (n1 + d1 * H)).abs() + (n0 - (n1 - d1 * H)).abs())
                    / (SEEDS * POINTS * 2) as f64;
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn simplex_2d_range() {
        for seed in 0..10 {
            let mut min = f64::INFINITY;
            let mut max = -f64::INFINITY;
            for y in 0..10 {
                for x in 0..100 {
                    let n = unsafe {
                        simplex_2d::<Scalar>(F64x1(x as f64 / 10.0), F64x1(y as f64 / 10.0), seed).0
                    };
                    min = min.min(n);
                    max = max.max(n);
                }
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn simplex_2d_deriv_sanity() {
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
                    let (value, d) = unsafe {
                        simplex_2d_deriv::<Scalar>(F64x1(center_x), F64x1(center_y), seed)
                    };
                    let (value, d) = (value.0, [d[0].0, d[1].0]);
                    let left = unsafe {
                        simplex_2d::<Scalar>(F64x1(center_x - H), F64x1(center_y), seed).0
                    };
                    let right = unsafe {
                        simplex_2d::<Scalar>(F64x1(center_x + H), F64x1(center_y), seed).0
                    };
                    let down = unsafe {
                        simplex_2d::<Scalar>(F64x1(center_x), F64x1(center_y - H), seed).0
                    };
                    let up = unsafe {
                        simplex_2d::<Scalar>(F64x1(center_x), F64x1(center_y + H), seed).0
                    };
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

    #[ignore]
    #[test]
    fn simplex_3d_range() {
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        const SEED: i64 = 0;
        for z in 0..10 {
            for y in 0..10 {
                for x in 0..10000 {
                    let n = unsafe {
                        simplex_3d::<Scalar>(
                            F64x1(x as f64 / 10.0),
                            F64x1(y as f64 / 10.0),
                            F64x1(z as f64 / 10.0),
                            SEED,
                        )
                        .0
                    };
                    min = min.min(n);
                    max = max.max(n);
                }
            }
        }
        check_bounds(min, max);
    }

    #[ignore]
    #[test]
    fn simplex_3d_deriv_sanity() {
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
                    let (value, d) = unsafe {
                        simplex_3d_deriv::<Scalar>(
                            F64x1(center_x),
                            F64x1(center_y),
                            F64x1(center_z),
                            SEED,
                        )
                    };
                    let (value, d) = (value.0, [d[0].0, d[1].0, d[2].0]);
                    let right = unsafe {
                        simplex_3d::<Scalar>(
                            F64x1(center_x + H),
                            F64x1(center_y),
                            F64x1(center_z),
                            SEED,
                        )
                        .0
                    };
                    let up = unsafe {
                        simplex_3d::<Scalar>(
                            F64x1(center_x),
                            F64x1(center_y + H),
                            F64x1(center_z),
                            SEED,
                        )
                        .0
                    };
                    let forward = unsafe {
                        simplex_3d::<Scalar>(
                            F64x1(center_x),
                            F64x1(center_y),
                            F64x1(center_z + H),
                            SEED,
                        )
                        .0
                    };
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
    fn simplex_4d_range() {
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        const SEED: i64 = 0;
        for w in 0..10 {
            for z in 0..10 {
                for y in 0..10 {
                    for x in 0..1000 {
                        let n = unsafe {
                            simplex_4d::<Scalar>(
                                F64x1(x as f64 / 10.0),
                                F64x1(y as f64 / 10.0),
                                F64x1(z as f64 / 10.0),
                                F64x1(w as f64 / 10.0),
                                SEED,
                            )
                            .0
                        };
                        min = min.min(n);
                        max = max.max(n);
                    }
                }
            }
        }
        check_bounds(min, max);
    }
}
