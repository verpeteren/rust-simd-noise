//! Steerable Perlin noise (Rice, SIGGRAPH 2025), 2D/3D.
//!
//! Two changes to classic Perlin: the corner gradient is projected through an
//! anisotropic metric, and the interpolation weight also uses that metric.
//! Values use the reference formulation's weighted sum; no extra normalization
//! is applied.

use simdeez::math::{SimdMathF32Core, SimdMathF32InverseTrig};
use simdeez::prelude::*;

const TAU: f32 = std::f32::consts::TAU;
const HASH_X: f32 = 64.253_75;
const HASH_Y: f32 = 23.275_365;
const HASH_Z: f32 = 86.296_78;
const HASH_SCALE: f32 = 59482.754;
const EPS: f32 = 1.0e-5;

/// Symmetric 2D metric, row-major `[m00, m01, m11]`.
pub type Metric2 = [f32; 3];
/// Symmetric 3D metric, row-major `[m00, m01, m02, m11, m12, m22]`.
pub type Metric3 = [f32; 6];

fn normalize2(v: [f32; 2]) -> [f32; 2] {
    let len = (v[0] * v[0] + v[1] * v[1]).sqrt();
    [v[0] / len, v[1] / len]
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

/// Build the 2D metric used by the kernel.
pub fn metric_2d(direction: [f32; 2], eigenvalue_sum: f32, k: f32) -> Metric2 {
    let d = normalize2(direction);
    let x = [d[0].abs().max(EPS), d[1].abs().max(EPS)];
    let e0 = normalize2([-x[1] / x[0], 1.0]);
    let e1 = normalize2([x[0] / x[1], 1.0]);
    let low = (eigenvalue_sum - k) / (2.0 - 1.0) - 1.0e-4;
    // evals [0, 1] -> [low, k]
    [
        low * e0[0] * e0[0] + k * e1[0] * e1[0],
        low * e0[0] * e0[1] + k * e1[0] * e1[1],
        low * e0[1] * e0[1] + k * e1[1] * e1[1],
    ]
}

/// Build the 3D metric used by the kernel.
pub fn metric_3d(direction: [f32; 3], eigenvalue_sum: f32, k: f32) -> Metric3 {
    let d = normalize3(direction);
    let x = [
        d[0].abs().max(EPS),
        d[1].abs().max(EPS),
        d[2].abs().max(EPS),
    ];
    let e0 = normalize3([-x[2] / x[0], 0.0, 1.0]);
    let e1 = normalize3([-x[1] / x[0], 1.0, 0.0]);
    let e2 = normalize3([x[0] / x[2], x[1] / x[2], 1.0]);
    let low = (eigenvalue_sum - k) / (3.0 - 1.0) - 1.0e-4;
    let evecs = [e0, e1, e2];
    let mapped = [low, low, k];
    let mut m = [0.0f32; 6];
    let idx = [[0, 1, 2], [1, 3, 4], [2, 4, 5]];
    for row in 0..3 {
        for col in row..3 {
            let mut value = 0.0;
            for axis in 0..3 {
                value += mapped[axis] * evecs[axis][row] * evecs[axis][col];
            }
            m[idx[row][col]] = value;
        }
    }
    m
}

#[inline(always)]
fn seed_offset(seed: i32) -> [f32; 3] {
    let s = seed as f32;
    [s * 0.123_456, s * 0.654_321, s * 0.987_654]
}

#[inline(always)]
fn random3<S: Simd>(p: [S::Vf32; 3]) -> S::Vf32 {
    let dot =
        p[0] * S::Vf32::set1(HASH_X) + p[1] * S::Vf32::set1(HASH_Y) + p[2] * S::Vf32::set1(HASH_Z);
    let value = dot.sin_u35() * S::Vf32::set1(HASH_SCALE);
    value - value.floor()
}

#[inline(always)]
fn smootherstep<S: Simd>(u: S::Vf32) -> S::Vf32 {
    let u2 = u * u;
    let u3 = u2 * u;
    let value = u3 * (u2 * S::Vf32::set1(6.0) - u * S::Vf32::set1(15.0) + S::Vf32::set1(10.0));
    value.max(S::Vf32::zeroes()).min(S::Vf32::set1(1.0))
}

#[inline(always)]
fn interp<S: Simd>(u: S::Vf32) -> S::Vf32 {
    S::Vf32::set1(1.0f32) - smootherstep::<S>(u.abs())
}

#[inline(always)]
fn smootherstep_deriv<S: Simd>(u: S::Vf32) -> S::Vf32 {
    // smootherstep clamps its input, so the derivative is zero outside [0, 1].
    let u = u.max(S::Vf32::zeroes()).min(S::Vf32::set1(1.0f32));
    // d/du (6u^5 - 15u^4 + 10u^3) = 30u^2 (u - 1)^2
    let om = u - S::Vf32::set1(1.0f32);
    u * u * om * om * S::Vf32::set1(30.0f32)
}

#[inline(always)]
fn sign<S: Simd>(v: S::Vf32) -> S::Vf32 {
    let negative = v.cmp_lt(S::Vf32::zeroes());
    negative.blendv(S::Vf32::set1(1.0f32), S::Vf32::set1(-1.0f32))
}

#[inline(always)]
fn interp_deriv<S: Simd>(u: S::Vf32) -> S::Vf32 {
    // d/du [1 - smootherstep(|u|)] = -sign(u) * smootherstep'(|u|)
    S::Vf32::zeroes() - sign::<S>(u) * smootherstep_deriv::<S>(u.abs())
}

#[inline(always)]
fn rsphere<S: Simd>(p: [S::Vf32; 3]) -> [S::Vf32; 3] {
    let u = random3::<S>([
        p[0] + S::Vf32::set1(0.01),
        p[1] + S::Vf32::set1(0.01),
        p[2] + S::Vf32::set1(0.01),
    ]);
    let v = random3::<S>([
        p[0] + S::Vf32::set1(0.02),
        p[1] + S::Vf32::set1(0.02),
        p[2] + S::Vf32::set1(0.02),
    ]);
    let w = random3::<S>([
        p[0] + S::Vf32::set1(0.03),
        p[1] + S::Vf32::set1(0.03),
        p[2] + S::Vf32::set1(0.03),
    ]);
    let theta = u * S::Vf32::set1(TAU);
    let phi = (v * S::Vf32::set1(2.0) - S::Vf32::set1(1.0)).acos_u35();
    let radius = (w.ln_u35() * S::Vf32::set1(1.0 / 3.0)).exp_u35();
    let sin_phi = phi.sin_u35();
    [
        radius * sin_phi * theta.cos_u35(),
        radius * sin_phi * theta.sin_u35(),
        radius * phi.cos_u35(),
    ]
}

#[inline(always)]
fn unit2<S: Simd>(p: [S::Vf32; 2]) -> [S::Vf32; 2] {
    let angle = random3::<S>([p[0], p[1], S::Vf32::zeroes()]) * S::Vf32::set1(TAU);
    [angle.cos_u35(), angle.sin_u35()]
}

/// Steerable Perlin value at a 2D point. Coordinates are in noise space.
#[inline(always)]
pub fn steerable_perlin_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
    metric: Metric2,
    gain: f32,
) -> S::Vf32 {
    let offset = seed_offset(seed);
    let px = x.floor();
    let py = y.floor();
    let fx = x - px;
    let fy = y - py;
    let blend_x = interp::<S>(fx);
    let blend_y = interp::<S>(fy);
    let m00 = S::Vf32::set1(metric[0]);
    let m01 = S::Vf32::set1(metric[1]);
    let m11 = S::Vf32::set1(metric[2]);

    let mut sum = S::Vf32::zeroes();
    for i in 0..=1 {
        for j in 0..=1 {
            let ox = S::Vf32::set1(i as f32);
            let oy = S::Vf32::set1(j as f32);
            let gx = px + ox + S::Vf32::set1(offset[0]);
            let gy = py + oy + S::Vf32::set1(offset[1]);
            let r = unit2::<S>([gx, gy]);
            let vx = ox - fx;
            let vy = oy - fy;
            let mvx = m00 * vx + m01 * vy;
            let mvy = m01 * vx + m11 * vy;
            let d = r[0] * mvx + r[1] * mvy;
            let q = vx * mvx + vy * mvy;
            let wx = (ox - blend_x).abs();
            let wy = (oy - blend_y).abs();
            sum += d * wx * wy * interp::<S>(q);
        }
    }
    sum * S::Vf32::set1(gain)
}

/// Steerable Perlin value and analytic gradient at a 2D point.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn steerable_perlin_2d_gradient<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
    metric: Metric2,
    gain: f32,
) -> (S::Vf32, [S::Vf32; 2]) {
    let offset = seed_offset(seed);
    let px = x.floor();
    let py = y.floor();
    let fx = x - px;
    let fy = y - py;
    let blend_x = interp::<S>(fx);
    let blend_y = interp::<S>(fy);
    let dblend_x = interp_deriv::<S>(fx);
    let dblend_y = interp_deriv::<S>(fy);
    let m00 = S::Vf32::set1(metric[0]);
    let m01 = S::Vf32::set1(metric[1]);
    let m11 = S::Vf32::set1(metric[2]);

    let mut sum = S::Vf32::zeroes();
    let mut gx = S::Vf32::zeroes();
    let mut gy = S::Vf32::zeroes();
    for i in 0..=1 {
        for j in 0..=1 {
            let ox = S::Vf32::set1(i as f32);
            let oy = S::Vf32::set1(j as f32);
            let gx0 = px + ox + S::Vf32::set1(offset[0]);
            let gy0 = py + oy + S::Vf32::set1(offset[1]);
            let r = unit2::<S>([gx0, gy0]);
            let vx = ox - fx;
            let vy = oy - fy;
            let mvx = m00 * vx + m01 * vy;
            let mvy = m01 * vx + m11 * vy;
            let d = r[0] * mvx + r[1] * mvy;
            let q = vx * mvx + vy * mvy;
            let wx = (ox - blend_x).abs();
            let wy = (oy - blend_y).abs();
            let iq = interp::<S>(q);
            let w = wx * wy * iq;
            sum += d * w;

            // v = o - f, so dd/df = -M r and dq/df = -2 M v.
            let dd_x = S::Vf32::zeroes() - (m00 * r[0] + m01 * r[1]);
            let dd_y = S::Vf32::zeroes() - (m01 * r[0] + m11 * r[1]);
            let dq_x = S::Vf32::set1(-2.0f32) * mvx;
            let dq_y = S::Vf32::set1(-2.0f32) * mvy;
            let diq_x = interp_deriv::<S>(q) * dq_x;
            let diq_y = interp_deriv::<S>(q) * dq_y;
            let dwx = (S::Vf32::zeroes() - sign::<S>(ox - blend_x)) * dblend_x;
            let dwy = (S::Vf32::zeroes() - sign::<S>(oy - blend_y)) * dblend_y;
            gx += dd_x * w + d * (dwx * wy * iq + wx * wy * diq_x);
            gy += dd_y * w + d * (wx * dwy * iq + wx * wy * diq_y);
        }
    }
    let gain = S::Vf32::set1(gain);
    (sum * gain, [gx * gain, gy * gain])
}

/// Steerable Perlin value and analytic gradient at a 3D point.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn steerable_perlin_3d_gradient<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
    metric: Metric3,
    gain: f32,
) -> (S::Vf32, [S::Vf32; 3]) {
    let offset = seed_offset(seed);
    let px = x.floor();
    let py = y.floor();
    let pz = z.floor();
    let fx = x - px;
    let fy = y - py;
    let fz = z - pz;
    let blend = [interp::<S>(fx), interp::<S>(fy), interp::<S>(fz)];
    let dblend = [
        interp_deriv::<S>(fx),
        interp_deriv::<S>(fy),
        interp_deriv::<S>(fz),
    ];
    let m00 = S::Vf32::set1(metric[0]);
    let m01 = S::Vf32::set1(metric[1]);
    let m02 = S::Vf32::set1(metric[2]);
    let m11 = S::Vf32::set1(metric[3]);
    let m12 = S::Vf32::set1(metric[4]);
    let m22 = S::Vf32::set1(metric[5]);

    let mut sum = S::Vf32::zeroes();
    let mut gradient = [S::Vf32::zeroes(); 3];
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let o = [
                    S::Vf32::set1(i as f32),
                    S::Vf32::set1(j as f32),
                    S::Vf32::set1(k as f32),
                ];
                let g = [
                    px + o[0] + S::Vf32::set1(offset[0]),
                    py + o[1] + S::Vf32::set1(offset[1]),
                    pz + o[2] + S::Vf32::set1(offset[2]),
                ];
                let r = rsphere::<S>(g);
                let vx = o[0] - fx;
                let vy = o[1] - fy;
                let vz = o[2] - fz;
                let mvx = m00 * vx + m01 * vy + m02 * vz;
                let mvy = m01 * vx + m11 * vy + m12 * vz;
                let mvz = m02 * vx + m12 * vy + m22 * vz;
                let d = r[0] * mvx + r[1] * mvy + r[2] * mvz;
                let q = vx * mvx + vy * mvy + vz * mvz;
                let w0 = (o[0] - blend[0]).abs();
                let w1 = (o[1] - blend[1]).abs();
                let w2 = (o[2] - blend[2]).abs();
                let iq = interp::<S>(q);
                sum += d * w0 * w1 * w2 * iq;

                let dd = [
                    S::Vf32::zeroes() - (m00 * r[0] + m01 * r[1] + m02 * r[2]),
                    S::Vf32::zeroes() - (m01 * r[0] + m11 * r[1] + m12 * r[2]),
                    S::Vf32::zeroes() - (m02 * r[0] + m12 * r[1] + m22 * r[2]),
                ];
                let dq = [
                    S::Vf32::set1(-2.0f32) * mvx,
                    S::Vf32::set1(-2.0f32) * mvy,
                    S::Vf32::set1(-2.0f32) * mvz,
                ];
                let diq = [
                    interp_deriv::<S>(q) * dq[0],
                    interp_deriv::<S>(q) * dq[1],
                    interp_deriv::<S>(q) * dq[2],
                ];
                let dw = [
                    (S::Vf32::zeroes() - sign::<S>(o[0] - blend[0])) * dblend[0],
                    (S::Vf32::zeroes() - sign::<S>(o[1] - blend[1])) * dblend[1],
                    (S::Vf32::zeroes() - sign::<S>(o[2] - blend[2])) * dblend[2],
                ];
                gradient[0] +=
                    dd[0] * w0 * w1 * w2 * iq + d * (dw[0] * w1 * w2 * iq + w0 * w1 * w2 * diq[0]);
                gradient[1] +=
                    dd[1] * w0 * w1 * w2 * iq + d * (w0 * dw[1] * w2 * iq + w0 * w1 * w2 * diq[1]);
                gradient[2] +=
                    dd[2] * w0 * w1 * w2 * iq + d * (w0 * w1 * dw[2] * iq + w0 * w1 * w2 * diq[2]);
            }
        }
    }
    let gain = S::Vf32::set1(gain);
    (
        sum * gain,
        [gradient[0] * gain, gradient[1] * gain, gradient[2] * gain],
    )
}

/// Steerable Perlin value at a 3D point. Coordinates are in noise space.
#[inline(always)]
pub fn steerable_perlin_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
    metric: Metric3,
    gain: f32,
) -> S::Vf32 {
    let offset = seed_offset(seed);
    let px = x.floor();
    let py = y.floor();
    let pz = z.floor();
    let fx = x - px;
    let fy = y - py;
    let fz = z - pz;
    let blend = [interp::<S>(fx), interp::<S>(fy), interp::<S>(fz)];
    let m00 = S::Vf32::set1(metric[0]);
    let m01 = S::Vf32::set1(metric[1]);
    let m02 = S::Vf32::set1(metric[2]);
    let m11 = S::Vf32::set1(metric[3]);
    let m12 = S::Vf32::set1(metric[4]);
    let m22 = S::Vf32::set1(metric[5]);

    let mut sum = S::Vf32::zeroes();
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let o = [
                    S::Vf32::set1(i as f32),
                    S::Vf32::set1(j as f32),
                    S::Vf32::set1(k as f32),
                ];
                let g = [
                    px + o[0] + S::Vf32::set1(offset[0]),
                    py + o[1] + S::Vf32::set1(offset[1]),
                    pz + o[2] + S::Vf32::set1(offset[2]),
                ];
                let r = rsphere::<S>(g);
                let vx = o[0] - fx;
                let vy = o[1] - fy;
                let vz = o[2] - fz;
                let mvx = m00 * vx + m01 * vy + m02 * vz;
                let mvy = m01 * vx + m11 * vy + m12 * vz;
                let mvz = m02 * vx + m12 * vy + m22 * vz;
                let d = r[0] * mvx + r[1] * mvy + r[2] * mvz;
                let q = vx * mvx + vy * mvy + vz * mvz;
                let w = (o[0] - blend[0]).abs()
                    * (o[1] - blend[1]).abs()
                    * (o[2] - blend[2]).abs()
                    * interp::<S>(q);
                sum += d * w;
            }
        }
    }
    sum * S::Vf32::set1(gain)
}
