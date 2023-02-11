use simdeez::*;

use super::NoiseType;

use crate::noise::cell;
use crate::noise::cell2;
use crate::noise::ridge::{ridge_1d, ridge_2d, ridge_3d, ridge_4d};
use crate::noise::simplex::*;

use std::f32;

macro_rules! get_1d_noise_helper  {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {
 {
    let dim = $Setting.dim;
    let freq_x = S::set1_ps($Setting.freq_x);
    let start_x = dim.x;
    let width = dim.width;
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
        let f = $f(S::mul_ps(x, freq_x) $(,$arg)*);
        max_s = S::max_ps(max_s, f);
        min_s = S::min_ps(min_s, f);
        S::storeu_ps(result.get_unchecked_mut(i), f);
        i += vector_width;
        x = S::add_ps(x, S::set1_ps(vector_width as f32));
    }
    if remainder != 0 {
        let f = $f(S::mul_ps(x, freq_x) $(,$arg)*);
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
    }
}

macro_rules! get_2d_noise_helper {
    ($Setting:expr,$f:expr $(,$arg:expr)*)=> {{
    let dim = $Setting.dim;
    let freq_x = S::set1_ps($Setting.freq_x);
    let freq_y = S::set1_ps($Setting.freq_y);
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;

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
            let f = $f(S::mul_ps(x, freq_x), S::mul_ps(y, freq_y) $(,$arg)*);
            max_s = S::max_ps(max_s, f);
            min_s = S::min_ps(min_s, f);
            S::storeu_ps(result.get_unchecked_mut(i), f);
            i += vector_width;
            x = S::add_ps(x, S::set1_ps(vector_width as f32));
        }
        if remainder != 0 {
            let f = $f(S::mul_ps(x, freq_x), S::mul_ps(y, freq_y) $(,$arg)*);
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

}};
}

macro_rules! get_3d_noise_helper {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {{
    let dim = $Setting.dim;
    let freq_x = S::set1_ps($Setting.freq_x);
    let freq_y = S::set1_ps($Setting.freq_y);
    let freq_z = S::set1_ps($Setting.freq_z);
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;

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
                let f = $f(S::mul_ps(x, freq_x), S::mul_ps(y, freq_y), S::mul_ps(z, freq_z) $(,$arg)*);
                max_s = S::max_ps(max_s, f);
                min_s = S::min_ps(min_s, f);
                S::storeu_ps(result.get_unchecked_mut(i), f);
                i += vector_width;
                x = S::add_ps(x, S::set1_ps(vector_width as f32));
            }
            if remainder != 0 {
            let f = $f(S::mul_ps(x, freq_x), S::mul_ps(y, freq_y), S::mul_ps(z, freq_z) $(,$arg)*);
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
}};
}

macro_rules! get_4d_noise_helper {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {{
    let dim = $Setting.dim;
    let freq_x = S::set1_ps($Setting.freq_x);
    let freq_y = S::set1_ps($Setting.freq_y);
    let freq_z = S::set1_ps($Setting.freq_z);
    let freq_w = S::set1_ps($Setting.freq_w);
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;

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
                    let f = $f(S::mul_ps(x, freq_x), S::mul_ps(y, freq_y), S::mul_ps(z, freq_z), S::mul_ps(w, freq_w) $(,$arg)*);
                    max_s = S::max_ps(max_s, f);
                    min_s = S::min_ps(min_s, f);
                    S::storeu_ps(result.get_unchecked_mut(i), f);
                    i += vector_width;
                    x = S::add_ps(x, S::set1_ps(vector_width as f32));
                }
                if remainder != 0 {
                    let f = $f(S::mul_ps(x, freq_x), S::mul_ps(y, freq_y), S::mul_ps(z, freq_z), S::mul_ps(w, freq_w) $(,$arg)*);
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
}};
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper!(
            s,
            fbm_1d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Ridge(s) => get_1d_noise_helper!(
            s,
            ridge_1d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Turbulence(s) => get_1d_noise_helper!(
            s,
            turbulence_1d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Gradient(s) => get_1d_noise_helper!(s, simplex_1d::<S>, s.dim.seed),
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_2d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper!(
            s,
            fbm_2d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Ridge(s) => get_2d_noise_helper!(
            s,
            ridge_2d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Turbulence(s) => get_2d_noise_helper!(
            s,
            turbulence_2d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Gradient(s) => get_2d_noise_helper!(s, simplex_2d::<S>, s.dim.seed),
        NoiseType::Cellular(s) => get_2d_noise_helper!(
            s,
            cell::cellular_2d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_ps(s.jitter),
            s.dim.seed
        ),
        NoiseType::Cellular2(s) => get_2d_noise_helper!(
            s,
            cell2::cellular2_2d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_ps(s.jitter),
            s.index0,
            s.index1,
            s.dim.seed
        ),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper!(
            s,
            fbm_3d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Ridge(s) => get_3d_noise_helper!(
            s,
            ridge_3d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Turbulence(s) => get_3d_noise_helper!(
            s,
            turbulence_3d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Gradient(s) => get_3d_noise_helper!(s, simplex_3d::<S>, s.dim.seed),
        NoiseType::Cellular(s) => get_3d_noise_helper!(
            s,
            cell::cellular_3d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_ps(s.jitter),
            s.dim.seed
        ),
        NoiseType::Cellular2(s) => get_3d_noise_helper!(
            s,
            cell2::cellular2_3d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_ps(s.jitter),
            s.index0,
            s.index1,
            s.dim.seed
        ),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper!(
            s,
            fbm_4d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Ridge(s) => get_4d_noise_helper!(
            s,
            ridge_4d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Turbulence(s) => get_4d_noise_helper!(
            s,
            turbulence_4d::<S>,
            S::set1_ps(s.lacunarity),
            S::set1_ps(s.gain),
            s.octaves,
            s.dim.seed
        ),
        NoiseType::Gradient(s) => get_4d_noise_helper!(s, simplex_4d::<S>, s.dim.seed),
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}
