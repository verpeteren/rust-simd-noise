use std::arch::x86_64::*;
use super::shared::*;

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

#[cfg(any(target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn simplex_2d_avx2(x: __m256, y: __m256)  {
    let s = _mm256_mul_ps(F2, _mm256_add_ps(x, y));
    let ips = _mm256_floor_ps(_mm256_add_ps(x, s));
    let jps = _mm256_floor_ps(_mm256_add_ps(y, s));

    let i = _mm256_cvtps_epi32(ips);
    let j = _mm256_cvtps_epi32(jps);

    let t = _mm256_mul_ps(_mm256_cvtepi32_ps(_mm256_add_epi32(i, j)), G2);

    let x0 = _mm256_sub_ps(x, _mm256_sub_ps(ips, t));
    let y0 = _mm256_sub_ps(y, _mm256_sub_ps(jps, t));

    let i1 =_mm256_and_si256(_mm256_set1_epi32(1), _mm256_castps_si256(_mm256_cmp_ps(x0, y0,_CMP_GE_OQ)));

    let j1 = _mm256_and_si256(_mm256_set1_epi32(1), _mm256_castps_si256(_mm256_cmp_ps(y0, x0,_CMP_GT_OQ)));
    

    let x1 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_cvtepi32_ps(i1)), G2);
    let y1 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_cvtepi32_ps(j1)), G2);
    let x2 = _mm256_add_ps(_mm256_sub_ps(x0, _mm256_set1_ps(1.0)), G22);
    let y2 = _mm256_add_ps(_mm256_sub_ps(y0, _mm256_set1_ps(1.0)), G22);

    let ii = _mm256_and_si256(i, _mm256_set1_epi32(0xff));    
    let jj = _mm256_and_si256(j, _mm256_set1_epi32(0xff));

    let gi0 = _mm256_i32gather_epi32(&PERM_MOD12 as *const i32,_mm256_add_epi32(ii,_mm256_i32gather_epi32(&PERM as *const i32,jj,4)),4);

    // QUESTION: can we fill these with data in one step instead of zeroing then filling them in the loop below
    /*
    let gi0 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0] + PERM[jj.array[0] as usize] as i32) as usize] as i32,
            PERM_MOD12[(ii.array[1] + PERM[jj.array[1] as usize] as i32) as usize] as i32,
            PERM_MOD12[(ii.array[2] + PERM[jj.array[2] as usize] as i32) as usize] as i32,
            PERM_MOD12[(ii.array[3] + PERM[jj.array[3] as usize] as i32) as usize] as i32
        ]
    };
    let gi1 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0]
                           + i1.array[0]
                           + PERM[(jj.array[0] + j1.array[0]) as usize] as i32)
                           as usize] as i32,
            PERM_MOD12[(ii.array[1]
                           + i1.array[1]
                           + PERM[(jj.array[1] + j1.array[1]) as usize] as i32)
                           as usize] as i32,
            PERM_MOD12[(ii.array[2]
                           + i1.array[2]
                           + PERM[(jj.array[2] + j1.array[2]) as usize] as i32)
                           as usize] as i32,
            PERM_MOD12[(ii.array[3]
                           + i1.array[3]
                           + PERM[(jj.array[3] + j1.array[3]) as usize] as i32)
                           as usize] as i32,
        ],
    };
    let gi2 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0] + 1 + PERM[(jj.array[0] as i32 + 1) as usize] as i32) as usize]
                as i32,
            PERM_MOD12[(ii.array[1] + 1 + PERM[(jj.array[1] as i32 + 1) as usize] as i32) as usize]
                as i32,
            PERM_MOD12[(ii.array[2] + 1 + PERM[(jj.array[2] as i32 + 1) as usize] as i32) as usize]
                as i32,
            PERM_MOD12[(ii.array[3] + 1 + PERM[(jj.array[3] as i32 + 1) as usize] as i32) as usize]
                as i32,
        ],
    };

   
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

    let gi0x = M128Array {
        array: [
            GRAD_X[gi0.array[0] as usize],
            GRAD_X[gi0.array[1] as usize],
            GRAD_X[gi0.array[2] as usize],
            GRAD_X[gi0.array[3] as usize]
        ]
    };
    let gi1x = M128Array {
        array: [
            GRAD_X[gi1.array[0] as usize],
            GRAD_X[gi1.array[1] as usize],
            GRAD_X[gi1.array[2] as usize],
            GRAD_X[gi1.array[3] as usize]
        ]
    };
    let gi2x = M128Array {
         array: [
            GRAD_X[gi2.array[0] as usize],
            GRAD_X[gi2.array[1] as usize],
            GRAD_X[gi2.array[2] as usize],
            GRAD_X[gi2.array[3] as usize]
        ]
    };
    let gi0y = M128Array {
         array: [
            GRAD_Y[gi0.array[0] as usize],
            GRAD_Y[gi0.array[1] as usize],
            GRAD_Y[gi0.array[2] as usize],
            GRAD_Y[gi0.array[3] as usize]
        ]
    };
    let gi1y = M128Array {
         array: [
            GRAD_Y[gi1.array[0] as usize],
            GRAD_Y[gi1.array[1] as usize],
            GRAD_Y[gi1.array[2] as usize],
            GRAD_Y[gi1.array[3] as usize]
        ]
    };
    let gi2y = M128Array {
         array: [
            GRAD_Y[gi2.array[0] as usize],
            GRAD_Y[gi2.array[1] as usize],
            GRAD_Y[gi2.array[2] as usize],
            GRAD_Y[gi2.array[3] as usize]
        ]
    };
    

    let mut n0 = _mm256_mul_ps(t0q, dot_simd(gi0x.simd, gi0y.simd, x0, y0));
    let mut n1 = _mm256_mul_ps(t1q, dot_simd(gi1x.simd, gi1y.simd, x1, y1));
    let mut n2 = _mm256_mul_ps(t2q, dot_simd(gi2x.simd, gi2y.simd, x2, y2));

    let mut cond = _mm256_cmplt_ps(t0, _mm256_setzero_ps());
    n0 = _mm256_blendv_ps(n0, _mm256_setzero_ps(), cond);
    cond = _mm256_cmplt_ps(t1, _mm256_setzero_ps());
    n1 = _mm256_blendv_ps(n1, _mm256_setzero_ps(), cond);
    cond = _mm256_cmplt_ps(t2, _mm256_setzero_ps());
    n2 = _mm256_blendv_ps(n2, _mm256_setzero_ps(), cond);

    _mm256_add_ps(n0, _mm256_add_ps(n1, n2))
    */
}