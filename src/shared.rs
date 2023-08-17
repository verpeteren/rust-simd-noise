use simdeez::prelude::*;

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
            let value = S::add_ps(
                S::mul_ps(S::Vf32::set1(multiplier), S::loadu_ps(&data[i])),
                S::Vf32::set1(offset),
            );
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
