#![feature(test)]
extern crate test;

pub mod avx2;
pub mod scalar;
mod shared;
mod shared_sse;
pub mod sse2;
pub mod sse41;

/// Specifies what type of noise to generate.
#[derive(Copy, Clone)]
pub enum NoiseType {
    /// Fractal brownian motion on top of simplex noise
    FBM,
    /// Turbulence on top of simplex noise
    Turbulence,
    /// Simplex Noise
    Normal,
}

/// Contains parameters for noise functions. When using
/// `Normal` noise, only frequency is used.
#[derive(Copy, Clone)]
pub struct FractalSettings {
    /// Higher frequency will appear to 'zoom' out, lower will appear to 'zoom' in. A good 
    /// starting value for experimentation is around 0.05
    pub freq: f32,
    /// Lacunarity affects how the octaves are layered together. A good starting value to
    /// experiment with is 0.5, change from there in 0.25 increments to see what it looks like.
    pub lacunarity: f32,
    /// Gain affects how the octaves are layered together. A good starting value is 2.0
    pub gain: f32,
    /// Specifies how many layers of nose to combine. More octaves can yeild more detail
    /// and will increase runtime linearlly.
    pub octaves: u8,
    /// The type of noise to generate.  
    pub noise_type: NoiseType,
}

/// Gets a width X height sized block of 2d noise, unscaled,
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    fractal_settings: FractalSettings,
) -> (Vec<f32>, f32, f32) {
    if is_x86_feature_detected!("avx2") {
        avx2::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    } else if is_x86_feature_detected!("sse4.1") {
        sse41::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    } else if is_x86_feature_detected!("sse2") {
        sse2::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    } else {
        scalar::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    }
}

/// Gets a width X height sized block of scaled 2d noise
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
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
        avx2::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    } else if is_x86_feature_detected!("sse4.1") {
        sse41::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    } else if is_x86_feature_detected!("sse2") {
        sse2::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    } else {
        scalar::get_2d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
pub fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    fractal_settings: FractalSettings,
) -> (Vec<f32>, f32, f32) {
    if is_x86_feature_detected!("avx2") {
        avx2::get_3d_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
        )
    } else if is_x86_feature_detected!("sse4.1") {
        sse41::get_3d_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
        )
    } else if is_x86_feature_detected!("sse2") {
        sse2::get_3d_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
        )
    } else {
        scalar::get_3d_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
        )
    }
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// using runtime CPU feature detection to pick the fastest method
/// between scalar, SSE2, SSE41, and AVX2
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
pub fn get_3d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    fractal_settings: FractalSettings,
    scaled_min: f32,
    scaled_max: f32,
) -> Vec<f32> {
    if is_x86_feature_detected!("avx2") {
        avx2::get_3d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    } else if is_x86_feature_detected!("sse4.1") {
        sse41::get_3d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    } else if is_x86_feature_detected!("sse2") {
        sse2::get_3d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    } else {
        scalar::get_3d_scaled_noise(
            start_x,
            width,
            start_y,
            height,
            start_z,
            depth,
            fractal_settings,
            scaled_min,
            scaled_max,
        )
    }
}

#[cfg(test)]
mod benchmarks {

    use super::*;
    use test::{black_box, Bencher};

    const FRACTAL_SETTINGS: FractalSettings = FractalSettings {
        freq: 0.04,
        lacunarity: 0.5,
        gain: 2.0,
        octaves: 3,
        noise_type: NoiseType::FBM,
    };

    #[bench]
    fn b2d_1_scalar(b: &mut Bencher) {
        b.iter(|| black_box(scalar::get_2d_noise(0.0, 1000, 0.0, 1000, FRACTAL_SETTINGS)));
    }
    #[bench]
    fn b2d_2_sse2(b: &mut Bencher) {
        b.iter(|| black_box(sse2::get_2d_noise(0.0, 1000, 0.0, 1000, FRACTAL_SETTINGS)));
    }
    #[bench]
    fn b2d_3_sse41(b: &mut Bencher) {
        b.iter(|| black_box(sse41::get_2d_noise(0.0, 1000, 0.0, 1000, FRACTAL_SETTINGS)));
    }
    #[bench]
    fn b2d_4_avx2d(b: &mut Bencher) {
        b.iter(|| black_box(avx2::get_2d_noise(0.0, 1000, 0.0,1000, FRACTAL_SETTINGS)));
    }
    #[bench]
    fn b3d_1_scalar(b: &mut Bencher) {
        b.iter(|| {
            black_box(scalar::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                FRACTAL_SETTINGS,
            ))
        });
    }
    #[bench]
    fn b3d_2_sse2(b: &mut Bencher) {
        b.iter(|| {
            black_box(sse2::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                FRACTAL_SETTINGS,
            ))
        });
    }
    #[bench]
    fn b3d_3_sse41(b: &mut Bencher) {
        b.iter(|| {
            black_box(sse41::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                FRACTAL_SETTINGS,
            ))
        });
    }
    #[bench]
    fn b3d_4_avx2(b: &mut Bencher) {
        b.iter(|| {
            black_box(avx2::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                FRACTAL_SETTINGS,
            ))
        });
    }
}
