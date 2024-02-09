use simdeez::prelude::*;

/// # Safety:
/// All array indices must be in-bounds.
#[inline(always)]
pub unsafe fn gather_32<S: Simd>(arr: &[i32], indices: S::Vi32) -> S::Vi32 {
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
    let width = S::Vi64::WIDTH;
    let mut dst = S::Vi64::zeroes();
    for i in 0..width {
        *dst.get_unchecked_mut(i) = *arr.get_unchecked(indices[i] as usize);
    }
    dst
}
