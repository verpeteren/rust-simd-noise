use crate::dimensional_being::DimensionalBeing;
use crate::noise::simplex_32::{simplex_1d_deriv, simplex_2d_deriv, simplex_3d_deriv};
use crate::NoiseInto;
use crate::{
    Cellular2Settings, CellularSettings, FbmSettings, GradientSettings, NoiseAndGradient,
    NoiseType, RidgeSettings, Settings, TurbulenceSettings,
};

use simdeez::prelude::*;

use std::f32;

pub trait Sample32<S: Simd>: DimensionalBeing + Settings {
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32;
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32;
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32;
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32;
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32;
    #[allow(clippy::too_many_arguments)]
    fn sample_6d(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
        t: S::Vf32,
    ) -> S::Vf32;
}

#[inline(always)]
unsafe fn get_1d_noise_into_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let start_x = dim.x;
    let width = dim.width;
    assert_eq!(
        result.len(),
        width,
        "noise output slice length must equal width"
    );
    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
    for _ in 0..width / vector_width {
        let f = settings.sample_1d(x * freq_x);
        max_s = max_s.max(f);
        min_s = min_s.min(f);
        f.copy_to_ptr_unaligned(result_ptr.add(i));
        i += vector_width;
        x += S::Vf32::set1(vector_width as f32);
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
unsafe fn get_1d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let width = settings.get_dimensions().width;
    let mut result = vec![0.0f32; width];
    let (min, max) = get_1d_noise_into_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_2d_noise_into_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    assert_eq!(
        result.len(),
        width * height,
        "noise output slice length must equal width * height"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut y = S::Vf32::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    for _ in 0..height {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        for _ in 0..width / vector_width {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += vector_width;
            x += S::Vf32::set1(vector_width as f32);
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
        y += S::Vf32::set1(1.0);
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
unsafe fn get_2d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height;
    let mut result = vec![0.0f32; len];
    let (min, max) = get_2d_noise_into_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_3d_noise_into_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    assert_eq!(
        result.len(),
        width * height * depth,
        "noise output slice length must equal width * height * depth"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);

    let mut z = S::Vf32::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += vector_width;
                x += S::Vf32::set1(vector_width as f32);
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
            y += S::Vf32::set1(1.0);
        }
        z += S::Vf32::set1(1.0);
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
unsafe fn get_3d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth;
    let mut result = vec![0.0f32; len];
    let (min, max) = get_3d_noise_into_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_4d_noise_into_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let freq_w = S::Vf32::set1(settings.get_freq_w());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;
    assert_eq!(
        result.len(),
        width * height * depth * time,
        "noise output slice length must equal width * height * depth * time"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut w = S::Vf32::set1(start_w);
    for _ in 0..time {
        let mut z = S::Vf32::set1(start_z);
        for _ in 0..depth {
            let mut y = S::Vf32::set1(start_y);
            for _ in 0..height {
                let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = settings.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    f.copy_to_ptr_unaligned(result_ptr.add(i));
                    i += vector_width;
                    x += S::Vf32::set1(vector_width as f32);
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
                y += S::Vf32::set1(1.0);
            }
            z += S::Vf32::set1(1.0);
        }
        w += S::Vf32::set1(1.0);
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
unsafe fn get_4d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth * dim.time;
    let mut result = vec![0.0f32; len];
    let (min, max) = get_4d_noise_into_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_5d_noise_into_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let freq_w = S::Vf32::set1(settings.get_freq_w());
    let freq_u = S::Vf32::set1(settings.get_freq_u());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;
    let start_u = dim.offset5;
    let extent5 = dim.extent5;
    assert_eq!(
        result.len(),
        width * height * depth * time * extent5,
        "noise output slice length must equal width * height * depth * time * extent5"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut u = S::Vf32::set1(start_u);
    for _ in 0..extent5 {
        let mut w = S::Vf32::set1(start_w);
        for _ in 0..time {
            let mut z = S::Vf32::set1(start_z);
            for _ in 0..depth {
                let mut y = S::Vf32::set1(start_y);
                for _ in 0..height {
                    let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
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
                        x += S::Vf32::set1(vector_width as f32);
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
                    y += S::Vf32::set1(1.0);
                }
                z += S::Vf32::set1(1.0);
            }
            w += S::Vf32::set1(1.0);
        }
        u += S::Vf32::set1(1.0);
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
unsafe fn get_5d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth * dim.time * dim.extent5;
    let mut result = vec![0.0f32; len];
    let (min, max) = get_5d_noise_into_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn get_6d_noise_into_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let freq_w = S::Vf32::set1(settings.get_freq_w());
    let freq_u = S::Vf32::set1(settings.get_freq_u());
    let freq_v = S::Vf32::set1(settings.get_freq_v());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;
    let start_u = dim.offset5;
    let extent5 = dim.extent5;
    let start_v = dim.offset6;
    let extent6 = dim.extent6;
    assert_eq!(
        result.len(),
        width * height * depth * time * extent5 * extent6,
        "noise output slice length must equal width * height * depth * time * extent5 * extent6"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut v = S::Vf32::set1(start_v);
    for _ in 0..extent6 {
        let mut u = S::Vf32::set1(start_u);
        for _ in 0..extent5 {
            let mut w = S::Vf32::set1(start_w);
            for _ in 0..time {
                let mut z = S::Vf32::set1(start_z);
                for _ in 0..depth {
                    let mut y = S::Vf32::set1(start_y);
                    for _ in 0..height {
                        let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
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
                            x += S::Vf32::set1(vector_width as f32);
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
                        y += S::Vf32::set1(1.0);
                    }
                    z += S::Vf32::set1(1.0);
                }
                w += S::Vf32::set1(1.0);
            }
            u += S::Vf32::set1(1.0);
        }
        v += S::Vf32::set1(1.0);
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
unsafe fn get_6d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.get_dimensions();
    let len = dim.width * dim.height * dim.depth * dim.time * dim.extent5 * dim.extent6;
    let mut result = vec![0.0f32; len];
    let (min, max) = get_6d_noise_into_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

// ---------------------------------------------------------------------------
// Value + analytic gradient (issue #24).
//
// These mirror the value-only `get_*_noise_*_helper_f32` grid loops above, but
// call the `simplex_*_deriv` kernels so every sample yields both the noise
// value and its gradient. Only plain gradient (simplex) noise is supported so
// far, so they take `&GradientSettings` directly instead of going through the
// `Sample32` trait (derivatives of the fractal variants are future work).
//
// The gradient is reported in *sample space*: component `k` is the derivative
// of the value with respect to the sample position along axis `k`. The kernel
// is evaluated at `position * freq`, so by the chain rule that equals
// `freq_k * dnoise/dcoord_k` — hence each kernel-space derivative is multiplied
// by its axis frequency before being stored.
// ---------------------------------------------------------------------------

#[inline(always)]
pub unsafe fn get_1d_gradient<S: Simd>(settings: &GradientSettings) -> NoiseAndGradient {
    let dim = settings.get_dimensions();
    let seed = dim.seed;
    let freq_x = settings.get_freq_x();
    let freq_x_s = S::Vf32::set1(freq_x);
    let start_x = dim.x;
    let width = dim.width;

    let mut values = vec![0.0f32; width];
    let mut gradients = vec![0.0f32; width];
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let vector_width = S::Vf32::WIDTH;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);

    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
    let mut i = 0;
    while i < width {
        let (f, d) = simplex_1d_deriv::<S>(x * freq_x_s, seed);
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
        x += S::Vf32::set1(vector_width as f32);
    }
    NoiseAndGradient {
        values,
        gradients,
        stride: 1,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_2d_gradient<S: Simd>(settings: &GradientSettings) -> NoiseAndGradient {
    let dim = settings.get_dimensions();
    let seed = dim.seed;
    let freq_x = settings.get_freq_x();
    let freq_y = settings.get_freq_y();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;

    let len = width * height;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 2];
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let vector_width = S::Vf32::WIDTH;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut y = S::Vf32::set1(start_y);
    for _ in 0..height {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < width {
            let (f, d) = simplex_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, seed);
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
            x += S::Vf32::set1(vector_width as f32);
            col += vector_width;
        }
        y += S::Vf32::set1(1.0);
    }
    NoiseAndGradient {
        values,
        gradients,
        stride: 2,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn get_3d_gradient<S: Simd>(settings: &GradientSettings) -> NoiseAndGradient {
    let dim = settings.get_dimensions();
    let seed = dim.seed;
    let freq_x = settings.get_freq_x();
    let freq_y = settings.get_freq_y();
    let freq_z = settings.get_freq_z();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;

    let len = width * height * depth;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 3];
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let vector_width = S::Vf32::WIDTH;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);

    let mut i = 0;
    let mut z = S::Vf32::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < width {
                let (f, d) = simplex_3d_deriv::<S>(x * freq_x_s, y * freq_y_s, z * freq_z_s, seed);
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
                x += S::Vf32::set1(vector_width as f32);
                col += vector_width;
            }
            y += S::Vf32::set1(1.0);
        }
        z += S::Vf32::set1(1.0);
    }
    NoiseAndGradient {
        values,
        gradients,
        stride: 3,
        min,
        max,
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_1d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_1d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_1d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_1d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_1d_noise_helper_f32::<S, Cellular2Settings>(*s),
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
        NoiseType::Fbm(s) => get_2d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_2d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_2d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_2d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_2d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_2d_noise_helper_f32::<S, Cellular2Settings>(*s),
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
        NoiseType::Fbm(s) => get_3d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_3d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_3d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_3d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_3d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_3d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_4d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_4d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_4d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_4d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_4d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_5d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_5d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_5d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_5d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_5d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_5d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_5d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_6d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_6d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => get_6d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => get_6d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => get_6d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => get_6d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => get_6d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

/// Emplace 1d noise into `result` (which must have length `width`); returns the
/// slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_1d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_1d_noise_into_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_1d_noise_into_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_1d_noise_into_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_1d_noise_into_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_1d_noise_into_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_1d_noise_into_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 2d noise into `result` (which must have length `width * height`);
/// returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_2d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_2d_noise_into_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_2d_noise_into_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_2d_noise_into_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_2d_noise_into_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_2d_noise_into_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_2d_noise_into_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 3d noise into `result` (which must have length
/// `width * height * depth`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_3d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_3d_noise_into_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_3d_noise_into_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_3d_noise_into_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_3d_noise_into_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_3d_noise_into_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_3d_noise_into_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 4d noise into `result` (which must have length
/// `width * height * depth * time`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_4d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_4d_noise_into_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_4d_noise_into_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_4d_noise_into_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_4d_noise_into_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_4d_noise_into_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_4d_noise_into_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 5d noise into `result` (which must have length
/// `width * height * depth * time * extent5`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_5d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_5d_noise_into_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_5d_noise_into_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_5d_noise_into_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_5d_noise_into_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_5d_noise_into_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_5d_noise_into_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 6d noise into `result` (which must have length
/// `width * height * depth * time * extent5 * extent6`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn get_6d_noise_into<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => get_6d_noise_into_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => get_6d_noise_into_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            get_6d_noise_into_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => get_6d_noise_into_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => get_6d_noise_into_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => get_6d_noise_into_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

// --- Fused typed (integer) emplace: SIMD noise compute + scalar map+cast store
//     directly into `&mut [T]`, with no output-sized f32 scratch. `mult`/`off`
//     map a noise value from [src_min, src_max] onto [T::RANGE_MIN, T::RANGE_MAX];
//     the per-lane `v * mult + off` is scalar f32 (no FMA), so results are
//     backend-independent and bit-match a manual map+cast. ---

#[inline(always)]
unsafe fn get_1d_noise_range_into_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let start_x = dim.x;
    let width = dim.width;
    assert_eq!(
        out.len(),
        width,
        "noise output slice length must equal width"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
    for _ in 0..width / vector_width {
        let f = settings.sample_1d(x * freq_x);
        for lane in 0..vector_width {
            out_ptr
                .add(i + lane)
                .write(T::from_noise(f[lane] * mult + off));
        }
        i += vector_width;
        x += S::Vf32::set1(vector_width as f32);
    }
    if remainder != 0 {
        let f = settings.sample_1d(x * freq_x);
        for j in 0..remainder {
            out_ptr.add(i).write(T::from_noise(f[j] * mult + off));
            i += 1;
        }
    }
}

#[inline(always)]
unsafe fn get_2d_noise_range_into_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    assert_eq!(
        out.len(),
        width * height,
        "noise output slice length must equal width * height"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut y = S::Vf32::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    for _ in 0..height {
        let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
        for _ in 0..width / vector_width {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            for lane in 0..vector_width {
                out_ptr
                    .add(i + lane)
                    .write(T::from_noise(f[lane] * mult + off));
            }
            i += vector_width;
            x += S::Vf32::set1(vector_width as f32);
        }
        if remainder != 0 {
            let f = settings.sample_2d(x * freq_x, y * freq_y);
            for j in 0..remainder {
                out_ptr.add(i).write(T::from_noise(f[j] * mult + off));
                i += 1;
            }
        }
        y += S::Vf32::set1(1.0);
    }
}

#[inline(always)]
unsafe fn get_3d_noise_range_into_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    assert_eq!(
        out.len(),
        width * height * depth,
        "noise output slice length must equal width * height * depth"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut z = S::Vf32::set1(start_z);
    for _ in 0..depth {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..height {
            let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..width / vector_width {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                for lane in 0..vector_width {
                    out_ptr
                        .add(i + lane)
                        .write(T::from_noise(f[lane] * mult + off));
                }
                i += vector_width;
                x += S::Vf32::set1(vector_width as f32);
            }
            if remainder != 0 {
                let f = settings.sample_3d(x * freq_x, y * freq_y, z * freq_z);
                for j in 0..remainder {
                    out_ptr.add(i).write(T::from_noise(f[j] * mult + off));
                    i += 1;
                }
            }
            y += S::Vf32::set1(1.0);
        }
        z += S::Vf32::set1(1.0);
    }
}

#[inline(always)]
unsafe fn get_4d_noise_range_into_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let freq_w = S::Vf32::set1(settings.get_freq_w());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;
    assert_eq!(
        out.len(),
        width * height * depth * time,
        "noise output slice length must equal width * height * depth * time"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut w = S::Vf32::set1(start_w);
    for _ in 0..time {
        let mut z = S::Vf32::set1(start_z);
        for _ in 0..depth {
            let mut y = S::Vf32::set1(start_y);
            for _ in 0..height {
                let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..width / vector_width {
                    let f = settings.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    for lane in 0..vector_width {
                        out_ptr
                            .add(i + lane)
                            .write(T::from_noise(f[lane] * mult + off));
                    }
                    i += vector_width;
                    x += S::Vf32::set1(vector_width as f32);
                }
                if remainder != 0 {
                    let f = settings.sample_4d(x * freq_x, y * freq_y, z * freq_z, w * freq_w);
                    for j in 0..remainder {
                        out_ptr.add(i).write(T::from_noise(f[j] * mult + off));
                        i += 1;
                    }
                }
                y += S::Vf32::set1(1.0);
            }
            z += S::Vf32::set1(1.0);
        }
        w += S::Vf32::set1(1.0);
    }
}

#[inline(always)]
unsafe fn get_5d_noise_range_into_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let freq_w = S::Vf32::set1(settings.get_freq_w());
    let freq_u = S::Vf32::set1(settings.get_freq_u());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;
    let start_u = dim.offset5;
    let extent5 = dim.extent5;
    assert_eq!(
        out.len(),
        width * height * depth * time * extent5,
        "noise output slice length must equal width * height * depth * time * extent5"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut u = S::Vf32::set1(start_u);
    for _ in 0..extent5 {
        let mut w = S::Vf32::set1(start_w);
        for _ in 0..time {
            let mut z = S::Vf32::set1(start_z);
            for _ in 0..depth {
                let mut y = S::Vf32::set1(start_y);
                for _ in 0..height {
                    let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                    for _ in 0..width / vector_width {
                        let f = settings.sample_5d(
                            x * freq_x,
                            y * freq_y,
                            z * freq_z,
                            w * freq_w,
                            u * freq_u,
                        );
                        for lane in 0..vector_width {
                            out_ptr
                                .add(i + lane)
                                .write(T::from_noise(f[lane] * mult + off));
                        }
                        i += vector_width;
                        x += S::Vf32::set1(vector_width as f32);
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
                            out_ptr.add(i).write(T::from_noise(f[j] * mult + off));
                            i += 1;
                        }
                    }
                    y += S::Vf32::set1(1.0);
                }
                z += S::Vf32::set1(1.0);
            }
            w += S::Vf32::set1(1.0);
        }
        u += S::Vf32::set1(1.0);
    }
}

#[inline(always)]
unsafe fn get_6d_noise_range_into_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.get_dimensions();
    let freq_x = S::Vf32::set1(settings.get_freq_x());
    let freq_y = S::Vf32::set1(settings.get_freq_y());
    let freq_z = S::Vf32::set1(settings.get_freq_z());
    let freq_w = S::Vf32::set1(settings.get_freq_w());
    let freq_u = S::Vf32::set1(settings.get_freq_u());
    let freq_v = S::Vf32::set1(settings.get_freq_v());
    let start_x = dim.x;
    let width = dim.width;
    let start_y = dim.y;
    let height = dim.height;
    let start_z = dim.z;
    let depth = dim.depth;
    let start_w = dim.w;
    let time = dim.time;
    let start_u = dim.offset5;
    let extent5 = dim.extent5;
    let start_v = dim.offset6;
    let extent6 = dim.extent6;
    assert_eq!(
        out.len(),
        width * height * depth * time * extent5 * extent6,
        "noise output slice length must equal width * height * depth * time * extent5 * extent6"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut v = S::Vf32::set1(start_v);
    for _ in 0..extent6 {
        let mut u = S::Vf32::set1(start_u);
        for _ in 0..extent5 {
            let mut w = S::Vf32::set1(start_w);
            for _ in 0..time {
                let mut z = S::Vf32::set1(start_z);
                for _ in 0..depth {
                    let mut y = S::Vf32::set1(start_y);
                    for _ in 0..height {
                        let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                        for _ in 0..width / vector_width {
                            let f = settings.sample_6d(
                                x * freq_x,
                                y * freq_y,
                                z * freq_z,
                                w * freq_w,
                                u * freq_u,
                                v * freq_v,
                            );
                            for lane in 0..vector_width {
                                out_ptr
                                    .add(i + lane)
                                    .write(T::from_noise(f[lane] * mult + off));
                            }
                            i += vector_width;
                            x += S::Vf32::set1(vector_width as f32);
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
                                out_ptr.add(i).write(T::from_noise(f[j] * mult + off));
                                i += 1;
                            }
                        }
                        y += S::Vf32::set1(1.0);
                    }
                    z += S::Vf32::set1(1.0);
                }
                w += S::Vf32::set1(1.0);
            }
            u += S::Vf32::set1(1.0);
        }
        v += S::Vf32::set1(1.0);
    }
}

/// Fused typed 1d emplace dispatcher (maps/casts straight into `out`).
#[inline(always)]
pub unsafe fn get_1d_noise_range_into<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            get_1d_noise_range_into_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            get_1d_noise_range_into_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => get_1d_noise_range_into_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            get_1d_noise_range_into_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            get_1d_noise_range_into_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            get_1d_noise_range_into_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 2d emplace dispatcher.
#[inline(always)]
pub unsafe fn get_2d_noise_range_into<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            get_2d_noise_range_into_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            get_2d_noise_range_into_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => get_2d_noise_range_into_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            get_2d_noise_range_into_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            get_2d_noise_range_into_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            get_2d_noise_range_into_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 3d emplace dispatcher.
#[inline(always)]
pub unsafe fn get_3d_noise_range_into<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            get_3d_noise_range_into_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            get_3d_noise_range_into_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => get_3d_noise_range_into_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            get_3d_noise_range_into_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            get_3d_noise_range_into_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            get_3d_noise_range_into_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 4d emplace dispatcher.
#[inline(always)]
pub unsafe fn get_4d_noise_range_into<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            get_4d_noise_range_into_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            get_4d_noise_range_into_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => get_4d_noise_range_into_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            get_4d_noise_range_into_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            get_4d_noise_range_into_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            get_4d_noise_range_into_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 5d emplace dispatcher.
#[inline(always)]
pub unsafe fn get_5d_noise_range_into<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            get_5d_noise_range_into_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            get_5d_noise_range_into_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => get_5d_noise_range_into_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            get_5d_noise_range_into_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            get_5d_noise_range_into_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            get_5d_noise_range_into_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 6d emplace dispatcher.
#[inline(always)]
pub unsafe fn get_6d_noise_range_into<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            get_6d_noise_range_into_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            get_6d_noise_range_into_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => get_6d_noise_range_into_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            get_6d_noise_range_into_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            get_6d_noise_range_into_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            get_6d_noise_range_into_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}
