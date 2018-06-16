#![allow(non_snake_case)]

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
const POINT_FIVE: __m256 = unsafe { M256Array { array: [0.5; 8] }.simd };

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

unsafe fn simplex_2d(x: __m256, y: __m256) -> __m256 {
    let s = _mm256_mul_ps(F2, _mm256_add_ps(x, y));
    let ips = _mm256_floor_ps(_mm256_add_ps(x, s));
    let jps = _mm256_floor_ps(_mm256_add_ps(y, s));

    let i = _mm256_cvtps_epi32(ips);
    let j = _mm256_cvtps_epi32(jps);

    let t = _mm256_mul_ps(_mm256_cvtepi32_ps(_mm256_add_epi32(i, j)), G2);

    let x0 = _mm256_sub_ps(x, _mm256_sub_ps(ips, t));
    let y0 = _mm256_sub_ps(y, _mm256_sub_ps(jps, t));

    let i1 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ)),
    );

    let j1 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_castps_si256(_mm256_cmp_ps(y0, x0, _CMP_GT_OQ)),
    );

    let x1 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i1)), G2);
    let y1 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j1)), G2);
    let x2 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_set1_ps(1.0)), G22);
    let y2 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_set1_ps(1.0)), G22);

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
            _mm256_add_epi32(ii, i1),
            _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(jj, j1), 4),
        ),
        4,
    );

    let gi2 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_add_epi32(ii, _mm256_set1_epi32(1)),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(jj, _mm256_set1_epi32(1)),
                4,
            ),
        ),
        4,
    );

    let t0 = _mm256_sub_ps(
        _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x0, x0)),
        _mm256_mul_ps(y0, y0),
    );
    let t1 = _mm256_sub_ps(
        _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x1, x1)),
        _mm256_mul_ps(y1, y1),
    );
    let t2 = _mm256_sub_ps(
        _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x2, x2)),
        _mm256_mul_ps(y2, y2),
    );

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
    n0 = _mm256_blendv_ps(n0, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(), _CMP_LT_OQ);
    n1 = _mm256_blendv_ps(n1, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(), _CMP_LT_OQ);
    n2 = _mm256_blendv_ps(n2, _mm256_setzero_ps(), cond);

    _mm256_add_ps(n0, _mm256_add_ps(n1, n2))
}

#[inline(always)]
unsafe fn _mm256_abs_ps(a: __m256) -> __m256 {
    let b = _mm256_set1_epi32(0x7fffffff);
    _mm256_and_ps(a, _mm256_castsi256_ps(b))
}

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
pub fn scale_array(scale_min: f32, scale_max: f32, min: f32, max: f32, data: &mut Vec<f32>) {
    unsafe {
        let scale_range = scale_max - scale_min;
        let range = max - min;
        let multiplier = scale_range / range;
        let offset = scale_min - min * multiplier;

        let mut i = 0;
        while i <= data.len() - 8 {
            _mm256_storeu_ps(
                &mut data[i],
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
}
unsafe fn get_2d_noise_helper(
    x: __m256,
    y: __m256,
    fractal_settings: FractalSettings,
) -> M256Array {
    M256Array {
        simd: match fractal_settings.noise_type {
            NoiseType::FBM => fbm_2d(
                x,
                y,
                _mm256_set1_ps(fractal_settings.freq),
                _mm256_set1_ps(fractal_settings.lacunarity),
                _mm256_set1_ps(fractal_settings.gain),
                fractal_settings.octaves,
            ),
            NoiseType::Turbulence => turbulence_2d(
                x,
                y,
                _mm256_set1_ps(fractal_settings.freq),
                _mm256_set1_ps(fractal_settings.lacunarity),
                _mm256_set1_ps(fractal_settings.gain),
                fractal_settings.octaves,
            ),
            NoiseType::Normal => simplex_2d(
                _mm256_mul_ps(x, _mm256_set1_ps(fractal_settings.freq)),
                _mm256_mul_ps(y, _mm256_set1_ps(fractal_settings.freq)),
            ),
        },
    }
}

pub fn get_2d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    fractal_settings: FractalSettings,
) -> (Vec<f32>, f32, f32) {
    unsafe {
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
                let f = get_2d_noise_helper(x, y, fractal_settings).simd;
                max_s.simd = _mm256_max_ps(max_s.simd, f);
                min_s.simd = _mm256_min_ps(min_s.simd, f);
                _mm256_storeu_ps(result.get_unchecked_mut(i), f);
                i += 8;
                x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
            }
            if remainder != 0 {
                let f = get_2d_noise_helper(x, y, fractal_settings);
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
}

pub fn get_2d_scaled_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    fractal_settings: FractalSettings,
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_2d_noise(start_x, width, start_y, height, fractal_settings);
    scale_array(scale_min, scale_max, min, max, &mut noise);
    noise
}

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

unsafe fn simplex_3d(x: __m256, y: __m256, z: __m256) -> __m256 {
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
        _mm256_set1_epi32(1),
        _mm256_and_si256(
            _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ)),
            _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_GE_OQ)),
        ),
    );
    let j1 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_and_si256(
            _mm256_castps_si256(_mm256_cmp_ps(y0, x0, _CMP_GT_OQ)),
            _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GE_OQ)),
        ),
    );
    let k1 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_and_si256(
            _mm256_castps_si256(_mm256_cmp_ps(z0, x0, _CMP_GT_OQ)),
            _mm256_castps_si256(_mm256_cmp_ps(z0, y0, _CMP_GT_OQ)),
        ),
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

    let i2 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_or_si256(i1, _mm256_or_si256(yx_xz, zx_xy)),
    );
    let j2 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_or_si256(j1, _mm256_or_si256(xy_yz, zy_yx)),
    );
    let k2 = _mm256_and_si256(
        _mm256_set1_epi32(1),
        _mm256_or_si256(k1, _mm256_or_si256(yz_zx, xz_zy)),
    );

    let x1 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i1)), G3);
    let y1 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j1)), G3);
    let z1 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k1)), G3);
    let x2 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i2)), F3);
    let y2 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j2)), F3);
    let z2 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k2)), F3);
    let x3 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_set1_ps(1.0)), POINT_FIVE);
    let y3 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_set1_ps(1.0)), POINT_FIVE);
    let z3 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_set1_ps(1.0)), POINT_FIVE);

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
            _mm256_add_epi32(ii, i1),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(
                    _mm256_add_epi32(jj, j1),
                    _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(kk, k1), 4),
                ),
                4,
            ),
        ),
        4,
    );
    let gi2 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_add_epi32(ii, i2),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(
                    _mm256_add_epi32(jj, j2),
                    _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(kk, k2), 4),
                ),
                4,
            ),
        ),
        4,
    );
    let gi3 = _mm256_i32gather_epi32(
        &PERM as *const i32,
        _mm256_add_epi32(
            _mm256_add_epi32(ii, _mm256_set1_epi32(1)),
            _mm256_i32gather_epi32(
                &PERM as *const i32,
                _mm256_add_epi32(
                    _mm256_add_epi32(jj, _mm256_set1_epi32(1)),
                    _mm256_i32gather_epi32(
                        &PERM as *const i32,
                        _mm256_add_epi32(kk, _mm256_set1_epi32(1)),
                        4,
                    ),
                ),
                4,
            ),
        ),
        4,
    );

    let t0 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x0, x0)),
            _mm256_mul_ps(y0, y0),
        ),
        _mm256_mul_ps(z0, z0),
    );
    let t1 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x1, x1)),
            _mm256_mul_ps(y1, y1),
        ),
        _mm256_mul_ps(z1, z1),
    );
    let t2 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x2, x2)),
            _mm256_mul_ps(y2, y2),
        ),
        _mm256_mul_ps(z2, z2),
    );
    let t3 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(POINT_FIVE, _mm256_mul_ps(x3, x3)),
            _mm256_mul_ps(y3, y3),
        ),
        _mm256_mul_ps(z3, z3),
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
    n0 = _mm256_blendv_ps(n0, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(), _CMP_LT_OQ);
    n1 = _mm256_blendv_ps(n1, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(), _CMP_LT_OQ);
    n2 = _mm256_blendv_ps(n2, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t3, _mm256_setzero_ps(), _CMP_LT_OQ);
    n3 = _mm256_blendv_ps(n3, _mm256_setzero_ps(), cond);

    _mm256_add_ps(n0, _mm256_add_ps(n1, _mm256_add_ps(n2, n3)))
}

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
        result = _mm256_add_ps(result, _mm256_mul_ps(simplex_3d(xf, yf, zf), amp));
    }

    result
}

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
unsafe fn get_3d_noise_helper(
    x: __m256,
    y: __m256,
    z: __m256,
    fractal_settings: FractalSettings,
) -> M256Array {
    M256Array {
        simd: match fractal_settings.noise_type {
            NoiseType::FBM => fbm_3d(
                x,
                y,
                z,
                _mm256_set1_ps(fractal_settings.freq),
                _mm256_set1_ps(fractal_settings.lacunarity),
                _mm256_set1_ps(fractal_settings.gain),
                fractal_settings.octaves,
            ),
            NoiseType::Turbulence => turbulence_3d(
                x,
                y,
                z,
                _mm256_set1_ps(fractal_settings.freq),
                _mm256_set1_ps(fractal_settings.lacunarity),
                _mm256_set1_ps(fractal_settings.gain),
                fractal_settings.octaves,
            ),
            NoiseType::Normal => simplex_3d(
                _mm256_mul_ps(x, _mm256_set1_ps(fractal_settings.freq)),
                _mm256_mul_ps(y, _mm256_set1_ps(fractal_settings.freq)),
                _mm256_mul_ps(z, _mm256_set1_ps(fractal_settings.freq)),
            ),
        },
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
    unsafe {
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
                    let f = get_3d_noise_helper(x, y, z, fractal_settings).simd;
                    max_s.simd = _mm256_max_ps(max_s.simd, f);
                    min_s.simd = _mm256_min_ps(min_s.simd, f);
                    _mm256_storeu_ps(&mut result[i], f);
                    i += 8;
                    x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
                }
                if remainder != 0 {
                    let f = get_3d_noise_helper(x, y, z, fractal_settings);
                    for j in 0..remainder {
                        let n = f.array[j];
                        result[i] = n;
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
        for i in 0..8 {
            if min_s.array[i] < min {
                min = min_s.array[i];
            }
            if max_s.array[i] > max {
                max = max_s.array[i];
            }
        }
        (result, min, max)
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
    scale_min: f32,
    scale_max: f32,
) -> Vec<f32> {
    let (mut noise, min, max) = get_3d_noise(
        start_x,
        width,
        start_y,
        height,
        start_z,
        depth,
        fractal_settings,
    );
    scale_array(scale_min, scale_max, min, max, &mut noise);
    noise
}
