extern crate simdeez;
use self::simdeez::*;
use super::*;
use shared::*;
use shared_sse::*;
use std::f32;

const F2: f32 = 0.36602540378;
const F3: f32 = 1.0 / 3.0;
const F4: f32 = 0.309016994;
const G2: f32 = 0.2113248654;
const G22: f32 = G2 * 2.0;
const G3: f32 = 1.0 / 6.0;
const G4: f32 = 0.138196601;

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
unsafe fn grad2<S: Simd>(hash: S::Vi32, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
    let h = S::and_si(hash, S::set1_epi32(7));
    let mask = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(4), h));
    let u = S::blendv_ps(y, x, mask);
    let v = S::mul_ps(S::set1_ps(2.0), S::blendv_ps(x, y, mask));

    let h_and_1 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(1)),
    ));
    let h_and_2 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(2)),
    ));

    S::add_ps(
        S::blendv_ps(S::sub_ps(S::setzero_ps(), u), u, h_and_1),
        S::blendv_ps(S::sub_ps(S::setzero_ps(), v), v, h_and_2),
    )
}

#[inline(always)]
pub unsafe fn simplex_2d<S: Simd>(x: S::Vf32, y: S::Vf32) -> S::Vf32 {
    let s = S::mul_ps(S::set1_ps(F2), S::add_ps(x, y));
    let ips = S::floor_ps(S::add_ps(x, s));
    let jps = S::floor_ps(S::add_ps(y, s));

    let i = S::cvtps_epi32(ips);
    let j = S::cvtps_epi32(jps);

    let t = S::mul_ps(S::cvtepi32_ps(S::add_epi32(i, j)), S::set1_ps(G2));

    let x0 = S::sub_ps(x, S::sub_ps(ips, t));
    let y0 = S::sub_ps(y, S::sub_ps(jps, t));

    let i1 = S::castps_si(S::cmpge_ps(x0, y0));

    let j1 = S::castps_si(S::cmpgt_ps(y0, x0));

    let x1 = S::add_ps(S::add_ps(x0, S::cvtepi32_ps(i1)), S::set1_ps(G2));
    let y1 = S::add_ps(S::add_ps(y0, S::cvtepi32_ps(j1)), S::set1_ps(G2));
    let x2 = S::add_ps(S::add_ps(x0, S::set1_ps(-1.0)), S::set1_ps(G22));
    let y2 = S::add_ps(S::add_ps(y0, S::set1_ps(-1.0)), S::set1_ps(G22));

    let ii = S::and_si(i, S::set1_epi32(0xff));
    let jj = S::and_si(j, S::set1_epi32(0xff));

    let gi0 = S::i32gather_epi32(&PERM, S::add_epi32(ii, S::i32gather_epi32(&PERM, jj)));

    let gi1 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, i1),
            S::i32gather_epi32(&PERM, S::sub_epi32(jj, j1)),
        ),
    );

    let gi2 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, S::set1_epi32(-1)),
            S::i32gather_epi32(&PERM, S::sub_epi32(jj, S::set1_epi32(-1))),
        ),
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let t0 = S::fnmadd_ps(y0, y0, S::fnmadd_ps(x0, x0, S::set1_ps(0.5)));
    let t1 = S::fnmadd_ps(y1, y1, S::fnmadd_ps(x1, x1, S::set1_ps(0.5)));
    let t2 = S::fnmadd_ps(y2, y2, S::fnmadd_ps(x2, x2, S::set1_ps(0.5)));

    let mut t0q = S::mul_ps(t0, t0);
    t0q = S::mul_ps(t0q, t0q);
    let mut t1q = S::mul_ps(t1, t1);
    t1q = S::mul_ps(t1q, t1q);
    let mut t2q = S::mul_ps(t2, t2);
    t2q = S::mul_ps(t2q, t2q);

    let mut n0 = S::mul_ps(t0q, grad2::<S>(gi0, x0, y0));
    let mut n1 = S::mul_ps(t1q, grad2::<S>(gi1, x1, y1));
    let mut n2 = S::mul_ps(t2q, grad2::<S>(gi2, x2, y2));

    let mut cond = S::cmplt_ps(t0, S::setzero_ps());
    n0 = S::andnot_ps(cond, n0);
    cond = S::cmplt_ps(t1, S::setzero_ps());
    n1 = S::andnot_ps(cond, n1);
    cond = S::cmplt_ps(t2, S::setzero_ps());
    n2 = S::andnot_ps(cond, n2);

    S::add_ps(n0, S::add_ps(n1, n2))
}
/// Get a single value of 2d fractal brownian motion.
#[inline(always)]
pub unsafe fn fbm_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut result = simplex_2d::<S>(xf, yf);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::fmadd_ps(simplex_2d::<S>(xf, yf), amp, result);
    }

    result
}

/// Get a single value of 2d ridge noise.
#[inline(always)]
pub unsafe fn ridge_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut result = S::sub_ps(S::set1_ps(1.0), S::abs_ps(simplex_2d::<S>(xf, yf)));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::fnmadd_ps(S::abs_ps(simplex_2d::<S>(xf, yf)), amp, S::set1_ps(1.0)),
        );
    }

    result
}
/// Get a single value of 2d turbulence.
#[inline(always)]
pub unsafe fn turbulence_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut result = S::abs_ps(simplex_2d::<S>(xf, yf));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::abs_ps(S::mul_ps(simplex_2d::<S>(xf, yf), amp)));
    }

    result
}

#[inline(always)]
unsafe fn get_2d_noise_helper<S: Simd>(x: S::Vf32, y: S::Vf32, noise_type: NoiseType) -> S::Vf32 {
    match noise_type {
        NoiseType::Fbm {
            freq,
            lacunarity,
            gain,
            octaves,
        } => fbm_2d::<S>(
            x,
            y,
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
        } => ridge_2d::<S>(
            x,
            y,
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
        } => turbulence_2d::<S>(
            x,
            y,
            S::set1_ps(freq),
            S::set1_ps(lacunarity),
            S::set1_ps(gain),
            octaves,
        ),
        NoiseType::Normal { freq } => simplex_2d::<S>(
            S::mul_ps(x, S::set1_ps(freq)),
            S::mul_ps(y, S::set1_ps(freq)),
        ),
        NoiseType::Cellular {
            freq,
            distance_function,
            return_type,
            jitter,
        } => panic!("ASD"),
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[inline(always)]
pub unsafe fn get_2d_noise<S: Simd>(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = S::set1_ps(f32::MAX);
    let mut max_s = S::set1_ps(f32::MIN);
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height);
    result.set_len(width * height);
    let mut y = S::set1_ps(start_y);
    let mut i = 0;
    let vector_width = S::WIDTH_BYTES / 4;
    let remainder = width % vector_width;
    let mut x_arr = Vec::with_capacity(vector_width);
    x_arr.set_len(vector_width);
    for i in (0..vector_width).rev() {
        x_arr[i] = start_x + i as f32;
    }
    for _ in 0..height {
        let mut x = S::loadu_ps(&x_arr[0]);
        for _ in 0..width / vector_width {
            let f = get_2d_noise_helper::<S>(x, y, noise_type);
            max_s = S::max_ps(max_s, f);
            min_s = S::min_ps(min_s, f);
            S::storeu_ps(result.get_unchecked_mut(i), f);
            i += vector_width;
            x = S::add_ps(x, S::set1_ps(vector_width as f32));
        }
        if remainder != 0 {
            let f = get_2d_noise_helper::<S>(x, y, noise_type);
            for j in 0..remainder {
                let n = S::get_lane_ps(f, i);
                *result.get_unchecked_mut(i) = n;
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
        y = S::add_ps(y, S::set1_ps(1.0));
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
pub unsafe fn scale_noise<S: Simd>(
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
