use self::simdeez::*;
use super::*;
use crate::cellular::*;
use crate::simplex::*;
use std::f32;

#[inline(always)]
unsafe fn get_1d_noise_helper<S: Simd>(x: S::Vf32, noise_type: NoiseType) -> S::Vf32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_1d::<S>(
            x,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_1d::<S>(
            x,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_1d::<S>(
            x,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Normal { freq } => simplex_1d::<S>(S::mul_ps(x, S::set1_ps(freq))),

        NoiseType::Cellular {
            freq: _,
            distance_function: _,
            return_type: _,
            jitter: _,
        } => panic!("1D Cell Noise Not Implemented"),
    }
}

#[inline(always)]
pub unsafe fn get_1d_noise<S: Simd>(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = S::set1_ps(f32::MAX);
    let mut max_s = S::set1_ps(f32::MIN);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result: Vec<f32> = Vec::with_capacity(width);
    result.set_len(width);
    let mut i = 0;
    let vector_width = S::VF32_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f32;
    }
    let mut x = S::loadu_ps(&x_arr[0]);

    for _ in 0..width / vector_width {
        let f = get_1d_noise_helper::<S>(x, noise_type);
        max_s = S::max_ps(max_s, f);
        min_s = S::min_ps(min_s, f);
        S::storeu_ps(result.get_unchecked_mut(i), f);
        i += vector_width;
        x = S::add_ps(x, S::set1_ps(vector_width as f32));
    }
    if remainder != 0 {
        let f = get_1d_noise_helper::<S>(x, noise_type);
        for j in 0..remainder {
            let n = f[j];
            *result.get_unchecked_mut(i) = n;
            // Note: This is unecessary for large images
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            i += 1;
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}

#[inline(always)]
unsafe fn get_2d_noise_helper<S: Simd>(x: S::Vf32, y: S::Vf32, noise_type: NoiseType) -> S::Vf32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_2d::<S>(
            x,
            y,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_2d::<S>(
            x,
            y,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_2d::<S>(
            x,
            y,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Normal { freq } => simplex_2d::<S>(
            S::mul_ps(x, S::set1_ps(freq)),
            S::mul_ps(y, S::set1_ps(freq)),
        ),
        NoiseType::Cellular {
            freq,
            distance_function,
            return_type,
            jitter,
        } => cellular_2d::<S>(
            S::mul_ps(x, S::set1_ps(freq)),
            S::mul_ps(y, S::set1_ps(freq)),
            distance_function,
            return_type,
            S::set1_ps(jitter),
        ),
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
pub unsafe fn get_2d_noise<S: Simd>(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = S::set1_ps(f32::MAX);
    let mut max_s = S::set1_ps(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height);
    result.set_len(width * height);
    let mut y = S::set1_ps(start_y);
    let mut i = 0;
    let vector_width = S::VF32_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f32;
    }
    for _ in 0..height {
        let mut x = S::loadu_ps(&x_arr[0]);
        for _ in 0..width / vector_width {
            let f = get_2d_noise_helper::<S>(x, y, noise_type);
            max_s = S::max_ps(max_s, f);
            min_s = S::min_ps(min_s, f);
            S::storeu_ps(result.get_unchecked_mut(i), f);
            i += vector_width;
            x = S::add_ps(x, S::set1_ps(vector_width as f32));
        }
        if remainder != 0 {
            let f = get_2d_noise_helper::<S>(x, y, noise_type);
            for j in 0..remainder {
                let n = f[j];
                *result.get_unchecked_mut(i) = n;
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
        y = S::add_ps(y, S::set1_ps(1.0));
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}
#[inline(always)]
unsafe fn get_3d_noise_helper<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    noise_type: NoiseType,
) -> S::Vf32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_3d::<S>(
            x,
            y,
            z,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_3d::<S>(
            x,
            y,
            z,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_3d::<S>(
            x,
            y,
            z,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Normal { freq } => simplex_3d::<S>(
            S::mul_ps(x, S::set1_ps(freq)),
            S::mul_ps(y, S::set1_ps(freq)),
            S::mul_ps(z, S::set1_ps(freq)),
        ),
        NoiseType::Cellular {
            freq,
            distance_function,
            return_type,
            jitter,
        } => cellular_3d::<S>(
            S::mul_ps(x, S::set1_ps(freq)),
            S::mul_ps(y, S::set1_ps(freq)),
            S::mul_ps(z, S::set1_ps(freq)),
            distance_function,
            return_type,
            S::set1_ps(jitter),
        ),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
pub unsafe fn get_3d_noise<S: Simd>(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = S::set1_ps(f32::MAX);
    let mut max_s = S::set1_ps(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height * depth);
    result.set_len(width * height * depth);
    let mut i = 0;
    let vector_width = S::VF32_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f32;
    }

    let mut z = S::set1_ps(start_z);
    for _ in 0..depth {
        let mut y = S::set1_ps(start_y);
        for _ in 0..height {
            let mut x = S::loadu_ps(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = get_3d_noise_helper::<S>(x, y, z, noise_type);
                max_s = S::max_ps(max_s, f);
                min_s = S::min_ps(min_s, f);
                S::storeu_ps(result.get_unchecked_mut(i), f);
                i += vector_width;
                x = S::add_ps(x, S::set1_ps(vector_width as f32));
            }
            if remainder != 0 {
                let f = get_3d_noise_helper::<S>(x, y, z, noise_type);
                for j in 0..remainder {
                    let n = f[j];
                    *result.get_unchecked_mut(i) = n;
                    if n < min {
                        min = n;
                    }
                    if n > max {
                        max = n;
                    }
                    i += 1;
                }
            }
            y = S::add_ps(y, S::set1_ps(1.0));
        }
        z = S::add_ps(z, S::set1_ps(1.0));
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}
#[inline(always)]
unsafe fn get_4d_noise_helper<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    noise_type: NoiseType,
) -> S::Vf32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_4d::<S>(
            x,
            y,
            z,
            w,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_4d::<S>(
            x,
            y,
            z,
            w,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_4d::<S>(
            x,
            y,
            z,
            w,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Normal { freq } => simplex_4d::<S>(
            S::mul_ps(x, S::set1_ps(freq)),
            S::mul_ps(y, S::set1_ps(freq)),
            S::mul_ps(z, S::set1_ps(freq)),
            S::mul_ps(w, S::set1_ps(freq)),
        ),
        NoiseType::Cellular { .. } => panic!("not yet implemented"),
    }
}
#[inline(always)]
pub unsafe fn get_4d_noise<S: Simd>(
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
    let mut min_s = S::set1_ps(f32::MAX);
    let mut max_s = S::set1_ps(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height * depth * time);
    result.set_len(width * height * depth * time);
    let mut i = 0;
    let vector_width = S::VF32_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f32;
    }
    let mut w = S::set1_ps(start_w);
    for _ in 0..time {
        let mut z = S::set1_ps(start_z);
        for _ in 0..depth {
            let mut y = S::set1_ps(start_y);
            for _ in 0..height {
                let mut x = S::loadu_ps(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = get_4d_noise_helper::<S>(x, y, z, w, noise_type);
                    max_s = S::max_ps(max_s, f);
                    min_s = S::min_ps(min_s, f);
                    S::storeu_ps(result.get_unchecked_mut(i), f);
                    i += vector_width;
                    x = S::add_ps(x, S::set1_ps(vector_width as f32));
                }
                if remainder != 0 {
                    let f = get_4d_noise_helper::<S>(x, y, z, w, noise_type);
                    for j in 0..remainder {
                        let n = f[j];
                        *result.get_unchecked_mut(i) = n;
                        // Note: This is unecessary for large images
                        if n < min {
                            min = n;
                        }
                        if n > max {
                            max = n;
                        }
                        i += 1;
                    }
                }
                y = S::add_ps(y, S::set1_ps(1.0));
            }
            z = S::add_ps(z, S::set1_ps(1.0));
        }
        w = S::add_ps(w, S::set1_ps(1.0));
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (result, min, max)
}
