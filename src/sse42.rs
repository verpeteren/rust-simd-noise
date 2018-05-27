use std::arch::x86_64::*;
use shared::*;

union M128Array {
    simd: __m128,
    array: [f32; 4],
}

union M128iArray {
    simd: __m128i,
    array: [i32; 4],
}

const F2: __m128 = unsafe {
    M128Array {
        array: [0.36602540378; 4],
    }.simd
};
const G2: __m128 = unsafe {
    M128Array {
        array: [0.2113248654; 4],
    }.simd
};
const G22: __m128 = unsafe {
    M128Array {
        array: [2.0 * 0.2113248654; 4],
    }.simd
};
const POINT_FIVE: __m128 = unsafe { M128Array { array: [0.5; 4] }.simd };


unsafe fn dot_simd(x1: __m128, x2: __m128, y1: __m128, y2: __m128) -> __m128 {
    _mm_add_ps(_mm_mul_ps(x1, x2), _mm_mul_ps(y1, y2))
}

#[cfg(any(target_arch = "x86_64"))]
#[target_feature(enable = "sse4.1")]
unsafe fn simplex_2d_sse41(x: __m128, y: __m128) -> __m128 {
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

    // QUESTION: can we fill these with data in one step instead of zeroing then filling them in the loop below
    let gi0 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0] + PERM[jj.array[0] as usize] ) as usize] ,
            PERM_MOD12[(ii.array[1] + PERM[jj.array[1] as usize] ) as usize] ,
            PERM_MOD12[(ii.array[2] + PERM[jj.array[2] as usize] ) as usize] ,
            PERM_MOD12[(ii.array[3] + PERM[jj.array[3] as usize] ) as usize] 
        ]
    };
    let gi1 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0]
                           + i1.array[0]
                           + PERM[(jj.array[0] + j1.array[0]) as usize])
                           as usize],
            PERM_MOD12[(ii.array[1]
                           + i1.array[1]
                           + PERM[(jj.array[1] + j1.array[1]) as usize])
                           as usize],
            PERM_MOD12[(ii.array[2]
                           + i1.array[2]
                           + PERM[(jj.array[2] + j1.array[2]) as usize])
                           as usize],
            PERM_MOD12[(ii.array[3]
                           + i1.array[3]
                           + PERM[(jj.array[3] + j1.array[3]) as usize])
                           as usize],
        ],
    };
    let gi2 = M128iArray {
        array: [
            PERM_MOD12[(ii.array[0] + 1 + PERM[(jj.array[0]  + 1) as usize] ) as usize],                
            PERM_MOD12[(ii.array[1] + 1 + PERM[(jj.array[1]  + 1) as usize] ) as usize],       
            PERM_MOD12[(ii.array[2] + 1 + PERM[(jj.array[2]  + 1) as usize] ) as usize],               
            PERM_MOD12[(ii.array[3] + 1 + PERM[(jj.array[3]  + 1) as usize] ) as usize],               
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

unsafe fn fbm_2d_sse41 (x : __m128 ,y: __m128 , freq : __m128 , lac: f32, gain:f32, octaves:i32)->__m128
{	
	let gain_s = _mm_set1_ps(gain);
	let lac_s = _mm_set1_ps(lac);
	let mut xf = _mm_mul_ps(x, freq);
	let mut yf = _mm_mul_ps(y, freq);
	let mut result = simplex_2d_sse41(xf, yf);	
	let mut amp = _mm_set1_ps(1.0);
	

	for _ in 1..octaves 
	{
		xf = _mm_mul_ps(xf, lac_s);
		yf = _mm_mul_ps(yf, lac_s);		
		amp = _mm_mul_ps(amp, gain_s);
		result = _mm_add_ps(result, _mm_mul_ps(simplex_2d_sse41(xf, yf),amp));
	}

	result
}


#[cfg(any(target_arch = "x86_64"))]
pub fn helper(a:f32,b:f32,c:f32,d:f32) -> (f32, f32, f32, f32) {
    unsafe {
        let mut result = M128Array {
            simd: _mm_setzero_ps(),
        };
        let x = _mm_set_ps(a,b,c,d);
        let y = _mm_set_ps(a,b,c,d);
        result.simd = simplex_2d_sse41(x, y);
        return (
            result.array[0],
            result.array[1],
            result.array[2],
            result.array[3],
        );
    }
}