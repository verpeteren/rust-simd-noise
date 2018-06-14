#![allow(non_snake_case)]
use super::*;
use shared::*;
use std::f32;

const F2: f32 = 0.36602540378;
const F3: f32 = 1.0 / 3.0;
const G2: f32 = 0.2113248654;
const G3: f32 = 1.0 / 6.0;
const POINT_FIVE: f32 = 0.5;

pub fn grad2(hash: i32, x: f32, y: f32) -> f32 {
    let h = hash & 7;
    let (u, v) = if h < 4 { (x, y) } else { (y, x) };

    let a = if (h & 1) != 0 { -u } else { u };
    let b = if (h & 2) != 0 { -2.0 * v } else { 2.0 * v };
    a + b
}

pub fn simplex_2d(x: f32, y: f32) -> f32 {
    let s = (x + y) * F2;
    let xs = x + s;
    let ys = y + s;
    let i = xs.floor() as i32;
    let j = ys.floor() as i32;
    let t = (i + j) as f32 * G2;
    let X0 = i as f32 - t;
    let Y0 = j as f32 - t;
    let x0 = x - X0;
    let y0 = y - Y0;

    let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

    0.0
}

fn dot(x1: f32, y1: f32, z1: f32, x2: f32, y2: f32, z2: f32) -> f32 {
    x1 * x2 + y1 * y2 + z1 * z2
}

pub fn grad3(hash: i32, x: f32, y: f32, z: f32) -> f32 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };
    let a = if (h & 1) != 0 { -u } else { u };
    let b = if (h & 2) != 0 { -v } else { v };
    a + b
}

pub fn simplex_3d(x: f32, y: f32, z: f32) -> f32 {
    let s = (x + y + z) * F3;
    let i = (x + s).floor() as i32;
    let j = (y + s).floor() as i32;
    let k = (z + s).floor() as i32;
    let t = (i + j + k) as f32 * G3;
    let X0 = i as f32 - t;
    let Y0 = j as f32 - t;
    let Z0 = k as f32 - t;
    let x0 = x - X0;
    let y0 = y - Y0;
    let z0 = z - Z0;

    let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
        if y0 >= z0 {
            (1, 0, 0, 1, 1, 0)
        } else if x0 >= z0 {
            (1, 0, 0, 1, 0, 1)
        } else {
            (0, 0, 1, 1, 0, 1)
        }
    } else {
        if y0 < z0 {
            (0, 0, 1, 0, 1, 1)
        } else if x0 < z0 {
            (0, 1, 0, 0, 1, 1)
        } else {
            (0, 1, 0, 1, 1, 0)
        }
    };

    let x1 = x0 - i1 as f32 + G3;
    let y1 = y0 - j1 as f32 + G3;
    let z1 = z0 - k1 as f32 + G3;
    let x2 = x0 - i2 as f32 + F3;
    let y2 = y0 - j2 as f32 + F3;
    let z2 = z0 - k2 as f32 + F3;
    let x3 = x0 - 1.0 + POINT_FIVE;
    let y3 = y0 - 1.0 + POINT_FIVE;
    let z3 = z0 - 1.0 + POINT_FIVE;

    let ii = i & 255;
    let jj = j & 255;
    let kk = k & 255;
    unsafe {
        let gi0 = *PERM_MOD12.get_unchecked(
            (ii + *PERM.get_unchecked((jj + *PERM.get_unchecked(kk as usize)) as usize)) as usize,
        ) as usize;
        let gi1 = *PERM_MOD12.get_unchecked(
            (ii
                + i1
                + *PERM.get_unchecked((jj + j1 + *PERM.get_unchecked((kk + k1) as usize)) as usize))
                as usize,
        ) as usize;
        let gi2 = *PERM_MOD12.get_unchecked(
            (ii
                + i2
                + *PERM.get_unchecked((jj + j2 + *PERM.get_unchecked((kk + k2) as usize)) as usize))
                as usize,
        ) as usize;
        let gi3 = *PERM_MOD12.get_unchecked(
            (ii
                + 1
                + *PERM.get_unchecked((jj + 1 + *PERM.get_unchecked((kk + 1) as usize)) as usize))
                as usize,
        ) as usize;

        let mut t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            t0 = t0 * t0;
            t0 * t0 * t0 * t0
                * dot(
                    *GRAD_X.get_unchecked(gi0),
                    *GRAD_Y.get_unchecked(gi0),
                    *GRAD_Z.get_unchecked(gi0),
                    x0,
                    y0,
                    z0,
                )
        };
        let mut t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            t1 = t1 * t1;
            t1 * t1 * t1 * t1
                * dot(
                    *GRAD_X.get_unchecked(gi1),
                    *GRAD_Y.get_unchecked(gi1),
                    *GRAD_Z.get_unchecked(gi1),
                    x1,
                    y1,
                    z1,
                )
        };
        let mut t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            t2 = t2 * t2;
            t2 * t2 * t2 * t2
                * dot(
                    *GRAD_X.get_unchecked(gi2),
                    *GRAD_Y.get_unchecked(gi2),
                    *GRAD_Z.get_unchecked(gi2),
                    x2,
                    y2,
                    z2,
                )
        };
        let mut t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        let n3 = if t3 < 0.0 {
            0.0
        } else {
            t3 = t3 * t3;
            t3 * t3 * t3 * t3
                * dot(
                    *GRAD_X.get_unchecked(gi3),
                    *GRAD_Y.get_unchecked(gi3),
                    *GRAD_Z.get_unchecked(gi3),
                    x3,
                    y3,
                    z3,
                )
        };

        n0 + n1 + n2 + n3
    }
}

pub fn fbm_3d(x: f32, y: f32, z: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut zf = z * freq;
    let mut result = simplex_3d(xf, yf, zf);
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        zf = zf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_3d(xf, yf, zf) * amp);
    }
    result
}

pub fn turbulence_3d(
    x: f32,
    y: f32,
    z: f32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut zf = z * freq;
    let mut result = simplex_3d(xf, yf, zf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        zf = zf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_3d(xf, yf, zf) * amp).abs();
    }
    result
}

pub fn get_3d_noise_helper(x: f32, y: f32, z: f32, fractal_settings: FractalSettings) -> f32 {
    match fractal_settings.noise_type {
        NoiseType::FBM => fbm_3d(
            x,
            y,
            z,
            fractal_settings.freq,
            fractal_settings.lacunarity,
            fractal_settings.gain,
            fractal_settings.octaves,
        ),
        NoiseType::Turbulence => turbulence_3d(
            x,
            y,
            z,
            fractal_settings.freq,
            fractal_settings.lacunarity,
            fractal_settings.gain,
            fractal_settings.octaves,
        ),
        NoiseType::Normal => simplex_3d(
            x * fractal_settings.freq,
            y * fractal_settings.freq,
            z * fractal_settings.freq,
        ),
    }
}

pub fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    fractal_settings: FractalSettings,
) -> (Vec<f32>, f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let noise = (0..width * height * depth)
        .map(|mut i| {
            let x = (i % width) as f32 + start_x;
            i = i / width;
            let y = (i % height) as f32 + start_y;
            let z = (i / height) as f32 + start_z;
            let f = get_3d_noise_helper(x, y, z, fractal_settings);
            if f < min {
                min = f;
            }
            if f > max {
                max = f;
            }
            f
        })
        .collect();
    (noise, min, max)
}

pub fn get_3d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    fractal_settings: FractalSettings,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_3d_noise(
        start_x,
        width,
        start_y,
        height,
        start_z,
        depth,
        fractal_settings,
    );
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    for f in &mut noise {
        *f = *f * multiplier + offset;
    }
    noise
}
