pub mod avx2;
mod shared;
pub mod sse2;
pub mod sse41;

use shared::*;

pub fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    fractal_settings: FractalSettings,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        return avx2::get_2d_noise(start_x, width, start_y, height, fractal_settings);
    } else if is_x86_feature_detected!("sse4.1") {
        return sse41::get_2d_noise(start_x, width, start_y, height, fractal_settings);
    } else if is_x86_feature_detected!("sse2") {
        return sse2::get_2d_noise(start_x, width, start_y, height, fractal_settings);
    } else {
        // TODO: add scalar fallback
        panic!("simdnoise requires SSE2 or better");
    }
}
