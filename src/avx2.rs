#![allow(non_snake_case)]

use super::*;
use shared::*;
use std::arch::x86_64::*;
use std::f32;

union M256Array {
    simd: __m256,
    array: [f32; 8],
}

union M256iArray {
    simd: __m256i,
    array: [i32; 8],
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

unsafe fn dot_simd(x1: __m256, y1: __m256, x2: __m256, y2: __m256) -> __m256 {
    _mm256_add_ps(_mm256_mul_ps(x1, x2), _mm256_mul_ps(y1, y2))
}

#[cfg(any(target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
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
        &PERM_MOD12 as *const i32,
        _mm256_add_epi32(ii, _mm256_i32gather_epi32(&PERM as *const i32, jj, 4)),
        4,
    );

    let gi1 = _mm256_i32gather_epi32(
        &PERM_MOD12 as *const i32,
        _mm256_add_epi32(
            _mm256_add_epi32(ii, i1),
            _mm256_i32gather_epi32(&PERM as *const i32, _mm256_add_epi32(jj, j1), 4),
        ),
        4,
    );

    let gi2 = _mm256_i32gather_epi32(
        &PERM_MOD12 as *const i32,
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

    let gi0x = _mm256_i32gather_ps(&GRAD_X as *const f32, gi0, 4);
    let gi1x = _mm256_i32gather_ps(&GRAD_X as *const f32, gi1, 4);
    let gi2x = _mm256_i32gather_ps(&GRAD_X as *const f32, gi2, 4);
    let gi0y = _mm256_i32gather_ps(&GRAD_Y as *const f32, gi0, 4);
    let gi1y = _mm256_i32gather_ps(&GRAD_Y as *const f32, gi1, 4);
    let gi2y = _mm256_i32gather_ps(&GRAD_Y as *const f32, gi2, 4);

    let mut n0 = _mm256_mul_ps(t0q, dot_simd(gi0x, gi0y, x0, y0));
    let mut n1 = _mm256_mul_ps(t1q, dot_simd(gi1x, gi1y, x1, y1));
    let mut n2 = _mm256_mul_ps(t2q, dot_simd(gi2x, gi2y, x2, y2));

    let mut cond = _mm256_cmp_ps(t0, _mm256_setzero_ps(), _CMP_LT_OQ);
    n0 = _mm256_blendv_ps(n0, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(), _CMP_LT_OQ);
    n1 = _mm256_blendv_ps(n1, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(), _CMP_LT_OQ);
    n2 = _mm256_blendv_ps(n2, _mm256_setzero_ps(), cond);

    _mm256_add_ps(n0, _mm256_add_ps(n1, n2))
}

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
    octaves: i32,
) -> __m256 {
    let mut xf = _mm256_mul_ps(x, freq);
    let mut yf = _mm256_mul_ps(y, freq);
    let mut result = simplex_2d(xf, yf);
    let mut amp = _mm256_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm256_mul_ps(xf, lac);
        yf = _mm256_mul_ps(yf, lac);
        amp = _mm256_mul_ps(amp, gain);
        result = _mm256_add_ps(result, _mm256_mul_ps(simplex_2d(xf, yf), amp));
    }

    result
}

pub unsafe fn turbulence_2d(
    x: __m256,
    y: __m256,
    freq: __m256,
    lac: __m256,
    gain: __m256,
    octaves: i32,
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
                _mm256_add_ps(
                    _mm256_mul_ps(_mm256_set1_ps(multiplier), _mm256_loadu_ps(&mut data[i])),
                    _mm256_set1_ps(offset),
                ),
            );
            i = i + 8;
        }
        i = data.len() - (data.len() % 8);
        while i < data.len() {
            data[i] = data[i] * multiplier + offset;
            i = i + 1;
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
        let mut result = Vec::with_capacity(width * height);
        result.set_len(width * height);
        let mut y = _mm256_set1_ps(start_y);
        let mut i = 0;
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
                _mm256_storeu_ps(&mut result[i], f);
                i = i + 8;
                x = _mm256_add_ps(x, _mm256_set1_ps(8.0));
            }
            if width % 8 != 0 {
                let f = get_2d_noise_helper(x, y, fractal_settings);
                for j in 0..width % 8 {
                    result[i] = f.array[j];
                    i = i + 1;
                }
            }
            y = _mm256_add_ps(y, _mm256_set1_ps(1.0));
        }
        let mut min = f32::MAX;
        let mut max = f32::MIN;
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

unsafe fn dot_simd_3d(
    x1: __m256,
    y1: __m256,
    z1: __m256,
    x2: __m256,
    y2: __m256,
    z2: __m256,
) -> __m256 {
    let xx = _mm256_mul_ps(x1, x2);
    let yy = _mm256_mul_ps(y1, y2);
    let zz = _mm256_mul_ps(z1, z2);
    _mm256_add_ps(xx, _mm256_add_ps(yy, zz))
}

unsafe fn simplex_3d(x: __m256, y: __m256, z: __m256) -> __m256 {
    let s = M256Array {
        simd: _mm256_mul_ps(F3, _mm256_add_ps(x, _mm256_add_ps(y, z))),
    };
    let mut i = M256iArray {
        simd: _mm256_cvtps_epi32(_mm256_floor_ps(_mm256_add_ps(x, s.simd))),
    };
    let mut j = M256iArray {
        simd: _mm256_cvtps_epi32(_mm256_floor_ps(_mm256_add_ps(y, s.simd))),
    };
    let mut k = M256iArray {
        simd: _mm256_cvtps_epi32(_mm256_floor_ps(_mm256_add_ps(z, s.simd))),
    };

    let t = _mm256_mul_ps(
        _mm256_cvtepi32_ps(_mm256_add_epi32(i.simd, _mm256_add_epi32(j.simd, k.simd))),
        G3,
    );
    let X0 = _mm256_sub_ps(_mm256_cvtepi32_ps(i.simd), t);
    let Y0 = _mm256_sub_ps(_mm256_cvtepi32_ps(j.simd), t);
    let Z0 = _mm256_sub_ps(_mm256_cvtepi32_ps(k.simd), t);
    let x0 = _mm256_sub_ps(x, X0);
    let y0 = _mm256_sub_ps(y, Y0);
    let z0 = _mm256_sub_ps(z, Z0);

    let i1 = M256iArray {
        simd: _mm256_and_si256(
            _mm256_set1_epi32(1),
            _mm256_and_si256(
                _mm256_castps_si256(_mm256_cmp_ps(x0, y0, _CMP_GE_OQ)),
                _mm256_castps_si256(_mm256_cmp_ps(x0, z0, _CMP_GE_OQ)),
            ),
        ),
    };
    let j1 = M256iArray {
        simd: _mm256_and_si256(
            _mm256_set1_epi32(1),
            _mm256_and_si256(
                _mm256_castps_si256(_mm256_cmp_ps(y0, x0, _CMP_GT_OQ)),
                _mm256_castps_si256(_mm256_cmp_ps(y0, z0, _CMP_GT_OQ)),
            ),
        ),
    };
    let k1 = M256iArray {
        simd: _mm256_and_si256(
            _mm256_set1_epi32(1),
            _mm256_and_si256(
                _mm256_castps_si256(_mm256_cmp_ps(z0, x0, _CMP_GT_OQ)),
                _mm256_castps_si256(_mm256_cmp_ps(z0, y0, _CMP_GT_OQ)),
            ),
        ),
    };

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

    let i2 = M256iArray {
        simd: _mm256_and_si256(
            _mm256_set1_epi32(1),
            _mm256_or_si256(i1.simd, _mm256_or_si256(yx_xz, zx_xy)),
        ),
    };
    let j2 = M256iArray {
        simd: _mm256_and_si256(
            _mm256_set1_epi32(1),
            _mm256_or_si256(j1.simd, _mm256_or_si256(xy_yz, zy_yx)),
        ),
    };
    let k2 = M256iArray {
        simd: _mm256_and_si256(
            _mm256_set1_epi32(1),
            _mm256_or_si256(k1.simd, _mm256_or_si256(yz_zx, xz_zy)),
        ),
    };

    let x1 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i1.simd)), G3);
    let y1 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j1.simd)), G3);
    let z1 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k1.simd)), G3);
    let x2 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i2.simd)), F3);
    let y2 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j2.simd)), F3);
    let z2 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_cvtepi32_ps(k2.simd)), F3);
    let x3 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_set1_ps(1.0)), POINT_FIVE);
    let y3 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_set1_ps(1.0)), POINT_FIVE);
    let z3 = _mm256_add_ps(_mm256_sub_ps(z0, _mm256_set1_ps(1.0)), POINT_FIVE);

    i.simd = _mm256_and_si256(i.simd, _mm256_set1_epi32(0xff));
    j.simd = _mm256_and_si256(j.simd, _mm256_set1_epi32(0xff));
    k.simd = _mm256_and_si256(k.simd, _mm256_set1_epi32(0xff));

    let gi0 = M256iArray {
        array: [
            PERM_MOD12
                [(i.array[0] + PERM[(j.array[0] + PERM[k.array[0] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[1] + PERM[(j.array[1] + PERM[k.array[1] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[2] + PERM[(j.array[2] + PERM[k.array[2] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[3] + PERM[(j.array[3] + PERM[k.array[3] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[4] + PERM[(j.array[4] + PERM[k.array[4] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[5] + PERM[(j.array[5] + PERM[k.array[5] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[6] + PERM[(j.array[6] + PERM[k.array[6] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[7] + PERM[(j.array[7] + PERM[k.array[7] as usize]) as usize]) as usize],
        ],
    };

    let gi1 = M256iArray {
        array: [
            PERM_MOD12[(i.array[0] + i1.array[0]
                           + PERM[(j.array[0]
                                      + j1.array[0]
                                      + PERM[(k.array[0] + k1.array[0]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[1] + i1.array[1]
                           + PERM[(j.array[1]
                                      + j1.array[1]
                                      + PERM[(k.array[1] + k1.array[1]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[2] + i1.array[2]
                           + PERM[(j.array[2]
                                      + j1.array[2]
                                      + PERM[(k.array[2] + k1.array[2]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[3] + i1.array[3]
                           + PERM[(j.array[3]
                                      + j1.array[3]
                                      + PERM[(k.array[3] + k1.array[3]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[4] + i1.array[4]
                           + PERM[(j.array[4]
                                      + j1.array[4]
                                      + PERM[(k.array[4] + k1.array[4]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[5] + i1.array[5]
                           + PERM[(j.array[5]
                                      + j1.array[5]
                                      + PERM[(k.array[5] + k1.array[5]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[6] + i1.array[6]
                           + PERM[(j.array[6]
                                      + j1.array[6]
                                      + PERM[(k.array[6] + k1.array[6]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[7] + i1.array[7]
                           + PERM[(j.array[7]
                                      + j1.array[7]
                                      + PERM[(k.array[7] + k1.array[7]) as usize])
                                      as usize]) as usize],
        ],
    };
    let gi2 = M256iArray {
        array: [
            PERM_MOD12[(i.array[0] + i2.array[0]
                           + PERM[(j.array[0]
                                      + j2.array[0]
                                      + PERM[(k.array[0] + k2.array[0]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[1] + i2.array[1]
                           + PERM[(j.array[1]
                                      + j2.array[1]
                                      + PERM[(k.array[1] + k2.array[1]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[2] + i2.array[2]
                           + PERM[(j.array[2]
                                      + j2.array[2]
                                      + PERM[(k.array[2] + k2.array[2]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[3] + i2.array[3]
                           + PERM[(j.array[3]
                                      + j2.array[3]
                                      + PERM[(k.array[3] + k2.array[3]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[4] + i2.array[4]
                           + PERM[(j.array[4]
                                      + j2.array[4]
                                      + PERM[(k.array[4] + k2.array[4]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[5] + i2.array[5]
                           + PERM[(j.array[5]
                                      + j2.array[5]
                                      + PERM[(k.array[5] + k2.array[5]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[6] + i2.array[6]
                           + PERM[(j.array[6]
                                      + j2.array[6]
                                      + PERM[(k.array[6] + k2.array[6]) as usize])
                                      as usize]) as usize],
            PERM_MOD12[(i.array[7] + i2.array[7]
                           + PERM[(j.array[7]
                                      + j2.array[7]
                                      + PERM[(k.array[7] + k2.array[7]) as usize])
                                      as usize]) as usize],
        ],
    };
    let gi3 = M256iArray {
        array: [
            PERM_MOD12[(i.array[0]
                           + 1
                           + PERM[(j.array[0] + 1 + PERM[k.array[0] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[1]
                           + 1
                           + PERM[(j.array[1] + 1 + PERM[k.array[1] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[2]
                           + 1
                           + PERM[(j.array[2] + 1 + PERM[k.array[2] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[3]
                           + 1
                           + PERM[(j.array[3] + 1 + PERM[k.array[3] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[4]
                           + 1
                           + PERM[(j.array[4] + 1 + PERM[k.array[4] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[5]
                           + 1
                           + PERM[(j.array[5] + 1 + PERM[k.array[5] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[6]
                           + 1
                           + PERM[(j.array[6] + 1 + PERM[k.array[6] as usize]) as usize])
                           as usize],
            PERM_MOD12[(i.array[7]
                           + 1
                           + PERM[(j.array[7] + 1 + PERM[k.array[7] as usize]) as usize])
                           as usize],
        ],
    };

    let t0 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(_mm256_set1_ps(0.6), _mm256_mul_ps(x0, x0)),
            _mm256_mul_ps(y0, y0),
        ),
        _mm256_mul_ps(z0, z0),
    );
    let t1 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(_mm256_set1_ps(0.6), _mm256_mul_ps(x1, x1)),
            _mm256_mul_ps(y1, y1),
        ),
        _mm256_mul_ps(z1, z1),
    );
    let t2 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(_mm256_set1_ps(0.6), _mm256_mul_ps(x2, x2)),
            _mm256_mul_ps(y2, y2),
        ),
        _mm256_mul_ps(z2, z2),
    );
    let t3 = _mm256_sub_ps(
        _mm256_sub_ps(
            _mm256_sub_ps(_mm256_set1_ps(0.6), _mm256_mul_ps(x3, x3)),
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

    let gi0x = M256Array {
        array: [
            GRAD_X[gi0.array[0] as usize],
            GRAD_X[gi0.array[1] as usize],
            GRAD_X[gi0.array[2] as usize],
            GRAD_X[gi0.array[3] as usize],
            GRAD_X[gi0.array[4] as usize],
            GRAD_X[gi0.array[5] as usize],
            GRAD_X[gi0.array[6] as usize],
            GRAD_X[gi0.array[7] as usize],
        ],
    };
    let gi0y = M256Array {
        array: [
            GRAD_Y[gi0.array[0] as usize],
            GRAD_Y[gi0.array[1] as usize],
            GRAD_Y[gi0.array[2] as usize],
            GRAD_Y[gi0.array[3] as usize],
            GRAD_Y[gi0.array[4] as usize],
            GRAD_Y[gi0.array[5] as usize],
            GRAD_Y[gi0.array[6] as usize],
            GRAD_Y[gi0.array[7] as usize],
        ],
    };
    let gi0z = M256Array {
        array: [
            GRAD_Z[gi0.array[0] as usize],
            GRAD_Z[gi0.array[1] as usize],
            GRAD_Z[gi0.array[2] as usize],
            GRAD_Z[gi0.array[3] as usize],
            GRAD_Z[gi0.array[4] as usize],
            GRAD_Z[gi0.array[5] as usize],
            GRAD_Z[gi0.array[6] as usize],
            GRAD_Z[gi0.array[7] as usize],
        ],
    };
    let gi1x = M256Array {
        array: [
            GRAD_X[gi1.array[0] as usize],
            GRAD_X[gi1.array[1] as usize],
            GRAD_X[gi1.array[2] as usize],
            GRAD_X[gi1.array[3] as usize],
            GRAD_X[gi1.array[4] as usize],
            GRAD_X[gi1.array[5] as usize],
            GRAD_X[gi1.array[6] as usize],
            GRAD_X[gi1.array[7] as usize],
        ],
    };
    let gi1y = M256Array {
        array: [
            GRAD_Y[gi1.array[0] as usize],
            GRAD_Y[gi1.array[1] as usize],
            GRAD_Y[gi1.array[2] as usize],
            GRAD_Y[gi1.array[3] as usize],
            GRAD_Y[gi1.array[4] as usize],
            GRAD_Y[gi1.array[5] as usize],
            GRAD_Y[gi1.array[6] as usize],
            GRAD_Y[gi1.array[7] as usize],
        ],
    };
    let gi1z = M256Array {
        array: [
            GRAD_Z[gi1.array[0] as usize],
            GRAD_Z[gi1.array[1] as usize],
            GRAD_Z[gi1.array[2] as usize],
            GRAD_Z[gi1.array[3] as usize],
            GRAD_Z[gi1.array[4] as usize],
            GRAD_Z[gi1.array[5] as usize],
            GRAD_Z[gi1.array[6] as usize],
            GRAD_Z[gi1.array[7] as usize],
        ],
    };
    let gi2x = M256Array {
        array: [
            GRAD_X[gi2.array[0] as usize],
            GRAD_X[gi2.array[1] as usize],
            GRAD_X[gi2.array[2] as usize],
            GRAD_X[gi2.array[3] as usize],
            GRAD_X[gi2.array[4] as usize],
            GRAD_X[gi2.array[5] as usize],
            GRAD_X[gi2.array[6] as usize],
            GRAD_X[gi2.array[7] as usize],
        ],
    };
    let gi2y = M256Array {
        array: [
            GRAD_Y[gi2.array[0] as usize],
            GRAD_Y[gi2.array[1] as usize],
            GRAD_Y[gi2.array[2] as usize],
            GRAD_Y[gi2.array[3] as usize],
            GRAD_Y[gi2.array[4] as usize],
            GRAD_Y[gi2.array[5] as usize],
            GRAD_Y[gi2.array[6] as usize],
            GRAD_Y[gi2.array[7] as usize],
        ],
    };
    let gi2z = M256Array {
        array: [
            GRAD_Z[gi2.array[0] as usize],
            GRAD_Z[gi2.array[1] as usize],
            GRAD_Z[gi2.array[2] as usize],
            GRAD_Z[gi2.array[3] as usize],
            GRAD_Z[gi2.array[4] as usize],
            GRAD_Z[gi2.array[5] as usize],
            GRAD_Z[gi2.array[6] as usize],
            GRAD_Z[gi2.array[7] as usize],
        ],
    };
    let gi3x = M256Array {
        array: [
            GRAD_X[gi3.array[0] as usize],
            GRAD_X[gi3.array[1] as usize],
            GRAD_X[gi3.array[2] as usize],
            GRAD_X[gi3.array[3] as usize],
            GRAD_X[gi3.array[4] as usize],
            GRAD_X[gi3.array[5] as usize],
            GRAD_X[gi3.array[6] as usize],
            GRAD_X[gi3.array[7] as usize],
        ],
    };
    let gi3y = M256Array {
        array: [
            GRAD_Y[gi3.array[0] as usize],
            GRAD_Y[gi3.array[1] as usize],
            GRAD_Y[gi3.array[2] as usize],
            GRAD_Y[gi3.array[3] as usize],
            GRAD_Y[gi3.array[4] as usize],
            GRAD_Y[gi3.array[5] as usize],
            GRAD_Y[gi3.array[6] as usize],
            GRAD_Y[gi3.array[7] as usize],
        ],
    };
    let gi3z = M256Array {
        array: [
            GRAD_Z[gi3.array[0] as usize],
            GRAD_Z[gi3.array[1] as usize],
            GRAD_Z[gi3.array[2] as usize],
            GRAD_Z[gi3.array[3] as usize],
            GRAD_Z[gi3.array[4] as usize],
            GRAD_Z[gi3.array[5] as usize],
            GRAD_Z[gi3.array[6] as usize],
            GRAD_Z[gi3.array[7] as usize],
        ],
    };

    let mut n0 = _mm256_mul_ps(
        t0q,
        dot_simd_3d(gi0x.simd, gi0y.simd, gi0z.simd, x0, y0, z0),
    );
    let mut n1 = _mm256_mul_ps(
        t1q,
        dot_simd_3d(gi1x.simd, gi1y.simd, gi1z.simd, x1, y1, z1),
    );
    let mut n2 = _mm256_mul_ps(
        t2q,
        dot_simd_3d(gi2x.simd, gi2y.simd, gi2z.simd, x2, y2, z2),
    );
    let mut n3 = _mm256_mul_ps(
        t3q,
        dot_simd_3d(gi3x.simd, gi3y.simd, gi3z.simd, x3, y3, z3),
    );

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
    octaves: i32,
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
    octaves: i32,
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
pub fn get_3d_noise(
    start_x: f32,
    width: usize,
    start_y: f32,
    height: usize,
    start_z: f32,
    depth: usize,
    fractal_settings: FractalSettings,
) -> Vec<f32> {
    unsafe {
        let mut result = Vec::with_capacity(width * height * depth);
        let mut z = _mm256_set1_ps(start_z);
        for _ in 0..depth {
            let mut y = _mm256_set1_ps(start_y);
            for _ in 0..height {
                let mut x = _mm256_set_ps(
                    start_x,
                    start_x + 1.0,
                    start_x + 2.0,
                    start_x + 3.0,
                    start_x + 4.0,
                    start_x + 5.0,
                    start_x + 6.0,
                    start_x + 7.0,
                );
                for _ in 0..width {
                    result.push(match fractal_settings.noise_type {
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
                    });
                    x = _mm256_add_ps(x, _mm256_set1_ps(4.0));
                }
                y = _mm256_add_ps(y, _mm256_set1_ps(1.0));
            }
            z = _mm256_add_ps(z, _mm256_set1_ps(1.0));
        }
        ::std::mem::transmute::<Vec<__m256>, Vec<f32>>(result)
    }
}
