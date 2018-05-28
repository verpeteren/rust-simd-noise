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
const F3: __m128 = unsafe {
    M128Array {
        array:[1.0/3.0;4]
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
const G3: __m128 = unsafe {
    M128Array {
        array: [1.0/6.0;4]
    }.simd
};
const POINT_FIVE: __m128 = unsafe { M128Array { array: [0.5; 4] }.simd };


unsafe fn dot_simd(x1: __m128, x2: __m128, y1: __m128, y2: __m128) -> __m128 {
    _mm_add_ps(_mm_mul_ps(x1, x2), _mm_mul_ps(y1, y2))
}

#[cfg(any(target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
unsafe fn simplex_2d_sse2(x: __m128, y: __m128) -> __m128 {
    let s = _mm_mul_ps(F2, _mm_add_ps(x, y));
 

    let mut ips = M128Array {
        simd: _mm_add_ps(x, s),
    };

    let mut jps = M128Array {
        simd: _mm_add_ps(y, s),
    };

    ips.array[0] = ips.array[0].floor();
    ips.array[1] = ips.array[1].floor();
    ips.array[2] = ips.array[2].floor();
    ips.array[3] = ips.array[3].floor();
    
    jps.array[0] = jps.array[0].floor();
    jps.array[1] = jps.array[1].floor();
    jps.array[2] = jps.array[2].floor();
    jps.array[3] = jps.array[3].floor();
    
    let i = _mm_cvtps_epi32(ips.simd);
    let j = _mm_cvtps_epi32(jps.simd);

    let t = _mm_mul_ps(_mm_cvtepi32_ps(_mm_add_epi32(i, j)), G2);

    let x0 = _mm_sub_ps(x, _mm_sub_ps(ips.simd, t));
    let y0 = _mm_sub_ps(y, _mm_sub_ps(jps.simd, t));

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
    n0 = _mm_or_ps(_mm_andnot_ps(cond,n0),_mm_and_ps(cond,_mm_setzero_ps()));
    cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
    n1 = _mm_or_ps(_mm_andnot_ps(cond,n1),_mm_and_ps(cond,_mm_setzero_ps()));
    cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_or_ps(_mm_andnot_ps(cond,n2),_mm_and_ps(cond,_mm_setzero_ps()));
    
    _mm_add_ps(n0, _mm_add_ps(n1, n2))
}

unsafe fn fbm_2d_sse2 (x : __m128 ,y: __m128 , freq : __m128 , lac: f32, gain:f32, octaves:i32)->__m128
{	
	let gain_s = _mm_set1_ps(gain);
	let lac_s = _mm_set1_ps(lac);
	let mut xf = _mm_mul_ps(x, freq);
	let mut yf = _mm_mul_ps(y, freq);
	let mut result = simplex_2d_sse2(xf, yf);	
	let mut amp = _mm_set1_ps(1.0);
	

	for _ in 1..octaves 
	{
		xf = _mm_mul_ps(xf, lac_s);
		yf = _mm_mul_ps(yf, lac_s);		
		amp = _mm_mul_ps(amp, gain_s);
		result = _mm_add_ps(result, _mm_mul_ps(simplex_2d_sse2(xf, yf),amp));
	}

	result
}

unsafe fn _mm_abs_ps(a:__m128) -> __m128
{
	let b = _mm_set1_epi32(0x7fffffff);
	_mm_and_ps(a,_mm_castsi128_ps(b))
}

unsafe fn turbulence_2d_sse2 (x : __m128 ,y: __m128 , freq : __m128 , lac: f32, gain:f32, octaves:i32)->__m128
{	
	let gain_s = _mm_set1_ps(gain);
	let lac_s = _mm_set1_ps(lac);
	let mut xf = _mm_mul_ps(x, freq);
	let mut yf = _mm_mul_ps(y, freq);
	let mut result = _mm_abs_ps(simplex_2d_sse2(xf, yf));
	let mut amp = _mm_set1_ps(1.0);
	

	for _ in 1..octaves 
	{
		xf = _mm_mul_ps(xf, lac_s);
		yf = _mm_mul_ps(yf, lac_s);		
		amp = _mm_mul_ps(amp, gain_s);
		result = _mm_add_ps(result,_mm_abs_ps(_mm_mul_ps(simplex_2d_sse2(xf, yf),amp)));
	}

	result
}

unsafe fn dot_simd_3d(x1: __m128, y1: __m128, z1: __m128, x2: __m128, y2: __m128, z2: __m128) -> __m128
{
	let xx = _mm_mul_ps(x1, x2);
	let yy = _mm_mul_ps(y1, y2);
	let zz = _mm_mul_ps(z1, z2);
	_mm_add_ps(xx, _mm_add_ps(yy, zz))
}

unsafe fn simplex_3d_sse2(x:__m128, y:__m128,z:__m128) -> __m128 {

    let s = M128Array{ simd:_mm_mul_ps(F3,_mm_add_ps(x,_mm_add_ps(y,z)))};
    let ips = M128Array{ simd: _mm_add_ps(x,s.simd)};
    let jps = M128Array{ simd: _mm_add_ps(y,s.simd)};
    let kps = M128Array{ simd: _mm_add_ps(z,s.simd)};

    let mut i = M128iArray{ array: [
        ips.array[0].floor() as i32,
        ips.array[1].floor() as i32,
        ips.array[2].floor() as i32,
        ips.array[3].floor() as i32]};
    let mut j = M128iArray{ array: [
        jps.array[0].floor() as i32,
        jps.array[1].floor() as i32,
        jps.array[2].floor() as i32,
        jps.array[3].floor() as i32]};
    let mut k = M128iArray{ array: [
        kps.array[0].floor() as i32,
        kps.array[1].floor() as i32,
        kps.array[2].floor() as i32,
        kps.array[3].floor() as i32]};
    
    let t = _mm_mul_ps(_mm_cvtepi32_ps(_mm_add_epi32(i.simd, _mm_add_epi32(j.simd, k.simd))), G3);
	let X0 = _mm_sub_ps(_mm_cvtepi32_ps(i.simd), t);
	let Y0 = _mm_sub_ps(_mm_cvtepi32_ps(j.simd), t);
	let Z0 = _mm_sub_ps(_mm_cvtepi32_ps(k.simd), t);
	let x0 = _mm_sub_ps(x, X0);
	let y0 = _mm_sub_ps(y, Y0);
	let z0 = _mm_sub_ps(z, Z0);

	let i1 = M128iArray{ 
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_and_si128(_mm_castps_si128(_mm_cmpge_ps(x0, y0)), _mm_castps_si128(_mm_cmpge_ps(x0, z0))))
    };
	let j1 = M128iArray{
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_and_si128(_mm_castps_si128(_mm_cmpgt_ps(y0, x0)), _mm_castps_si128(_mm_cmpgt_ps(y0, z0))))
    };
	let k1 = M128iArray{
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_and_si128(_mm_castps_si128(_mm_cmpgt_ps(z0, x0)), _mm_castps_si128(_mm_cmpgt_ps(z0, y0))))
    };

	//for i2
	let yx_xz = _mm_and_si128(_mm_castps_si128(_mm_cmpge_ps(x0, y0)), _mm_castps_si128(_mm_cmplt_ps(x0, z0)));
	let zx_xy = _mm_and_si128(_mm_castps_si128(_mm_cmpge_ps(x0, z0)), _mm_castps_si128(_mm_cmplt_ps(x0, y0)));

	//for j2
	let xy_yz = _mm_and_si128(_mm_castps_si128(_mm_cmplt_ps(x0, y0)), _mm_castps_si128(_mm_cmplt_ps(y0, z0)));
	let zy_yx = _mm_and_si128(_mm_castps_si128(_mm_cmpge_ps(y0, z0)), _mm_castps_si128(_mm_cmpge_ps(x0, y0)));

	//for k2
	let yz_zx = _mm_and_si128(_mm_castps_si128(_mm_cmplt_ps(y0, z0)), _mm_castps_si128(_mm_cmpge_ps(x0, z0)));
	let xz_zy = _mm_and_si128(_mm_castps_si128(_mm_cmplt_ps(x0, z0)), _mm_castps_si128(_mm_cmpge_ps(y0, z0)));

    let i2 = M128iArray{ 
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_or_si128(i1.simd, _mm_or_si128(yx_xz, zx_xy)))
    };
	let j2 = M128iArray{
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_or_si128(j1.simd, _mm_or_si128(xy_yz, zy_yx)))
    };
	let k2 = M128iArray {
        simd: _mm_and_si128(_mm_set1_epi32(1), _mm_or_si128(k1.simd, _mm_or_si128(yz_zx, xz_zy)))
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

    i.simd = _mm_and_si128(i.simd,_mm_set1_epi32(0xff));
    j.simd = _mm_and_si128(j.simd,_mm_set1_epi32(0xff));
    k.simd = _mm_and_si128(k.simd,_mm_set1_epi32(0xff));
 
    let gi0 = M128iArray {
        array: [
            PERM_MOD12[(i.array[0] + PERM[(j.array[0] + PERM[k.array[0] as usize])as usize]) as usize],
            PERM_MOD12[(i.array[1] + PERM[(j.array[1] + PERM[k.array[1] as usize])as usize]) as usize],
            PERM_MOD12[(i.array[2] + PERM[(j.array[2] + PERM[k.array[2] as usize])as usize]) as usize],
            PERM_MOD12[(i.array[3] + PERM[(j.array[3] + PERM[k.array[3] as usize])as usize]) as usize]
        ]
    };

    let gi1 = M128iArray {
        array: [
            PERM_MOD12[(i.array[0] + i1.array[0] + PERM[(j.array[0] + j1.array[0] + PERM[(k.array[0] + k1.array[0]) as usize])as usize]) as usize],
            PERM_MOD12[(i.array[1] + i1.array[1] + PERM[(j.array[1] + j1.array[1] + PERM[(k.array[1] + k1.array[1]) as usize])as usize]) as usize],
            PERM_MOD12[(i.array[2] + i1.array[2] + PERM[(j.array[2] + j1.array[2] + PERM[(k.array[2] + k1.array[2]) as usize])as usize]) as usize],
            PERM_MOD12[(i.array[3] + i1.array[3] + PERM[(j.array[3] + j1.array[3] + PERM[(k.array[3] + k1.array[3]) as usize])as usize]) as usize],
        ]
    };
    let gi2 = M128iArray {
        array: [
            PERM_MOD12[(i.array[0] + i2.array[0] + PERM[(j.array[0] + j2.array[0] + PERM[(k.array[0] + k2.array[0]) as usize])as usize]) as usize],
            PERM_MOD12[(i.array[1] + i2.array[1] + PERM[(j.array[1] + j2.array[1] + PERM[(k.array[1] + k2.array[1]) as usize])as usize]) as usize],
            PERM_MOD12[(i.array[2] + i2.array[2] + PERM[(j.array[2] + j2.array[2] + PERM[(k.array[2] + k2.array[2]) as usize])as usize]) as usize],
            PERM_MOD12[(i.array[3] + i2.array[3] + PERM[(j.array[3] + j2.array[3] + PERM[(k.array[3] + k2.array[3]) as usize])as usize]) as usize],
        ]
    };
    let gi3 = M128iArray {
        array: [
            PERM_MOD12[(i.array[0] + 1 + PERM[(j.array[0] + 1 + PERM[k.array[0] as usize])as usize]) as usize],
            PERM_MOD12[(i.array[1] + 1 + PERM[(j.array[1] + 1 + PERM[k.array[1] as usize])as usize]) as usize],
            PERM_MOD12[(i.array[2] + 1 + PERM[(j.array[2] + 1 + PERM[k.array[2] as usize])as usize]) as usize],
            PERM_MOD12[(i.array[3] + 1 + PERM[(j.array[3] + 1 + PERM[k.array[3] as usize])as usize]) as usize],
        ]
    };

    let t0 = _mm_sub_ps(_mm_sub_ps(_mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x0, x0)), _mm_mul_ps(y0, y0)), _mm_mul_ps(z0, z0));
	let t1 = _mm_sub_ps(_mm_sub_ps(_mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x1, x1)), _mm_mul_ps(y1, y1)), _mm_mul_ps(z1, z1));
	let t2 = _mm_sub_ps(_mm_sub_ps(_mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x2, x2)), _mm_mul_ps(y2, y2)), _mm_mul_ps(z2, z2));
	let t3 = _mm_sub_ps(_mm_sub_ps(_mm_sub_ps(_mm_set1_ps(0.6), _mm_mul_ps(x3, x3)), _mm_mul_ps(y3, y3)), _mm_mul_ps(z3, z3));

	//ti*ti*ti*ti
	let mut t0q = _mm_mul_ps(t0, t0);
	t0q = _mm_mul_ps(t0q, t0q);
	let mut t1q = _mm_mul_ps(t1, t1);
	t1q = _mm_mul_ps(t1q, t1q);
	let mut t2q = _mm_mul_ps(t2, t2);
	t2q = _mm_mul_ps(t2q, t2q);
	let mut t3q = _mm_mul_ps(t3, t3);
	t3q = _mm_mul_ps(t3q, t3q);

    let gi0x = M128Array{
        array:[
            GRAD_X[gi0.array[0] as usize],
            GRAD_X[gi0.array[1] as usize],
            GRAD_X[gi0.array[2] as usize],
            GRAD_X[gi0.array[3] as usize]
        ]
    };
    let gi0y = M128Array{
        array:[
            GRAD_Y[gi0.array[0] as usize],
            GRAD_Y[gi0.array[1] as usize],
            GRAD_Y[gi0.array[2] as usize],
            GRAD_Y[gi0.array[3] as usize]
        ]
    };
    let gi0z = M128Array{
        array:[
            GRAD_Z[gi0.array[0] as usize],
            GRAD_Z[gi0.array[1] as usize],
            GRAD_Z[gi0.array[2] as usize],
            GRAD_Z[gi0.array[3] as usize]
        ]
    };
    let gi1x = M128Array{
        array:[
            GRAD_X[gi1.array[0] as usize],
            GRAD_X[gi1.array[1] as usize],
            GRAD_X[gi1.array[2] as usize],
            GRAD_X[gi1.array[3] as usize]
        ]
    };
    let gi1y = M128Array{
        array:[
            GRAD_Y[gi1.array[0] as usize],
            GRAD_Y[gi1.array[1] as usize],
            GRAD_Y[gi1.array[2] as usize],
            GRAD_Y[gi1.array[3] as usize]
        ]
    };
    let gi1z = M128Array{
        array:[
            GRAD_Z[gi1.array[0] as usize],
            GRAD_Z[gi1.array[1] as usize],
            GRAD_Z[gi1.array[2] as usize],
            GRAD_Z[gi1.array[3] as usize]
        ]
    };
    let gi2x = M128Array{
        array:[
            GRAD_X[gi2.array[0] as usize],
            GRAD_X[gi2.array[1] as usize],
            GRAD_X[gi2.array[2] as usize],
            GRAD_X[gi2.array[3] as usize]
        ]
    };
    let gi2y = M128Array{
        array:[
            GRAD_Y[gi2.array[0] as usize],
            GRAD_Y[gi2.array[1] as usize],
            GRAD_Y[gi2.array[2] as usize],
            GRAD_Y[gi2.array[3] as usize]
        ]
    };
    let gi2z = M128Array{
        array:[
            GRAD_Z[gi2.array[0] as usize],
            GRAD_Z[gi2.array[1] as usize],
            GRAD_Z[gi2.array[2] as usize],
            GRAD_Z[gi2.array[3] as usize]
        ]
    };
    let gi3x = M128Array{
        array:[
            GRAD_X[gi3.array[0] as usize],
            GRAD_X[gi3.array[1] as usize],
            GRAD_X[gi3.array[2] as usize],
            GRAD_X[gi3.array[3] as usize]
        ]
    };
    let gi3y = M128Array{
        array:[
            GRAD_Y[gi3.array[0] as usize],
            GRAD_Y[gi3.array[1] as usize],
            GRAD_Y[gi3.array[2] as usize],
            GRAD_Y[gi3.array[3] as usize]
        ]
    };
    let gi3z = M128Array{
        array:[
            GRAD_Z[gi3.array[0] as usize],
            GRAD_Z[gi3.array[1] as usize],
            GRAD_Z[gi3.array[2] as usize],
            GRAD_Z[gi3.array[3] as usize]
        ]
    };

    let mut n0 = _mm_mul_ps(t0q, dot_simd_3d(gi0x.simd, gi0y.simd, gi0z.simd, x0, y0, z0));
    let mut n1 = _mm_mul_ps(t1q, dot_simd_3d(gi1x.simd, gi1y.simd, gi1z.simd, x1, y1, z1));
    let mut n2 = _mm_mul_ps(t2q, dot_simd_3d(gi2x.simd, gi2y.simd, gi2z.simd, x2, y2, z2));
    let mut n3 = _mm_mul_ps(t3q, dot_simd_3d(gi3x.simd, gi3y.simd, gi3z.simd, x3, y3, z3));

	//BlendV(x,y,z) Or(AndNot(z,x), And(z,y)) 
     //if ti < 0 then 0 else ni
	let mut cond = _mm_cmplt_ps(t0, _mm_setzero_ps());	
    n0 = _mm_or_ps(_mm_andnot_ps(cond,n0),_mm_and_ps(cond,_mm_setzero_ps()));
	cond = _mm_cmplt_ps(t1, _mm_setzero_ps());
	n1 = _mm_or_ps(_mm_andnot_ps(cond,n1),_mm_and_ps(cond,_mm_setzero_ps()));
	cond = _mm_cmplt_ps(t2, _mm_setzero_ps());
    n2 = _mm_or_ps(_mm_andnot_ps(cond,n2),_mm_and_ps(cond,_mm_setzero_ps()));
	cond = _mm_cmplt_ps(t3, _mm_setzero_ps());
    n3 = _mm_or_ps(_mm_andnot_ps(cond,n3),_mm_and_ps(cond,_mm_setzero_ps()));

	_mm_add_ps(n0, _mm_add_ps(n1, _mm_add_ps(n2, n3)))   
}
#[cfg(any(target_arch = "x86_64"))]
pub fn helper(a:f32,b:f32,c:f32,d:f32) -> (f32, f32, f32, f32) {
    unsafe {
        let mut result = M128Array {
            simd: _mm_setzero_ps(),
        };
        let x = _mm_set_ps(a,b,c,d);
        let y = _mm_set_ps(a,b,c,d);
        result.simd = simplex_2d_sse2(x, y);
        return (
            result.array[0],
            result.array[1],
            result.array[2],
            result.array[3],
        );
    }
}