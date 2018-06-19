//! AVX2 and FMA3 Accelerated noise functions.
//! CPUs since ~2013 (Intel) and ~2015 (AMD) support this.
//! It is about twice as fast as the SSE2 version.
//!
//! Use `is_x86_feature_detected!("avx2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 8, and when it is not small relative height and depth.
use super::*;
use shared::*;
use std::arch::x86_64::*;
use std::f32;

union M256Array {
    simd: __m256,
    array: [f32; 8],
}
const F2: __m256 = unsafe {
    M256Array {
        array: [0.36602540378; 8],
    }.simd
};
const F3: __m256 = unsafe {
    M256Array {
        array: [1.0 / 3.0; 8],
    }.simd
};
pub const F4: __m256 = unsafe {
    M256Array {
        array: [0.309016994; 8],
    }.simd
};
const G2: __m256 = unsafe {
    M256Array {
        array: [0.2113248654; 8],
    }.simd
};

const G22: __m256 = unsafe {
    M256Array {
        array: [2.0 * 0.2113248654; 8],
    }.simd
};
const G3: __m256 = unsafe {
    M256Array {
        array: [1.0 / 6.0; 8],
    }.simd
};
pub const G4: __m256 = unsafe {
    M256Array {
        array: [0.138196601; 8],
    }.simd
};
pub const G24: __m256 = unsafe {
    M256Array {
        array: [2.0 * 0.138196601; 8],
    }.simd
};

pub const G34: __m256 = unsafe {
    M256Array {
        array: [3.0 * 0.138196601; 8],
    }.simd
};

pub const G44: __m256 = unsafe {
    M256Array {
        array: [4.0 * 0.138196601; 8],
    }.simd
};

const POINT_FIVE: __m256 = unsafe { M256Array { array: [0.5; 8] }.simd };

#[target_feature(enable = "avx2")]
unsafe fn grad1_simd(hash: __m256i, x: __m256) -> __m256 {
    let h = _mm256_and_si256(hash, _mm256_set1_epi32(15));
    let v = _mm256_cvtepi32_ps(_mm256_and_si256(h, _mm256_set1_epi32(7)));

    let h_and_8 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(8)),
    ));
    let grad = _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), v), v, h_and_8);
    _mm256_mul_ps(grad, x)
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
unsafe fn simplex_1d(x: __m256) -> __m256 {
    let ips = _mm256_floor_ps(x);
    let mut i0 = _mm256_cvtps_epi32(ips);
    let i1 = _mm256_and_si256(
        _mm256_add_epi32(i0, _mm256_set1_epi32(1)),
        _mm256_set1_epi32(0xff),
    );

    let x0 = _mm256_sub_ps(x, ips);
    let x1 = _mm256_sub_ps(x0, _mm256_set1_ps(1.0));

    i0 = _mm256_and_si256(i0, _mm256_set1_epi32(0xff));
    let gi0 = _mm256_i32gather_epi32(&PERM as *const i32, i0, 4);
    let gi1 = _mm256_i32gather_epi32(&PERM as *const i32, i1, 4);

    let mut t0 = _mm256_sub_ps(_mm256_set1_ps(1.0), _mm256_mul_ps(x0, x0));
    t0 = _mm256_mul_ps(t0, t0);
    t0 = _mm256_mul_ps(t0, t0);
    let n0 = _mm256_mul_ps(t0, grad1_simd(gi0, x0));

    let mut t1 = _mm256_sub_ps(_mm256_set1_ps(1.0), _mm256_mul_ps(x1, x1));
    t1 = _mm256_mul_ps(t1, t1);
    t1 = _mm256_mul_ps(t1, t1);
    let n1 = _mm256_mul_ps(t1, grad1_simd(gi1, x1));

    _mm256_add_ps(n0, n1)
}
/// Get a single value of 1d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_1d(
    x: __m256,
    freq: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut result = simplex_1d(xf);
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lacunarity);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(result, _mm256_mul_ps(simplex_1d(xf), amp));
    }

    result
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_1d(
    x: __m256,
    freq: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut result = _mm256_sub_ps(_mm256_set1_ps(1.0), _mm256_abs_ps(simplex_1d(xf)));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lacunarity);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_sub_ps(
                _mm256_set1_ps(1.0),
                _mm256_abs_ps(_mm256_mul_ps(simplex_1d(xf), amp)),
            ),
        );
    }

    result
}

/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_1d(
    x: __m256,
    freq: __m256,
    lacunarity: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut result = _mm256_abs_ps(simplex_1d(xf));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lacunarity);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(result, _mm256_abs_ps(_mm256_mul_ps(simplex_1d(xf), amp)));
    }

    result
}

#[target_feature(enable = "avx2")]
unsafe fn get_1d_noise_helper(x: __m256, noise_type: NoiseType) -> M256Array {
    M256Array {
        simd: match noise_type {
            NoiseType::Fbm {
                freq,
                lacunarity,
                gain,
                octaves,
            } => fbm_1d(
                x,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Ridge {
                freq,
                lacunarity,
                gain,
                octaves,
            } => ridge_1d(
                x,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Turbulence {
                freq,
                lacunarity,
                gain,
                octaves,
            } => turbulence_1d(
                x,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_1d(_mm256_mul_ps(x, _mm256_set1_ps(freq))),
        },
    }
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_1d_noise(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M256Array {
        simd: _mm256_set1_ps(f32::MAX),
    };
    let mut max_s = M256Array {
        simd: _mm256_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width);
    result.set_len(width);
    let mut i = 0;
    let remainder = width % 8;
    let mut x = _mm256_set_ps(
        start_x + 7.0,
        start_x + 6.0,
        start_x + 5.0,
        start_x + 4.0,
        start_x + 3.0,
        start_x + 2.0,
        start_x + 1.0,
        start_x,
    );
    for _ in 0..width / 8 {
        let f = get_1d_noise_helper(x, noise_type).simd;
        max_s.simd = _mm256_max_ps(max_s.simd, f);
        min_s.simd = _mm256_min_ps(min_s.simd, f);
        _mm256_storeu_ps(result.get_unchecked_mut(i), f);
        i += 8;
        x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
    }
    if remainder != 0 {
        let f = get_1d_noise_helper(x, noise_type);
        for j in 0..remainder {
            let n = f.array[j];
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
    for i in 0..8 {
        if *min_s.array.get_unchecked(i) < min {
            min = *min_s.array.get_unchecked(i);
        }
        if *max_s.array.get_unchecked(i) > max {
            max = *max_s.array.get_unchecked(i);
        }
    }
    (result, min, max)
}

/// Gets a width sized block of scaled 2d noise
/// `start_x` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_1d_scaled_noise(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(start_x, width, noise_type);
    scale_array(scale_min, scale_max, min, max, &mut noise);
    noise
}

#[target_feature(enable = "avx2")]
unsafe fn grad2_simd(hash: __m256i, x: __m256, y: __m256) -> __m256 {
    let h = _mm256_and_si256(hash, _mm256_set1_epi32(7));
    let mask = _mm256_castsi256_ps(_mm256_cmpgt_epi32(_mm256_set1_epi32(4), h));
    let u = _mm256_blendv_ps(y, x, mask);
    let v = _mm256_mul_ps(_mm256_set1_ps(2.0), _mm256_blendv_ps(x, y, mask));

    let h_and_1 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(1)),
    ));
    let h_and_2 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(2)),
    ));

    _mm256_add_ps(
        _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), u), u, h_and_1),
        _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), v), v, h_and_2),
    )
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_2d(x: __m256, y: __m256) -> __m256 {
    let s = _mm256_mul_ps(F2, _mm256_add_ps(x, y));
    let ips = _mm256_floor_ps(_mm256_add_ps(x, s));
    let jps = _mm256_floor_ps(_mm256_add_ps(y, s));

    let i = _mm256_cvtps_epi32(ips);
    let j = _mm256_cvtps_epi32(jps);

    let t = _mm256_mul_ps(_mm256_cvtepi32_ps(_mm256_add_epi32(i, j)), G2);

    let x0 = _mm256_sub_ps(x, _mm256_sub_ps(ips, t));
    let y0 = _mm256_sub_ps(y, _mm256_sub_ps(jps, t));

    let i1 = _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ));

    let j1 = _mm256_castps_si256(_mm256_cmp_ps(y0, x0, _CMP_GT_OQ));

    let x1 = _mm256_add_ps(_mm256_add_ps(x0, _mm256_cvtepi32_ps(i1)), G2);
    let y1 = _mm256_add_ps(_mm256_add_ps(y0, _mm256_cvtepi32_ps(j1)), G2);
    let x2 = _mm256_add_ps(_mm256_add_ps(x0, _mm256_set1_ps(-1.0)), G22);
    let y2 = _mm256_add_ps(_mm256_add_ps(y0, _mm256_set1_ps(-1.0)), G22);

    let ii = _mm256_and_si256(i, _mm256_set1_epi32(0xff));
    let jj = _mm256_and_si256(j, _mm256_set1_epi32(0xff));

    let gi0 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(ii, _mm256_i32gather_epi32(&PERM as *const i32, jj, 4)),
        4,
    );

    let gi1 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_sub_epi32(ii, i1),
            _mm256_i32gather_epi32(&PERM as *const i32, _mm256_sub_epi32(jj, j1), 4),
        ),
        4,
    );

    let gi2 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_sub_epi32(ii, _mm256_set1_epi32(-1)),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_sub_epi32(jj, _mm256_set1_epi32(-1)),
                4,
            ),
        ),
        4,
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let t0 = _mm256_fnmadd_ps(y0, y0, _mm256_fnmadd_ps(x0, x0, POINT_FIVE));
    let t1 = _mm256_fnmadd_ps(y1, y1, _mm256_fnmadd_ps(x1, x1, POINT_FIVE));
    let t2 = _mm256_fnmadd_ps(y2, y2, _mm256_fnmadd_ps(x2, x2, POINT_FIVE));

    let mut t0q = _mm256_mul_ps(t0, t0);
    t0q = _mm256_mul_ps(t0q, t0q);
    let mut t1q = _mm256_mul_ps(t1, t1);
    t1q = _mm256_mul_ps(t1q, t1q);
    let mut t2q = _mm256_mul_ps(t2, t2);
    t2q = _mm256_mul_ps(t2q, t2q);

    let mut n0 = _mm256_mul_ps(t0q, grad2_simd(gi0, x0, y0));
    let mut n1 = _mm256_mul_ps(t1q, grad2_simd(gi1, x1, y1));
    let mut n2 = _mm256_mul_ps(t2q, grad2_simd(gi2, x2, y2));

    let mut cond = _mm256_cmp_ps(t0, _mm256_setzero_ps(), _CMP_LT_OQ);
    n0 = _mm256_andnot_ps(cond, n0);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(), _CMP_LT_OQ);
    n1 = _mm256_andnot_ps(cond, n1);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(), _CMP_LT_OQ);
    n2 = _mm256_andnot_ps(cond, n2);

    _mm256_add_ps(n0, _mm256_add_ps(n1, n2))
}

#[target_feature(enable = "avx2")]
unsafe fn _mm256_abs_ps(a: __m256) -> __m256 {
    let b = _mm256_set1_epi32(0x7fffffff);
    _mm256_and_ps(a, _mm256_castsi256_ps(b))
}

/// Get a single value of 2d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut result = simplex_2d(xf, yf);
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_fmadd_ps(simplex_2d(xf, yf), amp, result);
    }

    result
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut result = _mm256_sub_ps(_mm256_set1_ps(1.0), _mm256_abs_ps(simplex_2d(xf, yf)));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_fnmadd_ps(_mm256_abs_ps(simplex_2d(xf, yf)), amp, _mm256_set1_ps(1.0)),
        );
    }

    result
}
/// Get a single value of 2d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut result = _mm256_abs_ps(simplex_2d(xf, yf));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_abs_ps(_mm256_mul_ps(simplex_2d(xf, yf), amp)),
        );
    }

    result
}

#[target_feature(enable = "avx2")]
unsafe fn scale_array(scale_min: f32, scale_max: f32, min: f32, max: f32, data: &mut Vec<f32>) {
    let scale_range = scale_max - scale_min;
    let range = max - min;
    let multiplier = scale_range / range;
    let offset = scale_min - min * multiplier;

    let mut i = 0;
    while i <= data.len() - 8 {
        _mm256_storeu_ps(
            data.get_unchecked_mut(i),
            _mm256_fmadd_ps(
                _mm256_set1_ps(multiplier),
                _mm256_loadu_ps(data.get_unchecked_mut(i)),
                _mm256_set1_ps(offset),
            ),
        );
        i += 8;
    }
    i = data.len() - (data.len() % 8);
    while i < data.len() {
        *data.get_unchecked_mut(i) = *data.get_unchecked(i) * multiplier + offset;
        i += 1;
    }
}

#[target_feature(enable = "avx2")]
unsafe fn get_2d_noise_helper(x: __m256, y: __m256, noise_type: NoiseType) -> M256Array {
    M256Array {
        simd: match noise_type {
            NoiseType::Fbm {
                freq,
                lacunarity,
                gain,
                octaves,
            } => fbm_2d(
                x,
                y,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Ridge {
                freq,
                lacunarity,
                gain,
                octaves,
            } => ridge_2d(
                x,
                y,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Turbulence {
                freq,
                lacunarity,
                gain,
                octaves,
            } => turbulence_2d(
                x,
                y,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_2d(
                _mm256_mul_ps(x, _mm256_set1_ps(freq)),
                _mm256_mul_ps(y, _mm256_set1_ps(freq)),
            ),
        },
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M256Array {
        simd: _mm256_set1_ps(f32::MAX),
    };
    let mut max_s = M256Array {
        simd: _mm256_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height);
    result.set_len(width * height);
    let mut y = _mm256_set1_ps(start_y);
    let mut i = 0;
    let remainder = width % 8;
    for _ in 0..height {
        let mut x = _mm256_set_ps(
            start_x + 7.0,
            start_x + 6.0,
            start_x + 5.0,
            start_x + 4.0,
            start_x + 3.0,
            start_x + 2.0,
            start_x + 1.0,
            start_x,
        );
        for _ in 0..width / 8 {
            let f = get_2d_noise_helper(x, y, noise_type).simd;
            max_s.simd = _mm256_max_ps(max_s.simd, f);
            min_s.simd = _mm256_min_ps(min_s.simd, f);
            _mm256_storeu_ps(result.get_unchecked_mut(i), f);
            i += 8;
            x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
        }
        if remainder != 0 {
            let f = get_2d_noise_helper(x, y, noise_type);
            for j in 0..remainder {
                let n = *f.array.get_unchecked(j);
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
        y = _mm256_add_ps(y, _mm256_set1_ps(1.0));
    }
    for i in 0..8 {
        if *min_s.array.get_unchecked(i) < min {
            min = *min_s.array.get_unchecked(i);
        }
        if *max_s.array.get_unchecked(i) > max {
            max = *max_s.array.get_unchecked(i);
        }
    }
    (result, min, max)
}

/// Gets a width X height sized block of scaled 2d noise
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_2d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(start_x, width, start_y, height, noise_type);
    scale_array(scale_min, scale_max, min, max, &mut noise);
    noise
}

#[target_feature(enable = "avx2")]
unsafe fn grad3d_simd(hash: __m256i, x: __m256, y: __m256, z: __m256) -> __m256 {
    let h = _mm256_and_si256(hash, _mm256_set1_epi32(15));

    let mut u = _mm256_castsi256_ps(_mm256_cmpgt_epi32(_mm256_set1_epi32(8), h));
    u = _mm256_blendv_ps(y, x, u);

    let mut v = _mm256_castsi256_ps(_mm256_cmpgt_epi32(_mm256_set1_epi32(4), h));
    let mut h12_or_14 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_or_si256(
            _mm256_cmpeq_epi32(h, _mm256_set1_epi32(12)),
            _mm256_cmpeq_epi32(h, _mm256_set1_epi32(14)),
        ),
    ));
    h12_or_14 = _mm256_blendv_ps(x, z, h12_or_14);
    v = _mm256_blendv_ps(h12_or_14, y, v);

    let h_and_1 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(1)),
    ));
    let h_and_2 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(2)),
    ));

    _mm256_add_ps(
        _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), u), u, h_and_1),
        _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), v), v, h_and_2),
    )
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_3d(x: __m256, y: __m256, z: __m256) -> __m256 {
    let s = _mm256_mul_ps(F3, _mm256_add_ps(x, _mm256_add_ps(y, z)));

    let ips = _mm256_floor_ps(_mm256_add_ps(x, s));
    let jps = _mm256_floor_ps(_mm256_add_ps(y, s));
    let kps = _mm256_floor_ps(_mm256_add_ps(z, s));

    let i = _mm256_cvtps_epi32(ips);
    let j = _mm256_cvtps_epi32(jps);
    let k = _mm256_cvtps_epi32(kps);

    let t = _mm256_mul_ps(
        _mm256_cvtepi32_ps(_mm256_add_epi32(i, _mm256_add_epi32(j, k))),
        G3,
    );

    let x0 = _mm256_sub_ps(x, _mm256_sub_ps(ips, t));
    let y0 = _mm256_sub_ps(y, _mm256_sub_ps(jps, t));
    let z0 = _mm256_sub_ps(z, _mm256_sub_ps(kps, t));

    let i1 = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_GE_OQ)),
    );
    let j1 = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(y0, x0, _CMP_GT_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GE_OQ)),
    );
    let k1 = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(z0, x0, _CMP_GT_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(z0, y0, _CMP_GT_OQ)),
    );

    //for i2
    let yx_xz = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_LT_OQ)),
    );
    let zx_xy = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_GE_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_LT_OQ)),
    );

    //for j2
    let xy_yz = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_LT_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_LT_OQ)),
    );
    let zy_yx = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GE_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ)),
    );

    //for k2
    let yz_zx = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_LT_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_GE_OQ)),
    );
    let xz_zy = _mm256_and_si256(
        _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_LT_OQ)),
        _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GE_OQ)),
    );

    let i2 = _mm256_or_si256(i1, _mm256_or_si256(yx_xz, zx_xy));
    let j2 = _mm256_or_si256(j1, _mm256_or_si256(xy_yz, zy_yx));
    let k2 = _mm256_or_si256(k1, _mm256_or_si256(yz_zx, xz_zy));

    let x1 = _mm256_add_ps(_mm256_add_ps(x0, _mm256_cvtepi32_ps(i1)), G3);
    let y1 = _mm256_add_ps(_mm256_add_ps(y0, _mm256_cvtepi32_ps(j1)), G3);
    let z1 = _mm256_add_ps(_mm256_add_ps(z0, _mm256_cvtepi32_ps(k1)), G3);
    let x2 = _mm256_add_ps(_mm256_add_ps(x0, _mm256_cvtepi32_ps(i2)), F3);
    let y2 = _mm256_add_ps(_mm256_add_ps(y0, _mm256_cvtepi32_ps(j2)), F3);
    let z2 = _mm256_add_ps(_mm256_add_ps(z0, _mm256_cvtepi32_ps(k2)), F3);
    let x3 = _mm256_add_ps(_mm256_add_ps(x0, _mm256_set1_ps(-1.0)), POINT_FIVE);
    let y3 = _mm256_add_ps(_mm256_add_ps(y0, _mm256_set1_ps(-1.0)), POINT_FIVE);
    let z3 = _mm256_add_ps(_mm256_add_ps(z0, _mm256_set1_ps(-1.0)), POINT_FIVE);

    // Wrap indices at 256 so it will fit in the PERM array
    let ii = _mm256_and_si256(i, _mm256_set1_epi32(0xff));
    let jj = _mm256_and_si256(j, _mm256_set1_epi32(0xff));
    let kk = _mm256_and_si256(k, _mm256_set1_epi32(0xff));

    let gi0 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            ii,
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(jj, _mm256_i32gather_epi32(&PERM as *const i32, kk, 4)),
                4,
            ),
        ),
        4,
    );
    let gi1 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_sub_epi32(ii, i1),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(
                    _mm256_sub_epi32(jj, j1),
                    _mm256_i32gather_epi32(&PERM as *const i32, _mm256_sub_epi32(kk, k1), 4),
                ),
                4,
            ),
        ),
        4,
    );
    let gi2 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_sub_epi32(ii, i2),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(
                    _mm256_sub_epi32(jj, j2),
                    _mm256_i32gather_epi32(&PERM as *const i32, _mm256_sub_epi32(kk, k2), 4),
                ),
                4,
            ),
        ),
        4,
    );
    let gi3 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_sub_epi32(ii, _mm256_set1_epi32(-1)),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(
                    _mm256_sub_epi32(jj, _mm256_set1_epi32(-1)),
                    _mm256_i32gather_epi32(
                        &PERM as *const i32,
                        _mm256_sub_epi32(kk, _mm256_set1_epi32(-1)),
                        4,
                    ),
                ),
                4,
            ),
        ),
        4,
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y - z*z
    let t0 = _mm256_fnmadd_ps(
        z0,
        z0,
        _mm256_fnmadd_ps(y0, y0, _mm256_fnmadd_ps(x0, x0, POINT_FIVE)),
    );
    let t1 = _mm256_fnmadd_ps(
        z1,
        z1,
        _mm256_fnmadd_ps(y1, y1, _mm256_fnmadd_ps(x1, x1, POINT_FIVE)),
    );
    let t2 = _mm256_fnmadd_ps(
        z2,
        z2,
        _mm256_fnmadd_ps(y2, y2, _mm256_fnmadd_ps(x2, x2, POINT_FIVE)),
    );
    let t3 = _mm256_fnmadd_ps(
        z3,
        z3,
        _mm256_fnmadd_ps(y3, y3, _mm256_fnmadd_ps(x3, x3, POINT_FIVE)),
    );

    //ti*ti*ti*ti
    let mut t0q = _mm256_mul_ps(t0, t0);
    t0q = _mm256_mul_ps(t0q, t0q);
    let mut t1q = _mm256_mul_ps(t1, t1);
    t1q = _mm256_mul_ps(t1q, t1q);
    let mut t2q = _mm256_mul_ps(t2, t2);
    t2q = _mm256_mul_ps(t2q, t2q);
    let mut t3q = _mm256_mul_ps(t3, t3);
    t3q = _mm256_mul_ps(t3q, t3q);

    let mut n0 = _mm256_mul_ps(t0q, grad3d_simd(gi0, x0, y0, z0));
    let mut n1 = _mm256_mul_ps(t1q, grad3d_simd(gi1, x1, y1, z1));
    let mut n2 = _mm256_mul_ps(t2q, grad3d_simd(gi2, x2, y2, z2));
    let mut n3 = _mm256_mul_ps(t3q, grad3d_simd(gi3, x3, y3, z3));

    //if ti < 0 then 0 else ni
    let mut cond = _mm256_cmp_ps(t0, _mm256_setzero_ps(), _CMP_LT_OQ);
    n0 = _mm256_andnot_ps(cond, n0);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(), _CMP_LT_OQ);
    n1 = _mm256_andnot_ps(cond, n1);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(), _CMP_LT_OQ);
    n2 = _mm256_andnot_ps(cond, n2);
    cond = _mm256_cmp_ps(t3, _mm256_setzero_ps(), _CMP_LT_OQ);
    n3 = _mm256_andnot_ps(cond, n3);

    _mm256_add_ps(n0, _mm256_add_ps(n1, _mm256_add_ps(n2, n3)))
}

/// Get a single value of 3d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut zf = _mm256_mul_ps(z, freq);
    let mut result = simplex_3d(xf, yf, zf);
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        zf = _mm256_mul_ps(zf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_fmadd_ps(simplex_3d(xf, yf, zf), amp, result);
    }

    result
}

/// Get a single value of 3d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut zf = _mm256_mul_ps(z, freq);
    let mut result = _mm256_sub_ps(_mm256_set1_ps(1.0), _mm256_abs_ps(simplex_3d(xf, yf, zf)));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        zf = _mm256_mul_ps(zf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_fnmadd_ps(
                _mm256_abs_ps(simplex_3d(xf, yf, zf)),
                amp,
                _mm256_set1_ps(1.0),
            ),
        );
    }

    result
}

/// Get a single value of 3d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_3d(
    x: __m256,
    y: __m256,
    z: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut zf = _mm256_mul_ps(z, freq);
    let mut result = _mm256_abs_ps(simplex_3d(xf, yf, zf));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        zf = _mm256_mul_ps(zf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_abs_ps(_mm256_mul_ps(simplex_3d(xf, yf, zf), amp)),
        );
    }

    result
}

#[target_feature(enable = "avx2")]
unsafe fn get_3d_noise_helper(x: __m256, y: __m256, z: __m256, noise_type: NoiseType) -> M256Array {
    M256Array {
        simd: match noise_type {
            NoiseType::Fbm {
                freq,
                lacunarity,
                gain,
                octaves,
            } => fbm_3d(
                x,
                y,
                z,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Ridge {
                freq,
                lacunarity,
                gain,
                octaves,
            } => ridge_3d(
                x,
                y,
                z,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Turbulence {
                freq,
                lacunarity,
                gain,
                octaves,
            } => turbulence_3d(
                x,
                y,
                z,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_3d(
                _mm256_mul_ps(x, _mm256_set1_ps(freq)),
                _mm256_mul_ps(y, _mm256_set1_ps(freq)),
                _mm256_mul_ps(z, _mm256_set1_ps(freq)),
            ),
        },
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M256Array {
        simd: _mm256_set1_ps(f32::MAX),
    };
    let mut max_s = M256Array {
        simd: _mm256_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height * depth);
    result.set_len(width * height * depth);
    let mut i = 0;
    let remainder = width % 8;
    let mut z = _mm256_set1_ps(start_z);
    for _ in 0..depth {
        let mut y = _mm256_set1_ps(start_y);
        for _ in 0..height {
            let mut x = _mm256_set_ps(
                start_x + 7.0,
                start_x + 6.0,
                start_x + 5.0,
                start_x + 4.0,
                start_x + 3.0,
                start_x + 2.0,
                start_x + 1.0,
                start_x,
            );
            for _ in 0..width / 8 {
                let f = get_3d_noise_helper(x, y, z, noise_type).simd;
                max_s.simd = _mm256_max_ps(max_s.simd, f);
                min_s.simd = _mm256_min_ps(min_s.simd, f);
                _mm256_storeu_ps(result.get_unchecked_mut(i), f);
                i += 8;
                x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
            }
            if remainder != 0 {
                let f = get_3d_noise_helper(x, y, z, noise_type);
                for j in 0..remainder {
                    let n = *f.array.get_unchecked(j);
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
            y = _mm256_add_ps(y, _mm256_set1_ps(1.0));
        }
        z = _mm256_add_ps(z, _mm256_set1_ps(1.0));
    }
    for i in 0..8 {
        if *min_s.array.get_unchecked(i) < min {
            min = *min_s.array.get_unchecked(i);
        }
        if *max_s.array.get_unchecked(i) > max {
            max = *max_s.array.get_unchecked(i);
        }
    }
    (result, min, max)
}

/// Gets a width X height X depth sized block of scaled 3d noise
/// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_3d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) =
        get_3d_noise(start_x, width, start_y, height, start_z, depth, noise_type);
    scale_array(scale_min, scale_max, min, max, &mut noise);
    noise
}
#[target_feature(enable = "avx2")]
unsafe fn grad4_simd(hash: __m256i, x: __m256, y: __m256, z: __m256, t: __m256) -> __m256 {
    let h = _mm256_and_si256(hash, _mm256_set1_epi32(31));
    let mut mask = _mm256_castsi256_ps(_mm256_cmpgt_epi32(_mm256_set1_epi32(24), h));
    let u = _mm256_blendv_ps(y, x, mask);
    mask = _mm256_castsi256_ps(_mm256_cmpgt_epi32(_mm256_set1_epi32(16), h));
    let v = _mm256_blendv_ps(z, y, mask);
    mask = _mm256_castsi256_ps(_mm256_cmpgt_epi32(_mm256_set1_epi32(8), h));
    let w = _mm256_blendv_ps(t, z, mask);

    let h_and_1 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(1)),
    ));
    let h_and_2 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(2)),
    ));
    let h_and_4 = _mm256_castsi256_ps(_mm256_cmpeq_epi32(
        _mm256_setzero_si256(),
        _mm256_and_si256(h, _mm256_set1_epi32(4)),
    ));

    _mm256_add_ps(
        _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), u), u, h_and_1),
        _mm256_add_ps(
            _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), v), v, h_and_2),
            _mm256_blendv_ps(_mm256_sub_ps(_mm256_setzero_ps(), w), w, h_and_4),
        ),
    )
}
/// Get a single value of 4d simplex noise, results
/// are not scaled.
#[target_feature(enable = "avx2")]
pub unsafe fn simplex_4d(x: __m256, y: __m256, z: __m256, w: __m256) -> __m256 {
    let s = _mm256_mul_ps(F4, _mm256_add_ps(x, _mm256_add_ps(y, _mm256_add_ps(z, w))));

    let ips = _mm256_floor_ps(_mm256_add_ps(x, s));
    let jps = _mm256_floor_ps(_mm256_add_ps(y, s));
    let kps = _mm256_floor_ps(_mm256_add_ps(z, s));
    let lps = _mm256_floor_ps(_mm256_add_ps(w, s));

    let i = _mm256_cvtps_epi32(ips);
    let j = _mm256_cvtps_epi32(jps);
    let k = _mm256_cvtps_epi32(kps);
    let l = _mm256_cvtps_epi32(lps);

    let t = _mm256_mul_ps(
        _mm256_cvtepi32_ps(_mm256_add_epi32(
            i,
            _mm256_add_epi32(j, _mm256_add_epi32(k, l)),
        )),
        G4,
    );
    let x0 = _mm256_sub_ps(x, _mm256_sub_ps(ips, t));
    let y0 = _mm256_sub_ps(y, _mm256_sub_ps(jps, t));
    let z0 = _mm256_sub_ps(z, _mm256_sub_ps(kps, t));
    let w0 = _mm256_sub_ps(w, _mm256_sub_ps(lps, t));

    let cond = _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GT_OQ));
    let c1 = _mm256_and_si256(cond, _mm256_set1_epi32(32));
    let cond = _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_GT_OQ));
    let c2 = _mm256_and_si256(cond, _mm256_set1_epi32(16));
    let cond = _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GT_OQ));
    let c3 = _mm256_and_si256(cond, _mm256_set1_epi32(8));
    let cond = _mm256_castps_si256(_mm256_cmp_ps(x0, w0, _CMP_GT_OQ));
    let c4 = _mm256_and_si256(cond, _mm256_set1_epi32(4));
    let cond = _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GT_OQ));
    let c5 = _mm256_and_si256(cond, _mm256_set1_epi32(2));
    let cond = _mm256_castps_si256(_mm256_cmp_ps(z0, z0, _CMP_GT_OQ));
    let c6 = _mm256_and_si256(cond, _mm256_set1_epi32(1));
    let c = _mm256_or_si256(
        c1,
        _mm256_or_si256(
            c2,
            _mm256_or_si256(c3, _mm256_or_si256(c4, _mm256_or_si256(c5, c6))),
        ),
    );

    let sc = _mm256_i32gather_epi32(&SIMPLEX0 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(2));
    let i1 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX1 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(2));
    let j1 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX2 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(2));
    let k1 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX3 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(2));
    let l1 = _mm256_and_si256(cond, _mm256_set1_epi32(1));
    //---
    let sc = _mm256_i32gather_epi32(&SIMPLEX0 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(1));
    let i2 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX1 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(1));
    let j2 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX2 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(1));
    let k2 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX3 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(1));
    let l2 = _mm256_and_si256(cond, _mm256_set1_epi32(1));
    //--
    let sc = _mm256_i32gather_epi32(&SIMPLEX0 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(0));
    let i3 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX1 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(0));
    let j3 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX2 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(0));
    let k3 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let sc = _mm256_i32gather_epi32(&SIMPLEX3 as *const i32, c, 4);
    let cond = _mm256_cmpgt_epi32(sc, _mm256_set1_epi32(0));
    let l3 = _mm256_and_si256(cond, _mm256_set1_epi32(1));

    let x1 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i1)), G4);
    let y1 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j1)), G4);
    let z1 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k1)), G4);
    let w1 = _mm256_add_ps(_mm256_sub_ps(w0, _mm256_cvtepi32_ps(l1)), G4);
    let x2 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i2)), G24);
    let y2 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j2)), G24);
    let z2 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k2)), G24);
    let w2 = _mm256_add_ps(_mm256_sub_ps(w0, _mm256_cvtepi32_ps(l2)), G24);
    let x3 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i3)), G34);
    let y3 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j3)), G34);
    let z3 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k3)), G34);
    let w3 = _mm256_add_ps(_mm256_sub_ps(w0, _mm256_cvtepi32_ps(l3)), G34);
    let x4 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_set1_ps(1.0)), G44);
    let y4 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_set1_ps(1.0)), G44);
    let z4 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_set1_ps(1.0)), G44);
    let w4 = _mm256_add_ps(_mm256_sub_ps(w0, _mm256_set1_ps(1.0)), G44);

    let ii = _mm256_and_si256(i, _mm256_set1_epi32(0xff));
    let jj = _mm256_and_si256(j, _mm256_set1_epi32(0xff));
    let kk = _mm256_and_si256(k, _mm256_set1_epi32(0xff));
    let ll = _mm256_and_si256(l, _mm256_set1_epi32(0xff));

    let lp = _mm256_i32gather_epi32(&PERM as *const i32, ll, 4);
    let kp = _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(kk, lp), 4);
    let jp = _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(jj, kp), 4);
    let gi0 = _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(ii, jp), 4);

    let lp = _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(ll, l1), 4);
    let kp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(kk, k1), lp),
        4,
    );
    let jp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(jj, j1), kp),
        4,
    );
    let gi1 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(ii, i1), jp),
        4,
    );

    let lp = _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(ll, l2), 4);
    let kp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(kk, k2), lp),
        4,
    );
    let jp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(jj, j2), kp),
        4,
    );
    let gi2 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(ii, i2), jp),
        4,
    );

    let lp = _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(ll, l3), 4);
    let kp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(kk, k3), lp),
        4,
    );
    let jp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(jj, j3), kp),
        4,
    );
    let gi3 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(ii, i3), jp),
        4,
    );

    let lp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(ll, _mm256_set1_epi32(1)),
        4,
    );
    let kp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(kk, _mm256_set1_epi32(1)), lp),
        4,
    );
    let jp = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(jj, _mm256_set1_epi32(1)), kp),
        4,
    );
    let gi4 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(ii, _mm256_set1_epi32(1)), jp),
        4,
    );

    let t0 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(
                _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x0, x0)),
                _mm256_mul_ps(y0, y0),
            ),
            _mm256_mul_ps(z0, z0),
        ),
        _mm256_mul_ps(w0, w0),
    );
    let t1 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(
                _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x1, x1)),
                _mm256_mul_ps(y1, y1),
            ),
            _mm256_mul_ps(z1, z1),
        ),
        _mm256_mul_ps(w1, w1),
    );
    let t2 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(
                _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x2, x2)),
                _mm256_mul_ps(y2, y2),
            ),
            _mm256_mul_ps(z2, z2),
        ),
        _mm256_mul_ps(w2, w2),
    );
    let t3 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(
                _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x3, x3)),
                _mm256_mul_ps(y3, y3),
            ),
            _mm256_mul_ps(z3, z3),
        ),
        _mm256_mul_ps(w3, w3),
    );
    let t4 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(
                _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x4, x4)),
                _mm256_mul_ps(y4, y4),
            ),
            _mm256_mul_ps(z4, z4),
        ),
        _mm256_mul_ps(w4, w4),
    );
    //ti*ti*ti*ti
    let mut t0q = _mm256_mul_ps(t0, t0);
    t0q = _mm256_mul_ps(t0q, t0q);
    let mut t1q = _mm256_mul_ps(t1, t1);
    t1q = _mm256_mul_ps(t1q, t1q);
    let mut t2q = _mm256_mul_ps(t2, t2);
    t2q = _mm256_mul_ps(t2q, t2q);
    let mut t3q = _mm256_mul_ps(t3, t3);
    t3q = _mm256_mul_ps(t3q, t3q);
    let mut t4q = _mm256_mul_ps(t4, t4);
    t4q = _mm256_mul_ps(t4q, t4q);

    let mut n0 = _mm256_mul_ps(t0q, grad4_simd(gi0, x0, y0, z0, w0));
    let mut n1 = _mm256_mul_ps(t1q, grad4_simd(gi1, x1, y1, z1, w1));
    let mut n2 = _mm256_mul_ps(t2q, grad4_simd(gi2, x2, y2, z2, w2));
    let mut n3 = _mm256_mul_ps(t3q, grad4_simd(gi3, x3, y3, z3, w3));
    let mut n4 = _mm256_mul_ps(t4q, grad4_simd(gi4, x4, y4, z4, w4));

    //if ti < 0 then 0 else ni
    let mut cond = _mm256_cmp_ps(t0, _mm256_setzero_ps(), _CMP_LT_OQ);
    n0 = _mm256_andnot_ps(cond, n0);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(), _CMP_LT_OQ);
    n1 = _mm256_andnot_ps(cond, n1);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(), _CMP_LT_OQ);
    n2 = _mm256_andnot_ps(cond, n2);
    cond = _mm256_cmp_ps(t3, _mm256_setzero_ps(), _CMP_LT_OQ);
    n3 = _mm256_andnot_ps(cond, n3);
    cond = _mm256_cmp_ps(t4, _mm256_setzero_ps(), _CMP_LT_OQ);
    n4 = _mm256_andnot_ps(cond, n4);

    _mm256_add_ps(
        n0,
        _mm256_add_ps(n1, _mm256_add_ps(n2, _mm256_add_ps(n3, n4))),
    )
}
/// Get a single value of 4d fractal brownian motion.
#[target_feature(enable = "avx2")]
pub unsafe fn fbm_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut zf = _mm256_mul_ps(z, freq);
    let mut wf = _mm256_mul_ps(w, freq);
    let mut result = simplex_4d(xf, yf, zf, wf);
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        zf = _mm256_mul_ps(zf, lac);
        wf = _mm256_mul_ps(wf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(result, _mm256_mul_ps(simplex_4d(xf, yf, zf, wf), amp));
    }

    result
}

/// Get a single value of 4d ridge noise.
#[target_feature(enable = "avx2")]
pub unsafe fn ridge_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut zf = _mm256_mul_ps(z, freq);
    let mut wf = _mm256_mul_ps(w, freq);
    let mut result = _mm256_sub_ps(
        _mm256_set1_ps(1.0),
        _mm256_abs_ps(simplex_4d(xf, yf, zf, wf)),
    );
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        zf = _mm256_mul_ps(zf, lac);
        wf = _mm256_mul_ps(wf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_sub_ps(
                _mm256_set1_ps(1.0),
                _mm256_abs_ps(_mm256_mul_ps(simplex_4d(xf, yf, zf, wf), amp)),
            ),
        );
    }

    result
}

/// Get a single value of 4d turbulence.
#[target_feature(enable = "avx2")]
pub unsafe fn turbulence_4d(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: u8,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut zf = _mm256_mul_ps(z, freq);
    let mut wf = _mm256_mul_ps(w, freq);
    let mut result = _mm256_abs_ps(simplex_4d(xf, yf, zf, wf));
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        zf = _mm256_mul_ps(zf, lac);
        wf = _mm256_mul_ps(wf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(
            result,
            _mm256_abs_ps(_mm256_mul_ps(simplex_4d(xf, yf, zf, wf), amp)),
        );
    }

    result
}

#[target_feature(enable = "avx2")]
unsafe fn get_4d_noise_helper(
    x: __m256,
    y: __m256,
    z: __m256,
    w: __m256,
    noise_type: NoiseType,
) -> M256Array {
    M256Array {
        simd: match noise_type {
            NoiseType::Fbm {
                freq,
                lacunarity,
                gain,
                octaves,
            } => fbm_4d(
                x,
                y,
                z,
                w,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Ridge {
                freq,
                lacunarity,
                gain,
                octaves,
            } => ridge_4d(
                x,
                y,
                z,
                w,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Turbulence {
                freq,
                lacunarity,
                gain,
                octaves,
            } => turbulence_4d(
                x,
                y,
                z,
                w,
                _mm256_set1_ps(freq),
                _mm256_set1_ps(lacunarity),
                _mm256_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_4d(
                _mm256_mul_ps(x, _mm256_set1_ps(freq)),
                _mm256_mul_ps(y, _mm256_set1_ps(freq)),
                _mm256_mul_ps(z, _mm256_set1_ps(freq)),
                _mm256_mul_ps(w, _mm256_set1_ps(freq)),
            ),
        },
    }
}
/// Gets a width X height X depth x time sized block of 4d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "avx2")]
pub unsafe fn get_4d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M256Array {
        simd: _mm256_set1_ps(f32::MAX),
    };
    let mut max_s = M256Array {
        simd: _mm256_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height * depth * time);
    result.set_len(width * height * depth * time);
    let mut i = 0;
    let remainder = width % 8;
    let mut w = _mm256_set1_ps(start_w);
    for _ in 0..time {
        let mut z = _mm256_set1_ps(start_z);
        for _ in 0..depth {
            let mut y = _mm256_set1_ps(start_y);
            for _ in 0..height {
                let mut x = _mm256_set_ps(
                    start_x + 7.0,
                    start_x + 6.0,
                    start_x + 5.0,
                    start_x + 4.0,
                    start_x + 3.0,
                    start_x + 2.0,
                    start_x + 1.0,
                    start_x,
                );
                for _ in 0..width / 8 {
                    let f = get_4d_noise_helper(x, y, z, w, noise_type).simd;
                    max_s.simd = _mm256_max_ps(max_s.simd, f);
                    min_s.simd = _mm256_min_ps(min_s.simd, f);
                    _mm256_storeu_ps(result.get_unchecked_mut(i), f);
                    i += 8;
                    x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
                }
                if remainder != 0 {
                    let f = get_4d_noise_helper(x, y, z, w, noise_type);
                    for j in 0..remainder {
                        let n = f.array[j];
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
                y = _mm256_add_ps(y, _mm256_set1_ps(1.0));
            }
            z = _mm256_add_ps(z, _mm256_set1_ps(1.0));
        }
        w = _mm256_add_ps(w, _mm256_set1_ps(1.0));
    }
    for i in 0..8 {
        if *min_s.array.get_unchecked(i) < min {
            min = *min_s.array.get_unchecked(i);
        }
        if *max_s.array.get_unchecked(i) > max {
            max = *max_s.array.get_unchecked(i);
        }
    }
    (result, min, max)
}

/// Gets a width X height X depth X time sized block of scaled 4d noise
/// `start_*` can be used to provide an offset in the
/// coordinates.
/// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
#[target_feature(enable = "avx2")]
pub unsafe fn get_4d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    start_w: f32,
    time: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_4d_noise(
        start_x, width, start_y, height, start_z, depth, start_w, time, noise_type,
    );
    scale_array(scale_min, scale_max, min, max, &mut noise);
    noise
}

pub const SIMPLEX0: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 2, 2,
    1, 1, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 3, 3, 0, 3, 2, 0, 0, 0, 3, 0, 3, 3,
];

pub const SIMPLEX1: [i32; 64] = [
    1, 1, 0, 2, 0, 0, 0, 2, 2, 0, 3, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 3, 0, 0, 0, 3, 3,
    0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 2, 2,
];

pub const SIMPLEX2: [i32; 64] = [
    2, 3, 0, 3, 0, 0, 0, 3, 1, 0, 1, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    2, 3, 0, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1,
];

pub const SIMPLEX3: [i32; 64] = [
    3, 2, 0, 1, 0, 0, 0, 0, 3, 0, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 2, 0, 0, 0, 1, 0,
    3, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 2, 1, 0, 0, 3, 0, 0, 0, 2, 0, 1, 0,
];
