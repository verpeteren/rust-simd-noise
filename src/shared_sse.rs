#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
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
pub const F4: __m128 = unsafe {
    M128Array {
        array: [0.309016994; 4],
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
pub const G4: __m128 = unsafe {
    M128Array {
        array: [0.138196601; 4],
    }.simd
};
pub const G24: __m128 = unsafe {
    M128Array {
        array: [2.0 * 0.138196601; 4],
    }.simd
};

pub const G34: __m128 = unsafe {
    M128Array {
        array: [3.0 * 0.138196601; 4],
    }.simd
};

pub const G44: __m128 = unsafe {
    M128Array {
        array: [4.0 * 0.138196601; 4],
    }.simd
};
pub const X_PRIME: __m128i = unsafe { M128iArray { array:[1619;4] }.simd};
pub const Y_PRIME: __m128i = unsafe { M128iArray { array:[31337;4] }.simd};
pub const Z_PRIME: __m128i = unsafe { M128iArray { array:[6971;4] }.simd};
pub const W_PRIME: __m128i = unsafe { M128iArray { array:[1013;4] }.simd};

pub const CELL_DIVISOR: __m128 = unsafe { M128Array { array:[2147483648.0;4]}.simd};
pub const POINT_FIVE: __m128 = unsafe { M128Array { array: [0.5; 4] }.simd };

#[target_feature(enable = "sse2")]
pub unsafe fn _mm_abs_ps(a: __m128) -> __m128 {
    let b = _mm_set1_epi32(0x7fffffff);
    _mm_and_ps(a, _mm_castsi128_ps(b))
}
#[target_feature(enable = "sse2")]
pub unsafe fn blendvi_sse2(a: __m128i, b: __m128i, mask: __m128i) -> __m128i {
    _mm_or_si128(_mm_andnot_si128(mask, a), _mm_and_si128(mask, b))
}


#[target_feature(enable = "sse2")]
pub unsafe fn scale_array_sse(
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

    let mut i = 0;
    while i <= data.len() - 4 {
        _mm_storeu_ps(
            data.get_unchecked_mut(i),
            _mm_add_ps(
                _mm_mul_ps(_mm_set1_ps(multiplier), _mm_loadu_ps(&mut data[i])),
                _mm_set1_ps(offset),
            ),
        );
        i += 4;
    }
    i = data.len() - (data.len() % 4);
    while i < data.len() {
        *data.get_unchecked_mut(i) = data.get_unchecked(i) * multiplier + offset;
        i += 1;
    }
}
