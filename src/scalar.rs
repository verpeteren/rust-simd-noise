//! Non SIMD accelerated versions of noise functions,
//! guaranteed to work wherever rust works.
//! Simplex noise implementations here are ports of Stefan Gustavson's C implementations at:
//! https://github.com/stegu/perlin-noise

use super::*;
use crate::shared::*;
use std::f32;
const F2: f32 = 0.36602540378;
const F3: f32 = 1.0 / 3.0;
const F4: f32 = 0.309016994;
const G2: f32 = 0.2113248654;
const G22: f32 = G2 * 2.0;
const G3: f32 = 1.0 / 6.0;
const G4: f32 = 0.138196601;
const X_PRIME: i32 = 1619;
const Y_PRIME: i32 = 31337;
const Z_PRIME: i32 = 6971;

fn hash_2d(seed: i32, x: i32, y: i32) -> i32 {
    let mut hash = seed ^ (X_PRIME * x);
    hash ^= Y_PRIME * y;

    hash = hash.wrapping_mul(hash.wrapping_mul(hash.wrapping_mul(60493)));
    (hash >> 13) ^ hash
}
fn val_coord_2d(seed: i32, x: i32, y: i32) -> f32 {
    let mut n = seed ^ (X_PRIME * x);
    n ^= Y_PRIME * y;
    return n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(60493))) as f32 / 2147483648.0;
}

/// Get a single value of 2d cellular/voroni noise
pub fn cellular_2d(
    x: f32,
    y: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f32,
) -> f32 {
    let xr = x.round() as i32;
    let yr = y.round() as i32;

    let mut distance = f32::MAX;
    let mut xc = 0;
    let mut yc = 0;
    match distance_function {
        CellDistanceFunction::Euclidean => {
            for xi in xr - 1..xr + 2 {
                let xisubx = xi as f32 - x;
                for yi in yr - 1..yr + 2 {
                    let hi = hash_2d(1337, xi, yi) & 0xff;
                    let vx = xisubx + CELL_2D_X[hi as usize] as f32 * jitter;
                    let vy = yi as f32 - y + CELL_2D_Y[hi as usize] as f32 * jitter;
                    let new_dist = vx * vx + vy * vy;
                    if new_dist < distance {
                        distance = new_dist;
                        xc = xi;
                        yc = yi;
                    }
                }
            }
        }
        CellDistanceFunction::Manhattan => {
            for xi in xr - 1..xr + 2 {
                let xisubx = xi as f32 - x;
                for yi in yr - 1..yr + 2 {
                    let hi = hash_2d(1337, xi, yi) & 0xff;
                    let vx = xisubx + CELL_2D_X[hi as usize] as f32 * jitter;
                    let vy = yi as f32 - y + CELL_2D_Y[hi as usize] as f32 * jitter;
                    let new_dist = vx.abs() + vy.abs();
                    if new_dist < distance {
                        distance = new_dist;
                        xc = xi;
                        yc = yi;
                    }
                }
            }
        }
        CellDistanceFunction::Natural => {
            for xi in xr - 1..xr + 2 {
                let xisubx = xi as f32 - x;
                for yi in yr - 1..yr + 2 {
                    let hi = hash_2d(1337, xi, yi) & 0xff;
                    let vx = xisubx + CELL_2D_X[hi as usize] as f32 * jitter;
                    let vy = yi as f32 - y + CELL_2D_Y[hi as usize] as f32 * jitter;
                    let new_dist = vx.abs() + vy.abs() + (vx * vx + vy * vy);
                    if new_dist < distance {
                        distance = new_dist;
                        xc = xi;
                        yc = yi;
                    }
                }
            }
        }
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => val_coord_2d(1337, xc, yc),
    }
}

fn hash_3d(seed: i32, x: i32, y: i32, z: i32) -> i32 {
    let mut hash = seed ^ (X_PRIME * x);
    hash ^= Y_PRIME * y;
    hash ^= Z_PRIME * z;

    hash = hash.wrapping_mul(hash.wrapping_mul(hash.wrapping_mul(60493)));
    (hash >> 13) ^ hash
}

fn val_coord_3d(seed: i32, x: i32, y: i32, z: i32) -> f32 {
    let mut n = seed ^ (X_PRIME * x);
    n ^= Y_PRIME * y;
    n ^= Z_PRIME * z;
    return n.wrapping_mul(n.wrapping_mul(n.wrapping_mul(60493))) as f32 / 2147483648.0;
}

/// Get a single value of 2d cellular/voroni noise
pub fn cellular_3d(
    x: f32,
    y: f32,
    z: f32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: f32,
) -> f32 {
    let xr = x.round() as i32;
    let yr = y.round() as i32;
    let zr = z.round() as i32;
    let mut distance = f32::MAX;
    let mut xc = 0;
    let mut yc = 0;
    let mut zc = 0;
    match distance_function {
        CellDistanceFunction::Euclidean => {
            for xi in xr - 1..xr + 2 {
                let xisubx = xi as f32 - x;
                for yi in yr - 1..yr + 2 {
                    for zi in zr - 1..zr + 2 {
                        let hi = hash_3d(1337, xi, yi, zi) & 0xff;
                        let vx = xisubx + CELL_3D_X[hi as usize] as f32 * jitter;
                        let vy = yi as f32 - y + CELL_3D_Y[hi as usize] as f32 * jitter;
                        let vz = zi as f32 - z + CELL_3D_Z[hi as usize] as f32 * jitter;
                        let new_dist = vx * vx + vy * vy + vz * vz;
                        if new_dist < distance {
                            distance = new_dist;
                            xc = xi;
                            yc = yi;
                            zc = zi;
                        }
                    }
                }
            }
        }
        CellDistanceFunction::Manhattan => {
            for xi in xr - 1..xr + 2 {
                let xisubx = xi as f32 - x;
                for yi in yr - 1..yr + 2 {
                    for zi in zr - 1..zr + 2 {
                        let hi = hash_3d(1337, xi, yi, zi) & 0xff;
                        let vx = xisubx + CELL_3D_X[hi as usize] as f32 * jitter;
                        let vy = yi as f32 - y + CELL_3D_Y[hi as usize] as f32 * jitter;
                        let vz = zi as f32 - z + CELL_3D_Z[hi as usize] as f32 * jitter;
                        let new_dist = vx.abs() + vy.abs() + vz.abs();
                        if new_dist < distance {
                            distance = new_dist;
                            xc = xi;
                            yc = yi;
                            zc = zi;
                        }
                    }
                }
            }
        }
        CellDistanceFunction::Natural => {
            for xi in xr - 1..xr + 2 {
                let xisubx = xi as f32 - x;
                for yi in yr - 1..yr + 2 {
                    for zi in zr - 1..zr + 2 {
                        let hi = hash_3d(1337, xi, yi, zi) & 0xff;
                        let vx = xisubx + CELL_3D_X[hi as usize] as f32 * jitter;
                        let vy = yi as f32 - y + CELL_3D_Y[hi as usize] as f32 * jitter;
                        let vz = zi as f32 - z + CELL_3D_Z[hi as usize] as f32 * jitter;
                        let new_dist =
                            (vx.abs() + vy.abs() + vz.abs()) + (vx * vx + vy * vy + vz * vz);
                        if new_dist < distance {
                            distance = new_dist;
                            xc = xi;
                            yc = yi;
                            zc = zi;
                        }
                    }
                }
            }
        }
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => val_coord_3d(1337, xc, yc, zc),
    }
}

fn grad1(hash: i32, x: f32) -> f32 {
    let h = hash & 15;
    let grad = if h & 8 != 0 {
        -(1.0 + (h & 7) as f32)
    } else {
        1.0 + (h & 7) as f32
    };
    grad * x
}

pub fn simplex_1d(x: f32) -> f32 {
    let i0 = x.floor() as i32;
    let i1 = i0 + 1;
    let x0 = x - i0 as f32;
    let x1 = x0 - 1.0;

    let mut t0 = 1.0 - x0 * x0;
    t0 = t0 * t0;
    let n0 = t0 * t0 * grad1(PERM[(i0 & 0xff) as usize], x0);

    let mut t1 = 1.0 - x1 * x1;
    t1 = t1 * t1;
    let n1 = t1 * t1 * grad1(PERM[(i1 & 0xff) as usize], x1);

    n0 + n1
}

/// Get a single value of 1d fractal brownian motion.
pub fn fbm_1d(x: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut result = simplex_1d(xf);
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_1d(xf) * amp);
    }
    result
}

/// Get a single value of 1d turbulence.
pub fn turbulence_1d(x: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut result = simplex_1d(xf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_1d(xf) * amp).abs();
    }
    result
}

/// Get a single value of 1d ridge noise..
pub fn ridge_1d(x: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut result = 1.0 - simplex_1d(xf).abs();
    let mut amp = 1.0;
    for _ in 1..octaves {
        xf = xf * lacunarity;
        amp = amp * gain;
        result = result + (1.0 - (simplex_1d(xf) * amp).abs());
    }
    result
}

fn get_1d_noise_helper(x: f32, noise_type: NoiseType) -> f32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_1d(x, freq, lacunarity, gain, octaves),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_1d(x, freq, lacunarity, gain, octaves),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_1d(x, freq, lacunarity, gain, octaves),
        NoiseType::Gradient { freq } => simplex_1d(x * freq),
        NoiseType::Cellular {
            freq: _,
            distance_function: _,
            return_type: _,
            jitter: _,
        } => panic!("1D cell noise not implemented"),
    }
}
/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_1d_noise(start_x: f32, width: usize, noise_type: NoiseType) -> (Vec<f32>, f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width);
    unsafe {
        result.set_len(width);
    }
    let mut i = 0;
    let mut x = start_x;
    for _ in 0..width {
        let f = get_1d_noise_helper(x, noise_type);
        if f < min {
            min = f;
        }
        if f > max {
            max = f;
        }
        unsafe {
            *result.get_unchecked_mut(i) = f;
        }
        i += 1;
        x += 1.0;
    }
    (result, min, max)
}

/// Gets a width sized block of scaled 1d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_1d_scaled_noise(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(start_x, width, noise_type);
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    for f in &mut noise {
        *f = *f * multiplier + offset;
    }
    noise
}

fn grad2(hash: i32, x: f32, y: f32) -> f32 {
    let h = hash & 7;
    let (u, v) = if h < 4 { (x, y) } else { (y, x) };
    let a = if (h & 1) != 0 { -u } else { u };
    let b = if (h & 2) != 0 { -2.0 * v } else { 2.0 * v };
    a + b
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.
pub fn simplex_2d(x: f32, y: f32) -> f32 {
    let s = (x + y) * F2;
    let xs = x + s;
    let ys = y + s;
    let i = xs.floor() as i32;
    let j = ys.floor() as i32;
    let t = (i + j) as f32 * G2;
    let x0 = x - (i as f32 - t);
    let y0 = y - (j as f32 - t);

    let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };
    let x1 = x0 - i1 as f32 + G2;
    let y1 = y0 - j1 as f32 + G2;
    let x2 = x0 - 1.0 + G22;
    let y2 = y0 - 1.0 + G22;

    let ii = i & 0xff;
    let jj = j & 0xff;

    unsafe {
        let t0 = 0.5 - x0 * x0 - y0 * y0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            t0 * t0
                * t0
                * t0
                * grad2(
                    *PERM.get_unchecked((ii + *PERM.get_unchecked(jj as usize)) as usize),
                    x0,
                    y0,
                )
        };
        let t1 = 0.5 - x1 * x1 - y1 * y1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            t1 * t1
                * t1
                * t1
                * grad2(
                    *PERM.get_unchecked(
                        (ii + i1 + *PERM.get_unchecked((jj + j1) as usize)) as usize,
                    ),
                    x1,
                    y1,
                )
        };
        let t2 = 0.5 - x2 * x2 - y2 * y2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            t2 * t2
                * t2
                * t2
                * grad2(
                    *PERM.get_unchecked((ii + 1 + *PERM.get_unchecked((jj + 1) as usize)) as usize),
                    x2,
                    y2,
                )
        };

        n0 + n1 + n2
    }
}

/// Get a single value of 2d fractal brownian motion.
pub fn fbm_2d(x: f32, y: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut result = simplex_2d(xf, yf);
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_2d(xf, yf) * amp);
    }
    result
}

/// Get a single value of 2d turbulence.
pub fn turbulence_2d(x: f32, y: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut result = simplex_2d(xf, yf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_2d(xf, yf) * amp).abs();
    }
    result
}

/// Get a single value of 2d ridge noise..
pub fn ridge_2d(x: f32, y: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut result = 1.0 - simplex_2d(xf, yf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        amp = amp * gain;
        result = result + (1.0 - (simplex_2d(xf, yf) * amp).abs());
    }
    result
}

fn get_2d_noise_helper(x: f32, y: f32, noise_type: NoiseType) -> f32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_2d(x, y, freq, lacunarity, gain, octaves),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_2d(x, y, freq, lacunarity, gain, octaves),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_2d(x, y, freq, lacunarity, gain, octaves),
        NoiseType::Gradient { freq } => simplex_2d(x * freq, y * freq),
        NoiseType::Cellular {
            freq,
            distance_function,
            return_type,
            jitter,
        } => cellular_2d(x * freq, y * freq, distance_function, return_type, jitter),
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height);
    unsafe {
        result.set_len(width * height);
    }
    let mut i = 0;
    let mut y = start_y;
    for _ in 0..height {
        let mut x = start_x;
        for _ in 0..width {
            let f = get_2d_noise_helper(x, y, noise_type);
            if f < min {
                min = f;
            }
            if f > max {
                max = f;
            }
            unsafe {
                *result.get_unchecked_mut(i) = f;
            }
            i += 1;
            x += 1.0;
        }
        y += 1.0;
    }
    (result, min, max)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_2d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(start_x, width, start_y, height, noise_type);
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    for f in &mut noise {
        *f = *f * multiplier + offset;
    }
    noise
}

fn grad3(hash: i32, x: f32, y: f32, z: f32) -> f32 {
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

/// Get a single value of 3d simplex noise, results
/// are not scaled.
pub fn simplex_3d(x: f32, y: f32, z: f32) -> f32 {
    let s = (x + y + z) * F3;
    let i = (x + s).floor() as i32;
    let j = (y + s).floor() as i32;
    let k = (z + s).floor() as i32;
    let t = (i + j + k) as f32 * G3;
    let x0 = x - (i as f32 - t);
    let y0 = y - (j as f32 - t);
    let z0 = z - (k as f32 - t);

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
    let x3 = x0 - 0.5;
    let y3 = y0 - 0.5;
    let z3 = z0 - 0.5;

    let ii = i & 255;
    let jj = j & 255;
    let kk = k & 255;

    unsafe {
        let t0 = 0.5 - x0 * x0 - y0 * y0 - z0 * z0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let gi0 = *PERM.get_unchecked(
                (ii + *PERM.get_unchecked((jj + *PERM.get_unchecked(kk as usize)) as usize))
                    as usize,
            );

            t0 * t0 * t0 * t0 * grad3(gi0, x0, y0, z0)
        };
        let t1 = 0.5 - x1 * x1 - y1 * y1 - z1 * z1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let gi1 = *PERM.get_unchecked(
                (ii + i1
                    + *PERM.get_unchecked(
                        (jj + j1 + *PERM.get_unchecked((kk + k1) as usize)) as usize,
                    )) as usize,
            );

            t1 * t1 * t1 * t1 * grad3(gi1, x1, y1, z1)
        };
        let t2 = 0.5 - x2 * x2 - y2 * y2 - z2 * z2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let gi2 = *PERM.get_unchecked(
                (ii + i2
                    + *PERM.get_unchecked(
                        (jj + j2 + *PERM.get_unchecked((kk + k2) as usize)) as usize,
                    )) as usize,
            );

            t2 * t2 * t2 * t2 * grad3(gi2, x2, y2, z2)
        };
        let t3 = 0.5 - x3 * x3 - y3 * y3 - z3 * z3;
        let n3 = if t3 < 0.0 {
            0.0
        } else {
            let gi3 = *PERM.get_unchecked(
                (ii + 1
                    + *PERM
                        .get_unchecked((jj + 1 + *PERM.get_unchecked((kk + 1) as usize)) as usize))
                    as usize,
            );

            t3 * t3 * t3 * t3 * grad3(gi3, x3, y3, z3)
        };

        n0 + n1 + n2 + n3
    }
}

/// Get a single value of 3d fractal brownian motion.
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

/// Get a single value of 3d ridge noise.
pub fn ridge_3d(x: f32, y: f32, z: f32, freq: f32, lacunarity: f32, gain: f32, octaves: u8) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut zf = z * freq;
    let mut result = 1.0 - simplex_3d(xf, yf, zf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        zf = zf * lacunarity;
        amp = amp * gain;
        result = result + (1.0 - (simplex_3d(xf, yf, zf) * amp).abs());
    }
    result
}

/// Get a single value of 3d turbulence.
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

fn get_3d_noise_helper(x: f32, y: f32, z: f32, noise_type: NoiseType) -> f32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_3d(x, y, z, freq, lacunarity, gain, octaves),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_3d(x, y, z, freq, lacunarity, gain, octaves),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_3d(x, y, z, freq, lacunarity, gain, octaves),
        NoiseType::Gradient { freq } => simplex_3d(x * freq, y * freq, z * freq),
        NoiseType::Cellular {
            freq,
            distance_function,
            return_type,
            jitter,
        } => cellular_3d(
            x * freq,
            y * freq,
            z * freq,
            distance_function,
            return_type,
            jitter,
        ),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    let mut result = Vec::with_capacity(width * height * depth);
    unsafe {
        result.set_len(width * height * depth);
    }
    let mut i = 0;
    let mut z = start_z;
    for _ in 0..depth {
        let mut y = start_y;
        for _ in 0..height {
            let mut x = start_x;
            for _ in 0..width {
                let f = get_3d_noise_helper(x, y, z, noise_type);
                if f < min {
                    min = f;
                }
                if f > max {
                    max = f;
                }
                unsafe {
                    *result.get_unchecked_mut(i) = f;
                }
                i += 1;
                x += 1.0;
            }
            y += 1.0;
        }
        z += 1.0;
    }
    (result, min, max)
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_3d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) =
        get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type);
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    for f in &mut noise {
        *f = *f * multiplier + offset;
    }
    noise
}

fn grad4(hash: i32, x: f32, y: f32, z: f32, w: f32) -> f32 {
    let h = hash & 31;
    let u = if h < 24 { x } else { y };
    let v = if h < 16 { y } else { z };
    let w = if h < 8 { x } else { w };

    let a = if (h & 1) != 0 { -u } else { u };
    let b = if (h & 2) != 0 { -v } else { v };
    let c = if (h & 4) != 0 { -w } else { w };
    a + b + c
}

pub fn simplex_4d(x: f32, y: f32, z: f32, w: f32) -> f32 {
    let s = (x + y + z + w) * F4;
    let xs = x + s;
    let ys = y + s;
    let zs = z + s;
    let ws = w + s;
    let i = xs.floor() as i32;
    let j = ys.floor() as i32;
    let k = zs.floor() as i32;
    let l = ws.floor() as i32;

    let t = (i + j + k + l) as f32 * G4;
    let x0 = x - (i as f32 - t);
    let y0 = y - (j as f32 - t);
    let z0 = z - (k as f32 - t);
    let w0 = w - (l as f32 - t);

    let mut rankx: i32 = 0;
    let mut ranky: i32 = 0;
    let mut rankz: i32 = 0;
    let mut rankw: i32 = 0;

    if x0 > y0 {
        rankx += 1;
    } else {
        ranky += 1;
    }
    if x0 > z0 {
        rankx += 1;
    } else {
        rankz += 1;
    }
    if x0 > w0 {
        rankx += 1;
    } else {
        rankw += 1;
    }
    if y0 > z0 {
        ranky += 1;
    } else {
        rankz += 1;
    }
    if y0 > w0 {
        ranky += 1;
    } else {
        rankw += 1;
    }
    if z0 > w0 {
        rankz += 1;
    } else {
        rankw += 1;
    }

    let i1 = if rankx >= 3 { 1 } else { 0 };
    let j1 = if ranky >= 3 { 1 } else { 0 };
    let k1 = if rankz >= 3 { 1 } else { 0 };
    let l1 = if rankw >= 3 { 1 } else { 0 };

    let i2 = if rankx >= 2 { 1 } else { 0 };
    let j2 = if ranky >= 2 { 1 } else { 0 };
    let k2 = if rankz >= 2 { 1 } else { 0 };
    let l2 = if rankw >= 2 { 1 } else { 0 };

    let i3 = if rankx >= 1 { 1 } else { 0 };
    let j3 = if ranky >= 1 { 1 } else { 0 };
    let k3 = if rankz >= 1 { 1 } else { 0 };
    let l3 = if rankw >= 1 { 1 } else { 0 };

    let x1 = x0 - i1 as f32 + G4; // Offsets for second corner in (x,y,z,w) coords
    let y1 = y0 - j1 as f32 + G4;
    let z1 = z0 - k1 as f32 + G4;
    let w1 = w0 - l1 as f32 + G4;
    let x2 = x0 - i2 as f32 + 2.0 * G4; // Offsets for third corner in (x,y,z,w) coords
    let y2 = y0 - j2 as f32 + 2.0 * G4;
    let z2 = z0 - k2 as f32 + 2.0 * G4;
    let w2 = w0 - l2 as f32 + 2.0 * G4;
    let x3 = x0 - i3 as f32 + 3.0 * G4; // Offsets for fourth corner in (x,y,z,w) coords
    let y3 = y0 - j3 as f32 + 3.0 * G4;
    let z3 = z0 - k3 as f32 + 3.0 * G4;
    let w3 = w0 - l3 as f32 + 3.0 * G4;
    let x4 = x0 - 1.0 + 4.0 * G4; // Offsets for last corner in (x,y,z,w) coords
    let y4 = y0 - 1.0 + 4.0 * G4;
    let z4 = z0 - 1.0 + 4.0 * G4;
    let w4 = w0 - 1.0 + 4.0 * G4;

    let ii = i & 0xff;
    let jj = j & 0xff;
    let kk = k & 0xff;
    let ll = l & 0xff;

    let mut t0 = 0.5 - x0 * x0 - y0 * y0 - z0 * z0 - w0 * w0;
    let n0 = if t0 < 0.0 {
        0.0
    } else {
        t0 = t0 * t0;
        t0 = t0 * t0;
        let h = PERM[(ii + PERM[(jj + PERM[(kk + PERM[ll as usize]) as usize]) as usize]) as usize];
        t0 * grad4(h, x0, y0, z0, w0)
    };

    let mut t1 = 0.5 - x1 * x1 - y1 * y1 - z1 * z1 - w1 * w1;
    let n1 = if t1 < 0.0 {
        0.0
    } else {
        t1 = t1 * t1;
        t1 = t1 * t1;
        let h = PERM[(ii
            + i1
            + PERM[(jj + j1 + PERM[(kk + k1 + PERM[(ll + l1) as usize]) as usize]) as usize])
            as usize];
        t1 * grad4(h, x1, y1, z1, w1)
    };

    let mut t2 = 0.5 - x2 * x2 - y2 * y2 - z2 * z2 - w2 * w2;
    let n2 = if t2 < 0.0 {
        0.0
    } else {
        t2 = t2 * t2;
        t2 = t2 * t2;
        let h = PERM[(ii
            + i2
            + PERM[(jj + j2 + PERM[(kk + k2 + PERM[(ll + l2) as usize]) as usize]) as usize])
            as usize];
        t2 * grad4(h, x2, y2, z2, w2)
    };

    let mut t3 = 0.5 - x3 * x3 - y3 * y3 - z3 * z3 - w3 * w3;
    let n3 = if t3 < 0.0 {
        0.0
    } else {
        t3 = t3 * t3;
        t3 = t3 * t3;
        let h = PERM[(ii
            + i3
            + PERM[(jj + j3 + PERM[(kk + k3 + PERM[(ll + l3) as usize]) as usize]) as usize])
            as usize]
            & 31;
        t3 * grad4(h, x3, y3, z3, w3)
    };

    let mut t4 = 0.5 - x4 * x4 - y4 * y4 - z4 * z4 - w4 * w4;
    let n4 = if t4 < 0.0 {
        0.0
    } else {
        t4 = t4 * t4;
        t4 = t4 * t4;
        let h = PERM[(ii
            + 1
            + PERM[(jj + 1 + PERM[(kk + 1 + PERM[(ll + 1) as usize]) as usize]) as usize])
            as usize]
            & 31;
        t4 * grad4(h, x4, y4, z4, w4)
    };
    n0 + n1 + n2 + n3 + n4
}
/// Get a single value of 4d fractal brownian motion.
pub fn fbm_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut zf = z * freq;
    let mut wf = w * freq;
    let mut result = simplex_4d(xf, yf, zf, wf);
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        zf = zf * lacunarity;
        wf = wf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_4d(xf, yf, zf, wf) * amp);
    }
    result
}

/// Get a single value of 4d ridge noise.
pub fn ridge_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut zf = z * freq;
    let mut wf = w * freq;
    let mut result = 1.0 - simplex_4d(xf, yf, zf, wf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        zf = zf * lacunarity;
        wf = wf * lacunarity;
        amp = amp * gain;
        result = result + (1.0 - (simplex_4d(xf, yf, zf, wf) * amp).abs());
    }
    result
}

/// Get a single value of 4d turbulence.
pub fn turbulence_4d(
    x: f32,
    y: f32,
    z: f32,
    w: f32,
    freq: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
) -> f32 {
    let mut xf = x * freq;
    let mut yf = y * freq;
    let mut zf = z * freq;
    let mut wf = w * freq;
    let mut result = simplex_4d(xf, yf, zf, wf).abs();
    let mut amp = 1.0;

    for _ in 1..octaves {
        xf = xf * lacunarity;
        yf = yf * lacunarity;
        zf = zf * lacunarity;
        wf = wf * lacunarity;
        amp = amp * gain;
        result = result + (simplex_4d(xf, yf, zf, wf) * amp).abs();
    }
    result
}

fn get_4d_noise_helper(x: f32, y: f32, z: f32, w: f32, noise_type: NoiseType) -> f32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_4d(x, y, z, w, freq, lacunarity, gain, octaves),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_4d(x, y, z, w, freq, lacunarity, gain, octaves),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_4d(x, y, z, w, freq, lacunarity, gain, octaves),
        NoiseType::Gradient { freq } => simplex_4d(x * freq, y * freq, z * freq, w * freq),
        NoiseType::Cellular { .. } => panic!("not yet implemented"),
    }
}

/// Gets a width X height X depth x time sized block of 3d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_4d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    let mut result = Vec::with_capacity(width * height * depth * time);
    unsafe {
        result.set_len(width * height * depth * time);
    }
    let mut i = 0;
    let mut w = start_w;
    for _ in 0..time {
        let mut z = start_z;
        for _ in 0..depth {
            let mut y = start_y;
            for _ in 0..height {
                let mut x = start_x;
                for _ in 0..width {
                    let f = get_4d_noise_helper(x, y, z, w, noise_type);
                    if f < min {
                        min = f;
                    }
                    if f > max {
                        max = f;
                    }
                    unsafe {
                        *result.get_unchecked_mut(i) = f;
                    }
                    i += 1;
                    x += 1.0;
                }
                y += 1.0;
            }
            z += 1.0;
        }
        w += 1.0;
    }
    (result, min, max)
}

/// Gets a width X height X depth x time sized block of scaled 3d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_4d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(
        start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
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
