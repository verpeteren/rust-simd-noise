use simdeez::prelude::*;

use crate::{dimensional_being::DimensionalBeing, NoiseType};

/// Integer output types that noise can be quantized into by the `*_range_into`
/// emplace API. A source noise value in `[src_min, src_max]` is mapped linearly
/// onto this type's full range `[RANGE_MIN, RANGE_MAX]` and then cast; the cast
/// saturates (values outside the range clamp to the endpoints) and truncates
/// toward zero, matching `value as T`.
pub trait NoiseInto: Copy {
    /// Low end of the target range, as f32.
    const RANGE_MIN: f32;
    /// High end of the target range, as f32.
    const RANGE_MAX: f32;
    /// Saturating + truncating cast of an already range-mapped value.
    fn from_noise(mapped: f32) -> Self;
}

macro_rules! impl_noise_into {
    ($t:ty, $min:expr, $max:expr) => {
        impl NoiseInto for $t {
            const RANGE_MIN: f32 = $min;
            const RANGE_MAX: f32 = $max;
            #[inline(always)]
            fn from_noise(mapped: f32) -> Self {
                mapped as $t
            }
        }
    };
}

impl_noise_into!(u8, 0.0, u8::MAX as f32);
impl_noise_into!(u16, 0.0, u16::MAX as f32);
impl_noise_into!(u32, 0.0, u32::MAX as f32);
impl_noise_into!(i16, i16::MIN as f32, i16::MAX as f32);
impl_noise_into!(i32, i32::MIN as f32, i32::MAX as f32);


#[inline(always)]
pub unsafe fn scale_noise<S: Simd>(
    scale_min: f32,
    scale_max: f32,
    min: f32,
    max: f32,
    data: &mut [f32],
) {
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    let vector_width = S::Vf32::WIDTH;
    let mut i = 0;
    if data.len() >= vector_width {
        while i <= data.len() - vector_width {
            let value = (S::Vf32::set1(multiplier) * S::Vf32::load_from_ptr_unaligned(&data[i]))
                + S::Vf32::set1(offset);
            value.copy_to_ptr_unaligned(data.get_unchecked_mut(i));
            i += vector_width;
        }
    }
    i = data.len() - (data.len() % vector_width);
    while i < data.len() {
        *data.get_unchecked_mut(i) = data.get_unchecked(i) * multiplier + offset;
        i += 1;
    }
}

pub(crate) unsafe fn get_scaled_noise<S: Simd, F: Fn(&NoiseType) -> (Vec<f32>, f32, f32)>(
    noise_type: &NoiseType,
    noise_fn: F,
) -> Vec<f32> {
    let (mut noise, min, max) = noise_fn(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
    noise
}

/// f64 counterpart of [`scale_noise`]; affine-maps `[min, max]` onto
/// `[scale_min, scale_max]` in place over a caller-provided slice.
#[inline(always)]
pub unsafe fn scale_noise_64<S: Simd>(
    scale_min: f64,
    scale_max: f64,
    min: f64,
    max: f64,
    data: &mut [f64],
) {
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;
    let vector_width = S::Vf64::WIDTH;
    let mut i = 0;
    if data.len() >= vector_width {
        while i <= data.len() - vector_width {
            let value = (S::Vf64::set1(multiplier) * S::Vf64::load_from_ptr_unaligned(&data[i]))
                + S::Vf64::set1(offset);
            value.copy_to_ptr_unaligned(data.get_unchecked_mut(i));
            i += vector_width;
        }
    }
    i = data.len() - (data.len() % vector_width);
    while i < data.len() {
        *data.get_unchecked_mut(i) = data.get_unchecked(i) * multiplier + offset;
        i += 1;
    }
}

/// f64 counterpart of [`get_scaled_noise`]. The target range is passed
/// explicitly (as f64) rather than read from the f32 `NoiseDimensions`.
pub(crate) unsafe fn get_scaled_noise_64<S: Simd, F: Fn(&NoiseType) -> (Vec<f64>, f64, f64)>(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    noise_fn: F,
) -> Vec<f64> {
    let (mut noise, min, max) = noise_fn(noise_type);
    scale_noise_64::<S>(scale_min, scale_max, min, max, &mut noise);
    noise
}

/// Emplace+scale helper for f32: fills `out` via `noise_fn`, then scales it in
/// place to the target range read from the noise type's dimensions.
pub(crate) unsafe fn get_scaled_noise_into<S: Simd, F: Fn(&NoiseType, &mut [f32]) -> (f32, f32)>(
    noise_type: &NoiseType,
    out: &mut [f32],
    noise_fn: F,
) {
    let (min, max) = noise_fn(noise_type, out);
    let dim = noise_type.get_dimensions();
    scale_noise::<S>(dim.min, dim.max, min, max, out);
}

/// Emplace+scale helper for f64; the target range is passed explicitly.
pub(crate) unsafe fn get_scaled_noise_into_64<
    S: Simd,
    F: Fn(&NoiseType, &mut [f64]) -> (f64, f64),
>(
    noise_type: &NoiseType,
    scale_min: f64,
    scale_max: f64,
    out: &mut [f64],
    noise_fn: F,
) {
    let (min, max) = noise_fn(noise_type, out);
    scale_noise_64::<S>(scale_min, scale_max, min, max, out);
}
