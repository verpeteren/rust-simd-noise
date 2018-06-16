#![feature(test)]
extern crate test;

pub mod avx2;
pub mod scalar;
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
    pub octaves: u8,
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
        avx2::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    } else if is_x86_feature_detected!("sse4.1") {
        sse41::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    } else if is_x86_feature_detected!("sse2") {
        sse2::get_2d_noise(start_x, width, start_y, height, fractal_settings)
    } else {
        scalar::get_2d_noise(start_x, width, start_y, height, fractal_settings)
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
mod tests {

    use super::*;
    use test::{black_box, Bencher};

    const fractal_settings: FractalSettings = FractalSettings {
        freq: 0.04,
        lacunarity: 0.5,
        gain: 2.0,
        octaves: 3,
        noise_type: NoiseType::FBM,
    };

    #[bench]
    fn scalar_2d(b: &mut Bencher) {
        b.iter(|| black_box(scalar::get_2d_noise(0.0, 1000, 0.0, 1000, fractal_settings)));
    }
    #[bench]
    fn sse2_2d(b: &mut Bencher) {
        b.iter(|| black_box(sse2::get_2d_noise(0.0, 1000, 0.0, 1000, fractal_settings)));
    }
    #[bench]
    fn sse41_2d(b: &mut Bencher) {
        b.iter(|| black_box(sse41::get_2d_noise(0.0, 1000, 0.0, 1000, fractal_settings)));
    }
    #[bench]
    fn avx2_2d(b: &mut Bencher) {
        b.iter(|| black_box(avx2::get_2d_noise(0.0, 1000, 0.0, 1000, fractal_settings)));
    }
    #[bench]
    fn scalar_3d(b: &mut Bencher) {
        b.iter(|| {
            black_box(scalar::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                fractal_settings,
            ))
        });
    }
    #[bench]
    fn sse2_3d(b: &mut Bencher) {
        b.iter(|| {
            black_box(sse2::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                fractal_settings,
            ))
        });
    }
    #[bench]
    fn sse41_3d(b: &mut Bencher) {
        b.iter(|| {
            black_box(sse41::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                fractal_settings,
            ))
        });
    }
    #[bench]
    fn avx2_3d(b: &mut Bencher) {
        b.iter(|| {
            black_box(avx2::get_3d_noise(
                0.0,
                100,
                0.0,
                100,
                0.0,
                100,
                fractal_settings,
            ))
        });
    }
}
