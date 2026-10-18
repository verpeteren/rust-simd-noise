use simdeez::prelude::*;

use super::NoiseType;
use crate::dimensional_being::DimensionalBeing;
use crate::noise::cell2_64::{cellular2_2d_deriv, cellular2_3d_deriv};
use crate::noise::cell_64::{cellular_2d_deriv, cellular_3d_deriv};
use crate::Cell2ReturnType;

use crate::{
    Cellular2Settings, CellularSettings, FbmSettings, GradientSettings, NoiseAndGradient64,
    RidgeSettings, Settings, TurbulenceSettings,
};

use std::f64;

pub trait Sample64<S: Simd>: DimensionalBeing + Settings {
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64;
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64;
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64;
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64;
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64;
    #[allow(clippy::too_many_arguments)]
    fn sample_6d(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
        t: S::Vf64,
    ) -> S::Vf64;
}

/// f64 twin of `noise_helpers_32::SampleFractalDeriv32` (issue #24): value +
/// analytic gradient per sample, with respect to the coordinate(s) passed in
/// (the grid helper applies the per-axis frequency). Implemented for gradient,
/// FBM, ridge and turbulence; the `get_*_fractal_gradient_64` helpers are
/// generic over it.
pub trait SampleFractalDeriv64<S: Simd>: DimensionalBeing + Settings {
    fn sample_1d_deriv(&self, x: S::Vf64) -> (S::Vf64, S::Vf64);
    fn sample_2d_deriv(&self, x: S::Vf64, y: S::Vf64) -> (S::Vf64, [S::Vf64; 2]);
    fn sample_3d_deriv(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> (S::Vf64, [S::Vf64; 3]);
    fn sample_4d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 4]);
    fn sample_5d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 5]);
    #[allow(clippy::too_many_arguments)]
    fn sample_6d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
        t: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 6]);
}

#[inline(always)]
unsafe fn get_1d_noise_into_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
    result: &mut [f64],
) -> (f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    assert_eq!(
        result.len(),
        width,
        "noise output slice length must equal width"
    );
    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
    for _ in 0..width / vector_width {
        let f = settings.sample_1d(x * freq_x);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        f.copy_to_ptr_unaligned(result_ptr.add(i));
        i += vector_width;
        x += S::Vf64::set1(vector_width as f64);
    }
    if remainder != 0 {
        let f = settings.sample_1d(x * freq_x);
        for j in 0..remainder {
            let n = f[j];
            result_ptr.add(i).write(n);
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
    (min, max)
}

#[inline(always)]
unsafe fn get_1d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let width = settings.get_dimensions().width;
    let mut result = vec![0.0f64; width];
    let (min, max) = get_1d_noise_into_helper_f64::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_2d_noise_into_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
    result: &mut [f64],
) -> (f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    assert_eq!(
        result.len(),
        width * height,
        "noise output slice length must equal width * height"
    );

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut y = S::Vf64::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    for _ in 0..height {
        let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
        for _ in 0..width / vector_width {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += vector_width;
            x += S::Vf64::set1(vector_width as f64);
        }
        if remainder != 0 {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            for j in 0..remainder {
                let n = f[j];
                result_ptr.add(i).write(n);
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
        y += S::Vf64::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

#[inline(always)]
unsafe fn get_2d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height;
    let mut result = vec![0.0f64; len];
    let (min, max) = get_2d_noise_into_helper_f64::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_3d_noise_into_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
    result: &mut [f64],
) -> (f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(settings.get_freq_z() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    assert_eq!(
        result.len(),
        width * height * depth,
        "noise output slice length must equal width * height * depth"
    );

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += vector_width;
                x += S::Vf64::set1(vector_width as f64);
            }
            if remainder != 0 {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                for j in 0..remainder {
                    let n = f[j];
                    result_ptr.add(i).write(n);
                    if n < min {
                        min = n;
                    }
                    if n > max {
                        max = n;
                    }
                    i += 1;
                }
            }
            y += S::Vf64::set1(1.0);
        }
        z += S::Vf64::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

#[inline(always)]
unsafe fn get_3d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth;
    let mut result = vec![0.0f64; len];
    let (min, max) = get_3d_noise_into_helper_f64::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_4d_noise_into_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
    result: &mut [f64],
) -> (f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(settings.get_freq_z() as f64);
    let freq_w = S::Vf64::set1(settings.get_freq_w() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;
    assert_eq!(
        result.len(),
        width * height * depth * time,
        "noise output slice length must equal width * height * depth * time"
    );

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    let mut w = S::Vf64::set1(start_w);
    for _ in 0..time {
        let mut z = S::Vf64::set1(start_z);
        for _ in 0..depth {
            let mut y = S::Vf64::set1(start_y);
            for _ in 0..height {
                let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = settings.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    f.copy_to_ptr_unaligned(result_ptr.add(i));
                    i += vector_width;
                    x += S::Vf64::set1(vector_width as f64);
                }
                if remainder != 0 {
                    let f = settings.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    for j in 0..remainder {
                        let n = f[j];
                        result_ptr.add(i).write(n);
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
                y += S::Vf64::set1(1.0);
            }
            z += S::Vf64::set1(1.0);
        }
        w += S::Vf64::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

#[inline(always)]
unsafe fn get_4d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth * dim.time;
    let mut result = vec![0.0f64; len];
    let (min, max) = get_4d_noise_into_helper_f64::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_5d_noise_into_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
    result: &mut [f64],
) -> (f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(settings.get_freq_z() as f64);
    let freq_w = S::Vf64::set1(settings.get_freq_w() as f64);
    let freq_u = S::Vf64::set1(settings.get_freq_u() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;
    let start_u = dim.offset5 as f64;
    let extent5 = dim.extent5;
    assert_eq!(
        result.len(),
        width * height * depth * time * extent5,
        "noise output slice length must equal width * height * depth * time * extent5"
    );

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    let mut u = S::Vf64::set1(start_u);
    for _ in 0..extent5 {
        let mut w = S::Vf64::set1(start_w);
        for _ in 0..time {
            let mut z = S::Vf64::set1(start_z);
            for _ in 0..depth {
                let mut y = S::Vf64::set1(start_y);
                for _ in 0..height {
                    let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
                    for _ in 0..width / vector_width {
                        let f = settings.sample_5d(
                            x * freq_x,
                            y * freq_y,
                            z * freq_z,
                            w * freq_w,
                            u * freq_u,
                        );
                        max_s = max_s.max(f);
                        min_s = min_s.min(f);
                        f.copy_to_ptr_unaligned(result_ptr.add(i));
                        i += vector_width;
                        x += S::Vf64::set1(vector_width as f64);
                    }
                    if remainder != 0 {
                        let f = settings.sample_5d(
                            x * freq_x,
                            y * freq_y,
                            z * freq_z,
                            w * freq_w,
                            u * freq_u,
                        );
                        for j in 0..remainder {
                            let n = f[j];
                            result_ptr.add(i).write(n);
                            if n < min {
                                min = n;
                            }
                            if n > max {
                                max = n;
                            }
                            i += 1;
                        }
                    }
                    y += S::Vf64::set1(1.0);
                }
                z += S::Vf64::set1(1.0);
            }
            w += S::Vf64::set1(1.0);
        }
        u += S::Vf64::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

#[inline(always)]
unsafe fn get_5d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth * dim.time * dim.extent5;
    let mut result = vec![0.0f64; len];
    let (min, max) = get_5d_noise_into_helper_f64::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_6d_noise_into_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
    result: &mut [f64],
) -> (f64, f64) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf64::set1(settings.get_freq_x() as f64);
    let freq_y = S::Vf64::set1(settings.get_freq_y() as f64);
    let freq_z = S::Vf64::set1(settings.get_freq_z() as f64);
    let freq_w = S::Vf64::set1(settings.get_freq_w() as f64);
    let freq_u = S::Vf64::set1(settings.get_freq_u() as f64);
    let freq_v = S::Vf64::set1(settings.get_freq_v() as f64);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;
    let start_u = dim.offset5 as f64;
    let extent5 = dim.extent5;
    let start_v = dim.offset6 as f64;
    let extent6 = dim.extent6;
    assert_eq!(
        result.len(),
        width * height * depth * time * extent5 * extent6,
        "noise output slice length must equal width * height * depth * time * extent5 * extent6"
    );

    let mut min_s = S::Vf64::set1(f64::MAX);
    let mut max_s = S::Vf64::set1(f64::MIN);
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf64::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);
    let mut v = S::Vf64::set1(start_v);
    for _ in 0..extent6 {
        let mut u = S::Vf64::set1(start_u);
        for _ in 0..extent5 {
            let mut w = S::Vf64::set1(start_w);
            for _ in 0..time {
                let mut z = S::Vf64::set1(start_z);
                for _ in 0..depth {
                    let mut y = S::Vf64::set1(start_y);
                    for _ in 0..height {
                        let mut x = S::Vf64::load_from_ptr_unaligned(&x_arr[0]);
                        for _ in 0..width / vector_width {
                            let f = settings.sample_6d(
                                x * freq_x,
                                y * freq_y,
                                z * freq_z,
                                w * freq_w,
                                u * freq_u,
                                v * freq_v,
                            );
                            max_s = max_s.max(f);
                            min_s = min_s.min(f);
                            f.copy_to_ptr_unaligned(result_ptr.add(i));
                            i += vector_width;
                            x += S::Vf64::set1(vector_width as f64);
                        }
                        if remainder != 0 {
                            let f = settings.sample_6d(
                                x * freq_x,
                                y * freq_y,
                                z * freq_z,
                                w * freq_w,
                                u * freq_u,
                                v * freq_v,
                            );
                            for j in 0..remainder {
                                let n = f[j];
                                result_ptr.add(i).write(n);
                                if n < min {
                                    min = n;
                                }
                                if n > max {
                                    max = n;
                                }
                                i += 1;
                            }
                        }
                        y += S::Vf64::set1(1.0);
                    }
                    z += S::Vf64::set1(1.0);
                }
                w += S::Vf64::set1(1.0);
            }
            u += S::Vf64::set1(1.0);
        }
        v += S::Vf64::set1(1.0);
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

#[inline(always)]
unsafe fn get_6d_noise_helper_f64<S: Simd, Settings: Sample64<S>>(
    settings: Settings,
) -> (Vec<f64>, f64, f64) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth * dim.time * dim.extent5 * dim.extent6;
    let mut result = vec![0.0f64; len];
    let (min, max) = get_6d_noise_into_helper_f64::<S, Settings>(settings, &mut result);
    (result, min, max)
}

// f64 value + gradient grid helpers (issue #24), generic over
// `SampleFractalDeriv64` so one helper per dimension serves every variant. Twins
// of the f32 `get_*_fractal_gradient` helpers; the per-axis frequency scales the
// kernel-space gradient on the way out.

#[inline(always)]
pub unsafe fn get_1d_fractal_gradient<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let freq_x = settings.get_freq_x() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let start_x = dim.x as f64;
    let width = dim.width;

    let mut values = vec![0.0f64; width];
    let mut gradients = vec![0.0f64; width];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
    let mut i = 0;
    while i < width {
        let (f, d) = settings.sample_1d_deriv(x * freq_x_s);
        let count = (width - i).min(vector_width);
        for j in 0..count {
            let v = f[j];
            values[i] = v;
            gradients[i] = d[j] * freq_x;
            if v < min {
                min = v;
            }
            if v > max {
                max = v;
            }
            i += 1;
        }
        x += S::Vf64::set1(vector_width as f64);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 1,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_2d_fractal_gradient<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;

    let len = width * height;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 2];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut y = S::Vf64::set1(start_y);
    for _ in 0..height {
        let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < width {
            let (f, d) = settings.sample_2d_deriv(x * freq_x_s, y * freq_y_s);
            let count = (width - col).min(vector_width);
            for j in 0..count {
                let v = f[j];
                values[i] = v;
                gradients[i * 2] = d[0][j] * freq_x;
                gradients[i * 2 + 1] = d[1][j] * freq_y;
                if v < min {
                    min = v;
                }
                if v > max {
                    max = v;
                }
                i += 1;
            }
            x += S::Vf64::set1(vector_width as f64);
            col += vector_width;
        }
        y += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 2,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_3d_fractal_gradient<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_z = settings.get_freq_z() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let freq_z_s = S::Vf64::set1(freq_z);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let len = width * height * depth;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 3];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < width {
                let (f, d) = settings.sample_3d_deriv(x * freq_x_s, y * freq_y_s, z * freq_z_s);
                let count = (width - col).min(vector_width);
                for j in 0..count {
                    let v = f[j];
                    values[i] = v;
                    gradients[i * 3] = d[0][j] * freq_x;
                    gradients[i * 3 + 1] = d[1][j] * freq_y;
                    gradients[i * 3 + 2] = d[2][j] * freq_z;
                    if v < min {
                        min = v;
                    }
                    if v > max {
                        max = v;
                    }
                    i += 1;
                }
                x += S::Vf64::set1(vector_width as f64);
                col += vector_width;
            }
            y += S::Vf64::set1(1.0);
        }
        z += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 3,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_4d_fractal_gradient<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_z = settings.get_freq_z() as f64;
    let freq_w = settings.get_freq_w() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let freq_z_s = S::Vf64::set1(freq_z);
    let freq_w_s = S::Vf64::set1(freq_w);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;

    let len = width * height * depth * time;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 4];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut w = S::Vf64::set1(start_w);
    for _ in 0..time {
        let mut z = S::Vf64::set1(start_z);
        for _ in 0..depth {
            let mut y = S::Vf64::set1(start_y);
            for _ in 0..height {
                let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
                let mut col = 0;
                while col < width {
                    let (f, d) = settings.sample_4d_deriv(
                        x * freq_x_s,
                        y * freq_y_s,
                        z * freq_z_s,
                        w * freq_w_s,
                    );
                    let count = (width - col).min(vector_width);
                    for j in 0..count {
                        let v = f[j];
                        values[i] = v;
                        gradients[i * 4] = d[0][j] * freq_x;
                        gradients[i * 4 + 1] = d[1][j] * freq_y;
                        gradients[i * 4 + 2] = d[2][j] * freq_z;
                        gradients[i * 4 + 3] = d[3][j] * freq_w;
                        if v < min {
                            min = v;
                        }
                        if v > max {
                            max = v;
                        }
                        i += 1;
                    }
                    x += S::Vf64::set1(vector_width as f64);
                    col += vector_width;
                }
                y += S::Vf64::set1(1.0);
            }
            z += S::Vf64::set1(1.0);
        }
        w += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 4,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_5d_fractal_gradient<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_z = settings.get_freq_z() as f64;
    let freq_w = settings.get_freq_w() as f64;
    let freq_u = settings.get_freq_u() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let freq_z_s = S::Vf64::set1(freq_z);
    let freq_w_s = S::Vf64::set1(freq_w);
    let freq_u_s = S::Vf64::set1(freq_u);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;
    let start_u = dim.offset5 as f64;
    let extent5 = dim.extent5;

    let len = width * height * depth * time * extent5;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 5];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut u = S::Vf64::set1(start_u);
    for _ in 0..extent5 {
        let mut w = S::Vf64::set1(start_w);
        for _ in 0..time {
            let mut z = S::Vf64::set1(start_z);
            for _ in 0..depth {
                let mut y = S::Vf64::set1(start_y);
                for _ in 0..height {
                    let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
                    let mut col = 0;
                    while col < width {
                        let (f, d) = settings.sample_5d_deriv(
                            x * freq_x_s,
                            y * freq_y_s,
                            z * freq_z_s,
                            w * freq_w_s,
                            u * freq_u_s,
                        );
                        let count = (width - col).min(vector_width);
                        for j in 0..count {
                            let val = f[j];
                            values[i] = val;
                            gradients[i * 5] = d[0][j] * freq_x;
                            gradients[i * 5 + 1] = d[1][j] * freq_y;
                            gradients[i * 5 + 2] = d[2][j] * freq_z;
                            gradients[i * 5 + 3] = d[3][j] * freq_w;
                            gradients[i * 5 + 4] = d[4][j] * freq_u;
                            if val < min {
                                min = val;
                            }
                            if val > max {
                                max = val;
                            }
                            i += 1;
                        }
                        x += S::Vf64::set1(vector_width as f64);
                        col += vector_width;
                    }
                    y += S::Vf64::set1(1.0);
                }
                z += S::Vf64::set1(1.0);
            }
            w += S::Vf64::set1(1.0);
        }
        u += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 5,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_6d_fractal_gradient<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_z = settings.get_freq_z() as f64;
    let freq_w = settings.get_freq_w() as f64;
    let freq_u = settings.get_freq_u() as f64;
    let freq_v = settings.get_freq_v() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let freq_z_s = S::Vf64::set1(freq_z);
    let freq_w_s = S::Vf64::set1(freq_w);
    let freq_u_s = S::Vf64::set1(freq_u);
    let freq_v_s = S::Vf64::set1(freq_v);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;
    let start_w = dim.w as f64;
    let time = dim.time;
    let start_u = dim.offset5 as f64;
    let extent5 = dim.extent5;
    let start_v = dim.offset6 as f64;
    let extent6 = dim.extent6;

    let len = width * height * depth * time * extent5 * extent6;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 6];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut v = S::Vf64::set1(start_v);
    for _ in 0..extent6 {
        let mut u = S::Vf64::set1(start_u);
        for _ in 0..extent5 {
            let mut w = S::Vf64::set1(start_w);
            for _ in 0..time {
                let mut z = S::Vf64::set1(start_z);
                for _ in 0..depth {
                    let mut y = S::Vf64::set1(start_y);
                    for _ in 0..height {
                        let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
                        let mut col = 0;
                        while col < width {
                            let (f, d) = settings.sample_6d_deriv(
                                x * freq_x_s,
                                y * freq_y_s,
                                z * freq_z_s,
                                w * freq_w_s,
                                u * freq_u_s,
                                v * freq_v_s,
                            );
                            let count = (width - col).min(vector_width);
                            for j in 0..count {
                                let val = f[j];
                                values[i] = val;
                                gradients[i * 6] = d[0][j] * freq_x;
                                gradients[i * 6 + 1] = d[1][j] * freq_y;
                                gradients[i * 6 + 2] = d[2][j] * freq_z;
                                gradients[i * 6 + 3] = d[3][j] * freq_w;
                                gradients[i * 6 + 4] = d[4][j] * freq_u;
                                gradients[i * 6 + 5] = d[5][j] * freq_v;
                                if val < min {
                                    min = val;
                                }
                                if val > max {
                                    max = val;
                                }
                                i += 1;
                            }
                            x += S::Vf64::set1(vector_width as f64);
                            col += vector_width;
                        }
                        y += S::Vf64::set1(1.0);
                    }
                    z += S::Vf64::set1(1.0);
                }
                w += S::Vf64::set1(1.0);
            }
            u += S::Vf64::set1(1.0);
        }
        v += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 6,
        min,
        max,
    }
}

/// Euclidean F1-distance cellular value + analytic gradient, 2D (f64 twin of
/// [`crate::noise_helpers_32::get_2d_cellular_gradient`]).
#[inline(always)]
pub unsafe fn get_2d_cellular_gradient<S: Simd>(settings: &CellularSettings) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let seed = dim.seed as i64;
    let jitter = S::Vf64::set1(settings.jitter as f64);
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;

    let len = width * height;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 2];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut y = S::Vf64::set1(start_y);
    for _ in 0..height {
        let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < width {
            let (f, d) = cellular_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, jitter, seed);
            let count = (width - col).min(vector_width);
            for j in 0..count {
                let v = f[j];
                values[i] = v;
                gradients[i * 2] = d[0][j] * freq_x;
                gradients[i * 2 + 1] = d[1][j] * freq_y;
                if v < min {
                    min = v;
                }
                if v > max {
                    max = v;
                }
                i += 1;
            }
            x += S::Vf64::set1(vector_width as f64);
            col += vector_width;
        }
        y += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 2,
        min,
        max,
    }
}

/// Euclidean F1-distance cellular value + analytic gradient, 3D (f64 twin of
/// [`crate::noise_helpers_32::get_3d_cellular_gradient`]).
#[inline(always)]
pub unsafe fn get_3d_cellular_gradient<S: Simd>(settings: &CellularSettings) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let seed = dim.seed as i64;
    let jitter = S::Vf64::set1(settings.jitter as f64);
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_z = settings.get_freq_z() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let freq_z_s = S::Vf64::set1(freq_z);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let len = width * height * depth;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 3];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < width {
                let (f, d) =
                    cellular_3d_deriv::<S>(x * freq_x_s, y * freq_y_s, z * freq_z_s, jitter, seed);
                let count = (width - col).min(vector_width);
                for j in 0..count {
                    let v = f[j];
                    values[i] = v;
                    gradients[i * 3] = d[0][j] * freq_x;
                    gradients[i * 3 + 1] = d[1][j] * freq_y;
                    gradients[i * 3 + 2] = d[2][j] * freq_z;
                    if v < min {
                        min = v;
                    }
                    if v > max {
                        max = v;
                    }
                    i += 1;
                }
                x += S::Vf64::set1(vector_width as f64);
                col += vector_width;
            }
            y += S::Vf64::set1(1.0);
        }
        z += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 3,
        min,
        max,
    }
}

/// The scalar `return_type` combination of the two selected cellular2 distances
/// (f64 twin of [`crate::noise_helpers_32`]'s `cellular2_combine_value`).
#[inline(always)]
fn cellular2_combine_value(ret: Cell2ReturnType, a: f64, b: f64) -> f64 {
    match ret {
        Cell2ReturnType::Distance2 => b,
        Cell2ReturnType::Distance2Add => a + b,
        Cell2ReturnType::Distance2Sub => a - b,
        Cell2ReturnType::Distance2Mul => a * b,
        Cell2ReturnType::Distance2Div => a / b,
    }
}

/// The derivative of [`cellular2_combine_value`] along one axis (f64).
#[inline(always)]
fn cellular2_combine_grad(ret: Cell2ReturnType, a: f64, b: f64, ga: f64, gb: f64) -> f64 {
    match ret {
        Cell2ReturnType::Distance2 => gb,
        Cell2ReturnType::Distance2Add => ga + gb,
        Cell2ReturnType::Distance2Sub => ga - gb,
        Cell2ReturnType::Distance2Mul => b * ga + a * gb,
        Cell2ReturnType::Distance2Div => (ga * b - a * gb) / (b * b),
    }
}

/// Euclidean cellular2 value + analytic gradient, 2D (f64 twin of
/// [`crate::noise_helpers_32::get_2d_cellular2_gradient`]).
#[inline(always)]
pub unsafe fn get_2d_cellular2_gradient<S: Simd>(
    settings: &Cellular2Settings,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let seed = dim.seed as i64;
    let jitter = S::Vf64::set1(settings.jitter as f64);
    let ret = settings.return_type;
    let i0 = settings.index0;
    let i1 = settings.index1;
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;

    let len = width * height;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 2];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut y = S::Vf64::set1(start_y);
    for _ in 0..height {
        let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < width {
            let (dist, delta) =
                cellular2_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, jitter, i1, seed);
            let count = (width - col).min(vector_width);
            for j in 0..count {
                let a = dist[i0][j];
                let b = dist[i1][j];
                let v = cellular2_combine_value(ret, a, b);
                values[i] = v;
                let gx = cellular2_combine_grad(
                    ret,
                    a,
                    b,
                    -2.0 * delta[0][i0][j],
                    -2.0 * delta[0][i1][j],
                );
                let gy = cellular2_combine_grad(
                    ret,
                    a,
                    b,
                    -2.0 * delta[1][i0][j],
                    -2.0 * delta[1][i1][j],
                );
                gradients[i * 2] = gx * freq_x;
                gradients[i * 2 + 1] = gy * freq_y;
                if v < min {
                    min = v;
                }
                if v > max {
                    max = v;
                }
                i += 1;
            }
            x += S::Vf64::set1(vector_width as f64);
            col += vector_width;
        }
        y += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 2,
        min,
        max,
    }
}

/// Euclidean cellular2 value + analytic gradient, 3D (f64 twin of
/// [`crate::noise_helpers_32::get_3d_cellular2_gradient`]).
#[inline(always)]
pub unsafe fn get_3d_cellular2_gradient<S: Simd>(
    settings: &Cellular2Settings,
) -> NoiseAndGradient64 {
    let dim = settings.get_dimensions();
    let seed = dim.seed as i64;
    let jitter = S::Vf64::set1(settings.jitter as f64);
    let ret = settings.return_type;
    let i0 = settings.index0;
    let i1 = settings.index1;
    let freq_x = settings.get_freq_x() as f64;
    let freq_y = settings.get_freq_y() as f64;
    let freq_z = settings.get_freq_z() as f64;
    let freq_x_s = S::Vf64::set1(freq_x);
    let freq_y_s = S::Vf64::set1(freq_y);
    let freq_z_s = S::Vf64::set1(freq_z);
    let start_x = dim.x as f64;
    let width = dim.width;
    let start_y = dim.y as f64;
    let height = dim.height;
    let start_z = dim.z as f64;
    let depth = dim.depth;

    let len = width * height * depth;
    let mut values = vec![0.0f64; len];
    let mut gradients = vec![0.0f64; len * 3];
    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let vector_width = S::Vf64::WIDTH;
    let mut x_arr = Vec::<f64>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f64);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut z = S::Vf64::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf64::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf64::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < width {
                let (dist, delta) = cellular2_3d_deriv::<S>(
                    x * freq_x_s,
                    y * freq_y_s,
                    z * freq_z_s,
                    jitter,
                    i1,
                    seed,
                );
                let count = (width - col).min(vector_width);
                for j in 0..count {
                    let a = dist[i0][j];
                    let b = dist[i1][j];
                    let v = cellular2_combine_value(ret, a, b);
                    values[i] = v;
                    let gx = cellular2_combine_grad(
                        ret,
                        a,
                        b,
                        -2.0 * delta[0][i0][j],
                        -2.0 * delta[0][i1][j],
                    );
                    let gy = cellular2_combine_grad(
                        ret,
                        a,
                        b,
                        -2.0 * delta[1][i0][j],
                        -2.0 * delta[1][i1][j],
                    );
                    let gz = cellular2_combine_grad(
                        ret,
                        a,
                        b,
                        -2.0 * delta[2][i0][j],
                        -2.0 * delta[2][i1][j],
                    );
                    gradients[i * 3] = gx * freq_x;
                    gradients[i * 3 + 1] = gy * freq_y;
                    gradients[i * 3 + 2] = gz * freq_z;
                    if v < min {
                        min = v;
                    }
                    if v > max {
                        max = v;
                    }
                    i += 1;
                }
                x += S::Vf64::set1(vector_width as f64);
                col += vector_width;
            }
            y += S::Vf64::set1(1.0);
        }
        z += S::Vf64::set1(1.0);
    }
    NoiseAndGradient64 {
        values,
        gradients,
        stride: 3,
        min,
        max,
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_1d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_1d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_1d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_1d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_1d_noise_helper_f64::<S, Cellular2Settings>(*s),
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
        NoiseType::Fbm(s) => get_2d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_2d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_2d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_2d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_2d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_2d_noise_helper_f64::<S, Cellular2Settings>(*s),
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
        NoiseType::Fbm(s) => get_3d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_3d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_3d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_3d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_3d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_3d_noise_helper_f64::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_4d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_4d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_4d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_4d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_4d_noise_helper_f64::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_5d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_5d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_5d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_5d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_5d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_5d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_5d_noise_helper_f64::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_6d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_6d_noise_helper_f64::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_6d_noise_helper_f64::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_6d_noise_helper_f64::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_6d_noise_helper_f64::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_6d_noise_helper_f64::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_6d_noise_helper_f64::<S, Cellular2Settings>(*s),
    }
}

/// Emplace 1d f64 noise into `result` (length `width`); returns `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f64]) -> (f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_into_helper_f64::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_1d_noise_into_helper_f64::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_1d_noise_into_helper_f64::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_1d_noise_into_helper_f64::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_1d_noise_into_helper_f64::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_1d_noise_into_helper_f64::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 2d f64 noise into `result` (length `width * height`); returns `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_2d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f64]) -> (f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_into_helper_f64::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_2d_noise_into_helper_f64::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_2d_noise_into_helper_f64::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_2d_noise_into_helper_f64::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_2d_noise_into_helper_f64::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_2d_noise_into_helper_f64::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 3d f64 noise into `result` (length `width * height * depth`); returns `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f64]) -> (f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_into_helper_f64::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_3d_noise_into_helper_f64::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_3d_noise_into_helper_f64::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_3d_noise_into_helper_f64::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_3d_noise_into_helper_f64::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_3d_noise_into_helper_f64::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 4d f64 noise into `result` (length `width * height * depth * time`); returns `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f64]) -> (f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_into_helper_f64::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_4d_noise_into_helper_f64::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_4d_noise_into_helper_f64::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_4d_noise_into_helper_f64::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_4d_noise_into_helper_f64::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_4d_noise_into_helper_f64::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 5d f64 noise into `result` (length `width * height * depth * time * extent5`); returns `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_5d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f64]) -> (f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_5d_noise_into_helper_f64::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_5d_noise_into_helper_f64::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_5d_noise_into_helper_f64::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_5d_noise_into_helper_f64::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_5d_noise_into_helper_f64::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_5d_noise_into_helper_f64::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 6d f64 noise into `result` (length `width * height * depth * time * extent5 * extent6`); returns `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_6d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f64]) -> (f64, f64) {
    match noise_type {
        NoiseType::Fbm(s) => get_6d_noise_into_helper_f64::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_6d_noise_into_helper_f64::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_6d_noise_into_helper_f64::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_6d_noise_into_helper_f64::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_6d_noise_into_helper_f64::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_6d_noise_into_helper_f64::<S, Cellular2Settings>(*s, result),
    }
}
