use std::arch::x86_64::*;
use shared::*;

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
const POINT_FIVE: __m256 = unsafe {
    M256Array {
        array: [0.5; 8],
    }.simd
};

unsafe fn dot_simd(x1: __m256, x2: __m256, y1: __m256, y2: __m256) -> __m256 {
    _mm256_add_ps(_mm256_mul_ps(x1, x2), _mm256_mul_ps(y1, y2))
}


#[cfg(any(target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn simplex_2d_avx2(x: __m256, y: __m256) -> __m256 {
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

    let gi1 = _mm256_i32gather_epi32(&PERM_MOD12 as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(ii,i1),
        _mm256_i32gather_epi32(&PERM as *const i32,_mm256_add_epi32(jj,j1),4)),4);
    
    let gi2 = _mm256_i32gather_epi32(&PERM_MOD12 as *const i32,
        _mm256_add_epi32(_mm256_add_epi32(ii,_mm256_set1_epi32(1)),
        _mm256_i32gather_epi32(&PERM as *const i32,_mm256_add_epi32(jj,_mm256_set1_epi32(1)),4)),4);
   
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

    let gi0x = _mm256_i32gather_ps(&GRAD_X as *const f32,gi0,4);
    let gi1x = _mm256_i32gather_ps(&GRAD_X as *const f32,gi1,4);
    let gi2x = _mm256_i32gather_ps(&GRAD_X as *const f32,gi2,4);
    let gi0y = _mm256_i32gather_ps(&GRAD_Y as *const f32,gi0,4);
    let gi1y = _mm256_i32gather_ps(&GRAD_Y as *const f32,gi1,4);
    let gi2y = _mm256_i32gather_ps(&GRAD_Y as *const f32,gi2,4);
    

    let mut n0 = _mm256_mul_ps(t0q, dot_simd(gi0x, gi0y, x0, y0));
    let mut n1 = _mm256_mul_ps(t1q, dot_simd(gi1x, gi1y, x1, y1));
    let mut n2 = _mm256_mul_ps(t2q, dot_simd(gi2x, gi2y, x2, y2));

    let mut cond = _mm256_cmp_ps(t0, _mm256_setzero_ps(),_CMP_LT_OQ);
    n0 = _mm256_blendv_ps(n0, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t1, _mm256_setzero_ps(),_CMP_LT_OQ);
    n1 = _mm256_blendv_ps(n1, _mm256_setzero_ps(), cond);
    cond = _mm256_cmp_ps(t2, _mm256_setzero_ps(),_CMP_LT_OQ);
    n2 = _mm256_blendv_ps(n2, _mm256_setzero_ps(), cond);

    _mm256_add_ps(n0, _mm256_add_ps(n1, n2))
    
}


#[cfg(any(target_arch = "x86_64"))]
pub fn helper(a:f32,b:f32,c:f32,d:f32,e:f32,f:f32,g:f32,h:f32) -> (f32, f32, f32, f32,f32,f32,f32,f32) {
    unsafe {
        let mut result = M256Array {
            simd: _mm256_setzero_ps(),
        };
        let x = _mm256_set_ps(a,b,c,d,e,f,g,h);
        let y = _mm256_set_ps(a,b,c,d,e,f,g,h);
        result.simd = simplex_2d_avx2(x, y);
        return (
            result.array[0],
            result.array[1],
            result.array[2],
            result.array[3],
            result.array[4],
            result.array[5],
            result.array[6],
            result.array[7],
        );
    }
}