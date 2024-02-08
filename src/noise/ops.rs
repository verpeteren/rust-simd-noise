use simdeez::prelude::*;

#[inline(always)]
pub fn gather_32<S: Simd>(arr: &[i32], indices: S::Vi32) -> S::Vi32 {
    let width = S::Vi32::WIDTH;
    let mut dst = S::Vi32::zeroes();
    for i in 0..width {
        dst[i] = arr[indices[i] as usize];
    }
    dst
}

#[inline(always)]
pub fn gather_64<S: Simd>(arr: &[i64], indices: S::Vi64) -> S::Vi64 {
    let width = S::Vi64::WIDTH;
    let mut dst = S::Vi64::zeroes();
    for i in 0..width {
        dst[i] = arr[indices[i] as usize];
    }
    dst
}
