//! SSE2 Accelerated noise functions.
//! All 64bit intel/amd CPUs support this.
//! *save for one obscure server cpu*
//!
//! Use `is_x86_feature_detected!("sse2")` provided
//! by the Rust stanard library to detect at runtime.
//!
//! When using the `get_` functions, you will get a performance boost when width
//! is evenly divisble by 4, and when it is not small relative height and depth.

use super::*;
use shared::*;
use shared_sse::*;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
use std::f32;

// SSE2 has no blendv_sse2 instruction, so we do it by hand
#[target_feature(enable = "sse2")]
unsafe fn blendv_sse2(a: __m128, b: __m128, mask: __m128) -> __m128 {
    _mm_or_ps(_mm_andnot_ps(mask, a), _mm_and_ps(mask, b))
}

#[target_feature(enable = "sse2")]
unsafe fn floor_sse2(f: __m128) -> __m128 {
    let t = _mm_cvtepi32_ps(_mm_cvttps_epi32(f));
    _mm_sub_ps(t, _mm_and_ps(_mm_cmplt_ps(f, t), _mm_set1_ps(1.0)))
}

#[target_feature(enable = "sse2")]
unsafe fn grad1_simd(hash: __m128i, x: __m128) -> __m128 {
    let h = _mm_and_si128(hash, _mm_set1_epi32(15));
    let v = _mm_cvtepi32_ps(_mm_and_si128(h, _mm_set1_epi32(7)));

    let h_and_8 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(8)),
    ));
    let grad = blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), v), v, h_and_8);
    _mm_mul_ps(grad, x)
}

/// Get a single value of 1d simplex noise, results
/// are not scaled.
#[target_feature(enable = "sse2")]
unsafe fn simplex_1d(x: __m128) -> __m128 {
    let ips = floor_sse2(x);
    let mut i0 = M128iArray {
        simd: _mm_cvtps_epi32(ips),
    };
    let i1 = M128iArray {
        simd: _mm_and_si128(
            _mm_add_epi32(i0.simd, _mm_set1_epi32(1)),
            _mm_set1_epi32(0xff),
        ),
    };
    let x0 = _mm_sub_ps(x, ips);
    let x1 = _mm_sub_ps(x0, _mm_set1_ps(1.0));

    i0.simd = _mm_and_si128(i0.simd, _mm_set1_epi32(0xff));

    let gi0 = M128iArray {
        array: [
            PERM[i0.array[0] as usize],
            PERM[i0.array[1] as usize],
            PERM[i0.array[2] as usize],
            PERM[i0.array[3] as usize],
        ],
    };

    let gi1 = M128iArray {
        array: [
            PERM[i1.array[0] as usize],
            PERM[i1.array[1] as usize],
            PERM[i1.array[2] as usize],
            PERM[i1.array[3] as usize],
        ],
    };

    let mut t0 = _mm_sub_ps(_mm_set1_ps(1.0), _mm_mul_ps(x0, x0));
    t0 = _mm_mul_ps(t0, t0);
    t0 = _mm_mul_ps(t0, t0);
    let n0 = _mm_mul_ps(t0, grad1_simd(gi0.simd, x0));

    let mut t1 = _mm_sub_ps(_mm_set1_ps(1.0), _mm_mul_ps(x1, x1));
    t1 = _mm_mul_ps(t1, t1);
    t1 = _mm_mul_ps(t1, t1);
    let n1 = _mm_mul_ps(t1, grad1_simd(gi1.simd, x1));

    _mm_add_ps(n0, n1)
}
/// Get a single value of 1d fractal brownian motion.
#[target_feature(enable = "sse2")]
pub unsafe fn fbm_1d(
    x: __m128,
    freq: __m128,
    lacunarity: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut result = simplex_1d(xf);
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lacunarity);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_mul_ps(simplex_1d(xf), amp));
    }

    result
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "sse2")]
pub unsafe fn ridge_1d(
    x: __m128,
    freq: __m128,
    lacunarity: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut result = _mm_sub_ps(_mm_set1_ps(1.0), _mm_abs_ps(simplex_1d(xf)));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lacunarity);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(
            result,
            _mm_sub_ps(
                _mm_set1_ps(1.0),
                _mm_abs_ps(_mm_mul_ps(simplex_1d(xf), amp)),
            ),
        );
    }

    result
}

/// Get a single value of 2d turbulence.
#[target_feature(enable = "sse2")]
pub unsafe fn turbulence_1d(
    x: __m128,
    freq: __m128,
    lacunarity: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut result = _mm_abs_ps(simplex_1d(xf));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lacunarity);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_abs_ps(_mm_mul_ps(simplex_1d(xf), amp)));
    }

    result
}

#[target_feature(enable = "sse2")]
unsafe fn get_1d_noise_helper(x: __m128, noise_type: NoiseType) -> M128Array {
    M128Array {
        simd: match noise_type {
            NoiseType::Fbm {
                freq,
                lacunarity,
                gain,
                octaves,
            } => fbm_1d(
                x,
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
                octaves,
            ),
            NoiseType::Ridge {
                freq,
                lacunarity,
                gain,
                octaves,
            } => ridge_1d(
                x,
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
                octaves,
            ),
            NoiseType::Turbulence {
                freq,
                lacunarity,
                gain,
                octaves,
            } => turbulence_1d(
                x,
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_1d(_mm_mul_ps(x, _mm_set1_ps(freq))),
        },
    }
}

/// Gets a width sized block of 1d noise, unscaled.
/// `start_x` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "sse2")]
pub unsafe fn get_1d_noise(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M128Array {
        simd: _mm_set1_ps(f32::MAX),
    };
    let mut max_s = M128Array {
        simd: _mm_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width);
    result.set_len(width);
    let mut i = 0;
    let remainder = width % 4;
    let mut x = _mm_set_ps(start_x + 3.0, start_x + 2.0, start_x + 1.0, start_x);
    for _ in 0..width / 4 {
        let f = get_1d_noise_helper(x, noise_type).simd;
        max_s.simd = _mm_max_ps(max_s.simd, f);
        min_s.simd = _mm_min_ps(min_s.simd, f);
        _mm_storeu_ps(result.get_unchecked_mut(i), f);
        i += 4;
        x = _mm_add_ps(x, _mm_set1_ps(4.0));
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
    for i in 0..4 {
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
#[target_feature(enable = "sse2")]
pub unsafe fn get_1d_scaled_noise(
    start_x: f32,
    width: usize,
    noise_type: NoiseType,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_1d_noise(start_x, width, noise_type);
    scale_array_sse(scale_min, scale_max, min, max, &mut noise);
    noise
}

#[target_feature(enable = "sse2")]
unsafe fn grad2_simd(hash: __m128i, x: __m128, y: __m128) -> __m128 {
    let h = _mm_and_si128(hash, _mm_set1_epi32(7));
    let mask = _mm_castsi128_ps(_mm_cmplt_epi32(h, _mm_set1_epi32(4)));
    let u = blendv_sse2(y, x, mask);
    let v = _mm_mul_ps(_mm_set1_ps(2.0), blendv_sse2(x, y, mask));

    let h_and_1 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(1)),
    ));
    let h_and_2 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(2)),
    ));

    _mm_add_ps(
        blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), u), u, h_and_1),
        blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), v), v, h_and_2),
    )
}

/// Get a single value of 2d simplex noise, results
/// are not scaled.
#[target_feature(enable = "sse2")]
pub unsafe fn simplex_2d(x: __m128, y: __m128) -> __m128 {
    let s = _mm_mul_ps(F2, _mm_add_ps(x, y));

    let ips = floor_sse2(_mm_add_ps(x, s));
    let jps = floor_sse2(_mm_add_ps(y, s));

    let i = _mm_cvtps_epi32(ips);
    let j = _mm_cvtps_epi32(jps);

    let t = _mm_mul_ps(_mm_cvtepi32_ps(_mm_add_epi32(i, j)), G2);

    let x0 = _mm_sub_ps(x, _mm_sub_ps(ips, t));
    let y0 = _mm_sub_ps(y, _mm_sub_ps(jps, t));

    let i1 = M128iArray {
        simd: _mm_castps_si128(_mm_cmpge_ps(x0, y0)),
    };
    let j1 = M128iArray {
        simd: _mm_castps_si128(_mm_cmpgt_ps(y0, x0)),
    };

    let x1 = _mm_add_ps(_mm_add_ps(x0, _mm_cvtepi32_ps(i1.simd)), G2);
    let y1 = _mm_add_ps(_mm_add_ps(y0, _mm_cvtepi32_ps(j1.simd)), G2);
    let x2 = _mm_add_ps(_mm_add_ps(x0, _mm_set1_ps(-1.0)), G22);
    let y2 = _mm_add_ps(_mm_add_ps(y0, _mm_set1_ps(-1.0)), G22);

    let ii = M128iArray {
        simd: _mm_and_si128(i, _mm_set1_epi32(0xff)),
    };
    let jj = M128iArray {
        simd: _mm_and_si128(j, _mm_set1_epi32(0xff)),
    };

    let gi0 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0)
                    + *PERM.get_unchecked(*jj.array.get_unchecked(0) as usize))
                    as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1)
                    + *PERM.get_unchecked(*jj.array.get_unchecked(1) as usize))
                    as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2)
                    + *PERM.get_unchecked(*jj.array.get_unchecked(2) as usize))
                    as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3)
                    + *PERM.get_unchecked(*jj.array.get_unchecked(3) as usize))
                    as usize,
            ),
        ],
    };
    let gi1 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0) - *i1.array.get_unchecked(0)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(0) - *j1.array.get_unchecked(0)) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1) - *i1.array.get_unchecked(1)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(1) - *j1.array.get_unchecked(1)) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2) - *i1.array.get_unchecked(2)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(2) - *j1.array.get_unchecked(2)) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3) - *i1.array.get_unchecked(3)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(3) - *j1.array.get_unchecked(3)) as usize,
                    )) as usize,
            ),
        ],
    };
    let gi2 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0)
                    + 1
                    + *PERM.get_unchecked((*jj.array.get_unchecked(0) + 1) as usize))
                    as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1)
                    + 1
                    + *PERM.get_unchecked((*jj.array.get_unchecked(1) + 1) as usize))
                    as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2)
                    + 1
                    + *PERM.get_unchecked((*jj.array.get_unchecked(2) + 1) as usize))
                    as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3)
                    + 1
                    + *PERM.get_unchecked((*jj.array.get_unchecked(3) + 1) as usize))
                    as usize,
            ),
        ],
    };

    let t0 = _mm_sub_ps(
        _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x0, x0)),
        _mm_mul_ps(y0, y0),
    );
    let t1 = _mm_sub_ps(
        _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x1, x1)),
        _mm_mul_ps(y1, y1),
    );
    let t2 = _mm_sub_ps(
        _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x2, x2)),
        _mm_mul_ps(y2, y2),
    );

    let mut t0q = _mm_mul_ps(t0, t0);
    t0q = _mm_mul_ps(t0q, t0q);
    let mut t1q = _mm_mul_ps(t1, t1);
    t1q = _mm_mul_ps(t1q, t1q);
    let mut t2q = _mm_mul_ps(t2, t2);
    t2q = _mm_mul_ps(t2q, t2q);

    let mut n0 = _mm_mul_ps(t0q, grad2_simd(gi0.simd, x0, y0));
    let mut n1 = _mm_mul_ps(t1q, grad2_simd(gi1.simd, x1, y1));
    let mut n2 = _mm_mul_ps(t2q, grad2_simd(gi2.simd, x2, y2));

    let mut cond = _mm_cmplt_ps(t0, _mm_setzero_ps());
    n0 = _mm_andnot_ps(cond, n0);
    cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
    n1 = _mm_andnot_ps(cond, n1);
    cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_andnot_ps(cond, n2);

    _mm_add_ps(n0, _mm_add_ps(n1, n2))
}

/// Get a single value of 2d fractal brownian motion.
#[target_feature(enable = "sse2")]
pub unsafe fn fbm_2d(
    x: __m128,
    y: __m128,
    freq: __m128,
    lacunarity: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut result = simplex_2d(xf, yf);
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lacunarity);
        yf = _mm_mul_ps(yf, lacunarity);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_mul_ps(simplex_2d(xf, yf), amp));
    }

    result
}

/// Get a single value of 2d ridge noise.
#[target_feature(enable = "sse2")]
pub unsafe fn ridge_2d(
    x: __m128,
    y: __m128,
    freq: __m128,
    lacunarity: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut result = _mm_sub_ps(_mm_set1_ps(1.0), _mm_abs_ps(simplex_2d(xf, yf)));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lacunarity);
        yf = _mm_mul_ps(yf, lacunarity);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(
            result,
            _mm_sub_ps(
                _mm_set1_ps(1.0),
                _mm_abs_ps(_mm_mul_ps(simplex_2d(xf, yf), amp)),
            ),
        );
    }

    result
}

/// Get a single value of 2d turbulence.
#[target_feature(enable = "sse2")]
pub unsafe fn turbulence_2d(
    x: __m128,
    y: __m128,
    freq: __m128,
    lacunarity: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut result = _mm_abs_ps(simplex_2d(xf, yf));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lacunarity);
        yf = _mm_mul_ps(yf, lacunarity);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_abs_ps(_mm_mul_ps(simplex_2d(xf, yf), amp)));
    }

    result
}

#[target_feature(enable = "sse2")]
unsafe fn grad3d_simd(hash: __m128i, x: __m128, y: __m128, z: __m128) -> __m128 {
    let h = _mm_and_si128(hash, _mm_set1_epi32(15));

    let mut u = _mm_castsi128_ps(_mm_cmplt_epi32(h, _mm_set1_epi32(8)));
    u = blendv_sse2(y, x, u);

    let mut v = _mm_castsi128_ps(_mm_cmplt_epi32(h, _mm_set1_epi32(4)));
    let mut h12_or_14 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_or_si128(
            _mm_cmpeq_epi32(h, _mm_set1_epi32(12)),
            _mm_cmpeq_epi32(h, _mm_set1_epi32(14)),
        ),
    ));
    h12_or_14 = blendv_sse2(x, z, h12_or_14);
    v = blendv_sse2(h12_or_14, y, v);

    let h_and_1 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(1)),
    ));
    let h_and_2 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(2)),
    ));

    _mm_add_ps(
        blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), u), u, h_and_1),
        blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), v), v, h_and_2),
    )
}

/// Get a single value of 3d simplex noise, results
/// are not scaled.
#[target_feature(enable = "sse2")]
pub unsafe fn simplex_3d(x: __m128, y: __m128, z: __m128) -> __m128 {
    let s = _mm_mul_ps(F3, _mm_add_ps(x, _mm_add_ps(y, z)));

    let ips = floor_sse2(_mm_add_ps(x, s));
    let jps = floor_sse2(_mm_add_ps(y, s));
    let kps = floor_sse2(_mm_add_ps(z, s));

    let i = _mm_cvtps_epi32(ips);
    let j = _mm_cvtps_epi32(jps);
    let k = _mm_cvtps_epi32(kps);

    let t = _mm_mul_ps(_mm_cvtepi32_ps(_mm_add_epi32(i, _mm_add_epi32(j, k))), G3);
    let x0 = _mm_sub_ps(x, _mm_sub_ps(ips, t));
    let y0 = _mm_sub_ps(y, _mm_sub_ps(jps, t));
    let z0 = _mm_sub_ps(z, _mm_sub_ps(kps, t));

    /* Really tricky section handling a series of nested conditionals 
   * This table can be helpful for following the logic
             ijk1 ijk2
	x>=y>=z -> 100  110
	x>z>y   -> 100  101
	z>x>y   -> 001  101
	z>y>x   -> 001  011
	y>z>x   -> 010  011
	y>x>=z  -> 010  110
	*/
    let i1 = M128iArray {
        simd: _mm_and_si128(
            _mm_castps_si128(_mm_cmpge_ps(x0, y0)),
            _mm_castps_si128(_mm_cmpge_ps(x0, z0)),
        ),
    };
    let j1 = M128iArray {
        simd: _mm_and_si128(
            _mm_castps_si128(_mm_cmpgt_ps(y0, x0)),
            _mm_castps_si128(_mm_cmpge_ps(y0, z0)),
        ),
    };
    let k1 = M128iArray {
        simd: _mm_and_si128(
            _mm_castps_si128(_mm_cmpgt_ps(z0, x0)),
            _mm_castps_si128(_mm_cmpgt_ps(z0, y0)),
        ),
    };

    //for i2
    let yx_xz = _mm_and_si128(
        _mm_castps_si128(_mm_cmpge_ps(x0, y0)),
        _mm_castps_si128(_mm_cmplt_ps(x0, z0)),
    );
    let zx_xy = _mm_and_si128(
        _mm_castps_si128(_mm_cmpge_ps(x0, z0)),
        _mm_castps_si128(_mm_cmplt_ps(x0, y0)),
    );

    //for j2
    let xy_yz = _mm_and_si128(
        _mm_castps_si128(_mm_cmplt_ps(x0, y0)),
        _mm_castps_si128(_mm_cmplt_ps(y0, z0)),
    );
    let zy_yx = _mm_and_si128(
        _mm_castps_si128(_mm_cmpge_ps(y0, z0)),
        _mm_castps_si128(_mm_cmpge_ps(x0, y0)),
    );

    //for k2
    let yz_zx = _mm_and_si128(
        _mm_castps_si128(_mm_cmplt_ps(y0, z0)),
        _mm_castps_si128(_mm_cmpge_ps(x0, z0)),
    );
    let xz_zy = _mm_and_si128(
        _mm_castps_si128(_mm_cmplt_ps(x0, z0)),
        _mm_castps_si128(_mm_cmpge_ps(y0, z0)),
    );

    let i2 = M128iArray {
        simd: _mm_or_si128(i1.simd, _mm_or_si128(yx_xz, zx_xy)),
    };
    let j2 = M128iArray {
        simd: _mm_or_si128(j1.simd, _mm_or_si128(xy_yz, zy_yx)),
    };
    let k2 = M128iArray {
        simd: _mm_or_si128(k1.simd, _mm_or_si128(yz_zx, xz_zy)),
    };

    let x1 = _mm_add_ps(_mm_add_ps(x0, _mm_cvtepi32_ps(i1.simd)), G3);
    let y1 = _mm_add_ps(_mm_add_ps(y0, _mm_cvtepi32_ps(j1.simd)), G3);
    let z1 = _mm_add_ps(_mm_add_ps(z0, _mm_cvtepi32_ps(k1.simd)), G3);

    let x2 = _mm_add_ps(_mm_add_ps(x0, _mm_cvtepi32_ps(i2.simd)), F3);
    let y2 = _mm_add_ps(_mm_add_ps(y0, _mm_cvtepi32_ps(j2.simd)), F3);
    let z2 = _mm_add_ps(_mm_add_ps(z0, _mm_cvtepi32_ps(k2.simd)), F3);
    let x3 = _mm_add_ps(_mm_add_ps(x0, _mm_set1_ps(-1.0)), POINT_FIVE);
    let y3 = _mm_add_ps(_mm_add_ps(y0, _mm_set1_ps(-1.0)), POINT_FIVE);
    let z3 = _mm_add_ps(_mm_add_ps(z0, _mm_set1_ps(-1.0)), POINT_FIVE);

    let ii = M128iArray {
        simd: _mm_and_si128(i, _mm_set1_epi32(0xff)),
    };
    let jj = M128iArray {
        simd: _mm_and_si128(j, _mm_set1_epi32(0xff)),
    };
    let kk = M128iArray {
        simd: _mm_and_si128(k, _mm_set1_epi32(0xff)),
    };

    let gi0 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(0)
                            + *PERM.get_unchecked(*kk.array.get_unchecked(0) as usize))
                            as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(1)
                            + *PERM.get_unchecked(*kk.array.get_unchecked(1) as usize))
                            as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(2)
                            + *PERM.get_unchecked(*kk.array.get_unchecked(2) as usize))
                            as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(3)
                            + *PERM.get_unchecked(*kk.array.get_unchecked(3) as usize))
                            as usize,
                    )) as usize,
            ),
        ],
    };

    let gi1 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0) - *i1.array.get_unchecked(0)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(0) - *j1.array.get_unchecked(0)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(0) - *k1.array.get_unchecked(0)) as usize,
                            )) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1) - *i1.array.get_unchecked(1)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(1) - *j1.array.get_unchecked(1)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(1) - *k1.array.get_unchecked(1)) as usize,
                            )) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2) - *i1.array.get_unchecked(2)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(2) - *j1.array.get_unchecked(2)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(2) - *k1.array.get_unchecked(2)) as usize,
                            )) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3) - *i1.array.get_unchecked(3)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(3) - *j1.array.get_unchecked(3)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(3) - *k1.array.get_unchecked(3)) as usize,
                            )) as usize,
                    )) as usize,
            ),
        ],
    };
    let gi2 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0) - *i2.array.get_unchecked(0)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(0) - *j2.array.get_unchecked(0)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(0) - *k2.array.get_unchecked(0)) as usize,
                            )) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1) - *i2.array.get_unchecked(1)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(1) - *j2.array.get_unchecked(1)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(1) - *k2.array.get_unchecked(1)) as usize,
                            )) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2) - *i2.array.get_unchecked(2)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(2) - *j2.array.get_unchecked(2)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(2) - *k2.array.get_unchecked(2)) as usize,
                            )) as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3) - *i2.array.get_unchecked(3)
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(3) - *j2.array.get_unchecked(3)
                            + *PERM.get_unchecked(
                                (*kk.array.get_unchecked(3) - *k2.array.get_unchecked(3)) as usize,
                            )) as usize,
                    )) as usize,
            ),
        ],
    };
    let gi3 = M128iArray {
        array: [
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(0) + 1
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(0)
                            + 1
                            + *PERM.get_unchecked((*kk.array.get_unchecked(0) + 1) as usize))
                            as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(1) + 1
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(1)
                            + 1
                            + *PERM.get_unchecked((*kk.array.get_unchecked(1) + 1) as usize))
                            as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(2) + 1
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(2)
                            + 1
                            + *PERM.get_unchecked((*kk.array.get_unchecked(2) + 1) as usize))
                            as usize,
                    )) as usize,
            ),
            *PERM.get_unchecked(
                (*ii.array.get_unchecked(3) + 1
                    + *PERM.get_unchecked(
                        (*jj.array.get_unchecked(3)
                            + 1
                            + *PERM.get_unchecked((*kk.array.get_unchecked(3) + 1) as usize))
                            as usize,
                    )) as usize,
            ),
        ],
    };

    let t0 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x0, x0)),
            _mm_mul_ps(y0, y0),
        ),
        _mm_mul_ps(z0, z0),
    );
    let t1 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x1, x1)),
            _mm_mul_ps(y1, y1),
        ),
        _mm_mul_ps(z1, z1),
    );
    let t2 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x2, x2)),
            _mm_mul_ps(y2, y2),
        ),
        _mm_mul_ps(z2, z2),
    );
    let t3 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x3, x3)),
            _mm_mul_ps(y3, y3),
        ),
        _mm_mul_ps(z3, z3),
    );
    //ti*ti*ti*ti
    let mut t0q = _mm_mul_ps(t0, t0);
    t0q = _mm_mul_ps(t0q, t0q);
    let mut t1q = _mm_mul_ps(t1, t1);
    t1q = _mm_mul_ps(t1q, t1q);
    let mut t2q = _mm_mul_ps(t2, t2);
    t2q = _mm_mul_ps(t2q, t2q);
    let mut t3q = _mm_mul_ps(t3, t3);
    t3q = _mm_mul_ps(t3q, t3q);

    let mut n0 = _mm_mul_ps(t0q, grad3d_simd(gi0.simd, x0, y0, z0));
    let mut n1 = _mm_mul_ps(t1q, grad3d_simd(gi1.simd, x1, y1, z1));
    let mut n2 = _mm_mul_ps(t2q, grad3d_simd(gi2.simd, x2, y2, z2));
    let mut n3 = _mm_mul_ps(t3q, grad3d_simd(gi3.simd, x3, y3, z3));

    //if ti < 0 then 0 else ni
    let mut cond = _mm_cmplt_ps(t0, _mm_setzero_ps());
    n0 = _mm_andnot_ps(cond, n0);
    cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
    n1 = _mm_andnot_ps(cond, n1);
    cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_andnot_ps(cond, n2);
    cond = _mm_cmplt_ps(t3, _mm_setzero_ps());
    n3 = _mm_andnot_ps(cond, n3);

    _mm_add_ps(n0, _mm_add_ps(n1, _mm_add_ps(n2, n3)))
}

/// Get a single value of 3d fractal brownian motion.
#[target_feature(enable = "sse2")]
pub unsafe fn fbm_3d(
    x: __m128,
    y: __m128,
    z: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut zf = _mm_mul_ps(z, freq);
    let mut result = simplex_3d(xf, yf, zf);
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        zf = _mm_mul_ps(zf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_mul_ps(simplex_3d(xf, yf, zf), amp));
    }

    result
}

/// Get a single value of 3d ridge noise.
#[target_feature(enable = "sse2")]
pub unsafe fn ridge_3d(
    x: __m128,
    y: __m128,
    z: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut zf = _mm_mul_ps(z, freq);
    let mut result = _mm_sub_ps(_mm_set1_ps(1.0), _mm_abs_ps(simplex_3d(xf, yf, zf)));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        zf = _mm_mul_ps(zf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(
            result,
            _mm_sub_ps(
                _mm_set1_ps(1.0),
                _mm_abs_ps(_mm_mul_ps(simplex_3d(xf, yf, zf), amp)),
            ),
        );
    }

    result
}

/// Get a single value of 3d turbulence.
#[target_feature(enable = "sse2")]
pub unsafe fn turbulence_3d(
    x: __m128,
    y: __m128,
    z: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut zf = _mm_mul_ps(z, freq);
    let mut result = _mm_abs_ps(simplex_3d(xf, yf, zf));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        zf = _mm_mul_ps(zf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_abs_ps(_mm_mul_ps(simplex_3d(xf, yf, zf), amp)));
    }

    result
}

#[target_feature(enable = "sse2")]
unsafe fn get_2d_noise_helper(x: __m128, y: __m128, noise_type: NoiseType) -> M128Array {
    M128Array {
        simd: match noise_type {
            NoiseType::Fbm {
                freq,
                lacunarity,
                gain,
                octaves,
            } => fbm_2d(
                x,
                y,
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_2d(
                _mm_mul_ps(x, _mm_set1_ps(freq)),
                _mm_mul_ps(y, _mm_set1_ps(freq)),
            ),
        },
    }
}

/// Gets a width X height sized block of 2d noise, unscaled.
/// `start_x` and `start_y` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "sse2")]
pub unsafe fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M128Array {
        simd: _mm_set1_ps(f32::MAX),
    };
    let mut max_s = M128Array {
        simd: _mm_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height);
    result.set_len(width * height);
    let mut y = _mm_set1_ps(start_y);
    let mut i = 0;
    let remainder = width % 4;
    for _ in 0..height {
        let mut x = _mm_set_ps(start_x + 3.0, start_x + 2.0, start_x + 1.0, start_x);
        for _ in 0..width / 4 {
            let f = get_2d_noise_helper(x, y, noise_type).simd;
            max_s.simd = _mm_max_ps(max_s.simd, f);
            min_s.simd = _mm_min_ps(min_s.simd, f);
            _mm_storeu_ps(result.get_unchecked_mut(i), f);
            i += 4;
            x = _mm_add_ps(x, _mm_set1_ps(4.0));
        }
        if remainder != 0 {
            let f = get_2d_noise_helper(x, y, noise_type);
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
        y = _mm_add_ps(y, _mm_set1_ps(1.0));
    }
    for i in 0..4 {
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
#[target_feature(enable = "sse2")]
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
    scale_array_sse(scale_min, scale_max, min, max, &mut noise);
    noise
}

#[target_feature(enable = "sse2")]
unsafe fn get_3d_noise_helper(x: __m128, y: __m128, z: __m128, noise_type: NoiseType) -> M128Array {
    M128Array {
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_3d(
                _mm_mul_ps(x, _mm_set1_ps(freq)),
                _mm_mul_ps(y, _mm_set1_ps(freq)),
                _mm_mul_ps(z, _mm_set1_ps(freq)),
            ),
        },
    }
}

/// Gets a width X height X depth sized block of 3d noise, unscaled,
/// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "sse2")]
pub unsafe fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    noise_type: NoiseType,
) -> (Vec<f32>, f32, f32) {
    let mut min_s = M128Array {
        simd: _mm_set1_ps(f32::MAX),
    };
    let mut max_s = M128Array {
        simd: _mm_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height * depth);
    result.set_len(width * height * depth);
    let mut z = _mm_set1_ps(start_z);
    let mut i = 0;
    let remainder = width % 4;
    for _ in 0..depth {
        let mut y = _mm_set1_ps(start_y);
        for _ in 0..height {
            let mut x = _mm_set_ps(start_x + 3.0, start_x + 2.0, start_x + 1.0, start_x);
            for _ in 0..width / 4 {
                let f = get_3d_noise_helper(x, y, z, noise_type).simd;
                max_s.simd = _mm_max_ps(max_s.simd, f);
                min_s.simd = _mm_min_ps(min_s.simd, f);
                _mm_storeu_ps(result.get_unchecked_mut(i), f);
                i += 4;
                x = _mm_add_ps(x, _mm_set1_ps(4.0));
            }
            if remainder != 0 {
                let f = get_3d_noise_helper(x, y, z, noise_type);
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
            y = _mm_add_ps(y, _mm_set1_ps(1.0));
        }
        z = _mm_add_ps(z, _mm_set1_ps(1.0));
    }
    for i in 0..4 {
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
#[target_feature(enable = "sse2")]
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
    scale_array_sse(scale_min, scale_max, min, max, &mut noise);
    noise
}

#[target_feature(enable = "sse2")]
unsafe fn grad4_simd(hash: __m128i, x: __m128, y: __m128, z: __m128, t: __m128) -> __m128 {
    let h = _mm_and_si128(hash, _mm_set1_epi32(31));
    let mut mask = _mm_castsi128_ps(_mm_cmplt_epi32(h, _mm_set1_epi32(24)));
    let u = blendv_sse2(y, x, mask);
    mask = _mm_castsi128_ps(_mm_cmplt_epi32(h, _mm_set1_epi32(16)));
    let v = blendv_sse2(z, y, mask);
    mask = _mm_castsi128_ps(_mm_cmplt_epi32(h, _mm_set1_epi32(8)));
    let w = blendv_sse2(t, z, mask);

    let h_and_1 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(1)),
    ));
    let h_and_2 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(2)),
    ));
    let h_and_4 = _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_setzero_si128(),
        _mm_and_si128(h, _mm_set1_epi32(4)),
    ));

    _mm_add_ps(
        blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), u), u, h_and_1),
        _mm_add_ps(
            blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), v), v, h_and_2),
            blendv_sse2(_mm_sub_ps(_mm_setzero_ps(), w), w, h_and_4),
        ),
    )
}
/// Get a single value of 4d simplex noise, results
/// are not scaled.
#[target_feature(enable = "sse2")]
pub unsafe fn simplex_4d(x: __m128, y: __m128, z: __m128, w: __m128) -> __m128 {
    let s = _mm_mul_ps(F4, _mm_add_ps(x, _mm_add_ps(y, _mm_add_ps(z, w))));

    let ips = floor_sse2(_mm_add_ps(x, s));
    let jps = floor_sse2(_mm_add_ps(y, s));
    let kps = floor_sse2(_mm_add_ps(z, s));
    let lps = floor_sse2(_mm_add_ps(w, s));

    let i = _mm_cvtps_epi32(ips);
    let j = _mm_cvtps_epi32(jps);
    let k = _mm_cvtps_epi32(kps);
    let l = _mm_cvtps_epi32(lps);

    let t = _mm_mul_ps(
        _mm_cvtepi32_ps(_mm_add_epi32(i, _mm_add_epi32(j, _mm_add_epi32(k, l)))),
        G4,
    );
    let x0 = _mm_sub_ps(x, _mm_sub_ps(ips, t));
    let y0 = _mm_sub_ps(y, _mm_sub_ps(jps, t));
    let z0 = _mm_sub_ps(z, _mm_sub_ps(kps, t));
    let w0 = _mm_sub_ps(w, _mm_sub_ps(lps, t));

    let mut cond = _mm_castps_si128(_mm_cmpgt_ps(x0, y0));
    let c1 = _mm_and_si128(cond, _mm_set1_epi32(32));
    cond = _mm_castps_si128(_mm_cmpgt_ps(x0, z0));
    let c2 = _mm_and_si128(cond, _mm_set1_epi32(16));
    cond = _mm_castps_si128(_mm_cmpgt_ps(y0, z0));
    let c3 = _mm_and_si128(cond, _mm_set1_epi32(8));
    cond = _mm_castps_si128(_mm_cmpgt_ps(x0, w0));
    let c4 = _mm_and_si128(cond, _mm_set1_epi32(4));
    cond = _mm_castps_si128(_mm_cmpgt_ps(y0, z0));
    let c5 = _mm_and_si128(cond, _mm_set1_epi32(2));
    cond = _mm_castps_si128(_mm_cmpgt_ps(z0, z0));
    let c6 = _mm_and_si128(cond, _mm_set1_epi32(1));
    let c = M128iArray {
        simd: _mm_or_si128(
            c1,
            _mm_or_si128(c2, _mm_or_si128(c3, _mm_or_si128(c4, _mm_or_si128(c5, c6)))),
        ),
    };
    //TODO for this and SSE41 see if using the broken up SIMPLEX arrays is faster ala avx2
    let i1 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(0) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(0) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(0) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(0) >= 3) as i32,
        ],
    };
    let j1 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(1) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(1) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(1) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(1) >= 3) as i32,
        ],
    };
    let k1 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(2) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(2) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(2) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(2) >= 3) as i32,
        ],
    };
    let l1 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(3) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(3) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(3) >= 3) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(3) >= 3) as i32,
        ],
    };
    //---------------
    let i2 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(0) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(0) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(0) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(0) >= 2) as i32,
        ],
    };
    let j2 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(1) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(1) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(1) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(1) >= 2) as i32,
        ],
    };
    let k2 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(2) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(2) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(2) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(2) >= 2) as i32,
        ],
    };
    let l2 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(3) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(3) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(3) >= 2) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(3) >= 2) as i32,
        ],
    };
    //--------------------
    let i3 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(0) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(0) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(0) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(0) >= 1) as i32,
        ],
    };
    let j3 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(1) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(1) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(1) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(1) >= 1) as i32,
        ],
    };
    let k3 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(2) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(2) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(2) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(2) >= 1) as i32,
        ],
    };
    let l3 = M128iArray {
        array: [
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(0) as usize)
                .get_unchecked(3) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(1) as usize)
                .get_unchecked(3) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(2) as usize)
                .get_unchecked(3) >= 1) as i32,
            (*SIMPLEX
                .get_unchecked(*c.array.get_unchecked(3) as usize)
                .get_unchecked(3) >= 1) as i32,
        ],
    };

    let x1 = _mm_add_ps(_mm_sub_ps(x0, _mm_cvtepi32_ps(i1.simd)), G4);
    let y1 = _mm_add_ps(_mm_sub_ps(y0, _mm_cvtepi32_ps(j1.simd)), G4);
    let z1 = _mm_add_ps(_mm_sub_ps(z0, _mm_cvtepi32_ps(k1.simd)), G4);
    let w1 = _mm_add_ps(_mm_sub_ps(w0, _mm_cvtepi32_ps(l1.simd)), G4);
    let x2 = _mm_add_ps(_mm_sub_ps(x0, _mm_cvtepi32_ps(i2.simd)), G24);
    let y2 = _mm_add_ps(_mm_sub_ps(y0, _mm_cvtepi32_ps(j2.simd)), G24);
    let z2 = _mm_add_ps(_mm_sub_ps(z0, _mm_cvtepi32_ps(k2.simd)), G24);
    let w2 = _mm_add_ps(_mm_sub_ps(w0, _mm_cvtepi32_ps(l2.simd)), G24);
    let x3 = _mm_add_ps(_mm_sub_ps(x0, _mm_cvtepi32_ps(i3.simd)), G34);
    let y3 = _mm_add_ps(_mm_sub_ps(y0, _mm_cvtepi32_ps(j3.simd)), G34);
    let z3 = _mm_add_ps(_mm_sub_ps(z0, _mm_cvtepi32_ps(k3.simd)), G34);
    let w3 = _mm_add_ps(_mm_sub_ps(w0, _mm_cvtepi32_ps(l3.simd)), G34);
    let x4 = _mm_add_ps(_mm_sub_ps(x0, _mm_set1_ps(1.0)), G44);
    let y4 = _mm_add_ps(_mm_sub_ps(y0, _mm_set1_ps(1.0)), G44);
    let z4 = _mm_add_ps(_mm_sub_ps(z0, _mm_set1_ps(1.0)), G44);
    let w4 = _mm_add_ps(_mm_sub_ps(w0, _mm_set1_ps(1.0)), G44);

    let ii = M128iArray {
        simd: _mm_and_si128(i, _mm_set1_epi32(0xff)),
    };
    let jj = M128iArray {
        simd: _mm_and_si128(j, _mm_set1_epi32(0xff)),
    };
    let kk = M128iArray {
        simd: _mm_and_si128(k, _mm_set1_epi32(0xff)),
    };
    let ll = M128iArray {
        simd: _mm_and_si128(l, _mm_set1_epi32(0xff)),
    };

    let gi0 = M128iArray {
        array: [
            {
                let lp = *PERM.get_unchecked(*ll.array.get_unchecked(0) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(0) + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(0) + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(0) + jp) as usize)
            },
            {
                let lp = *PERM.get_unchecked(*ll.array.get_unchecked(1) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(1) + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(1) + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(1) + jp) as usize)
            },
            {
                let lp = *PERM.get_unchecked(*ll.array.get_unchecked(2) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(2) + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(2) + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(2) + jp) as usize)
            },
            {
                let lp = *PERM.get_unchecked(*ll.array.get_unchecked(3) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(3) + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(3) + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(3) + jp) as usize)
            },
        ],
    };

    let gi1 = M128iArray {
        array: [
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(0) + l1.array.get_unchecked(0)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(0) + *k1.array.get_unchecked(0) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(0) + *j1.array.get_unchecked(0) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(0) + *i1.array.get_unchecked(0) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(1) + l1.array.get_unchecked(1)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(1) + *k1.array.get_unchecked(1) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(1) + *j1.array.get_unchecked(1) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(1) + *i1.array.get_unchecked(1) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(2) + l1.array.get_unchecked(2)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(2) + *k1.array.get_unchecked(2) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(2) + *j1.array.get_unchecked(2) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(2) + *i1.array.get_unchecked(2) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(3) + l1.array.get_unchecked(3)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(3) + *k1.array.get_unchecked(3) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(3) + *j1.array.get_unchecked(3) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(3) + *i1.array.get_unchecked(3) + jp) as usize,
                )
            },
        ],
    };
    let gi2 = M128iArray {
        array: [
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(0) + *l2.array.get_unchecked(0)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(0) + *k2.array.get_unchecked(0) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(0) + *j2.array.get_unchecked(0) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(0) + *i2.array.get_unchecked(0) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(1) + *l2.array.get_unchecked(1)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(1) + *k2.array.get_unchecked(1) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(1) + *j2.array.get_unchecked(1) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(1) + *i2.array.get_unchecked(1) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(2) + *l2.array.get_unchecked(2)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(2) + *k2.array.get_unchecked(2) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(2) + *j2.array.get_unchecked(2) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(2) + *i2.array.get_unchecked(2) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(3) + *l2.array.get_unchecked(3)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(3) + *k2.array.get_unchecked(3) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(3) + *j2.array.get_unchecked(3) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(3) + *i2.array.get_unchecked(3) + jp) as usize,
                )
            },
        ],
    };
    let gi3 = M128iArray {
        array: [
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(0) + *l3.array.get_unchecked(0)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(0) + *k3.array.get_unchecked(0) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(0) + *j3.array.get_unchecked(0) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(0) + *i3.array.get_unchecked(0) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(1) + *l3.array.get_unchecked(1)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(1) + *k3.array.get_unchecked(1) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(1) + *j3.array.get_unchecked(1) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(1) + *i3.array.get_unchecked(1) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(2) + *l3.array.get_unchecked(2)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(2) + *k3.array.get_unchecked(2) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(2) + *j3.array.get_unchecked(2) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(2) + *i3.array.get_unchecked(2) + jp) as usize,
                )
            },
            {
                let lp = *PERM.get_unchecked(
                    (*ll.array.get_unchecked(3) + *l3.array.get_unchecked(3)) as usize,
                );
                let kp = *PERM.get_unchecked(
                    (*kk.array.get_unchecked(3) + *k3.array.get_unchecked(3) + lp) as usize,
                );
                let jp = *PERM.get_unchecked(
                    (*jj.array.get_unchecked(3) + *j3.array.get_unchecked(3) + kp) as usize,
                );
                *PERM.get_unchecked(
                    (*ii.array.get_unchecked(3) + *i3.array.get_unchecked(3) + jp) as usize,
                )
            },
        ],
    };
    let gi4 = M128iArray {
        array: [
            {
                let lp = *PERM.get_unchecked((*ll.array.get_unchecked(0) + 1) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(0) + 1 + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(0) + 1 + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(0) + 1 + jp) as usize)
            },
            {
                let lp = *PERM.get_unchecked((*ll.array.get_unchecked(1) + 1) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(1) + 1 + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(1) + 1 + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(1) + 1 + jp) as usize)
            },
            {
                let lp = *PERM.get_unchecked((*ll.array.get_unchecked(2) + 1) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(2) + 1 + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(2) + 1 + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(2) + 1 + jp) as usize)
            },
            {
                let lp = *PERM.get_unchecked((*ll.array.get_unchecked(3) + 1) as usize);
                let kp = *PERM.get_unchecked((*kk.array.get_unchecked(3) + 1 + lp) as usize);
                let jp = *PERM.get_unchecked((*jj.array.get_unchecked(3) + 1 + kp) as usize);
                *PERM.get_unchecked((*ii.array.get_unchecked(3) + 1 + jp) as usize)
            },
        ],
    };

    let t0 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(
                _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x0, x0)),
                _mm_mul_ps(y0, y0),
            ),
            _mm_mul_ps(z0, z0),
        ),
        _mm_mul_ps(w0, w0),
    );
    let t1 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(
                _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x1, x1)),
                _mm_mul_ps(y1, y1),
            ),
            _mm_mul_ps(z1, z1),
        ),
        _mm_mul_ps(w1, w1),
    );
    let t2 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(
                _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x2, x2)),
                _mm_mul_ps(y2, y2),
            ),
            _mm_mul_ps(z2, z2),
        ),
        _mm_mul_ps(w2, w2),
    );
    let t3 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(
                _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x3, x3)),
                _mm_mul_ps(y3, y3),
            ),
            _mm_mul_ps(z3, z3),
        ),
        _mm_mul_ps(w3, w3),
    );
    let t4 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(
                _mm_sub_ps(POINT_FIVE, _mm_mul_ps(x4, x4)),
                _mm_mul_ps(y4, y4),
            ),
            _mm_mul_ps(z4, z4),
        ),
        _mm_mul_ps(w4, w4),
    );
    //ti*ti*ti*ti
    let mut t0q = _mm_mul_ps(t0, t0);
    t0q = _mm_mul_ps(t0q, t0q);
    let mut t1q = _mm_mul_ps(t1, t1);
    t1q = _mm_mul_ps(t1q, t1q);
    let mut t2q = _mm_mul_ps(t2, t2);
    t2q = _mm_mul_ps(t2q, t2q);
    let mut t3q = _mm_mul_ps(t3, t3);
    t3q = _mm_mul_ps(t3q, t3q);
    let mut t4q = _mm_mul_ps(t4, t4);
    t4q = _mm_mul_ps(t4q, t4q);

    let mut n0 = _mm_mul_ps(t0q, grad4_simd(gi0.simd, x0, y0, z0, w0));
    let mut n1 = _mm_mul_ps(t1q, grad4_simd(gi1.simd, x1, y1, z1, w1));
    let mut n2 = _mm_mul_ps(t2q, grad4_simd(gi2.simd, x2, y2, z2, w2));
    let mut n3 = _mm_mul_ps(t3q, grad4_simd(gi3.simd, x3, y3, z3, w3));
    let mut n4 = _mm_mul_ps(t4q, grad4_simd(gi4.simd, x4, y4, z4, w4));

    //if ti < 0 then 0 else ni
    let mut cond = _mm_cmplt_ps(t0, _mm_setzero_ps());
    n0 = _mm_andnot_ps(cond, n0);
    cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
    n1 = _mm_andnot_ps(cond, n1);
    cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_andnot_ps(cond, n2);
    cond = _mm_cmplt_ps(t3, _mm_setzero_ps());
    n3 = _mm_andnot_ps(cond, n3);
    cond = _mm_cmplt_ps(t4, _mm_setzero_ps());
    n4 = _mm_andnot_ps(cond, n4);

    _mm_add_ps(n0, _mm_add_ps(n1, _mm_add_ps(n2, _mm_add_ps(n3, n4))))
}
/// Get a single value of 4d fractal brownian motion.
#[target_feature(enable = "sse2")]
pub unsafe fn fbm_4d(
    x: __m128,
    y: __m128,
    z: __m128,
    w: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut zf = _mm_mul_ps(z, freq);
    let mut wf = _mm_mul_ps(w, freq);
    let mut result = simplex_4d(xf, yf, zf, wf);
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        zf = _mm_mul_ps(zf, lac);
        wf = _mm_mul_ps(wf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_mul_ps(simplex_4d(xf, yf, zf, wf), amp));
    }

    result
}

/// Get a single value of 4d ridge noise.
#[target_feature(enable = "sse2")]
pub unsafe fn ridge_4d(
    x: __m128,
    y: __m128,
    z: __m128,
    w: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut zf = _mm_mul_ps(z, freq);
    let mut wf = _mm_mul_ps(w, freq);
    let mut result = _mm_sub_ps(_mm_set1_ps(1.0), _mm_abs_ps(simplex_4d(xf, yf, zf, wf)));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        zf = _mm_mul_ps(zf, lac);
        wf = _mm_mul_ps(wf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(
            result,
            _mm_sub_ps(
                _mm_set1_ps(1.0),
                _mm_abs_ps(_mm_mul_ps(simplex_4d(xf, yf, zf, wf), amp)),
            ),
        );
    }

    result
}

/// Get a single value of 4d turbulence.
#[target_feature(enable = "sse2")]
pub unsafe fn turbulence_4d(
    x: __m128,
    y: __m128,
    z: __m128,
    w: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: u8,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut zf = _mm_mul_ps(z, freq);
    let mut wf = _mm_mul_ps(w, freq);
    let mut result = _mm_abs_ps(simplex_4d(xf, yf, zf, wf));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        zf = _mm_mul_ps(zf, lac);
        wf = _mm_mul_ps(wf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(
            result,
            _mm_abs_ps(_mm_mul_ps(simplex_4d(xf, yf, zf, wf), amp)),
        );
    }

    result
}

#[target_feature(enable = "sse2")]
unsafe fn get_4d_noise_helper(
    x: __m128,
    y: __m128,
    z: __m128,
    w: __m128,
    noise_type: NoiseType,
) -> M128Array {
    M128Array {
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
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
                _mm_set1_ps(freq),
                _mm_set1_ps(lacunarity),
                _mm_set1_ps(gain),
                octaves,
            ),
            NoiseType::Normal { freq } => simplex_4d(
                _mm_mul_ps(x, _mm_set1_ps(freq)),
                _mm_mul_ps(y, _mm_set1_ps(freq)),
                _mm_mul_ps(z, _mm_set1_ps(freq)),
                _mm_mul_ps(w, _mm_set1_ps(freq)),
            ),
        },
    }
}
/// Gets a width X height X depth x time sized block of 4d noise, unscaled,
/// `start_*` can be used to provide an offset in the
/// coordinates. Results are unscaled, 'min' and 'max' noise values
/// are returned so you can scale and transform the noise as you see fit
/// in a single pass.
#[target_feature(enable = "sse2")]
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
    let mut min_s = M128Array {
        simd: _mm_set1_ps(f32::MAX),
    };
    let mut max_s = M128Array {
        simd: _mm_set1_ps(f32::MIN),
    };
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    let mut result = Vec::with_capacity(width * height * depth * time);
    result.set_len(width * height * depth * time);
    let mut i = 0;
    let remainder = width % 4;
    let mut w = _mm_set1_ps(start_w);
    for _ in 0..time {
        let mut z = _mm_set1_ps(start_z);
        for _ in 0..depth {
            let mut y = _mm_set1_ps(start_y);
            for _ in 0..height {
                let mut x = _mm_set_ps(start_x + 3.0, start_x + 2.0, start_x + 1.0, start_x);
                for _ in 0..width / 4 {
                    let f = get_4d_noise_helper(x, y, z, w, noise_type).simd;
                    max_s.simd = _mm_max_ps(max_s.simd, f);
                    min_s.simd = _mm_min_ps(min_s.simd, f);
                    _mm_storeu_ps(result.get_unchecked_mut(i), f);
                    i += 4;
                    x = _mm_add_ps(x, _mm_set1_ps(4.0));
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
                y = _mm_add_ps(y, _mm_set1_ps(1.0));
            }
            z = _mm_add_ps(z, _mm_set1_ps(1.0));
        }
        w = _mm_add_ps(w, _mm_set1_ps(1.0));
    }
    for i in 0..4 {
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
#[target_feature(enable = "sse2")]
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
    scale_array_sse(scale_min, scale_max, min, max, &mut noise);
    noise
}
