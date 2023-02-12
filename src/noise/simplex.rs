//! Low-level simplex noise primitives
//!
//! Useful for writing your own SIMD-generic code for use cases not covered by the higher level
//! interfaces.

use crate::noise::gradient::{grad1, grad2, grad3d, grad3d_dot, grad4};

use simdeez::Simd;

use std::f32;

/// Skew factor for 2D simplex noise
const F2_32: f32 = 0.36602540378;
/// Skew factor for 3D simplex noise
const F3: f32 = 1.0 / 3.0;
/// Skew factor for 4D simplex noise
const F4: f32 = 0.309016994;
/// Unskew factor for 2D simplex noise
const G2: f32 = 0.2113248654;
const G22: f32 = G2 * 2.0;
/// Unskew factor for 3D simplex noise
const G3: f32 = 1.0 / 6.0;
const G33: f32 = 3.0 / 6.0 - 1.0;
/// Unskew factor for 4D simplex noise
const G4: f32 = 0.138196601;
const G24: f32 = 2.0 * G4;
const G34: f32 = 3.0 * G4;
const G44: f32 = 4.0 * G4;

const X_PRIME: i32 = 1619;
const Y_PRIME: i32 = 31337;
const Z_PRIME: i32 = 6791;

const PERM: [i32; 512] = [
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
pub unsafe fn simplex_1d_deriv<S: Simd>(x: S::Vf32, seed: i32) -> (S::Vf32, S::Vf32) {
    // Gradients are selected deterministically based on the whole part of `x`
    let ips = S::fast_floor_ps(x);
    let mut i0 = S::cvtps_epi32(ips);
    let i1 = S::and_epi32(S::add_epi32(i0, S::set1_epi32(1)), S::set1_epi32(0xff));

    // the fractional part of x, i.e. the distance to the left gradient node. 0 ≤ x0 < 1.
    let x0 = S::sub_ps(x, ips);
    // signed distance to the right gradient node
    let x1 = S::sub_ps(x0, S::set1_ps(1.0));

    i0 = S::and_epi32(i0, S::set1_epi32(0xff));
    let gi0 = S::i32gather_epi32(&PERM, i0);
    let gi1 = S::i32gather_epi32(&PERM, i1);

    // Compute the contribution from the first gradient
    let x20 = S::mul_ps(x0, x0); // x^2_0
    let t0 = S::sub_ps(S::set1_ps(1.0), x20); // t_0
    let t20 = S::mul_ps(t0, t0); // t^2_0
    let t40 = S::mul_ps(t20, t20); // t^4_0
    let gx0 = grad1::<S>(seed, gi0);
    let n0 = S::mul_ps(t40, gx0 * x0);
    // n0 = (1 - x0^2)^4 * x0 * grad

    // Compute the contribution from the second gradient
    let x21 = S::mul_ps(x1, x1); // x^2_1
    let t1 = S::sub_ps(S::set1_ps(1.0), x21); // t_1
    let t21 = S::mul_ps(t1, t1); // t^2_1
    let t41 = S::mul_ps(t21, t21); // t^4_1
    let gx1 = grad1::<S>(seed, gi1);
    let n1 = S::mul_ps(t41, gx1 * x1);

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

    let value = S::add_ps(n0, n1) * S::set1_ps(SCALE);
    let derivative =
        ((t20 * t0 * gx0 * x20 + t21 * t1 * gx1 * x21) * S::set1_ps(-8.0) + t40 * gx0 + t41 * gx1)
            * S::set1_ps(SCALE);
    (value, derivative)
}

/// Samples 1-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub unsafe fn simplex_1d<S: Simd>(x: S::Vf32, seed: i32) -> S::Vf32 {
    simplex_1d_deriv::<S>(x, seed).0
}

/// Samples 2-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub unsafe fn simplex_2d<S: Simd>(x: S::Vf32, y: S::Vf32, seed: i32) -> S::Vf32 {
    simplex_2d_deriv::<S>(x, y, seed).0
}

/// Like `simplex_2d`, but also computes the derivative
#[inline(always)]
pub unsafe fn simplex_2d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 2]) {
    // Skew to distort simplexes with side length sqrt(2)/sqrt(3) until they make up
    // squares
    let s = S::mul_ps(S::set1_ps(F2_32), S::add_ps(x, y));
    let ips = S::floor_ps(S::add_ps(x, s));
    let jps = S::floor_ps(S::add_ps(y, s));

    // Integer coordinates for the base vertex of the triangle
    let i = S::cvtps_epi32(ips);
    let j = S::cvtps_epi32(jps);

    let t = S::mul_ps(S::cvtepi32_ps(S::add_epi32(i, j)), S::set1_ps(G2));

    // Unskewed distances to the first point of the enclosing simplex
    let x0 = S::sub_ps(x, S::sub_ps(ips, t));
    let y0 = S::sub_ps(y, S::sub_ps(jps, t));

    let i1 = S::castps_epi32(S::cmpge_ps(x0, y0));

    let j1 = S::castps_epi32(S::cmpgt_ps(y0, x0));

    // Distances to the second and third points of the enclosing simplex
    let x1 = S::add_ps(S::add_ps(x0, S::cvtepi32_ps(i1)), S::set1_ps(G2));
    let y1 = S::add_ps(S::add_ps(y0, S::cvtepi32_ps(j1)), S::set1_ps(G2));
    let x2 = S::add_ps(S::add_ps(x0, S::set1_ps(-1.0)), S::set1_ps(G22));
    let y2 = S::add_ps(S::add_ps(y0, S::set1_ps(-1.0)), S::set1_ps(G22));

    let ii = S::and_epi32(i, S::set1_epi32(0xff));
    let jj = S::and_epi32(j, S::set1_epi32(0xff));

    let gi0 = S::i32gather_epi32(&PERM, S::add_epi32(ii, S::i32gather_epi32(&PERM, jj)));

    let gi1 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, i1),
            S::i32gather_epi32(&PERM, S::sub_epi32(jj, j1)),
        ),
    );

    let gi2 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, S::set1_epi32(-1)),
            S::i32gather_epi32(&PERM, S::sub_epi32(jj, S::set1_epi32(-1))),
        ),
    );

    // Weights associated with the gradients at each corner
    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let mut t0 = S::fnmadd_ps(y0, y0, S::fnmadd_ps(x0, x0, S::set1_ps(0.5)));
    let mut t1 = S::fnmadd_ps(y1, y1, S::fnmadd_ps(x1, x1, S::set1_ps(0.5)));
    let mut t2 = S::fnmadd_ps(y2, y2, S::fnmadd_ps(x2, x2, S::set1_ps(0.5)));

    // Zero out negative weights
    t0 &= S::cmpge_ps(t0, S::setzero_ps());
    t1 &= S::cmpge_ps(t1, S::setzero_ps());
    t2 &= S::cmpge_ps(t2, S::setzero_ps());

    let t20 = S::mul_ps(t0, t0);
    let t40 = S::mul_ps(t20, t20);
    let t21 = S::mul_ps(t1, t1);
    let t41 = S::mul_ps(t21, t21);
    let t22 = S::mul_ps(t2, t2);
    let t42 = S::mul_ps(t22, t22);

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
    let scale = S::set1_ps(45.26450774985561631259);
    let value = S::add_ps(n0, S::add_ps(n1, n2)) * scale;
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
        dnoise_dx *= S::set1_ps(-8.0);
        dnoise_dy *= S::set1_ps(-8.0);
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
pub unsafe fn simplex_3d<S: Simd>(x: S::Vf32, y: S::Vf32, z: S::Vf32, seed: i32) -> S::Vf32 {
    simplex_3d_deriv::<S>(x, y, z, seed).0
}

/// Like `simplex_3d`, but also computes the derivative
#[inline(always)]
pub unsafe fn simplex_3d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
) -> (S::Vf32, [S::Vf32; 3]) {
    // Find skewed simplex grid coordinates associated with the input coordinates
    let f = S::mul_ps(S::set1_ps(F3), S::add_ps(S::add_ps(x, y), z));
    let mut x0 = S::fast_floor_ps(S::add_ps(x, f));
    let mut y0 = S::fast_floor_ps(S::add_ps(y, f));
    let mut z0 = S::fast_floor_ps(S::add_ps(z, f));

    // Integer grid coordinates
    let i = S::mullo_epi32(S::cvtps_epi32(x0), S::set1_epi32(X_PRIME));
    let j = S::mullo_epi32(S::cvtps_epi32(y0), S::set1_epi32(Y_PRIME));
    let k = S::mullo_epi32(S::cvtps_epi32(z0), S::set1_epi32(Z_PRIME));

    // Compute distance from first simplex vertex to input coordinates
    let g = S::mul_ps(S::set1_ps(G3), S::add_ps(S::add_ps(x0, y0), z0));
    x0 = S::sub_ps(x, S::sub_ps(x0, g));
    y0 = S::sub_ps(y, S::sub_ps(y0, g));
    z0 = S::sub_ps(z, S::sub_ps(z0, g));

    let x0_ge_y0 = S::cmpge_ps(x0, y0);
    let y0_ge_z0 = S::cmpge_ps(y0, z0);
    let x0_ge_z0 = S::cmpge_ps(x0, z0);

    let i1 = x0_ge_y0 & x0_ge_z0;
    let j1 = S::andnot_ps(x0_ge_y0, y0_ge_z0);
    let k1 = S::andnot_ps(x0_ge_z0, !y0_ge_z0);

    let i2 = x0_ge_y0 | x0_ge_z0;
    let j2 = (!x0_ge_y0) | y0_ge_z0;
    let k2 = !(x0_ge_z0 & y0_ge_z0);

    // Compute distances from remaining simplex vertices to input coordinates
    let x1 = S::add_ps(S::sub_ps(x0, i1 & S::set1_ps(1.0)), S::set1_ps(G3));
    let y1 = S::add_ps(S::sub_ps(y0, j1 & S::set1_ps(1.0)), S::set1_ps(G3));
    let z1 = S::add_ps(S::sub_ps(z0, k1 & S::set1_ps(1.0)), S::set1_ps(G3));

    let x2 = S::add_ps(S::sub_ps(x0, i2 & S::set1_ps(1.0)), S::set1_ps(F3));
    let y2 = S::add_ps(S::sub_ps(y0, j2 & S::set1_ps(1.0)), S::set1_ps(F3));
    let z2 = S::add_ps(S::sub_ps(z0, k2 & S::set1_ps(1.0)), S::set1_ps(F3));

    let x3 = S::add_ps(x0, S::set1_ps(G33));
    let y3 = S::add_ps(y0, S::set1_ps(G33));
    let z3 = S::add_ps(z0, S::set1_ps(G33));

    // Compute base weight factors associated with each vertex, `0.6 - v . v` where v is the
    // distance to the vertex. Strictly the constant should be 0.5, but 0.6 is thought by Gustavson
    // to give visually better results at the cost of subtle discontinuities.
    //#define SIMDf_NMUL_ADD(a,b,c) = SIMDf_SUB(c, SIMDf_MUL(a,b)
    let mut t0 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(S::set1_ps(0.6), S::mul_ps(x0, x0)),
            S::mul_ps(y0, y0),
        ),
        S::mul_ps(z0, z0),
    );
    let mut t1 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(S::set1_ps(0.6), S::mul_ps(x1, x1)),
            S::mul_ps(y1, y1),
        ),
        S::mul_ps(z1, z1),
    );
    let mut t2 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(S::set1_ps(0.6), S::mul_ps(x2, x2)),
            S::mul_ps(y2, y2),
        ),
        S::mul_ps(z2, z2),
    );
    let mut t3 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(S::set1_ps(0.6), S::mul_ps(x3, x3)),
            S::mul_ps(y3, y3),
        ),
        S::mul_ps(z3, z3),
    );

    // Zero out negative weights
    t0 &= S::cmpge_ps(t0, S::setzero_ps());
    t1 &= S::cmpge_ps(t1, S::setzero_ps());
    t2 &= S::cmpge_ps(t2, S::setzero_ps());
    t3 &= S::cmpge_ps(t3, S::setzero_ps());

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

    let v1x = S::add_epi32(i, S::and_epi32(S::castps_epi32(i1), S::set1_epi32(X_PRIME)));
    let v1y = S::add_epi32(j, S::and_epi32(S::castps_epi32(j1), S::set1_epi32(Y_PRIME)));
    let v1z = S::add_epi32(k, S::and_epi32(S::castps_epi32(k1), S::set1_epi32(Z_PRIME)));
    let g1 = grad3d_dot::<S>(seed, v1x, v1y, v1z, x1, y1, z1);
    let v1 = t41 * g1;

    let v2x = S::add_epi32(i, S::and_epi32(S::castps_epi32(i2), S::set1_epi32(X_PRIME)));
    let v2y = S::add_epi32(j, S::and_epi32(S::castps_epi32(j2), S::set1_epi32(Y_PRIME)));
    let v2z = S::add_epi32(k, S::and_epi32(S::castps_epi32(k2), S::set1_epi32(Z_PRIME)));
    let g2 = grad3d_dot::<S>(seed, v2x, v2y, v2z, x2, y2, z2);
    let v2 = t42 * g2;

    //SIMDf v3 = SIMDf_MASK(n3, SIMDf_MUL(SIMDf_MUL(t3, t3), FUNC(GradCoord)(seed, SIMDi_ADD(i, SIMDi_NUM(xPrime)), SIMDi_ADD(j, SIMDi_NUM(yPrime)), SIMDi_ADD(k, SIMDi_NUM(zPrime)), x3, y3, z3)));
    let v3x = S::add_epi32(i, S::set1_epi32(X_PRIME));
    let v3y = S::add_epi32(j, S::set1_epi32(Y_PRIME));
    let v3z = S::add_epi32(k, S::set1_epi32(Z_PRIME));
    //define SIMDf_MASK(m,a) SIMDf_AND(SIMDf_CAST_TO_FLOAT(m),a)
    let g3 = grad3d_dot::<S>(seed, v3x, v3y, v3z, x3, y3, z3);
    let v3 = t43 * g3;

    let p1 = S::add_ps(v3, v2);
    let p2 = S::add_ps(p1, v1);

    // Scaling factor found by numerical approximation
    let scale = S::set1_ps(32.69587493801679);
    let result = S::add_ps(p2, v0) * scale;
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
        dnoise_dx *= S::set1_ps(-8.0);
        dnoise_dy *= S::set1_ps(-8.0);
        dnoise_dz *= S::set1_ps(-8.0);
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
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    //
    // Determine which simplex these points lie in, and compute the distance along each axis to each
    // vertex of the simplex
    //

    let s = S::mul_ps(S::set1_ps(F4), S::add_ps(x, S::add_ps(y, S::add_ps(z, w))));

    let ips = S::floor_ps(S::add_ps(x, s));
    let jps = S::floor_ps(S::add_ps(y, s));
    let kps = S::floor_ps(S::add_ps(z, s));
    let lps = S::floor_ps(S::add_ps(w, s));

    let i = S::cvtps_epi32(ips);
    let j = S::cvtps_epi32(jps);
    let k = S::cvtps_epi32(kps);
    let l = S::cvtps_epi32(lps);

    let t = S::mul_ps(
        S::cvtepi32_ps(S::add_epi32(i, S::add_epi32(j, S::add_epi32(k, l)))),
        S::set1_ps(G4),
    );
    let x0 = S::sub_ps(x, S::sub_ps(ips, t));
    let y0 = S::sub_ps(y, S::sub_ps(jps, t));
    let z0 = S::sub_ps(z, S::sub_ps(kps, t));
    let w0 = S::sub_ps(w, S::sub_ps(lps, t));

    let mut rank_x = S::setzero_epi32();
    let mut rank_y = S::setzero_epi32();
    let mut rank_z = S::setzero_epi32();
    let mut rank_w = S::setzero_epi32();

    let cond = S::castps_epi32(S::cmpgt_ps(x0, y0));
    rank_x = S::add_epi32(rank_x, S::and_epi32(cond, S::set1_epi32(1)));
    rank_y = S::add_epi32(rank_y, S::andnot_epi32(cond, S::set1_epi32(1)));
    let cond = S::castps_epi32(S::cmpgt_ps(x0, z0));
    rank_x = S::add_epi32(rank_x, S::and_epi32(cond, S::set1_epi32(1)));
    rank_z = S::add_epi32(rank_z, S::andnot_epi32(cond, S::set1_epi32(1)));
    let cond = S::castps_epi32(S::cmpgt_ps(x0, w0));
    rank_x = S::add_epi32(rank_x, S::and_epi32(cond, S::set1_epi32(1)));
    rank_w = S::add_epi32(rank_w, S::andnot_epi32(cond, S::set1_epi32(1)));
    let cond = S::castps_epi32(S::cmpgt_ps(y0, z0));
    rank_y = S::add_epi32(rank_y, S::and_epi32(cond, S::set1_epi32(1)));
    rank_z = S::add_epi32(rank_z, S::andnot_epi32(cond, S::set1_epi32(1)));
    let cond = S::castps_epi32(S::cmpgt_ps(y0, w0));
    rank_y = S::add_epi32(rank_y, S::and_epi32(cond, S::set1_epi32(1)));
    rank_w = S::add_epi32(rank_w, S::andnot_epi32(cond, S::set1_epi32(1)));
    let cond = S::castps_epi32(S::cmpgt_ps(z0, w0));
    rank_z = S::add_epi32(rank_z, S::and_epi32(cond, S::set1_epi32(1)));
    rank_w = S::add_epi32(rank_w, S::andnot_epi32(cond, S::set1_epi32(1)));

    let cond = S::cmpgt_epi32(rank_x, S::set1_epi32(2));
    let i1 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_y, S::set1_epi32(2));
    let j1 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_z, S::set1_epi32(2));
    let k1 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_w, S::set1_epi32(2));
    let l1 = S::and_epi32(S::set1_epi32(1), cond);

    let cond = S::cmpgt_epi32(rank_x, S::set1_epi32(1));
    let i2 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_y, S::set1_epi32(1));
    let j2 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_z, S::set1_epi32(1));
    let k2 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_w, S::set1_epi32(1));
    let l2 = S::and_epi32(S::set1_epi32(1), cond);

    let cond = S::cmpgt_epi32(rank_x, S::setzero_epi32());
    let i3 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_y, S::setzero_epi32());
    let j3 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_z, S::setzero_epi32());
    let k3 = S::and_epi32(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_w, S::setzero_epi32());
    let l3 = S::and_epi32(S::set1_epi32(1), cond);

    let x1 = S::add_ps(S::sub_ps(x0, S::cvtepi32_ps(i1)), S::set1_ps(G4));
    let y1 = S::add_ps(S::sub_ps(y0, S::cvtepi32_ps(j1)), S::set1_ps(G4));
    let z1 = S::add_ps(S::sub_ps(z0, S::cvtepi32_ps(k1)), S::set1_ps(G4));
    let w1 = S::add_ps(S::sub_ps(w0, S::cvtepi32_ps(l1)), S::set1_ps(G4));
    let x2 = S::add_ps(S::sub_ps(x0, S::cvtepi32_ps(i2)), S::set1_ps(G24));
    let y2 = S::add_ps(S::sub_ps(y0, S::cvtepi32_ps(j2)), S::set1_ps(G24));
    let z2 = S::add_ps(S::sub_ps(z0, S::cvtepi32_ps(k2)), S::set1_ps(G24));
    let w2 = S::add_ps(S::sub_ps(w0, S::cvtepi32_ps(l2)), S::set1_ps(G24));
    let x3 = S::add_ps(S::sub_ps(x0, S::cvtepi32_ps(i3)), S::set1_ps(G34));
    let y3 = S::add_ps(S::sub_ps(y0, S::cvtepi32_ps(j3)), S::set1_ps(G34));
    let z3 = S::add_ps(S::sub_ps(z0, S::cvtepi32_ps(k3)), S::set1_ps(G34));
    let w3 = S::add_ps(S::sub_ps(w0, S::cvtepi32_ps(l3)), S::set1_ps(G34));
    let x4 = S::add_ps(S::sub_ps(x0, S::set1_ps(1.0)), S::set1_ps(G44));
    let y4 = S::add_ps(S::sub_ps(y0, S::set1_ps(1.0)), S::set1_ps(G44));
    let z4 = S::add_ps(S::sub_ps(z0, S::set1_ps(1.0)), S::set1_ps(G44));
    let w4 = S::add_ps(S::sub_ps(w0, S::set1_ps(1.0)), S::set1_ps(G44));

    let ii = S::and_epi32(i, S::set1_epi32(0xff));
    let jj = S::and_epi32(j, S::set1_epi32(0xff));
    let kk = S::and_epi32(k, S::set1_epi32(0xff));
    let ll = S::and_epi32(l, S::set1_epi32(0xff));

    let lp = S::i32gather_epi32(&PERM, ll);
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(kk, lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(jj, kp));
    let gi0 = S::i32gather_epi32(&PERM, S::add_epi32(ii, jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, l1));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, k1), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, j1), kp));
    let gi1 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, i1), jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, l2));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, k2), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, j2), kp));
    let gi2 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, i2), jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, l3));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, k3), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, j3), kp));
    let gi3 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, i3), jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, S::set1_epi32(1)));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, S::set1_epi32(1)), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, S::set1_epi32(1)), kp));
    let gi4 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, S::set1_epi32(1)), jp));

    //
    // Compute base weight factors associated with each vertex
    //

    let t0 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x0, x0)),
                S::mul_ps(y0, y0),
            ),
            S::mul_ps(z0, z0),
        ),
        S::mul_ps(w0, w0),
    );
    let t1 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x1, x1)),
                S::mul_ps(y1, y1),
            ),
            S::mul_ps(z1, z1),
        ),
        S::mul_ps(w1, w1),
    );
    let t2 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x2, x2)),
                S::mul_ps(y2, y2),
            ),
            S::mul_ps(z2, z2),
        ),
        S::mul_ps(w2, w2),
    );
    let t3 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x3, x3)),
                S::mul_ps(y3, y3),
            ),
            S::mul_ps(z3, z3),
        ),
        S::mul_ps(w3, w3),
    );
    let t4 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x4, x4)),
                S::mul_ps(y4, y4),
            ),
            S::mul_ps(z4, z4),
        ),
        S::mul_ps(w4, w4),
    );
    // Cube each weight
    let mut t0q = S::mul_ps(t0, t0);
    t0q = S::mul_ps(t0q, t0q);
    let mut t1q = S::mul_ps(t1, t1);
    t1q = S::mul_ps(t1q, t1q);
    let mut t2q = S::mul_ps(t2, t2);
    t2q = S::mul_ps(t2q, t2q);
    let mut t3q = S::mul_ps(t3, t3);
    t3q = S::mul_ps(t3q, t3q);
    let mut t4q = S::mul_ps(t4, t4);
    t4q = S::mul_ps(t4q, t4q);

    let mut n0 = S::mul_ps(t0q, grad4::<S>(seed, gi0, x0, y0, z0, w0));
    let mut n1 = S::mul_ps(t1q, grad4::<S>(seed, gi1, x1, y1, z1, w1));
    let mut n2 = S::mul_ps(t2q, grad4::<S>(seed, gi2, x2, y2, z2, w2));
    let mut n3 = S::mul_ps(t3q, grad4::<S>(seed, gi3, x3, y3, z3, w3));
    let mut n4 = S::mul_ps(t4q, grad4::<S>(seed, gi4, x4, y4, z4, w4));

    // Discard contributions whose base weight factors are negative
    let mut cond = S::cmplt_ps(t0, S::setzero_ps());
    n0 = S::andnot_ps(cond, n0);
    cond = S::cmplt_ps(t1, S::setzero_ps());
    n1 = S::andnot_ps(cond, n1);
    cond = S::cmplt_ps(t2, S::setzero_ps());
    n2 = S::andnot_ps(cond, n2);
    cond = S::cmplt_ps(t3, S::setzero_ps());
    n3 = S::andnot_ps(cond, n3);
    cond = S::cmplt_ps(t4, S::setzero_ps());
    n4 = S::andnot_ps(cond, n4);

    // Scaling factor found by numerical approximation
    S::add_ps(n0, S::add_ps(n1, S::add_ps(n2, S::add_ps(n3, n4)))) * S::set1_ps(62.77772078955791)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simdeez::scalar::{F32x1, Scalar};

    fn check_bounds(min: f32, max: f32) {
        assert!(min < -0.75 && min >= -1.0, "min out of range {}", min);
        assert!(max > 0.75 && max <= 1.0, "max out of range: {}", max);
    }

    #[test]
    fn simplex_1d_range() {
        for seed in 0..10 {
            let mut min = f32::INFINITY;
            let mut max = -f32::INFINITY;
            for x in 0..1000 {
                let n = unsafe { simplex_1d::<Scalar>(F32x1(x as f32 / 10.0), seed).0 };
                min = min.min(n);
                max = max.max(n);
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn simplex_1d_deriv_sanity() {
        let mut avg_err = 0.0;
        const SEEDS: i32 = 10;
        const POINTS: i32 = 1000;
        for seed in 0..SEEDS {
            for x in 0..POINTS {
                // Offset a bit so we don't check derivative at lattice points, where it's always zero
                let center = x as f32 / 10.0 + 0.1234;
                const H: f32 = 0.01;
                let n0 = unsafe { simplex_1d::<Scalar>(F32x1(center - H), seed).0 };
                let (n1, d1) = unsafe { simplex_1d_deriv::<Scalar>(F32x1(center), seed) };
                let n2 = unsafe { simplex_1d::<Scalar>(F32x1(center + H), seed).0 };
                let (n1, d1) = (n1.0, d1.0);
                avg_err += ((n2 - (n1 + d1 * H)).abs() + (n0 - (n1 - d1 * H)).abs())
                    / (SEEDS * POINTS * 2) as f32;
            }
        }
        assert!(avg_err < 1e-3);
    }

    #[test]
    fn simplex_2d_range() {
        for seed in 0..10 {
            let mut min = f32::INFINITY;
            let mut max = -f32::INFINITY;
            for y in 0..10 {
                for x in 0..100 {
                    let n = unsafe {
                        simplex_2d::<Scalar>(F32x1(x as f32 / 10.0), F32x1(y as f32 / 10.0), seed).0
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
        const SEEDS: i32 = 10;
        const POINTS: i32 = 10;
        for seed in 0..SEEDS {
            for y in 0..POINTS {
                for x in 0..POINTS {
                    // Offset a bit so we don't check derivative at lattice points, where it's always zero
                    let center_x = x as f32 / 10.0 + 0.1234;
                    let center_y = y as f32 / 10.0 + 0.1234;
                    const H: f32 = 0.01;
                    let (value, d) = unsafe {
                        simplex_2d_deriv::<Scalar>(F32x1(center_x), F32x1(center_y), seed)
                    };
                    let (value, d) = (value.0, [d[0].0, d[1].0]);
                    let left = unsafe {
                        simplex_2d::<Scalar>(F32x1(center_x - H), F32x1(center_y), seed).0
                    };
                    let right = unsafe {
                        simplex_2d::<Scalar>(F32x1(center_x + H), F32x1(center_y), seed).0
                    };
                    let down = unsafe {
                        simplex_2d::<Scalar>(F32x1(center_x), F32x1(center_y - H), seed).0
                    };
                    let up = unsafe {
                        simplex_2d::<Scalar>(F32x1(center_x), F32x1(center_y + H), seed).0
                    };
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
    fn simplex_3d_range() {
        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;
        const SEED: i32 = 0;
        for z in 0..10 {
            for y in 0..10 {
                for x in 0..10000 {
                    let n = unsafe {
                        simplex_3d::<Scalar>(
                            F32x1(x as f32 / 10.0),
                            F32x1(y as f32 / 10.0),
                            F32x1(z as f32 / 10.0),
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

    #[test]
    fn simplex_3d_deriv_sanity() {
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
                    let (value, d) = unsafe {
                        simplex_3d_deriv::<Scalar>(
                            F32x1(center_x),
                            F32x1(center_y),
                            F32x1(center_z),
                            SEED,
                        )
                    };
                    let (value, d) = (value.0, [d[0].0, d[1].0, d[2].0]);
                    let right = unsafe {
                        simplex_3d::<Scalar>(
                            F32x1(center_x + H),
                            F32x1(center_y),
                            F32x1(center_z),
                            SEED,
                        )
                        .0
                    };
                    let up = unsafe {
                        simplex_3d::<Scalar>(
                            F32x1(center_x),
                            F32x1(center_y + H),
                            F32x1(center_z),
                            SEED,
                        )
                        .0
                    };
                    let forward = unsafe {
                        simplex_3d::<Scalar>(
                            F32x1(center_x),
                            F32x1(center_y),
                            F32x1(center_z + H),
                            SEED,
                        )
                        .0
                    };
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
    fn simplex_4d_range() {
        let mut min = f32::INFINITY;
        let mut max = -f32::INFINITY;
        const SEED: i32 = 0;
        for w in 0..10 {
            for z in 0..10 {
                for y in 0..10 {
                    for x in 0..1000 {
                        let n = unsafe {
                            simplex_4d::<Scalar>(
                                F32x1(x as f32 / 10.0),
                                F32x1(y as f32 / 10.0),
                                F32x1(z as f32 / 10.0),
                                F32x1(w as f32 / 10.0),
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
