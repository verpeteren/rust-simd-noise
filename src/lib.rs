use std::arch::x86_64::*;

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

const GRAD_X: [f32; 12] = [
    1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0,
];

const GRAD_Y: [f32; 12] = [
    1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 1.0, -1.0,
];

const GRAD_Z: [f32; 12] = [
    0.0, 0.0, 0.0, 0.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0,
];

const PERM: [u8; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

const PERM_MOD12: [u8; 512] = [
    7, 4, 5, 7, 6, 3, 11, 1, 9, 11, 0, 5, 2, 5, 7, 9, 8, 0, 7, 6, 9, 10, 8, 3, 1, 0, 9, 10, 11, 10,
    6, 4, 7, 0, 6, 3, 0, 2, 5, 2, 10, 0, 3, 11, 9, 11, 11, 8, 9, 9, 9, 4, 9, 5, 8, 3, 6, 8, 5, 4,
    3, 0, 8, 7, 2, 9, 11, 2, 7, 0, 3, 10, 5, 2, 2, 3, 11, 3, 1, 2, 0, 7, 1, 2, 4, 9, 8, 5, 7, 10,
    5, 4, 4, 6, 11, 6, 5, 1, 3, 5, 1, 0, 8, 1, 5, 4, 0, 7, 4, 5, 6, 1, 8, 4, 3, 10, 8, 8, 3, 2, 8,
    4, 1, 6, 5, 6, 3, 4, 4, 1, 10, 10, 4, 3, 5, 10, 2, 3, 10, 6, 3, 10, 1, 8, 3, 2, 11, 11, 11, 4,
    10, 5, 2, 9, 4, 6, 7, 3, 2, 9, 11, 8, 8, 2, 8, 10, 7, 10, 5, 9, 5, 11, 11, 7, 4, 9, 9, 10, 3,
    1, 7, 2, 0, 2, 7, 5, 8, 4, 10, 5, 4, 8, 2, 6, 1, 0, 11, 10, 2, 1, 10, 6, 0, 0, 11, 11, 6, 1, 9,
    3, 1, 7, 9, 2, 11, 11, 1, 0, 10, 7, 1, 7, 10, 1, 4, 0, 0, 8, 7, 1, 2, 9, 7, 4, 6, 2, 6, 8, 1,
    9, 6, 6, 7, 5, 0, 0, 3, 9, 8, 3, 6, 6, 11, 1, 0, 0, 7, 4, 5, 7, 6, 3, 11, 1, 9, 11, 0, 5, 2, 5,
    7, 9, 8, 0, 7, 6, 9, 10, 8, 3, 1, 0, 9, 10, 11, 10, 6, 4, 7, 0, 6, 3, 0, 2, 5, 2, 10, 0, 3, 11,
    9, 11, 11, 8, 9, 9, 9, 4, 9, 5, 8, 3, 6, 8, 5, 4, 3, 0, 8, 7, 2, 9, 11, 2, 7, 0, 3, 10, 5, 2,
    2, 3, 11, 3, 1, 2, 0, 7, 1, 2, 4, 9, 8, 5, 7, 10, 5, 4, 4, 6, 11, 6, 5, 1, 3, 5, 1, 0, 8, 1, 5,
    4, 0, 7, 4, 5, 6, 1, 8, 4, 3, 10, 8, 8, 3, 2, 8, 4, 1, 6, 5, 6, 3, 4, 4, 1, 10, 10, 4, 3, 5,
    10, 2, 3, 10, 6, 3, 10, 1, 8, 3, 2, 11, 11, 11, 4, 10, 5, 2, 9, 4, 6, 7, 3, 2, 9, 11, 8, 8, 2,
    8, 10, 7, 10, 5, 9, 5, 11, 11, 7, 4, 9, 9, 10, 3, 1, 7, 2, 0, 2, 7, 5, 8, 4, 10, 5, 4, 8, 2, 6,
    1, 0, 11, 10, 2, 1, 10, 6, 0, 0, 11, 11, 6, 1, 9, 3, 1, 7, 9, 2, 11, 11, 1, 0, 10, 7, 1, 7, 10,
    1, 4, 0, 0, 8, 7, 1, 2, 9, 7, 4, 6, 2, 6, 8, 1, 9, 6, 6, 7, 5, 0, 0, 3, 9, 8, 3, 6, 6, 11, 1,
    0, 0,
];

unsafe fn dot_simd(x1: __m128, x2: __m128, y1: __m128, y2: __m128) -> __m128 {
    _mm_add_ps(_mm_mul_ps(x1, x2), _mm_mul_ps(y1, y2))
}

#[cfg(any(target_arch = "x86_64"))]
#[target_feature(enable = "sse2")]
unsafe fn simplex_2d_sse2(x: __m128, y: __m128) -> __m128 {
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
    let mut gi0 = M128iArray {
        simd: _mm_setzero_si128(),
    };
    let mut gi1 = M128iArray {
        simd: _mm_setzero_si128(),
    };
    let mut gi2 = M128iArray {
        simd: _mm_setzero_si128(),
    };

    // QUESTION: could/should I use iterators here?
    for i in 0..4 {
        // QUESTION: Is there a better way to do this without all of the usize and i32 casting?
        gi0.array[i] =
            PERM_MOD12[(ii.array[i] + PERM[jj.array[i] as usize] as i32) as usize] as i32;

        gi1.array[i] = PERM_MOD12[(ii.array[i]
                                      + i1.array[i]
                                      + PERM[(jj.array[i] + j1.array[i]) as usize] as i32)
                                      as usize] as i32;
        gi2.array[i] = PERM_MOD12
            [(ii.array[i] + 1 + PERM[(jj.array[i] as i32 + 1) as usize] as i32) as usize]
            as i32;
    }

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

    let mut gi0x = M128Array {
        simd: _mm_setzero_ps(),
    };
    let mut gi1x = M128Array {
        simd: _mm_setzero_ps(),
    };
    let mut gi2x = M128Array {
        simd: _mm_setzero_ps(),
    };
    let mut gi0y = M128Array {
        simd: _mm_setzero_ps(),
    };
    let mut gi1y = M128Array {
        simd: _mm_setzero_ps(),
    };
    let mut gi2y = M128Array {
        simd: _mm_setzero_ps(),
    };

    for i in 0..4 {
        gi0x.array[i] = GRAD_X[gi0.array[i] as usize];
        gi0y.array[i] = GRAD_Y[gi0.array[i] as usize];

        gi1x.array[i] = GRAD_X[gi1.array[i] as usize];
        gi1y.array[i] = GRAD_Y[gi1.array[i] as usize];

        gi2x.array[i] = GRAD_X[gi2.array[i] as usize];
        gi2y.array[i] = GRAD_Y[gi2.array[i] as usize];
    }

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

#[cfg(any(target_arch = "x86_64"))]
pub fn helper() -> (f32, f32, f32, f32) {
    unsafe {
        let mut result = M128Array {
            simd: _mm_setzero_ps(),
        };
        let x = _mm_set_ps(1.0, 0.5, 0.4, 0.3);
        let y = _mm_set_ps(1.0, 0.5, 0.4, 0.3);
        result.simd = simplex_2d_sse2(x, y);
        return (
            result.array[0],
            result.array[1],
            result.array[2],
            result.array[3],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn first_test() {
        let (a, b, c, d) = helper();
        println!("{},{},{},{}", a, b, c, d);
    }
}
