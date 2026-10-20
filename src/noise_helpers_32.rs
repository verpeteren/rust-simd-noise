use crate::dimensional_being::DimensionalBeing;
use crate::noise::cell2_32::{cellular2_2d_deriv, cellular2_3d_deriv};
use crate::noise::cell_32::{cellular_2d_deriv, cellular_3d_deriv};
use crate::noise::fbm_32::{fbm_1d_deriv, fbm_2d_deriv, fbm_3d_deriv};
use crate::noise::simplex_32::{simplex_1d_deriv, simplex_2d_deriv, simplex_3d_deriv};
use crate::NoiseInto;
use crate::{
    Cell2ReturnType, Cellular2Settings, CellularSettings, FbmSettings, GradientSettings,
    NoiseAndGradient, NoiseType, RidgeSettings, Settings, TurbulenceSettings,
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

/// Value + analytic gradient sampling for the fractal variants (issue #24),
/// the `Sample32` analogue used by the `get_*_fractal_gradient` helpers. Each
/// method returns the sampled value and its gradient **with respect to the
/// coordinate(s) passed in** (the per-axis frequency factor is applied by the
/// grid helper, not here).
///
/// Implemented for every variant with a closed-form derivative — gradient, FBM,
/// ridge and turbulence. The 1D–3D `get_*_gradient`/`get_*_fbm_gradient` helpers
/// call their kernels directly; the 4D–6D `get_*_fractal_gradient` helpers are
/// generic over this trait, so all four variants share one grid loop per
/// dimension there.
pub trait SampleFractalDeriv32<S: Simd>: DimensionalBeing + Settings {
    fn sample_1d_deriv(&self, x: S::Vf32) -> (S::Vf32, S::Vf32);
    fn sample_2d_deriv(&self, x: S::Vf32, y: S::Vf32) -> (S::Vf32, [S::Vf32; 2]);
    fn sample_3d_deriv(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> (S::Vf32, [S::Vf32; 3]);
    fn sample_4d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 4]);
    fn sample_5d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 5]);
    #[allow(clippy::too_many_arguments)]
    fn sample_6d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
        t: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 6]);
}

#[inline(always)]
unsafe fn fill_1d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    assert_eq!(
        result.len(),
        extent_x,
        "noise output slice length must equal width"
    );
    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
    for _ in 0..extent_x / vector_width {
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
unsafe fn generate_1d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let extent_x = settings.dimensions().extent_x;
    let mut result = vec![0.0f32; extent_x];
    let (min, max) = fill_1d_noise_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn fill_2d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    assert_eq!(
        result.len(),
        extent_x * extent_y,
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
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        for _ in 0..extent_x / vector_width {
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
unsafe fn generate_2d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.dimensions();
    let len = dim.extent_x * dim.extent_y;
    let mut result = vec![0.0f32; len];
    let (min, max) = fill_2d_noise_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn fill_3d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    assert_eq!(
        result.len(),
        extent_x * extent_y * extent_z,
        "noise output slice length must equal width * height * depth"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);

    let mut z = S::Vf32::set1(start_z);
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..extent_x / vector_width {
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
unsafe fn generate_3d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.dimensions();
    let len = dim.extent_x * dim.extent_y * dim.extent_z;
    let mut result = vec![0.0f32; len];
    let (min, max) = fill_3d_noise_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn fill_4d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let freq_w = S::Vf32::set1(settings.freq_w());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    assert_eq!(
        result.len(),
        extent_x * extent_y * extent_z * extent_w,
        "noise output slice length must equal width * height * depth * time"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut w = S::Vf32::set1(start_w);
    for _ in 0..extent_w {
        let mut z = S::Vf32::set1(start_z);
        for _ in 0..extent_z {
            let mut y = S::Vf32::set1(start_y);
            for _ in 0..extent_y {
                let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..extent_x / vector_width {
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
unsafe fn generate_4d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.dimensions();
    let len = dim.extent_x * dim.extent_y * dim.extent_z * dim.extent_w;
    let mut result = vec![0.0f32; len];
    let (min, max) = fill_4d_noise_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn fill_5d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let freq_w = S::Vf32::set1(settings.freq_w());
    let freq_u = S::Vf32::set1(settings.freq_u());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    let start_u = dim.offset_u;
    let extent_u = dim.extent_u;
    assert_eq!(
        result.len(),
        extent_x * extent_y * extent_z * extent_w * extent_u,
        "noise output slice length must equal width * height * depth * time * extent5"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut u = S::Vf32::set1(start_u);
    for _ in 0..extent_u {
        let mut w = S::Vf32::set1(start_w);
        for _ in 0..extent_w {
            let mut z = S::Vf32::set1(start_z);
            for _ in 0..extent_z {
                let mut y = S::Vf32::set1(start_y);
                for _ in 0..extent_y {
                    let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                    for _ in 0..extent_x / vector_width {
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
unsafe fn generate_5d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.dimensions();
    let len = dim.extent_x * dim.extent_y * dim.extent_z * dim.extent_w * dim.extent_u;
    let mut result = vec![0.0f32; len];
    let (min, max) = fill_5d_noise_helper_f32::<S, Settings>(settings, &mut result);
    (result, min, max)
}

#[inline(always)]
unsafe fn fill_6d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
    result: &mut [f32],
) -> (f32, f32) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let freq_w = S::Vf32::set1(settings.freq_w());
    let freq_u = S::Vf32::set1(settings.freq_u());
    let freq_v = S::Vf32::set1(settings.freq_v());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    let start_u = dim.offset_u;
    let extent_u = dim.extent_u;
    let start_v = dim.offset_v;
    let extent_v = dim.extent_v;
    assert_eq!(
        result.len(),
        extent_x * extent_y * extent_z * extent_w * extent_u * extent_v,
        "noise output slice length must equal width * height * depth * time * extent5 * extent6"
    );

    let mut min_s = S::Vf32::set1(f32::MAX);
    let mut max_s = S::Vf32::set1(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut v = S::Vf32::set1(start_v);
    for _ in 0..extent_v {
        let mut u = S::Vf32::set1(start_u);
        for _ in 0..extent_u {
            let mut w = S::Vf32::set1(start_w);
            for _ in 0..extent_w {
                let mut z = S::Vf32::set1(start_z);
                for _ in 0..extent_z {
                    let mut y = S::Vf32::set1(start_y);
                    for _ in 0..extent_y {
                        let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                        for _ in 0..extent_x / vector_width {
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
unsafe fn generate_6d_noise_helper_f32<S: Simd, Settings: Sample32<S>>(
    settings: Settings,
) -> (Vec<f32>, f32, f32) {
    let dim = settings.dimensions();
    let len = dim.extent_x * dim.extent_y * dim.extent_z * dim.extent_w * dim.extent_u * dim.extent_v;
    let mut result = vec![0.0f32; len];
    let (min, max) = fill_6d_noise_helper_f32::<S, Settings>(settings, &mut result);
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
pub unsafe fn generate_1d_gradient<S: Simd>(settings: &GradientSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let freq_x = settings.freq_x();
    let freq_x_s = S::Vf32::set1(freq_x);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;

    let mut values = vec![0.0f32; extent_x];
    let mut gradients = vec![0.0f32; extent_x];
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
    while i < extent_x {
        let (f, d) = simplex_1d_deriv::<S>(x * freq_x_s, seed);
        let count = (extent_x - i).min(vector_width);
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
pub unsafe fn generate_2d_gradient<S: Simd>(settings: &GradientSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;

    let len = extent_x * extent_y;
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
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < extent_x {
            let (f, d) = simplex_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, seed);
            let count = (extent_x - col).min(vector_width);
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
pub unsafe fn generate_3d_gradient<S: Simd>(settings: &GradientSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;

    let len = extent_x * extent_y * extent_z;
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
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < extent_x {
                let (f, d) = simplex_3d_deriv::<S>(x * freq_x_s, y * freq_y_s, z * freq_z_s, seed);
                let count = (extent_x - col).min(vector_width);
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

// FBM (fractal) value + gradient. Same grid loops as the plain-gradient helpers
// above, but each sample calls the `fbm_*_deriv` kernels (which fold the octave
// stack and its chain-rule factors), and the per-axis frequency is applied to
// the gradient on the way out.

#[inline(always)]
pub unsafe fn generate_1d_fbm_gradient<S: Simd>(settings: &FbmSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let lac = S::Vf32::set1(settings.lacunarity);
    let gain = S::Vf32::set1(settings.gain);
    let octaves = settings.octaves;
    let freq_x = settings.freq_x();
    let freq_x_s = S::Vf32::set1(freq_x);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;

    let mut values = vec![0.0f32; extent_x];
    let mut gradients = vec![0.0f32; extent_x];
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
    while i < extent_x {
        let (f, d) = fbm_1d_deriv::<S>(x * freq_x_s, lac, gain, octaves, seed);
        let count = (extent_x - i).min(vector_width);
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
pub unsafe fn generate_2d_fbm_gradient<S: Simd>(settings: &FbmSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let lac = S::Vf32::set1(settings.lacunarity);
    let gain = S::Vf32::set1(settings.gain);
    let octaves = settings.octaves;
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;

    let len = extent_x * extent_y;
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
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < extent_x {
            let (f, d) = fbm_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, lac, gain, octaves, seed);
            let count = (extent_x - col).min(vector_width);
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
pub unsafe fn generate_3d_fbm_gradient<S: Simd>(settings: &FbmSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let lac = S::Vf32::set1(settings.lacunarity);
    let gain = S::Vf32::set1(settings.gain);
    let octaves = settings.octaves;
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;

    let len = extent_x * extent_y * extent_z;
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
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < extent_x {
                let (f, d) = fbm_3d_deriv::<S>(
                    x * freq_x_s,
                    y * freq_y_s,
                    z * freq_z_s,
                    lac,
                    gain,
                    octaves,
                    seed,
                );
                let count = (extent_x - col).min(vector_width);
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

// Ridge/turbulence value + gradient. Same grid loops as the FBM helpers above,
// but generic over `SampleFractalDeriv32` so one set of helpers serves both
// variants; each sample's kernel-space gradient is scaled by the per-axis
// frequency on the way out.

#[inline(always)]
pub unsafe fn generate_1d_fractal_gradient<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let freq_x = settings.freq_x();
    let freq_x_s = S::Vf32::set1(freq_x);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;

    let mut values = vec![0.0f32; extent_x];
    let mut gradients = vec![0.0f32; extent_x];
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
    while i < extent_x {
        let (f, d) = settings.sample_1d_deriv(x * freq_x_s);
        let count = (extent_x - i).min(vector_width);
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
pub unsafe fn generate_2d_fractal_gradient<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;

    let len = extent_x * extent_y;
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
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < extent_x {
            let (f, d) = settings.sample_2d_deriv(x * freq_x_s, y * freq_y_s);
            let count = (extent_x - col).min(vector_width);
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
pub unsafe fn generate_3d_fractal_gradient<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;

    let len = extent_x * extent_y * extent_z;
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
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < extent_x {
                let (f, d) = settings.sample_3d_deriv(x * freq_x_s, y * freq_y_s, z * freq_z_s);
                let count = (extent_x - col).min(vector_width);
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
pub unsafe fn generate_4d_fractal_gradient<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_w = settings.freq_w();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let freq_w_s = S::Vf32::set1(freq_w);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;

    let len = extent_x * extent_y * extent_z * extent_w;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 4];
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
    let mut w = S::Vf32::set1(start_w);
    for _ in 0..extent_w {
        let mut z = S::Vf32::set1(start_z);
        for _ in 0..extent_z {
            let mut y = S::Vf32::set1(start_y);
            for _ in 0..extent_y {
                let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
                let mut col = 0;
                while col < extent_x {
                    let (f, d) = settings.sample_4d_deriv(
                        x * freq_x_s,
                        y * freq_y_s,
                        z * freq_z_s,
                        w * freq_w_s,
                    );
                    let count = (extent_x - col).min(vector_width);
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
                    x += S::Vf32::set1(vector_width as f32);
                    col += vector_width;
                }
                y += S::Vf32::set1(1.0);
            }
            z += S::Vf32::set1(1.0);
        }
        w += S::Vf32::set1(1.0);
    }
    NoiseAndGradient {
        values,
        gradients,
        stride: 4,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn generate_5d_fractal_gradient<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_w = settings.freq_w();
    let freq_u = settings.freq_u();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let freq_w_s = S::Vf32::set1(freq_w);
    let freq_u_s = S::Vf32::set1(freq_u);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    let start_u = dim.offset_u;
    let extent_u = dim.extent_u;

    let len = extent_x * extent_y * extent_z * extent_w * extent_u;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 5];
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
    let mut u = S::Vf32::set1(start_u);
    for _ in 0..extent_u {
        let mut w = S::Vf32::set1(start_w);
        for _ in 0..extent_w {
            let mut z = S::Vf32::set1(start_z);
            for _ in 0..extent_z {
                let mut y = S::Vf32::set1(start_y);
                for _ in 0..extent_y {
                    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
                    let mut col = 0;
                    while col < extent_x {
                        let (f, d) = settings.sample_5d_deriv(
                            x * freq_x_s,
                            y * freq_y_s,
                            z * freq_z_s,
                            w * freq_w_s,
                            u * freq_u_s,
                        );
                        let count = (extent_x - col).min(vector_width);
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
                        x += S::Vf32::set1(vector_width as f32);
                        col += vector_width;
                    }
                    y += S::Vf32::set1(1.0);
                }
                z += S::Vf32::set1(1.0);
            }
            w += S::Vf32::set1(1.0);
        }
        u += S::Vf32::set1(1.0);
    }
    NoiseAndGradient {
        values,
        gradients,
        stride: 5,
        min,
        max,
    }
}

#[inline(always)]
pub unsafe fn generate_6d_fractal_gradient<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_w = settings.freq_w();
    let freq_u = settings.freq_u();
    let freq_v = settings.freq_v();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let freq_w_s = S::Vf32::set1(freq_w);
    let freq_u_s = S::Vf32::set1(freq_u);
    let freq_v_s = S::Vf32::set1(freq_v);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    let start_u = dim.offset_u;
    let extent_u = dim.extent_u;
    let start_v = dim.offset_v;
    let extent_v = dim.extent_v;

    let len = extent_x * extent_y * extent_z * extent_w * extent_u * extent_v;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 6];
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
    let mut v = S::Vf32::set1(start_v);
    for _ in 0..extent_v {
        let mut u = S::Vf32::set1(start_u);
        for _ in 0..extent_u {
            let mut w = S::Vf32::set1(start_w);
            for _ in 0..extent_w {
                let mut z = S::Vf32::set1(start_z);
                for _ in 0..extent_z {
                    let mut y = S::Vf32::set1(start_y);
                    for _ in 0..extent_y {
                        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
                        let mut col = 0;
                        while col < extent_x {
                            let (f, d) = settings.sample_6d_deriv(
                                x * freq_x_s,
                                y * freq_y_s,
                                z * freq_z_s,
                                w * freq_w_s,
                                u * freq_u_s,
                                v * freq_v_s,
                            );
                            let count = (extent_x - col).min(vector_width);
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
                            x += S::Vf32::set1(vector_width as f32);
                            col += vector_width;
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
    NoiseAndGradient {
        values,
        gradients,
        stride: 6,
        min,
        max,
    }
}

// Euclidean F1-distance cellular value + gradient. Like the fractal-gradient
// grid loops but calling the hard-coded-Euclidean `cellular_*_deriv` kernels
// with the settings' jitter/seed; the per-axis frequency scales the gradient.

#[inline(always)]
pub unsafe fn generate_2d_cellular_gradient<S: Simd>(settings: &CellularSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let jitter = S::Vf32::set1(settings.jitter);
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;

    let len = extent_x * extent_y;
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
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < extent_x {
            let (f, d) = cellular_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, jitter, seed);
            let count = (extent_x - col).min(vector_width);
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
pub unsafe fn generate_3d_cellular_gradient<S: Simd>(settings: &CellularSettings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let jitter = S::Vf32::set1(settings.jitter);
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;

    let len = extent_x * extent_y * extent_z;
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
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < extent_x {
                let (f, d) =
                    cellular_3d_deriv::<S>(x * freq_x_s, y * freq_y_s, z * freq_z_s, jitter, seed);
                let count = (extent_x - col).min(vector_width);
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

/// The scalar `return_type` combination of the two selected cellular2 distances
/// (`a = distance[index0]`, `b = distance[index1]`), matching `Cell2Return`.
#[inline(always)]
fn cellular2_combine_value(ret: Cell2ReturnType, a: f32, b: f32) -> f32 {
    match ret {
        Cell2ReturnType::Distance2 => b,
        Cell2ReturnType::Distance2Add => a + b,
        Cell2ReturnType::Distance2Sub => a - b,
        Cell2ReturnType::Distance2Mul => a * b,
        Cell2ReturnType::Distance2Div => a / b,
    }
}

/// The derivative of [`cellular2_combine_value`] along one axis, given the two
/// distances `a`/`b` and the per-axis derivatives `ga = ∂a`, `gb = ∂b`.
#[inline(always)]
fn cellular2_combine_grad(ret: Cell2ReturnType, a: f32, b: f32, ga: f32, gb: f32) -> f32 {
    match ret {
        Cell2ReturnType::Distance2 => gb,
        Cell2ReturnType::Distance2Add => ga + gb,
        Cell2ReturnType::Distance2Sub => ga - gb,
        Cell2ReturnType::Distance2Mul => b * ga + a * gb,
        Cell2ReturnType::Distance2Div => (ga * b - a * gb) / (b * b),
    }
}

/// Euclidean cellular2 value + analytic gradient, 2D. The `return_type`
/// combination of F1/F2 and its gradient are applied per lane; borders between
/// Voronoi cells (and slot switches) are discontinuities, as for the value.
#[inline(always)]
pub unsafe fn generate_2d_cellular2_gradient<S: Simd>(settings: &Cellular2Settings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let jitter = S::Vf32::set1(settings.jitter);
    let ret = settings.return_type;
    let i0 = settings.index0;
    let i1 = settings.index1;
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;

    let len = extent_x * extent_y;
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
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let mut col = 0;
        while col < extent_x {
            let (dist, delta) =
                cellular2_2d_deriv::<S>(x * freq_x_s, y * freq_y_s, jitter, i1, seed);
            let count = (extent_x - col).min(vector_width);
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

/// Euclidean cellular2 value + analytic gradient, 3D.
#[inline(always)]
pub unsafe fn generate_3d_cellular2_gradient<S: Simd>(settings: &Cellular2Settings) -> NoiseAndGradient {
    let dim = settings.dimensions();
    let seed = dim.seed;
    let jitter = S::Vf32::set1(settings.jitter);
    let ret = settings.return_type;
    let i0 = settings.index0;
    let i1 = settings.index1;
    let freq_x = settings.freq_x();
    let freq_y = settings.freq_y();
    let freq_z = settings.freq_z();
    let freq_x_s = S::Vf32::set1(freq_x);
    let freq_y_s = S::Vf32::set1(freq_y);
    let freq_z_s = S::Vf32::set1(freq_z);
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;

    let len = extent_x * extent_y * extent_z;
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
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let mut col = 0;
            while col < extent_x {
                let (dist, delta) = cellular2_3d_deriv::<S>(
                    x * freq_x_s,
                    y * freq_y_s,
                    z * freq_z_s,
                    jitter,
                    i1,
                    seed,
                );
                let count = (extent_x - col).min(vector_width);
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
pub unsafe fn generate_1d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => generate_1d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => generate_1d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => generate_1d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => generate_1d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => generate_1d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => generate_1d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn generate_2d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => generate_2d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => generate_2d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => generate_2d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => generate_2d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => generate_2d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => generate_2d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn generate_3d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => generate_3d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => generate_3d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => generate_3d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => generate_3d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => generate_3d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => generate_3d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn generate_4d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => generate_4d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => generate_4d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => generate_4d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => generate_4d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => generate_4d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => generate_4d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn generate_5d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => generate_5d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => generate_5d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => generate_5d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => generate_5d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => generate_5d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => generate_5d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

#[inline(always)]
#[allow(dead_code)]
pub unsafe fn generate_6d_noise<S: Simd>(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => generate_6d_noise_helper_f32::<S, FbmSettings>(*s),
        NoiseType::Ridge(s) => generate_6d_noise_helper_f32::<S, RidgeSettings>(*s),
        NoiseType::Turbulence(s) => generate_6d_noise_helper_f32::<S, TurbulenceSettings>(*s),
        NoiseType::Gradient(s) => generate_6d_noise_helper_f32::<S, GradientSettings>(*s),
        NoiseType::Cellular(s) => generate_6d_noise_helper_f32::<S, CellularSettings>(*s),
        NoiseType::Cellular2(s) => generate_6d_noise_helper_f32::<S, Cellular2Settings>(*s),
    }
}

/// Emplace 1d noise into `result` (which must have length `width`); returns the
/// slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn fill_1d_noise<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => fill_1d_noise_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => fill_1d_noise_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            fill_1d_noise_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => fill_1d_noise_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => fill_1d_noise_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => fill_1d_noise_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 2d noise into `result` (which must have length `width * height`);
/// returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn fill_2d_noise<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => fill_2d_noise_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => fill_2d_noise_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            fill_2d_noise_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => fill_2d_noise_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => fill_2d_noise_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => fill_2d_noise_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 3d noise into `result` (which must have length
/// `width * height * depth`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn fill_3d_noise<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => fill_3d_noise_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => fill_3d_noise_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            fill_3d_noise_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => fill_3d_noise_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => fill_3d_noise_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => fill_3d_noise_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 4d noise into `result` (which must have length
/// `width * height * depth * time`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn fill_4d_noise<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => fill_4d_noise_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => fill_4d_noise_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            fill_4d_noise_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => fill_4d_noise_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => fill_4d_noise_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => fill_4d_noise_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 5d noise into `result` (which must have length
/// `width * height * depth * time * extent5`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn fill_5d_noise<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => fill_5d_noise_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => fill_5d_noise_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            fill_5d_noise_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => fill_5d_noise_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => fill_5d_noise_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => fill_5d_noise_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

/// Emplace 6d noise into `result` (which must have length
/// `width * height * depth * time * extent5 * extent6`); returns the slice's `(min, max)`.
#[inline(always)]
#[allow(dead_code)]
pub unsafe fn fill_6d_noise<S: Simd>(noise_type: &NoiseType, result: &mut [f32]) -> (f32, f32) {
    match noise_type {
        NoiseType::Fbm(s) => fill_6d_noise_helper_f32::<S, FbmSettings>(*s, result),
        NoiseType::Ridge(s) => fill_6d_noise_helper_f32::<S, RidgeSettings>(*s, result),
        NoiseType::Turbulence(s) => {
            fill_6d_noise_helper_f32::<S, TurbulenceSettings>(*s, result)
        }
        NoiseType::Gradient(s) => fill_6d_noise_helper_f32::<S, GradientSettings>(*s, result),
        NoiseType::Cellular(s) => fill_6d_noise_helper_f32::<S, CellularSettings>(*s, result),
        NoiseType::Cellular2(s) => fill_6d_noise_helper_f32::<S, Cellular2Settings>(*s, result),
    }
}

// --- Fused typed (integer) emplace: SIMD noise compute + scalar map+cast store
//     directly into `&mut [T]`, with no output-sized f32 scratch. `mult`/`off`
//     map a noise value from [src_min, src_max] onto [T::RANGE_MIN, T::RANGE_MAX];
//     the per-lane `v * mult + off` is scalar f32 (no FMA), so results are
//     backend-independent and bit-match a manual map+cast. ---

#[inline(always)]
unsafe fn fill_1d_noise_range_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    assert_eq!(
        out.len(),
        extent_x,
        "noise output slice length must equal width"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
    for _ in 0..extent_x / vector_width {
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
unsafe fn fill_2d_noise_range_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    assert_eq!(
        out.len(),
        extent_x * extent_y,
        "noise output slice length must equal width * height"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut y = S::Vf32::set1(start_y);
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    for _ in 0..extent_y {
        let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
        for _ in 0..extent_x / vector_width {
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
unsafe fn fill_3d_noise_range_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    assert_eq!(
        out.len(),
        extent_x * extent_y * extent_z,
        "noise output slice length must equal width * height * depth"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut z = S::Vf32::set1(start_z);
    for _ in 0..extent_z {
        let mut y = S::Vf32::set1(start_y);
        for _ in 0..extent_y {
            let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
            for _ in 0..extent_x / vector_width {
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
unsafe fn fill_4d_noise_range_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let freq_w = S::Vf32::set1(settings.freq_w());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    assert_eq!(
        out.len(),
        extent_x * extent_y * extent_z * extent_w,
        "noise output slice length must equal width * height * depth * time"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut w = S::Vf32::set1(start_w);
    for _ in 0..extent_w {
        let mut z = S::Vf32::set1(start_z);
        for _ in 0..extent_z {
            let mut y = S::Vf32::set1(start_y);
            for _ in 0..extent_y {
                let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                for _ in 0..extent_x / vector_width {
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
unsafe fn fill_5d_noise_range_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let freq_w = S::Vf32::set1(settings.freq_w());
    let freq_u = S::Vf32::set1(settings.freq_u());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    let start_u = dim.offset_u;
    let extent_u = dim.extent_u;
    assert_eq!(
        out.len(),
        extent_x * extent_y * extent_z * extent_w * extent_u,
        "noise output slice length must equal width * height * depth * time * extent5"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut u = S::Vf32::set1(start_u);
    for _ in 0..extent_u {
        let mut w = S::Vf32::set1(start_w);
        for _ in 0..extent_w {
            let mut z = S::Vf32::set1(start_z);
            for _ in 0..extent_z {
                let mut y = S::Vf32::set1(start_y);
                for _ in 0..extent_y {
                    let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                    for _ in 0..extent_x / vector_width {
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
unsafe fn fill_6d_noise_range_helper_f32<S: Simd, T: NoiseInto, Settings: Sample32<S>>(
    settings: Settings,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    let dim = settings.dimensions();
    let freq_x = S::Vf32::set1(settings.freq_x());
    let freq_y = S::Vf32::set1(settings.freq_y());
    let freq_z = S::Vf32::set1(settings.freq_z());
    let freq_w = S::Vf32::set1(settings.freq_w());
    let freq_u = S::Vf32::set1(settings.freq_u());
    let freq_v = S::Vf32::set1(settings.freq_v());
    let start_x = dim.offset_x;
    let extent_x = dim.extent_x;
    let start_y = dim.offset_y;
    let extent_y = dim.extent_y;
    let start_z = dim.offset_z;
    let extent_z = dim.extent_z;
    let start_w = dim.offset_w;
    let extent_w = dim.extent_w;
    let start_u = dim.offset_u;
    let extent_u = dim.extent_u;
    let start_v = dim.offset_v;
    let extent_v = dim.extent_v;
    assert_eq!(
        out.len(),
        extent_x * extent_y * extent_z * extent_w * extent_u * extent_v,
        "noise output slice length must equal width * height * depth * time * extent5 * extent6"
    );
    let mult = (T::RANGE_MAX - T::RANGE_MIN) / (src_max - src_min);
    let off = T::RANGE_MIN - src_min * mult;
    let out_ptr = out.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = extent_x % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(start_x + i as f32);
    }
    x_arr.set_len(vector_width);
    let mut v = S::Vf32::set1(start_v);
    for _ in 0..extent_v {
        let mut u = S::Vf32::set1(start_u);
        for _ in 0..extent_u {
            let mut w = S::Vf32::set1(start_w);
            for _ in 0..extent_w {
                let mut z = S::Vf32::set1(start_z);
                for _ in 0..extent_z {
                    let mut y = S::Vf32::set1(start_y);
                    for _ in 0..extent_y {
                        let mut x = S::Vf32::load_from_ptr_unaligned(&x_arr[0]);
                        for _ in 0..extent_x / vector_width {
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
pub unsafe fn fill_1d_noise_range<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            fill_1d_noise_range_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            fill_1d_noise_range_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => fill_1d_noise_range_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            fill_1d_noise_range_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            fill_1d_noise_range_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            fill_1d_noise_range_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 2d emplace dispatcher.
#[inline(always)]
pub unsafe fn fill_2d_noise_range<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            fill_2d_noise_range_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            fill_2d_noise_range_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => fill_2d_noise_range_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            fill_2d_noise_range_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            fill_2d_noise_range_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            fill_2d_noise_range_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 3d emplace dispatcher.
#[inline(always)]
pub unsafe fn fill_3d_noise_range<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            fill_3d_noise_range_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            fill_3d_noise_range_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => fill_3d_noise_range_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            fill_3d_noise_range_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            fill_3d_noise_range_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            fill_3d_noise_range_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 4d emplace dispatcher.
#[inline(always)]
pub unsafe fn fill_4d_noise_range<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            fill_4d_noise_range_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            fill_4d_noise_range_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => fill_4d_noise_range_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            fill_4d_noise_range_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            fill_4d_noise_range_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            fill_4d_noise_range_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 5d emplace dispatcher.
#[inline(always)]
pub unsafe fn fill_5d_noise_range<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            fill_5d_noise_range_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            fill_5d_noise_range_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => fill_5d_noise_range_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            fill_5d_noise_range_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            fill_5d_noise_range_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            fill_5d_noise_range_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}

/// Fused typed 6d emplace dispatcher.
#[inline(always)]
pub unsafe fn fill_6d_noise_range<S: Simd, T: NoiseInto>(
    noise_type: &NoiseType,
    src_min: f32,
    src_max: f32,
    out: &mut [T],
) {
    match noise_type {
        NoiseType::Fbm(s) => {
            fill_6d_noise_range_helper_f32::<S, T, FbmSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Ridge(s) => {
            fill_6d_noise_range_helper_f32::<S, T, RidgeSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Turbulence(s) => fill_6d_noise_range_helper_f32::<S, T, TurbulenceSettings>(
            *s, src_min, src_max, out,
        ),
        NoiseType::Gradient(s) => {
            fill_6d_noise_range_helper_f32::<S, T, GradientSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular(s) => {
            fill_6d_noise_range_helper_f32::<S, T, CellularSettings>(*s, src_min, src_max, out)
        }
        NoiseType::Cellular2(s) => {
            fill_6d_noise_range_helper_f32::<S, T, Cellular2Settings>(*s, src_min, src_max, out)
        }
    }
}
