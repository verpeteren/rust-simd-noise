use simdeez::prelude::*;

/// # Safety:
/// All array indices must be in-bounds.
#[inline(always)]
pub unsafe fn gather_32<S: Simd>(arr: &[i32], indices: S::Vi32) -> S::Vi32 {
    #[cfg(target_arch = "x86_64")]
    {
        use std::{
            any::TypeId,
            arch::x86_64::{__m256i, _mm256_i32gather_epi32},
        };
        // If the underlying SIMD implementation is AVX2, use the AVX2 gather intrinsic. In my testing, the compiler
        // will constant-fold away the branch.
        if TypeId::of::<<S::Vi32 as SimdConsts>::UnderlyingType>() == TypeId::of::<__m256i>() {
            let reg256 = std::mem::transmute_copy::<_, __m256i>(&indices.underlying_value());
            let gathered = _mm256_i32gather_epi32::<4>(arr.as_ptr(), reg256);
            return S::Vi32::from_underlying_value(std::mem::transmute_copy::<_, _>(&gathered));
        }
    }

    // Fallback scalar implementation.
    let width = S::Vi32::WIDTH;
    let mut dst = S::Vi32::zeroes();
    for i in 0..width {
        *dst.get_unchecked_mut(i) = *arr.get_unchecked(indices[i] as usize);
    }
    dst
}

/// # Safety:
/// All array indices must be in-bounds.
#[inline(always)]
pub unsafe fn gather_64<S: Simd>(arr: &[i64], indices: S::Vi64) -> S::Vi64 {
    #[cfg(target_arch = "x86_64")]
    {
        use std::{
            any::TypeId,
            arch::x86_64::{__m256i, _mm256_i64gather_epi64},
        };
        // If the underlying SIMD implementation is AVX2, use the AVX2 gather intrinsic. In my testing, the compiler
        // will constant-fold away the branch.
        if TypeId::of::<<S::Vi64 as SimdConsts>::UnderlyingType>() == TypeId::of::<__m256i>() {
            let reg256 = std::mem::transmute_copy::<_, __m256i>(&indices.underlying_value());
            let gathered = _mm256_i64gather_epi64::<4>(arr.as_ptr(), reg256);
            return S::Vi64::from_underlying_value(std::mem::transmute_copy::<_, _>(&gathered));
        }
    }

    let width = S::Vi64::WIDTH;
    let mut dst = S::Vi64::zeroes();
    for i in 0..width {
        *dst.get_unchecked_mut(i) = *arr.get_unchecked(indices[i] as usize);
    }
    dst
}
