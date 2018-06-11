use std::arch::x86_64::*;

pub union M128Array {
    pub simd: __m128,
    pub array: [f32; 4],
}

pub union M128iArray {
    pub simd: __m128i,
    pub array: [i32; 4],
}

pub const F2: __m128 = unsafe {
    M128Array {
        array: [0.36602540378; 4],
    }.simd
};
pub const F3: __m128 = unsafe {
    M128Array {
        array: [1.0 / 3.0; 4],
    }.simd
};
pub const G2: __m128 = unsafe {
    M128Array {
        array: [0.2113248654; 4],
    }.simd
};
pub const G22: __m128 = unsafe {
    M128Array {
        array: [2.0 * 0.2113248654; 4],
    }.simd
};
pub const G3: __m128 = unsafe {
    M128Array {
        array: [1.0 / 6.0; 4],
    }.simd
};
pub const POINT_FIVE: __m128 = unsafe { M128Array { array: [0.5; 4] }.simd };

pub unsafe fn dot_simd(x1: __m128, y1: __m128, x2: __m128, y2: __m128) -> __m128 {
    _mm_add_ps(_mm_mul_ps(x1, x2), _mm_mul_ps(y1, y2))
}

pub unsafe fn _mm_abs_ps(a: __m128) -> __m128 {
    let b = _mm_set1_epi32(0x7fffffff);
    _mm_and_ps(a, _mm_castsi128_ps(b))
}

pub fn scale_array_sse(scale_min: f32, scale_max: f32, min: f32, max: f32, data: &mut Vec<f32>) {
    unsafe {
        let scale_range = scale_max - scale_min;
        let range = max - min;
        let multiplier = scale_range / range;
        let offset = scale_min - min * multiplier;

        let mut i = 0;
        while i <= data.len() - 4 {
            _mm_storeu_ps(
                &mut data[i],
                _mm_add_ps(
                    _mm_mul_ps(_mm_set1_ps(multiplier), _mm_loadu_ps(&mut data[i])),
                    _mm_set1_ps(offset),
                ),
            );
            i = i + 4;
        }
        i = data.len() - (data.len() % 4);
        while i < data.len() {
            data[i] = data[i] * multiplier + offset;
            i = i + 1;
        }
    }
}
