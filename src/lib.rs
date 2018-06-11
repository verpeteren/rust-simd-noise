pub mod avx2;
mod shared;
mod shared_sse;
pub mod sse2;
pub mod sse41;

#[derive(Copy, Clone)]
pub enum NoiseType {
    FBM,
    Turbulence,
    Normal,
}
#[derive(Copy, Clone)]
pub struct FractalSettings {
    pub freq: f32,
    pub lacunarity: f32,
    pub gain: f32,
    pub octaves: i32,
    pub noise_type: NoiseType,
}

pub fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    fractal_settings: FractalSettings,
) -> (Vec<f32>, f32, f32) {
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

pub fn get_2d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    fractal_settings: FractalSettings,
    scaled_min: f32,
    scaled_max: f32,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        return avx2::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        );
    } else if is_x86_feature_detected!("sse4.1") {
        return sse41::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        );
    } else if is_x86_feature_detected!("sse2") {
        return sse2::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        );
    } else {
        // TODO: add scalar fallback
        panic!("simdnoise requires SSE2 or better");
    }
}
