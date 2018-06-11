#![allow(non_snake_case)]

use super::*;
use shared::*;
use shared_sse::*;
use std::arch::x86_64::*;
use std::f32;

unsafe fn simplex_2d(x: __m128, y: __m128) -> __m128 {
    let s = _mm_mul_ps(F2, _mm_add_ps(x, y));
    let ips = _mm_floor_ps(_mm_add_ps(x, s));
    let jps = _mm_floor_ps(_mm_add_ps(y, s));

    let i = _mm_cvtps_epi32(ips);
    let j = _mm_cvtps_epi32(jps);

    let t = _mm_mul_ps(_mm_cvtepi32_ps(_mm_add_epi32(i, j)), G2);

    let x0 = _mm_sub_ps(x, _mm_sub_ps(ips, t));
    let y0 = _mm_sub_ps(y, _mm_sub_ps(jps, t));

    let i1 = M128iArray {
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_castps_si128(_mm_cmpge_ps(x0, y0))),
    };
    let j1 = M128iArray {
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_castps_si128(_mm_cmpgt_ps(y0, x0))),
    };

    let x1 = _mm_add_ps(_mm_sub_ps(x0, _mm_cvtepi32_ps(i1.simd)), G2);
    let y1 = _mm_add_ps(_mm_sub_ps(y0, _mm_cvtepi32_ps(j1.simd)), G2);
    let x2 = _mm_add_ps(_mm_sub_ps(x0, _mm_set1_ps(1.0)), G22);
    let y2 = _mm_add_ps(_mm_sub_ps(y0, _mm_set1_ps(1.0)), G22);

    let ii = M128iArray {
        simd: _mm_and_si128(i, _mm_set1_epi32(0xff)),
    };
    let jj = M128iArray {
        simd: _mm_and_si128(j, _mm_set1_epi32(0xff)),
    };

    let gi0 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0] + PERM[jj.array[0] as usize]) as usize],
            PERM_MOD12[(ii.array[1] + PERM[jj.array[1] as usize]) as usize],
            PERM_MOD12[(ii.array[2] + PERM[jj.array[2] as usize]) as usize],
            PERM_MOD12[(ii.array[3] + PERM[jj.array[3] as usize]) as usize],
        ],
    };
    let gi1 = M128iArray {
        array: [
            PERM_MOD12
                [(ii.array[0] + i1.array[0] + PERM[(jj.array[0] + j1.array[0]) as usize]) as usize],
            PERM_MOD12
                [(ii.array[1] + i1.array[1] + PERM[(jj.array[1] + j1.array[1]) as usize]) as usize],
            PERM_MOD12
                [(ii.array[2] + i1.array[2] + PERM[(jj.array[2] + j1.array[2]) as usize]) as usize],
            PERM_MOD12
                [(ii.array[3] + i1.array[3] + PERM[(jj.array[3] + j1.array[3]) as usize]) as usize],
        ],
    };
    let gi2 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0] + 1 + PERM[(jj.array[0] + 1) as usize]) as usize],
            PERM_MOD12[(ii.array[1] + 1 + PERM[(jj.array[1] + 1) as usize]) as usize],
            PERM_MOD12[(ii.array[2] + 1 + PERM[(jj.array[2] + 1) as usize]) as usize],
            PERM_MOD12[(ii.array[3] + 1 + PERM[(jj.array[3] + 1) as usize]) as usize],
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

    let gi0x = M128Array {
        array: [
            GRAD_X[gi0.array[0] as usize],
            GRAD_X[gi0.array[1] as usize],
            GRAD_X[gi0.array[2] as usize],
            GRAD_X[gi0.array[3] as usize],
        ],
    };
    let gi1x = M128Array {
        array: [
            GRAD_X[gi1.array[0] as usize],
            GRAD_X[gi1.array[1] as usize],
            GRAD_X[gi1.array[2] as usize],
            GRAD_X[gi1.array[3] as usize],
        ],
    };
    let gi2x = M128Array {
        array: [
            GRAD_X[gi2.array[0] as usize],
            GRAD_X[gi2.array[1] as usize],
            GRAD_X[gi2.array[2] as usize],
            GRAD_X[gi2.array[3] as usize],
        ],
    };
    let gi0y = M128Array {
        array: [
            GRAD_Y[gi0.array[0] as usize],
            GRAD_Y[gi0.array[1] as usize],
            GRAD_Y[gi0.array[2] as usize],
            GRAD_Y[gi0.array[3] as usize],
        ],
    };
    let gi1y = M128Array {
        array: [
            GRAD_Y[gi1.array[0] as usize],
            GRAD_Y[gi1.array[1] as usize],
            GRAD_Y[gi1.array[2] as usize],
            GRAD_Y[gi1.array[3] as usize],
        ],
    };
    let gi2y = M128Array {
        array: [
            GRAD_Y[gi2.array[0] as usize],
            GRAD_Y[gi2.array[1] as usize],
            GRAD_Y[gi2.array[2] as usize],
            GRAD_Y[gi2.array[3] as usize],
        ],
    };

    let mut n0 = _mm_mul_ps(t0q, dot_simd(gi0x.simd, gi0y.simd, x0, y0));
    let mut n1 = _mm_mul_ps(t1q, dot_simd(gi1x.simd, gi1y.simd, x1, y1));
    let mut n2 = _mm_mul_ps(t2q, dot_simd(gi2x.simd, gi2y.simd, x2, y2));

    let mut cond = _mm_cmplt_ps(t0, _mm_setzero_ps());
    n0 = _mm_blendv_ps(n0, _mm_setzero_ps(), cond);
    cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
    n1 = _mm_blendv_ps(n1, _mm_setzero_ps(), cond);
    cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_blendv_ps(n2, _mm_setzero_ps(), cond);

    _mm_add_ps(n0, _mm_add_ps(n1, n2))
}

pub unsafe fn fbm_2d(
    x: __m128,
    y: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: i32,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut result = simplex_2d(xf, yf);
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_mul_ps(simplex_2d(xf, yf), amp));
    }

    result
}

pub unsafe fn turbulence_2d(
    x: __m128,
    y: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: i32,
) -> __m128 {
    let mut xf = _mm_mul_ps(x, freq);
    let mut yf = _mm_mul_ps(y, freq);
    let mut result = _mm_abs_ps(simplex_2d(xf, yf));
    let mut amp = _mm_set1_ps(1.0);

    for _ in 1..octaves {
        xf = _mm_mul_ps(xf, lac);
        yf = _mm_mul_ps(yf, lac);
        amp = _mm_mul_ps(amp, gain);
        result = _mm_add_ps(result, _mm_abs_ps(_mm_mul_ps(simplex_2d(xf, yf), amp)));
    }

    result
}

unsafe fn dot_simd_3d(
    x1: __m128,
    y1: __m128,
    z1: __m128,
    x2: __m128,
    y2: __m128,
    z2: __m128,
) -> __m128 {
    let xx = _mm_mul_ps(x1, x2);
    let yy = _mm_mul_ps(y1, y2);
    let zz = _mm_mul_ps(z1, z2);
    _mm_add_ps(xx, _mm_add_ps(yy, zz))
}

unsafe fn simplex_3d(x: __m128, y: __m128, z: __m128) -> __m128 {
    let s = M128Array {
        simd: _mm_mul_ps(F3, _mm_add_ps(x, _mm_add_ps(y, z))),
    };
    let mut i = M128iArray {
        simd: _mm_cvtps_epi32(_mm_floor_ps(_mm_add_ps(x, s.simd))),
    };
    let mut j = M128iArray {
        simd: _mm_cvtps_epi32(_mm_floor_ps(_mm_add_ps(y, s.simd))),
    };
    let mut k = M128iArray {
        simd: _mm_cvtps_epi32(_mm_floor_ps(_mm_add_ps(z, s.simd))),
    };

    let t = _mm_mul_ps(
        _mm_cvtepi32_ps(_mm_add_epi32(i.simd, _mm_add_epi32(j.simd, k.simd))),
        G3,
    );
    let X0 = _mm_sub_ps(_mm_cvtepi32_ps(i.simd), t);
    let Y0 = _mm_sub_ps(_mm_cvtepi32_ps(j.simd), t);
    let Z0 = _mm_sub_ps(_mm_cvtepi32_ps(k.simd), t);
    let x0 = _mm_sub_ps(x, X0);
    let y0 = _mm_sub_ps(y, Y0);
    let z0 = _mm_sub_ps(z, Z0);

    let i1 = M128iArray {
        simd: _mm_and_si128(
            _mm_set1_epi32(1),
            _mm_and_si128(
                _mm_castps_si128(_mm_cmpge_ps(x0, y0)),
                _mm_castps_si128(_mm_cmpge_ps(x0, z0)),
            ),
        ),
    };
    let j1 = M128iArray {
        simd: _mm_and_si128(
            _mm_set1_epi32(1),
            _mm_and_si128(
                _mm_castps_si128(_mm_cmpgt_ps(y0, x0)),
                _mm_castps_si128(_mm_cmpgt_ps(y0, z0)),
            ),
        ),
    };
    let k1 = M128iArray {
        simd: _mm_and_si128(
            _mm_set1_epi32(1),
            _mm_and_si128(
                _mm_castps_si128(_mm_cmpgt_ps(z0, x0)),
                _mm_castps_si128(_mm_cmpgt_ps(z0, y0)),
            ),
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
        simd: _mm_and_si128(
            _mm_set1_epi32(1),
            _mm_or_si128(i1.simd, _mm_or_si128(yx_xz, zx_xy)),
        ),
    };
    let j2 = M128iArray {
        simd: _mm_and_si128(
            _mm_set1_epi32(1),
            _mm_or_si128(j1.simd, _mm_or_si128(xy_yz, zy_yx)),
        ),
    };
    let k2 = M128iArray {
        simd: _mm_and_si128(
            _mm_set1_epi32(1),
            _mm_or_si128(k1.simd, _mm_or_si128(yz_zx, xz_zy)),
        ),
    };

    let x1 = _mm_add_ps(_mm_sub_ps(x0, _mm_cvtepi32_ps(i1.simd)), G3);
    let y1 = _mm_add_ps(_mm_sub_ps(y0, _mm_cvtepi32_ps(j1.simd)), G3);
    let z1 = _mm_add_ps(_mm_sub_ps(z0, _mm_cvtepi32_ps(k1.simd)), G3);
    let x2 = _mm_add_ps(_mm_sub_ps(x0, _mm_cvtepi32_ps(i2.simd)), F3);
    let y2 = _mm_add_ps(_mm_sub_ps(y0, _mm_cvtepi32_ps(j2.simd)), F3);
    let z2 = _mm_add_ps(_mm_sub_ps(z0, _mm_cvtepi32_ps(k2.simd)), F3);
    let x3 = _mm_add_ps(_mm_sub_ps(x0, _mm_set1_ps(1.0)), POINT_FIVE);
    let y3 = _mm_add_ps(_mm_sub_ps(y0, _mm_set1_ps(1.0)), POINT_FIVE);
    let z3 = _mm_add_ps(_mm_sub_ps(z0, _mm_set1_ps(1.0)), POINT_FIVE);

    i.simd = _mm_and_si128(i.simd, _mm_set1_epi32(0xff));
    j.simd = _mm_and_si128(j.simd, _mm_set1_epi32(0xff));
    k.simd = _mm_and_si128(k.simd, _mm_set1_epi32(0xff));

    let gi0 = M128iArray {
        array: [
            PERM_MOD12
                [(i.array[0] + PERM[(j.array[0] + PERM[k.array[0] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[1] + PERM[(j.array[1] + PERM[k.array[1] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[2] + PERM[(j.array[2] + PERM[k.array[2] as usize]) as usize]) as usize],
            PERM_MOD12
                [(i.array[3] + PERM[(j.array[3] + PERM[k.array[3] as usize]) as usize]) as usize],
        ],
    };

    let gi1 = M128iArray {
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
        ],
    };
    let gi2 = M128iArray {
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
        ],
    };
    let gi3 = M128iArray {
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
        ],
    };

    let t0 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x0, x0)),
            _mm_mul_ps(y0, y0),
        ),
        _mm_mul_ps(z0, z0),
    );
    let t1 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x1, x1)),
            _mm_mul_ps(y1, y1),
        ),
        _mm_mul_ps(z1, z1),
    );
    let t2 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x2, x2)),
            _mm_mul_ps(y2, y2),
        ),
        _mm_mul_ps(z2, z2),
    );
    let t3 = _mm_sub_ps(
        _mm_sub_ps(
            _mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x3, x3)),
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

    let gi0x = M128Array {
        array: [
            GRAD_X[gi0.array[0] as usize],
            GRAD_X[gi0.array[1] as usize],
            GRAD_X[gi0.array[2] as usize],
            GRAD_X[gi0.array[3] as usize],
        ],
    };
    let gi0y = M128Array {
        array: [
            GRAD_Y[gi0.array[0] as usize],
            GRAD_Y[gi0.array[1] as usize],
            GRAD_Y[gi0.array[2] as usize],
            GRAD_Y[gi0.array[3] as usize],
        ],
    };
    let gi0z = M128Array {
        array: [
            GRAD_Z[gi0.array[0] as usize],
            GRAD_Z[gi0.array[1] as usize],
            GRAD_Z[gi0.array[2] as usize],
            GRAD_Z[gi0.array[3] as usize],
        ],
    };
    let gi1x = M128Array {
        array: [
            GRAD_X[gi1.array[0] as usize],
            GRAD_X[gi1.array[1] as usize],
            GRAD_X[gi1.array[2] as usize],
            GRAD_X[gi1.array[3] as usize],
        ],
    };
    let gi1y = M128Array {
        array: [
            GRAD_Y[gi1.array[0] as usize],
            GRAD_Y[gi1.array[1] as usize],
            GRAD_Y[gi1.array[2] as usize],
            GRAD_Y[gi1.array[3] as usize],
        ],
    };
    let gi1z = M128Array {
        array: [
            GRAD_Z[gi1.array[0] as usize],
            GRAD_Z[gi1.array[1] as usize],
            GRAD_Z[gi1.array[2] as usize],
            GRAD_Z[gi1.array[3] as usize],
        ],
    };
    let gi2x = M128Array {
        array: [
            GRAD_X[gi2.array[0] as usize],
            GRAD_X[gi2.array[1] as usize],
            GRAD_X[gi2.array[2] as usize],
            GRAD_X[gi2.array[3] as usize],
        ],
    };
    let gi2y = M128Array {
        array: [
            GRAD_Y[gi2.array[0] as usize],
            GRAD_Y[gi2.array[1] as usize],
            GRAD_Y[gi2.array[2] as usize],
            GRAD_Y[gi2.array[3] as usize],
        ],
    };
    let gi2z = M128Array {
        array: [
            GRAD_Z[gi2.array[0] as usize],
            GRAD_Z[gi2.array[1] as usize],
            GRAD_Z[gi2.array[2] as usize],
            GRAD_Z[gi2.array[3] as usize],
        ],
    };
    let gi3x = M128Array {
        array: [
            GRAD_X[gi3.array[0] as usize],
            GRAD_X[gi3.array[1] as usize],
            GRAD_X[gi3.array[2] as usize],
            GRAD_X[gi3.array[3] as usize],
        ],
    };
    let gi3y = M128Array {
        array: [
            GRAD_Y[gi3.array[0] as usize],
            GRAD_Y[gi3.array[1] as usize],
            GRAD_Y[gi3.array[2] as usize],
            GRAD_Y[gi3.array[3] as usize],
        ],
    };
    let gi3z = M128Array {
        array: [
            GRAD_Z[gi3.array[0] as usize],
            GRAD_Z[gi3.array[1] as usize],
            GRAD_Z[gi3.array[2] as usize],
            GRAD_Z[gi3.array[3] as usize],
        ],
    };

    let mut n0 = _mm_mul_ps(
        t0q,
        dot_simd_3d(gi0x.simd, gi0y.simd, gi0z.simd, x0, y0, z0),
    );
    let mut n1 = _mm_mul_ps(
        t1q,
        dot_simd_3d(gi1x.simd, gi1y.simd, gi1z.simd, x1, y1, z1),
    );
    let mut n2 = _mm_mul_ps(
        t2q,
        dot_simd_3d(gi2x.simd, gi2y.simd, gi2z.simd, x2, y2, z2),
    );
    let mut n3 = _mm_mul_ps(
        t3q,
        dot_simd_3d(gi3x.simd, gi3y.simd, gi3z.simd, x3, y3, z3),
    );

    //if ti < 0 then 0 else ni
    let mut cond = _mm_cmplt_ps(t0, _mm_setzero_ps());
    n0 = _mm_blendv_ps(n0, _mm_setzero_ps(), cond);
    cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
    n1 = _mm_blendv_ps(n1, _mm_setzero_ps(), cond);
    cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_blendv_ps(n2, _mm_setzero_ps(), cond);
    cond = _mm_cmplt_ps(t3, _mm_setzero_ps());
    n3 = _mm_blendv_ps(n3, _mm_setzero_ps(), cond);

    _mm_add_ps(n0, _mm_add_ps(n1, _mm_add_ps(n2, n3)))
}

pub unsafe fn fbm_3d(
    x: __m128,
    y: __m128,
    z: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: i32,
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

pub unsafe fn turbulence_3d(
    x: __m128,
    y: __m128,
    z: __m128,
    freq: __m128,
    lac: __m128,
    gain: __m128,
    octaves: i32,
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

unsafe fn get_2d_noise_helper(
    x: __m128,
    y: __m128,
    fractal_settings: FractalSettings,
) -> M128Array {
    M128Array {
        simd: match fractal_settings.noise_type {
            NoiseType::FBM => fbm_2d(
                x,
                y,
                _mm_set1_ps(fractal_settings.freq),
                _mm_set1_ps(fractal_settings.lacunarity),
                _mm_set1_ps(fractal_settings.gain),
                fractal_settings.octaves,
            ),
            NoiseType::Turbulence => turbulence_2d(
                x,
                y,
                _mm_set1_ps(fractal_settings.freq),
                _mm_set1_ps(fractal_settings.lacunarity),
                _mm_set1_ps(fractal_settings.gain),
                fractal_settings.octaves,
            ),
            NoiseType::Normal => simplex_2d(
                _mm_mul_ps(x, _mm_set1_ps(fractal_settings.freq)),
                _mm_mul_ps(y, _mm_set1_ps(fractal_settings.freq)),
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
        let mut min_s = M128Array {
            simd: _mm_set1_ps(f32::MAX),
        };
        let mut max_s = M128Array {
            simd: _mm_set1_ps(f32::MIN),
        };
        let mut result = Vec::with_capacity(width * height);
        result.set_len(width * height);
        let mut y = _mm_set1_ps(start_y);
        let mut i = 0;
        for _ in 0..height {
            let mut x = _mm_set_ps(start_x + 3.0, start_x + 2.0, start_x + 1.0, start_x);
            for _ in 0..width / 4 {
                let f = get_2d_noise_helper(x, y, fractal_settings).simd;
                max_s.simd = _mm_max_ps(max_s.simd, f);
                min_s.simd = _mm_min_ps(min_s.simd, f);
                _mm_storeu_ps(&mut result[i], f);
                i = i + 4;
                x = _mm_add_ps(x, _mm_set1_ps(4.0));
            }
            if width % 4 != 0 {
                let f = get_2d_noise_helper(x, y, fractal_settings);
                for j in 0..width % 4 {
                    result[i] = f.array[j];
                    i = i + 1;
                }
            }
            y = _mm_add_ps(y, _mm_set1_ps(1.0));
        }
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        for i in 0..4 {
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
    scale_array_sse(scale_min, scale_max, min, max, &mut noise);
    noise
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
        let mut z = _mm_set1_ps(start_z);
        for _ in 0..depth {
            let mut y = _mm_set1_ps(start_y);
            for _ in 0..height {
                let mut x = _mm_set_ps(start_x, start_x + 1.0, start_x + 2.0, start_x + 3.0);
                for _ in 0..width {
                    result.push(match fractal_settings.noise_type {
                        NoiseType::FBM => fbm_3d(
                            x,
                            y,
                            z,
                            _mm_set1_ps(fractal_settings.freq),
                            _mm_set1_ps(fractal_settings.lacunarity),
                            _mm_set1_ps(fractal_settings.gain),
                            fractal_settings.octaves,
                        ),
                        NoiseType::Turbulence => turbulence_3d(
                            x,
                            y,
                            z,
                            _mm_set1_ps(fractal_settings.freq),
                            _mm_set1_ps(fractal_settings.lacunarity),
                            _mm_set1_ps(fractal_settings.gain),
                            fractal_settings.octaves,
                        ),
                        NoiseType::Normal => simplex_3d(
                            _mm_mul_ps(x, _mm_set1_ps(fractal_settings.freq)),
                            _mm_mul_ps(y, _mm_set1_ps(fractal_settings.freq)),
                            _mm_mul_ps(z, _mm_set1_ps(fractal_settings.freq)),
                        ),
                    });
                    x = _mm_add_ps(x, _mm_set1_ps(4.0));
                }
                y = _mm_add_ps(y, _mm_set1_ps(1.0));
            }
            z = _mm_add_ps(z, _mm_set1_ps(1.0));
        }
        ::std::mem::transmute::<Vec<__m128>, Vec<f32>>(result)
    }
}

#[cfg(test)]
mod tests {
    fn test_2d() {
        let r = get_2d_noise(-1, 1, 100, -1, 1, 100);
    }
    #[test]
    fn test_3d() {
        let r = get_3d_noise(-1, 1, 100, -1, 1, 100, -1, 1, 100);
    }
}
