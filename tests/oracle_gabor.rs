//! Independent scalar oracle for the Gabor-family kernels.
//!
//! These are not canonical Lagae-Gabor values: the crate implements a
//! documented Gabor-family sparse convolution. This file re-implements that
//! formulation independently in scalar `f64`, using the same hash, seed and
//! lattice rules, and compares it with the public point API. It catches SIMD
//! lane, hash and normalization bugs that block-vs-point tests cannot, because
//! those compare the same kernel against itself.

use simdnoise::point::{
    sample_gabor_points_2d, sample_gabor_points_2d_with_gradient, sample_gabor_points_3d,
    sample_gabor_points_3d_with_gradient, sample_gabor_points_4d,
    sample_gabor_points_4d_with_gradient, sample_phasor_points_2d,
    sample_phasor_points_2d_with_gradient, sample_phasor_points_3d,
    sample_phasor_points_3d_with_gradient,
};
use simdnoise::{GaborSettings, NoiseBuilder};

const TAU: f64 = std::f64::consts::TAU;
const PI: f64 = std::f64::consts::PI;
const RAND_MASK: i32 = 0x3FF;

fn hash3(seed: i32, i: i32, j: i32, k: i32) -> i32 {
    let mut hash = i ^ seed;
    hash ^= j;
    hash ^= k;
    hash = hash
        .wrapping_mul(hash)
        .wrapping_mul(60493)
        .wrapping_mul(hash);
    (((hash as u32) >> 13) as i32) ^ hash
}

fn hash4(seed: i32, i: i32, j: i32, k: i32, l: i32) -> i32 {
    let mut hash = i ^ seed;
    hash ^= j;
    hash ^= k;
    hash ^= l;
    hash = hash
        .wrapping_mul(hash)
        .wrapping_mul(60493)
        .wrapping_mul(hash);
    (((hash as u32) >> 13) as i32) ^ hash
}

fn rand01(hash: i32, shift: i32) -> f64 {
    f64::from((hash >> shift) & RAND_MASK) / 1024.0
}

fn normalized<const N: usize>(orientation: [f32; N]) -> [f64; N] {
    let len = orientation
        .iter()
        .map(|v| f64::from(*v) * f64::from(*v))
        .sum::<f64>()
        .sqrt();
    std::array::from_fn(|i| f64::from(orientation[i]) / len)
}

struct Params {
    seed: i32,
    cell_scale: f64,
    carrier_frequency: f64,
    bandwidth: f64,
    radius: i32,
    gain: f64,
}

fn gabor2(settings: &GaborSettings, point: [f32; 2]) -> f64 {
    let p = Params {
        seed: settings.dimensions().seed,
        cell_scale: f64::from(settings.impulse_density).powf(0.5),
        carrier_frequency: f64::from(settings.carrier_frequency),
        bandwidth: f64::from(settings.bandwidth),
        radius: settings.radius,
        gain: f64::from(settings.gain),
    };
    let orientation = normalized([settings.orientation_x, settings.orientation_y]);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x) * p.cell_scale;
    let y = f64::from(point[1]) * f64::from(settings.frequency_y) * p.cell_scale;
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let inv_sigma2_pi = PI / (p.bandwidth * p.bandwidth);
    let omega = p.carrier_frequency * TAU;

    let mut sum = 0.0;
    let mut count = 0.0;
    for dx in -p.radius..=p.radius {
        for dy in -p.radius..=p.radius {
            let cx = ix + dx;
            let cy = iy + dy;
            let h0 = hash3(p.seed, cx, cy, 0);
            let h1 = hash3(p.seed.wrapping_add(0x5F35_6495), cx, cy, 0);
            let qx = f64::from(cx) + rand01(h0, 0);
            let qy = f64::from(cy) + rand01(h0, 10);
            let phase = rand01(h1, 0) * TAU;
            let rx = x - qx;
            let ry = y - qy;
            let r2 = rx * rx + ry * ry;
            let envelope = (-r2 * inv_sigma2_pi).exp();
            let projection = rx * orientation[0] + ry * orientation[1];
            sum += envelope * (projection * omega + phase).cos();
            count += 1.0;
        }
    }
    sum * p.gain / count
}

fn gabor3(settings: &GaborSettings, point: [f32; 3]) -> f64 {
    let p = Params {
        seed: settings.dimensions().seed,
        cell_scale: f64::from(settings.impulse_density).powf(1.0 / 3.0),
        carrier_frequency: f64::from(settings.carrier_frequency),
        bandwidth: f64::from(settings.bandwidth),
        radius: settings.radius,
        gain: f64::from(settings.gain),
    };
    let orientation = normalized([
        settings.orientation_x,
        settings.orientation_y,
        settings.orientation_z,
    ]);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x) * p.cell_scale;
    let y = f64::from(point[1]) * f64::from(settings.frequency_y) * p.cell_scale;
    let z = f64::from(point[2]) * f64::from(settings.frequency_z) * p.cell_scale;
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let iz = z.floor() as i32;
    let inv_sigma2_pi = PI / (p.bandwidth * p.bandwidth);
    let omega = p.carrier_frequency * TAU;

    let mut sum = 0.0;
    let mut count = 0.0;
    for dx in -p.radius..=p.radius {
        for dy in -p.radius..=p.radius {
            for dz in -p.radius..=p.radius {
                let cx = ix + dx;
                let cy = iy + dy;
                let cz = iz + dz;
                let h0 = hash3(p.seed, cx, cy, cz);
                let h1 = hash3(p.seed.wrapping_add(0x5F35_6495), cx, cy, cz);
                let qx = f64::from(cx) + rand01(h0, 0);
                let qy = f64::from(cy) + rand01(h0, 10);
                let qz = f64::from(cz) + rand01(h0, 20);
                let phase = rand01(h1, 0) * TAU;
                let rx = x - qx;
                let ry = y - qy;
                let rz = z - qz;
                let r2 = rx * rx + ry * ry + rz * rz;
                let envelope = (-r2 * inv_sigma2_pi).exp();
                let projection = rx * orientation[0] + ry * orientation[1] + rz * orientation[2];
                sum += envelope * (projection * omega + phase).cos();
                count += 1.0;
            }
        }
    }
    sum * p.gain / count
}

fn gabor4(settings: &GaborSettings, point: [f32; 4]) -> f64 {
    let p = Params {
        seed: settings.dimensions().seed,
        cell_scale: f64::from(settings.impulse_density).powf(1.0 / 4.0),
        carrier_frequency: f64::from(settings.carrier_frequency),
        bandwidth: f64::from(settings.bandwidth),
        radius: settings.radius,
        gain: f64::from(settings.gain),
    };
    let orientation = normalized([
        settings.orientation_x,
        settings.orientation_y,
        settings.orientation_z,
        settings.orientation_w,
    ]);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x) * p.cell_scale;
    let y = f64::from(point[1]) * f64::from(settings.frequency_y) * p.cell_scale;
    let z = f64::from(point[2]) * f64::from(settings.frequency_z) * p.cell_scale;
    let w = f64::from(point[3]) * f64::from(settings.frequency_w) * p.cell_scale;
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let iz = z.floor() as i32;
    let iw = w.floor() as i32;
    let inv_sigma2_pi = PI / (p.bandwidth * p.bandwidth);
    let omega = p.carrier_frequency * TAU;

    let mut sum = 0.0;
    let mut count = 0.0;
    for dx in -p.radius..=p.radius {
        for dy in -p.radius..=p.radius {
            for dz in -p.radius..=p.radius {
                for dw in -p.radius..=p.radius {
                    let cx = ix + dx;
                    let cy = iy + dy;
                    let cz = iz + dz;
                    let cw = iw + dw;
                    let h0 = hash4(p.seed, cx, cy, cz, cw);
                    let h1 = hash4(p.seed.wrapping_add(0x5F35_6495), cx, cy, cz, cw);
                    let qx = f64::from(cx) + rand01(h0, 0);
                    let qy = f64::from(cy) + rand01(h0, 10);
                    let qz = f64::from(cz) + rand01(h0, 20);
                    let qw = f64::from(cw) + rand01(h1, 0);
                    let phase = rand01(h1, 10) * TAU;
                    let rx = x - qx;
                    let ry = y - qy;
                    let rz = z - qz;
                    let rw = w - qw;
                    let r2 = rx * rx + ry * ry + rz * rz + rw * rw;
                    let envelope = (-r2 * inv_sigma2_pi).exp();
                    let projection = rx * orientation[0]
                        + ry * orientation[1]
                        + rz * orientation[2]
                        + rw * orientation[3];
                    sum += envelope * (projection * omega + phase).cos();
                    count += 1.0;
                }
            }
        }
    }
    sum * p.gain / count
}

fn phasor2(settings: &GaborSettings, point: [f32; 2]) -> (f64, f64) {
    let p = Params {
        seed: settings.dimensions().seed,
        cell_scale: f64::from(settings.impulse_density).powf(0.5),
        carrier_frequency: f64::from(settings.carrier_frequency),
        bandwidth: f64::from(settings.bandwidth),
        radius: settings.radius,
        gain: f64::from(settings.gain),
    };
    let orientation = normalized([settings.orientation_x, settings.orientation_y]);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x) * p.cell_scale;
    let y = f64::from(point[1]) * f64::from(settings.frequency_y) * p.cell_scale;
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let inv_sigma2_pi = PI / (p.bandwidth * p.bandwidth);
    let omega = p.carrier_frequency * TAU;

    let mut real = 0.0;
    let mut imag = 0.0;
    let mut count = 0.0;
    for dx in -p.radius..=p.radius {
        for dy in -p.radius..=p.radius {
            let cx = ix + dx;
            let cy = iy + dy;
            let h0 = hash3(p.seed, cx, cy, 0);
            let h1 = hash3(p.seed.wrapping_add(0x5F35_6495), cx, cy, 0);
            let qx = f64::from(cx) + rand01(h0, 0);
            let qy = f64::from(cy) + rand01(h0, 10);
            let phase = rand01(h1, 0) * TAU;
            let rx = x - qx;
            let ry = y - qy;
            let r2 = rx * rx + ry * ry;
            let envelope = (-r2 * inv_sigma2_pi).exp();
            let angle = (rx * orientation[0] + ry * orientation[1]) * omega + phase;
            real += envelope * angle.cos();
            imag += envelope * angle.sin();
            count += 1.0;
        }
    }
    (real * p.gain / count, imag * p.gain / count)
}

fn phasor3(settings: &GaborSettings, point: [f32; 3]) -> (f64, f64) {
    let p = Params {
        seed: settings.dimensions().seed,
        cell_scale: f64::from(settings.impulse_density).powf(1.0 / 3.0),
        carrier_frequency: f64::from(settings.carrier_frequency),
        bandwidth: f64::from(settings.bandwidth),
        radius: settings.radius,
        gain: f64::from(settings.gain),
    };
    let orientation = normalized([
        settings.orientation_x,
        settings.orientation_y,
        settings.orientation_z,
    ]);
    let x = f64::from(point[0]) * f64::from(settings.frequency_x) * p.cell_scale;
    let y = f64::from(point[1]) * f64::from(settings.frequency_y) * p.cell_scale;
    let z = f64::from(point[2]) * f64::from(settings.frequency_z) * p.cell_scale;
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let iz = z.floor() as i32;
    let inv_sigma2_pi = PI / (p.bandwidth * p.bandwidth);
    let omega = p.carrier_frequency * TAU;

    let mut real = 0.0;
    let mut imag = 0.0;
    let mut count = 0.0;
    for dx in -p.radius..=p.radius {
        for dy in -p.radius..=p.radius {
            for dz in -p.radius..=p.radius {
                let cx = ix + dx;
                let cy = iy + dy;
                let cz = iz + dz;
                let h0 = hash3(p.seed, cx, cy, cz);
                let h1 = hash3(p.seed.wrapping_add(0x5F35_6495), cx, cy, cz);
                let qx = f64::from(cx) + rand01(h0, 0);
                let qy = f64::from(cy) + rand01(h0, 10);
                let qz = f64::from(cz) + rand01(h0, 20);
                let phase = rand01(h1, 0) * TAU;
                let rx = x - qx;
                let ry = y - qy;
                let rz = z - qz;
                let r2 = rx * rx + ry * ry + rz * rz;
                let envelope = (-r2 * inv_sigma2_pi).exp();
                let angle = (rx * orientation[0] + ry * orientation[1] + rz * orientation[2])
                    * omega
                    + phase;
                real += envelope * angle.cos();
                imag += envelope * angle.sin();
                count += 1.0;
            }
        }
    }
    (real * p.gain / count, imag * p.gain / count)
}

fn close(actual: f32, expected: f64, tolerance: f64) {
    let actual = f64::from(actual);
    assert!(
        (actual - expected).abs() <= tolerance,
        "oracle mismatch: actual {actual}, expected {expected}, tolerance {tolerance}"
    );
}

const TOLERANCE: f64 = 2.0e-4;

fn gabor_2d_settings() -> GaborSettings {
    let mut settings = NoiseBuilder::gabor_2d(1, 1);
    settings
        .with_frequency_2d(0.02, 0.03)
        .with_seed(7)
        .with_orientation(1.0, 0.35, 0.0)
        .with_bandwidth(2.0)
        .with_impulse_density(1.3)
        .with_carrier_frequency(1.25)
        .with_radius(2)
        .with_gain(1.4);
    settings
}

#[test]
fn gabor_2d_matches_scalar_oracle() {
    let settings = gabor_2d_settings();
    let points = [[3.25, -1.5], [0.125, 0.25], [-7.75, 2.5], [11.0, -0.125]];
    for point in points {
        let mut value = [0.0f32; 1];
        let mut sample = [simdnoise::point::PointSample2::default(); 1];
        sample_gabor_points_2d(&settings, &[point], &mut value);
        sample_gabor_points_2d_with_gradient(&settings, &[point], &mut sample);
        let expected = gabor2(&settings, point);
        close(value[0], expected, TOLERANCE);
        close(sample[0].value, expected, TOLERANCE);
    }
}

#[test]
fn gabor_3d_matches_scalar_oracle() {
    let mut settings = NoiseBuilder::gabor_3d(1, 1, 1);
    settings
        .with_frequency_3d(0.02, 0.03, 0.04)
        .with_seed(11)
        .with_orientation(1.0, -0.25, 0.5)
        .with_bandwidth(1.75)
        .with_impulse_density(1.1)
        .with_carrier_frequency(0.9)
        .with_radius(2)
        .with_gain(1.2);
    let points = [[3.25, -1.5, 0.75], [0.0, 0.0, 0.0], [-7.75, 2.5, 0.125]];
    for point in points {
        let mut value = [0.0f32; 1];
        let mut sample = [simdnoise::point::PointSample3::default(); 1];
        sample_gabor_points_3d(&settings, &[point], &mut value);
        sample_gabor_points_3d_with_gradient(&settings, &[point], &mut sample);
        let expected = gabor3(&settings, point);
        close(value[0], expected, TOLERANCE);
        close(sample[0].value, expected, TOLERANCE);
    }
}

#[test]
fn gabor_4d_matches_scalar_oracle() {
    let mut settings = NoiseBuilder::gabor_4d(1, 1, 1, 1);
    settings
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05)
        .with_seed(17)
        .with_orientation_4d(1.0, -0.25, 0.5, -0.125)
        .with_bandwidth(1.5)
        .with_impulse_density(1.0)
        .with_carrier_frequency(1.1)
        .with_radius(1)
        .with_gain(1.3);
    let points = [
        [3.25, -1.5, 0.75, 0.125],
        [0.0, 0.0, 0.0, 0.0],
        [-7.75, 2.5, 0.125, -0.5],
    ];
    for point in points {
        let mut value = [0.0f32; 1];
        let mut sample = [simdnoise::point::PointSample4::default(); 1];
        sample_gabor_points_4d(&settings, &[point], &mut value);
        sample_gabor_points_4d_with_gradient(&settings, &[point], &mut sample);
        let expected = gabor4(&settings, point);
        close(value[0], expected, TOLERANCE);
        close(sample[0].value, expected, TOLERANCE);
    }
}

#[test]
fn phasor_2d_matches_scalar_oracle() {
    let settings = gabor_2d_settings();
    let points = [[3.25, -1.5], [0.125, 0.25], [-7.75, 2.5]];
    for point in points {
        let mut value = [simdnoise::point::PhasorValue2::default(); 1];
        let mut sample = [simdnoise::point::PhasorSample2::default(); 1];
        sample_phasor_points_2d(&settings, &[point], &mut value);
        sample_phasor_points_2d_with_gradient(&settings, &[point], &mut sample);
        let (real, imag) = phasor2(&settings, point);
        close(value[0].real, real, TOLERANCE);
        close(value[0].imag, imag, TOLERANCE);
        close(sample[0].real, real, TOLERANCE);
        close(sample[0].imag, imag, TOLERANCE);
    }
}

#[test]
fn phasor_3d_matches_scalar_oracle() {
    let mut settings = NoiseBuilder::gabor_3d(1, 1, 1);
    settings
        .with_frequency_3d(0.02, 0.03, 0.04)
        .with_seed(11)
        .with_orientation(1.0, -0.25, 0.5)
        .with_bandwidth(1.75)
        .with_impulse_density(1.1)
        .with_carrier_frequency(0.9)
        .with_radius(2)
        .with_gain(1.2);
    let points = [[3.25, -1.5, 0.75], [0.0, 0.0, 0.0], [-7.75, 2.5, 0.125]];
    for point in points {
        let mut value = [simdnoise::point::PhasorValue3::default(); 1];
        let mut sample = [simdnoise::point::PhasorSample3::default(); 1];
        sample_phasor_points_3d(&settings, &[point], &mut value);
        sample_phasor_points_3d_with_gradient(&settings, &[point], &mut sample);
        let (real, imag) = phasor3(&settings, point);
        close(value[0].real, real, TOLERANCE);
        close(value[0].imag, imag, TOLERANCE);
        close(sample[0].real, real, TOLERANCE);
        close(sample[0].imag, imag, TOLERANCE);
    }
}
