use self::simdeez::*;

use super::*;

use crate::cellular_64::*;
use crate::simplex_64::*;

use std::f64;

macro_rules! get_1d_noise_helper_f64  {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {
 {
    let dim = $Setting.dim;
    let freq_x = S::set1_pd($Setting.freq_x as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let mut min_s = S::set1_pd(f64::MAX);
    let mut max_s = S::set1_pd(f64::MIN);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result: Vec<f64> = Vec::with_capacity(width);
    result.set_len(width);
    let mut i = 0;
    let vector_width = S::VF64_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }
    let mut x = S::loadu_pd(&x_arr[0]);
    for _ in 0..width / vector_width {
        let f = $f(S::mul_pd(x, freq_x) $(,$arg)*);
        max_s = S::max_pd(max_s, f);
        min_s = S::min_pd(min_s, f);
        S::storeu_pd(result.get_unchecked_mut(i), f);
        i += vector_width;
        x = S::add_pd(x, S::set1_pd(vector_width as f64));
    }
    if remainder != 0 {
        let f = $f(S::mul_pd(x, freq_x) $(,$arg)*);
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

macro_rules! get_2d_noise_helper_f64 {
    ($Setting:expr,$f:expr $(,$arg:expr)*)=> {{
    let dim = $Setting.dim;
    let freq_x = S::set1_pd($Setting.freq_x as f64);
    let freq_y = S::set1_pd($Setting.freq_y as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;

    let mut min_s = S::set1_pd(f64::MAX);
    let mut max_s = S::set1_pd(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::with_capacity(width * height);
    result.set_len(width * height);
    let mut y = S::set1_pd(start_y);
    let mut i = 0;
    let vector_width = S::VF64_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }
    for _ in 0..height {
        let mut x = S::loadu_pd(&x_arr[0]);
        for _ in 0..width / vector_width {
            let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y) $(,$arg)*);
            max_s = S::max_pd(max_s, f);
            min_s = S::min_pd(min_s, f);
            S::storeu_pd(result.get_unchecked_mut(i), f);
            i += vector_width;
            x = S::add_pd(x, S::set1_pd(vector_width as f64));
        }
        if remainder != 0 {
            let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y) $(,$arg)*);
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
        y = S::add_pd(y, S::set1_pd(1.0));
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

macro_rules! get_3d_noise_helper_f64 {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {{
    let dim = $Setting.dim;
    let freq_x = S::set1_pd($Setting.freq_x as f64);
    let freq_y = S::set1_pd($Setting.freq_y as f64);
    let freq_z = S::set1_pd($Setting.freq_z as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let mut min_s = S::set1_pd(f64::MAX);
    let mut max_s = S::set1_pd(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::with_capacity(width * height * depth);
    result.set_len(width * height * depth);
    let mut i = 0;
    let vector_width = S::VF64_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }

    let mut z = S::set1_pd(start_z);
    for _ in 0..depth {
        let mut y = S::set1_pd(start_y);
        for _ in 0..height {
            let mut x = S::loadu_pd(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y), S::mul_pd(z, freq_z) $(,$arg)*);
                max_s = S::max_pd(max_s, f);
                min_s = S::min_pd(min_s, f);
                S::storeu_pd(result.get_unchecked_mut(i), f);
                i += vector_width;
                x = S::add_pd(x, S::set1_pd(vector_width as f64));
            }
            if remainder != 0 {
            let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y), S::mul_pd(z, freq_z) $(,$arg)*);
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
            y = S::add_pd(y, S::set1_pd(1.0));
        }
        z = S::add_pd(z, S::set1_pd(1.0));
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

macro_rules! get_4d_noise_helper_f64 {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {{
    let dim = $Setting.dim;
    let freq_x = S::set1_pd($Setting.freq_x as f64);
    let freq_y = S::set1_pd($Setting.freq_y as f64);
    let freq_z = S::set1_pd($Setting.freq_z as f64);
    let freq_w = S::set1_pd($Setting.freq_w as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;

    let mut min_s = S::set1_pd(f64::MAX);
    let mut max_s = S::set1_pd(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::with_capacity(width * height * depth * time);
    result.set_len(width * height * depth * time);
    let mut i = 0;
    let vector_width = S::VF64_WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }
    let mut w = S::set1_pd(start_w);
    for _ in 0..time {
        let mut z = S::set1_pd(start_z);
        for _ in 0..depth {
            let mut y = S::set1_pd(start_y);
            for _ in 0..height {
                let mut x = S::loadu_pd(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y), S::mul_pd(z, freq_z), S::mul_pd(w, freq_w) $(,$arg)*);
                    max_s = S::max_pd(max_s, f);
                    min_s = S::min_pd(min_s, f);
                    S::storeu_pd(result.get_unchecked_mut(i), f);
                    i += vector_width;
                    x = S::add_pd(x, S::set1_pd(vector_width as f64));
                }
                if remainder != 0 {
                    let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y), S::mul_pd(z, freq_z), S::mul_pd(w, freq_w) $(,$arg)*);
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
                y = S::add_pd(y, S::set1_pd(1.0));
            }
            z = S::add_pd(z, S::set1_pd(1.0));
        }
        w = S::add_pd(w, S::set1_pd(1.0));
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
pub unsafe fn get_1d_noise_f64<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f64!(
            s,
            fbm_1d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Ridge(s) => get_1d_noise_helper_f64!(
            s,
            ridge_1d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Turbulence(s) => get_1d_noise_helper_f64!(
            s,
            turbulence_1d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Gradient(s) => get_1d_noise_helper_f64!(s, simplex_1d::<S>, s.dim.seed as i64),
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
pub unsafe fn get_2d_noise_f64<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper_f64!(
            s,
            fbm_2d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Ridge(s) => get_2d_noise_helper_f64!(
            s,
            ridge_2d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Turbulence(s) => get_2d_noise_helper_f64!(
            s,
            turbulence_2d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Gradient(s) => get_2d_noise_helper_f64!(s, simplex_2d::<S>, s.dim.seed as i64),
        NoiseType::Cellular(s) => get_2d_noise_helper_f64!(
            s,
            cellular_2d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_pd(s.jitter as f64),
            s.dim.seed as i64
        ),
        NoiseType::Cellular2(s) => get_2d_noise_helper_f64!(
            s,
            cellular2_2d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_pd(s.jitter as f64),
            s.index0,
            s.index1,
            s.dim.seed as i64
        ),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
pub unsafe fn get_3d_noise_f64<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper_f64!(
            s,
            fbm_3d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Ridge(s) => get_3d_noise_helper_f64!(
            s,
            ridge_3d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Turbulence(s) => get_3d_noise_helper_f64!(
            s,
            turbulence_3d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Gradient(s) => get_3d_noise_helper_f64!(s, simplex_3d::<S>, s.dim.seed as i64),
        NoiseType::Cellular(s) => get_3d_noise_helper_f64!(
            s,
            cellular_3d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_pd(s.jitter as f64),
            s.dim.seed as i64
        ),
        NoiseType::Cellular2(s) => get_3d_noise_helper_f64!(
            s,
            cellular2_3d::<S>,
            s.distance_function,
            s.return_type,
            S::set1_pd(s.jitter as f64),
            s.index0,
            s.index1,
            s.dim.seed as i64
        ),
    }
}

#[inline(always)]
pub unsafe fn get_4d_noise_f64<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper_f64!(
            s,
            fbm_4d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Ridge(s) => get_4d_noise_helper_f64!(
            s,
            ridge_4d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Turbulence(s) => get_4d_noise_helper_f64!(
            s,
            turbulence_4d::<S>,
            S::set1_pd(s.lacunarity as f64),
            S::set1_pd(s.gain as f64),
            s.octaves,
            s.dim.seed as i64
        ),
        NoiseType::Gradient(s) => get_4d_noise_helper_f64!(s, simplex_4d::<S>, s.dim.seed as i64),
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}
