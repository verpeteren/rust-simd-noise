use simdeez::prelude::*;

use super::NoiseType;
use crate::dimensional_being::DimensionalBeing;

use crate::noise::cell2_64::{cellular2_2d, cellular2_3d};
use crate::noise::cell_64::{cellular_2d, cellular_3d};
use crate::noise::fbm_64::{fbm_1d, fbm_2d, fbm_3d, fbm_4d};
use crate::noise::ridge_64::{ridge_1d, ridge_2d, ridge_3d, ridge_4d};
use crate::noise::simplex_64::{simplex_1d, simplex_2d, simplex_3d, simplex_4d};
use crate::noise::turbulence_64::{turbulence_1d, turbulence_2d, turbulence_3d, turbulence_4d};

use std::f64;

macro_rules! get_1d_noise_helper_f64  {
    ($Setting:expr,$f:expr $(,$arg:expr)*) => {
 {
    let dim = $Setting.get_dimensions();
    let freq_x = S::Vf64::set1($Setting.freq_x as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result: Vec<f64> = Vec::with_capacity(width);
    result.set_len(width);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }
    let mut x = S::loadu_pd(&x_arr[0]);
    for _ in 0..width / vector_width {
        let f = $f(S::mul_pd(x, freq_x) $(,$arg)*);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        S::storeu_pd(result.get_unchecked_mut(i), f);
        i += vector_width;
        x = S::add_pd(x, S::Vf64::set1(vector_width as f64));
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
    let dim = $Setting.get_dimensions();
    let freq_x = S::Vf64::set1($Setting.freq_x as f64);
    let freq_y = S::Vf64::set1($Setting.freq_y as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::with_capacity(width * height);
    result.set_len(width * height);
    let mut y = S::Vf64::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
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
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            S::storeu_pd(result.get_unchecked_mut(i), f);
            i += vector_width;
            x = S::add_pd(x, S::Vf64::set1(vector_width as f64));
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
        y = S::add_pd(y, S::Vf64::set1(1.0));
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
    let dim = $Setting.get_dimensions();
    let freq_x = S::Vf64::set1($Setting.freq_x as f64);
    let freq_y = S::Vf64::set1($Setting.freq_y as f64);
    let freq_z = S::Vf64::set1($Setting.freq_z as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::with_capacity(width * height * depth);
    result.set_len(width * height * depth);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }

    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::loadu_pd(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y), S::mul_pd(z, freq_z) $(,$arg)*);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                S::storeu_pd(result.get_unchecked_mut(i), f);
                i += vector_width;
                x = S::add_pd(x, S::Vf64::set1(vector_width as f64));
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
            y = S::add_pd(y, S::Vf64::set1(1.0));
        }
        z = S::add_pd(z, S::Vf64::set1(1.0));
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
    let dim = $Setting.get_dimensions();
    let freq_x = S::Vf64::set1($Setting.freq_x as f64);
    let freq_y = S::Vf64::set1($Setting.freq_y as f64);
    let freq_z = S::Vf64::set1($Setting.freq_z as f64);
    let freq_w = S::Vf64::set1($Setting.freq_w as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut result = Vec::with_capacity(width * height * depth * time);
    result.set_len(width * height * depth * time);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f64;
    }
    let mut w = S::Vf64::set1(start_w);
    for _ in 0..time {
        let mut z = S::Vf64::set1(start_z);
        for _ in 0..depth {
            let mut y = S::Vf64::set1(start_y);
            for _ in 0..height {
                let mut x = S::loadu_pd(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = $f(S::mul_pd(x, freq_x), S::mul_pd(y, freq_y), S::mul_pd(z, freq_z), S::mul_pd(w, freq_w) $(,$arg)*);
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    S::storeu_pd(result.get_unchecked_mut(i), f);
                    i += vector_width;
                    x = S::add_pd(x, S::Vf64::set1(vector_width as f64));
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
                y = S::add_pd(y, S::Vf64::set1(1.0));
            }
            z = S::add_pd(z, S::Vf64::set1(1.0));
        }
        w = S::add_pd(w, S::Vf64::set1(1.0));
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
pub unsafe fn get_1d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f64!(
            s,
            fbm_1d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Ridge(s) => get_1d_noise_helper_f64!(
            s,
            ridge_1d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Turbulence(s) => get_1d_noise_helper_f64!(
            s,
            turbulence_1d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Gradient(s) => {
            get_1d_noise_helper_f64!(s, simplex_1d::<S>, s.get_dimensions().seed as i64)
        }
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
pub unsafe fn get_2d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_helper_f64!(
            s,
            fbm_2d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Ridge(s) => get_2d_noise_helper_f64!(
            s,
            ridge_2d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Turbulence(s) => get_2d_noise_helper_f64!(
            s,
            turbulence_2d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Gradient(s) => {
            get_2d_noise_helper_f64!(s, simplex_2d::<S>, s.get_dimensions().seed as i64)
        }
        NoiseType::Cellular(s) => get_2d_noise_helper_f64!(
            s,
            cellular_2d::<S>,
            s.distance_function,
            s.return_type,
            S::Vf64::set1(s.jitter as f64),
            s.get_dimensions().seed as i64
        ),
        NoiseType::Cellular2(s) => get_2d_noise_helper_f64!(
            s,
            cellular2_2d::<S>,
            s.distance_function,
            s.return_type,
            S::Vf64::set1(s.jitter as f64),
            s.index0,
            s.index1,
            s.get_dimensions().seed as i64
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
pub unsafe fn get_3d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_helper_f64!(
            s,
            fbm_3d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Ridge(s) => get_3d_noise_helper_f64!(
            s,
            ridge_3d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Turbulence(s) => get_3d_noise_helper_f64!(
            s,
            turbulence_3d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Gradient(s) => {
            get_3d_noise_helper_f64!(s, simplex_3d::<S>, s.get_dimensions().seed as i64)
        }
        NoiseType::Cellular(s) => get_3d_noise_helper_f64!(
            s,
            cellular_3d::<S>,
            s.distance_function,
            s.return_type,
            S::Vf64::set1(s.jitter as f64),
            s.get_dimensions().seed as i64
        ),
        NoiseType::Cellular2(s) => get_3d_noise_helper_f64!(
            s,
            cellular2_3d::<S>,
            s.distance_function,
            s.return_type,
            S::Vf64::set1(s.jitter as f64),
            s.index0,
            s.index1,
            s.get_dimensions().seed as i64
        ),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper_f64!(
            s,
            fbm_4d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Ridge(s) => get_4d_noise_helper_f64!(
            s,
            ridge_4d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Turbulence(s) => get_4d_noise_helper_f64!(
            s,
            turbulence_4d::<S>,
            S::Vf64::set1(s.lacunarity as f64),
            S::Vf64::set1(s.gain as f64),
            s.octaves,
            s.get_dimensions().seed as i64
        ),
        NoiseType::Gradient(s) => {
            get_4d_noise_helper_f64!(s, simplex_4d::<S>, s.get_dimensions().seed as i64)
        }
        NoiseType::Cellular(_) => {
            panic!("not implemented");
        }
        NoiseType::Cellular2(_) => {
            panic!("not implemented");
        }
    }
}
