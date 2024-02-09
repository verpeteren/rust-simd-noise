use simdeez::prelude::*;

use crate::{dimensional_being::DimensionalBeing, NoiseType};

#[inline(always)]
pub unsafe fn scale_noise<S: Simd>(
    scale_min: f32,
    scale_max: f32,
    min: f32,
    max: f32,
    data: &mut Vec<f32>,
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

pub(crate) unsafe fn get_scaled_noise<S: Simd, F: Fn(&NoiseType) -> (Vec<f32>, f32, f32)>(noise_type: &NoiseType, noise_fn: F) -> Vec<f32> {
    let (mut noise, min, max) = noise_fn(noise_type);
    let dim = noise_type.get_dimensions();
    scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
    noise
}
