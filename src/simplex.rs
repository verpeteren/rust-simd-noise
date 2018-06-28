extern crate simdeez;
use self::simdeez::*;
use super::*;
use shared::*;
use shared_sse::*;
use std::f32;

#[inline(always)]
pub unsafe fn grad1_simd<S: Simd>(hash: S::Vi32, x: S::Vf32) -> S::Vf32 {
    let h = S::and_si(hash, S::set1_epi32(15));
    let v = S::cvtepi32_ps(S::and_si(h, S::set1_epi32(7)));

    let h_and_8 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(8)),
    ));
    let grad = S::blendv_ps(S::sub_ps(S::setzero_ps(), v), v, h_and_8);
    S::mul_ps(grad, x)
}
#[inline(always)]
pub unsafe fn simplex_1d<S: Simd>(x: S::Vf32) -> S::Vf32 {
    let ips = S::fastfloor_ps(x);
    let mut i0 = S::cvtps_epi32(ips);
    let i1 = S::and_si(S::add_epi32(i0, S::set1_epi32(1)), S::set1_epi32(0xff));

    let x0 = S::sub_ps(x, ips);
    let x1 = S::sub_ps(x0, S::set1_ps(1.0));

    i0 = S::and_si(i0, S::set1_epi32(0xff));
    let gi0 = S::i32gather_epi32(&PERM, i0);
    let gi1 = S::i32gather_epi32(&PERM, i1);

    let mut t0 = S::sub_ps(S::set1_ps(1.0), S::mul_ps(x0, x0));
    t0 = S::mul_ps(t0, t0);
    t0 = S::mul_ps(t0, t0);
    let n0 = S::mul_ps(t0, grad1_simd::<S>(gi0, x0));

    let mut t1 = S::sub_ps(S::set1_ps(1.0), S::mul_ps(x1, x1));
    t1 = S::mul_ps(t1, t1);
    t1 = S::mul_ps(t1, t1);
    let n1 = S::mul_ps(t1, grad1_simd::<S>(gi1, x1));

    S::add_ps(n0, n1)
}

#[inline(always)]
pub unsafe fn fbm_1d<S: Simd>(
    x: S::Vf32,
    freq: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut result = simplex_1d::<S>(xf);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::mul_ps(simplex_1d::<S>(xf), amp));
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_1d<S: Simd>(
    x: S::Vf32,
    freq: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut result = S::sub_ps(S::set1_ps(1.0), S::abs_ps(simplex_1d::<S>(xf)));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::sub_ps(
                S::set1_ps(1.0),
                S::abs_ps(S::mul_ps(simplex_1d::<S>(xf), amp)),
            ),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_1d<S: Simd>(
    x: S::Vf32,
    freq: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut result = S::abs_ps(simplex_1d::<S>(xf));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::abs_ps(S::mul_ps(simplex_1d::<S>(xf), amp)));
    }

    result
}

#[inline(always)]
unsafe fn get_1d_noise_helper<S: Simd>(x: S::Vf32, noise_type: NoiseType) -> S::Vf32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_1d::<S>(
            x,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Ridge {
            freq,
            lacunarity,
            gain,
            octaves,
        } => ridge_1d::<S>(
            x,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Turbulence {
            freq,
            lacunarity,
            gain,
            octaves,
        } => turbulence_1d::<S>(
            x,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Normal { freq } => simplex_1d::<S>(S::mul_ps(x, S::set1_ps(freq))),

        NoiseType::Cellular {
            freq,
            distance_function,
            return_type,
            jitter,
        } => panic!("bla"),
    }
}

#[inline(always)]
pub unsafe fn get_1d_noise<S: Simd>(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = S::set1_ps(f32::MAX);
    let mut max_s = S::set1_ps(f32::MIN);

    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result: Vec<f32> = Vec::with_capacity(width);
    result.set_len(width);
    let mut i = 0;
    let vector_width = S::WIDTH_BYTES / 4;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f32;
    }
    let mut x = S::loadu_ps(&x_arr[0]);

    for _ in 0..width / vector_width {
        let f = get_1d_noise_helper::<S>(x, noise_type);
        max_s = S::max_ps(max_s, f);
        min_s = S::min_ps(min_s, f);
        S::storeu_ps(result.get_unchecked_mut(i), f);
        i += vector_width;
        x = S::add_ps(x, S::set1_ps(vector_width as f32));
    }
    if remainder != 0 {
        let f = get_1d_noise_helper::<S>(x, noise_type);
        for j in 0..remainder {
            let n = S::get_lane_ps(f, j);
            *result.get_unchecked_mut(i) = n;
            // Note: This is unecessary for large images
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            i += 1;
        }
    }
    for i in 0..vector_width {
        if S::get_lane_ps(min_s, i) < min {
            min = S::get_lane_ps(min_s, i);
        }
        if S::get_lane_ps(max_s, i) > max {
            max = S::get_lane_ps(max_s, i);
        }
    }
    (result, min, max)
}

#[inline(always)]
pub unsafe fn get_1d_scaled_noise<S: Simd>(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise::<S>(start_x, width, noise_type);
    scale_array::<S>(scale_min, scale_max, min, max, &mut noise);
    noise
}

#[inline(always)]
pub unsafe fn scale_array<S: Simd>(
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
    let vector_width = S::WIDTH_BYTES / 4;
    let mut i = 0;
    while i <= data.len() - vector_width {
        let value = S::add_ps(
            S::mul_ps(S::set1_ps(multiplier), S::loadu_ps(&data[i])),
            S::set1_ps(offset),
        );
        S::storeu_ps(data.get_unchecked_mut(i), value);
        i += vector_width;
    }
    i = data.len() - (data.len() % vector_width);
    while i < data.len() {
        *data.get_unchecked_mut(i) = data.get_unchecked(i) * multiplier + offset;
        i += 1;
    }
}
