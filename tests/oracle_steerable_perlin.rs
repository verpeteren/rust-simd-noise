//! Independent scalar oracle for steerable Perlin noise.
//!
//! Mirrors the documented formulation (and the SIGGRAPH 2025 reference for 3D)
//! in scalar `f64`: sin hash, random sphere gradients, anisotropic metric,
//! smootherstep weights and the weighted corner sum. Compares block output at
//! one-sample offsets against the SIMD kernel.

use simdnoise::{NoiseDimensions, SteerablePerlinSettings};

const TAU: f64 = std::f64::consts::TAU;
const HASH_X: f64 = 64.253_75;
const HASH_Y: f64 = 23.275_365;
const HASH_Z: f64 = 86.296_78;
const HASH_SCALE: f64 = 59482.754;
const EPS: f64 = 1.0e-5;

fn random3(p: [f64; 3]) -> f64 {
    // The reference implementation is C float; match its hash precision.
    let dot =
        p[0] as f32 * HASH_X as f32 + p[1] as f32 * HASH_Y as f32 + p[2] as f32 * HASH_Z as f32;
    let value = dot.sin() * HASH_SCALE as f32;
    f64::from(value - value.floor())
}

fn smootherstep(x: f64) -> f64 {
    let x = x.clamp(0.0, 1.0);
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

fn interp(u: f64) -> f64 {
    1.0 - smootherstep(u.abs())
}

fn normalize2(v: [f64; 2]) -> [f64; 2] {
    let len = (v[0] * v[0] + v[1] * v[1]).sqrt();
    [v[0] / len, v[1] / len]
}

fn normalize3(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn metric2(direction: [f32; 2], eigenvalue_sum: f32, k: f32) -> [f32; 3] {
    let d = normalize2([f64::from(direction[0]), f64::from(direction[1])]);
    let x = [d[0].abs().max(EPS), d[1].abs().max(EPS)];
    let e0 = normalize2([-x[1] / x[0], 1.0]);
    let e1 = normalize2([x[0] / x[1], 1.0]);
    let low = f64::from(eigenvalue_sum - k) - 1.0e-4;
    let k = f64::from(k);
    [
        (low * e0[0] * e0[0] + k * e1[0] * e1[0]) as f32,
        (low * e0[0] * e0[1] + k * e1[0] * e1[1]) as f32,
        (low * e0[1] * e0[1] + k * e1[1] * e1[1]) as f32,
    ]
}

fn metric3(direction: [f32; 3], eigenvalue_sum: f32, k: f32) -> [f32; 6] {
    let d = normalize3([
        f64::from(direction[0]),
        f64::from(direction[1]),
        f64::from(direction[2]),
    ]);
    let x = [
        d[0].abs().max(EPS),
        d[1].abs().max(EPS),
        d[2].abs().max(EPS),
    ];
    let e0 = normalize3([-x[2] / x[0], 0.0, 1.0]);
    let e1 = normalize3([-x[1] / x[0], 1.0, 0.0]);
    let e2 = normalize3([x[0] / x[2], x[1] / x[2], 1.0]);
    let low = (f64::from(eigenvalue_sum) - f64::from(k)) / 2.0 - 1.0e-4;
    let k = f64::from(k);
    let evecs = [e0, e1, e2];
    let mapped = [low, low, k];
    let idx = [[0, 1, 2], [1, 3, 4], [2, 4, 5]];
    let mut m = [0.0f32; 6];
    for row in 0..3 {
        for col in row..3 {
            let mut value = 0.0;
            for axis in 0..3 {
                value += mapped[axis] * evecs[axis][row] * evecs[axis][col];
            }
            m[idx[row][col]] = value as f32;
        }
    }
    m
}

fn seed_offset(seed: i32) -> [f64; 3] {
    let s = f64::from(seed);
    [s * 0.123_456, s * 0.654_321, s * 0.987_654]
}

fn rsphere(p: [f64; 3]) -> [f64; 3] {
    let u = random3([p[0] + 0.01, p[1] + 0.01, p[2] + 0.01]);
    let v = random3([p[0] + 0.02, p[1] + 0.02, p[2] + 0.02]);
    let w = random3([p[0] + 0.03, p[1] + 0.03, p[2] + 0.03]);
    let theta = u * TAU;
    let phi = (2.0 * v - 1.0).acos();
    let radius = w.powf(1.0 / 3.0);
    let sin_phi = phi.sin();
    [
        radius * sin_phi * theta.cos(),
        radius * sin_phi * theta.sin(),
        radius * phi.cos(),
    ]
}

fn unit2(p: [f64; 2]) -> [f64; 2] {
    let angle = random3([p[0], p[1], 0.0]) * TAU;
    [angle.cos(), angle.sin()]
}

fn gabor_like_2d(settings: &SteerablePerlinSettings, point: [f32; 2], metric: [f32; 3]) -> f64 {
    let offset = seed_offset(settings.dimensions().seed);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x);
    let y = f64::from(point[1]) * f64::from(settings.frequency_y);
    let px = x.floor();
    let py = y.floor();
    let fx = x - px;
    let fy = y - py;
    let blend_x = interp(fx);
    let blend_y = interp(fy);
    let m00 = f64::from(metric[0]);
    let m01 = f64::from(metric[1]);
    let m11 = f64::from(metric[2]);
    let mut sum = 0.0;
    for i in 0..=1 {
        for j in 0..=1 {
            let ox = f64::from(i);
            let oy = f64::from(j);
            let r = unit2([px + ox + offset[0], py + oy + offset[1]]);
            let vx = ox - fx;
            let vy = oy - fy;
            let mvx = m00 * vx + m01 * vy;
            let mvy = m01 * vx + m11 * vy;
            let d = r[0] * mvx + r[1] * mvy;
            let q = vx * mvx + vy * mvy;
            sum += d * (ox - blend_x).abs() * (oy - blend_y).abs() * interp(q);
        }
    }
    sum * f64::from(settings.gain)
}

fn steerable_3d(settings: &SteerablePerlinSettings, point: [f32; 3], metric: [f32; 6]) -> f64 {
    let offset = seed_offset(settings.dimensions().seed);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x);
    let y = f64::from(point[1]) * f64::from(settings.frequency_y);
    let z = f64::from(point[2]) * f64::from(settings.frequency_z);
    let px = x.floor();
    let py = y.floor();
    let pz = z.floor();
    let fx = x - px;
    let fy = y - py;
    let fz = z - pz;
    let blend = [interp(fx), interp(fy), interp(fz)];
    let m00 = f64::from(metric[0]);
    let m01 = f64::from(metric[1]);
    let m02 = f64::from(metric[2]);
    let m11 = f64::from(metric[3]);
    let m12 = f64::from(metric[4]);
    let m22 = f64::from(metric[5]);
    let mut sum = 0.0;
    for i in 0..=1 {
        for j in 0..=1 {
            for k in 0..=1 {
                let o = [f64::from(i), f64::from(j), f64::from(k)];
                let r = rsphere([
                    px + o[0] + offset[0],
                    py + o[1] + offset[1],
                    pz + o[2] + offset[2],
                ]);
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
                    * interp(q);
                sum += d * w;
            }
        }
    }
    sum * f64::from(settings.gain)
}

fn close(actual: f32, expected: f64, tolerance: f64) {
    let actual = f64::from(actual);
    assert!(
        (actual - expected).abs() <= tolerance,
        "oracle mismatch: actual {actual}, expected {expected}, tolerance {tolerance}"
    );
}

const TOLERANCE: f64 = 2.0e-3;

fn settings2(direction: [f32; 2], seed: i32, point: [f32; 2]) -> SteerablePerlinSettings {
    let mut dimension = NoiseDimensions::default(2);
    dimension.offset_x = point[0];
    dimension.offset_y = point[1];
    dimension.extent_x = 1;
    dimension.extent_y = 1;
    let mut settings = SteerablePerlinSettings::default(dimension);
    settings
        .with_frequency_2d(0.07, 0.03)
        .with_seed(seed)
        .with_direction(direction[0], direction[1], 0.0)
        .with_metric(4.0, 0.51)
        .with_gain(1.3);
    settings
}

fn settings3(direction: [f32; 3], seed: i32, point: [f32; 3]) -> SteerablePerlinSettings {
    let mut dimension = NoiseDimensions::default(3);
    dimension.offset_x = point[0];
    dimension.offset_y = point[1];
    dimension.offset_z = point[2];
    dimension.extent_x = 1;
    dimension.extent_y = 1;
    dimension.extent_z = 1;
    let mut settings = SteerablePerlinSettings::default(dimension);
    settings
        .with_frequency_3d(0.07, 0.03, 0.05)
        .with_seed(seed)
        .with_direction(direction[0], direction[1], direction[2])
        .with_metric(4.0, 0.51)
        .with_gain(1.1);
    settings
}

#[test]
fn steerable_perlin_2d_matches_scalar_oracle() {
    for (direction, seed) in [([1.0, 0.0], 7), ([0.25, -1.0], 23), ([0.0, 1.0], 41)] {
        let metric = metric2(direction, 4.0, 0.51);
        for point in [[3.25, -1.5], [0.125, 0.25], [-7.75, 2.5]] {
            let settings = settings2(direction, seed, point);
            let (values, _, _) = simdnoise::steerable_perlin_2d(&settings);
            let expected = gabor_like_2d(&settings, point, metric);
            close(values[0], expected, TOLERANCE);
        }
    }
}

#[test]
fn steerable_perlin_3d_matches_scalar_oracle() {
    for (direction, seed) in [([1.0, 0.0, 0.0], 11), ([0.25, -1.0, 0.5], 31)] {
        let metric = metric3(direction, 4.0, 0.51);
        for point in [[3.25, -1.5, 0.75], [0.125, 0.25, -0.5], [-7.75, 2.5, 0.125]] {
            let settings = settings3(direction, seed, point);
            let (values, _, _) = simdnoise::steerable_perlin_3d(&settings);
            let expected = steerable_3d(&settings, point, metric);
            close(values[0], expected, TOLERANCE);
        }
    }
}
