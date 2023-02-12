use crate::noise::gradient_64::{grad1, grad2, grad3d, grad4};

use simdeez::Simd;

const F2_64: f64 = 0.36602540378;
const F3_64: f64 = 1.0 / 3.0;
const F4_64: f64 = 0.309016994;
const G2: f64 = 0.2113248654;
const G22: f64 = G2 * 2.0;
const G3: f64 = 1.0 / 6.0;
const G4: f64 = 0.138196601;
const G24: f64 = 2.0 * G4;
const G34: f64 = 3.0 * G4;
const G44: f64 = 4.0 * G4;

const PERM64: [i64; 512] = [
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

/// Samples 1-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub unsafe fn simplex_1d<S: Simd>(x: S::Vf64, seed: i64) -> S::Vf64 {
    let ipd = S::fast_floor_pd(x);
    let mut i0 = S::cvtpd_epi64(ipd);
    let i1 = S::and_epi64(S::add_epi64(i0, S::set1_epi64(1)), S::set1_epi64(0xff));

    let x0 = S::sub_pd(x, ipd);
    let x1 = S::sub_pd(x0, S::set1_pd(1.0));

    i0 = S::and_epi64(i0, S::set1_epi64(0xff));
    let gi0 = S::i64gather_epi64(&PERM64, i0);
    let gi1 = S::i64gather_epi64(&PERM64, i1);

    let mut t0 = S::sub_pd(S::set1_pd(1.0), S::mul_pd(x0, x0));
    t0 = S::mul_pd(t0, t0);
    t0 = S::mul_pd(t0, t0);
    let n0 = S::mul_pd(t0, grad1::<S>(seed, gi0, x0));

    let mut t1 = S::sub_pd(S::set1_pd(1.0), S::mul_pd(x1, x1));
    t1 = S::mul_pd(t1, t1);
    t1 = S::mul_pd(t1, t1);
    let n1 = S::mul_pd(t1, grad1::<S>(seed, gi1, x1));

    S::add_pd(n0, n1) * S::set1_pd(256.0 / (81.0 * 7.0))
}

/// Samples 2-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub unsafe fn simplex_2d<S: Simd>(x: S::Vf64, y: S::Vf64, seed: i64) -> S::Vf64 {
    let s = S::mul_pd(S::set1_pd(F2_64), S::add_pd(x, y));
    let ipd = S::floor_pd(S::add_pd(x, s));
    let jpd = S::floor_pd(S::add_pd(y, s));

    let i = S::cvtpd_epi64(ipd);
    let j = S::cvtpd_epi64(jpd);

    let t = S::mul_pd(S::cvtepi64_pd(S::add_epi64(i, j)), S::set1_pd(G2));

    let x0 = S::sub_pd(x, S::sub_pd(ipd, t));
    let y0 = S::sub_pd(y, S::sub_pd(jpd, t));

    let i1 = S::castpd_epi64(S::cmpge_pd(x0, y0));

    let j1 = S::castpd_epi64(S::cmpgt_pd(y0, x0));

    let x1 = S::add_pd(S::add_pd(x0, S::cvtepi64_pd(i1)), S::set1_pd(G2));
    let y1 = S::add_pd(S::add_pd(y0, S::cvtepi64_pd(j1)), S::set1_pd(G2));
    let x2 = S::add_pd(S::add_pd(x0, S::set1_pd(-1.0)), S::set1_pd(G22));
    let y2 = S::add_pd(S::add_pd(y0, S::set1_pd(-1.0)), S::set1_pd(G22));

    let ii = S::and_epi64(i, S::set1_epi64(0xff));
    let jj = S::and_epi64(j, S::set1_epi64(0xff));

    let gi0 = S::i64gather_epi64(&PERM64, S::add_epi64(ii, S::i64gather_epi64(&PERM64, jj)));

    let gi1 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(
            S::sub_epi64(ii, i1),
            S::i64gather_epi64(&PERM64, S::sub_epi64(jj, j1)),
        ),
    );

    let gi2 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(
            S::sub_epi64(ii, S::set1_epi64(-1)),
            S::i64gather_epi64(&PERM64, S::sub_epi64(jj, S::set1_epi64(-1))),
        ),
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let t0 = S::fnmadd_pd(y0, y0, S::fnmadd_pd(x0, x0, S::set1_pd(0.5)));
    let t1 = S::fnmadd_pd(y1, y1, S::fnmadd_pd(x1, x1, S::set1_pd(0.5)));
    let t2 = S::fnmadd_pd(y2, y2, S::fnmadd_pd(x2, x2, S::set1_pd(0.5)));

    let mut t0q = S::mul_pd(t0, t0);
    t0q = S::mul_pd(t0q, t0q);
    let mut t1q = S::mul_pd(t1, t1);
    t1q = S::mul_pd(t1q, t1q);
    let mut t2q = S::mul_pd(t2, t2);
    t2q = S::mul_pd(t2q, t2q);

    let mut n0 = S::mul_pd(t0q, grad2::<S>(seed, gi0, x0, y0));
    let mut n1 = S::mul_pd(t1q, grad2::<S>(seed, gi1, x1, y1));
    let mut n2 = S::mul_pd(t2q, grad2::<S>(seed, gi2, x2, y2));

    let mut cond = S::cmplt_pd(t0, S::setzero_pd());
    n0 = S::andnot_pd(cond, n0);
    cond = S::cmplt_pd(t1, S::setzero_pd());
    n1 = S::andnot_pd(cond, n1);
    cond = S::cmplt_pd(t2, S::setzero_pd());
    n2 = S::andnot_pd(cond, n2);

    S::add_pd(n0, S::add_pd(n1, n2)) * S::set1_pd(45.26450774985561631259)
}

#[inline(always)]
pub unsafe fn simplex_3d<S: Simd>(x: S::Vf64, y: S::Vf64, z: S::Vf64, seed: i64) -> S::Vf64 {
    let s = S::mul_pd(S::set1_pd(F3_64), S::add_pd(x, S::add_pd(y, z)));

    let ipd = S::floor_pd(S::add_pd(x, s));
    let jpd = S::floor_pd(S::add_pd(y, s));
    let kpd = S::floor_pd(S::add_pd(z, s));

    let i = S::cvtpd_epi64(ipd);
    let j = S::cvtpd_epi64(jpd);
    let k = S::cvtpd_epi64(kpd);

    let t = S::mul_pd(
        S::cvtepi64_pd(S::add_epi64(i, S::add_epi64(j, k))),
        S::set1_pd(G3),
    );

    let x0 = S::sub_pd(x, S::sub_pd(ipd, t));
    let y0 = S::sub_pd(y, S::sub_pd(jpd, t));
    let z0 = S::sub_pd(z, S::sub_pd(kpd, t));

    let i1 = S::and_epi64(
        S::castpd_epi64(S::cmpge_pd(x0, y0)),
        S::castpd_epi64(S::cmpge_pd(x0, z0)),
    );
    let j1 = S::and_epi64(
        S::castpd_epi64(S::cmpgt_pd(y0, x0)),
        S::castpd_epi64(S::cmpge_pd(y0, z0)),
    );
    let k1 = S::and_epi64(
        S::castpd_epi64(S::cmpgt_pd(z0, x0)),
        S::castpd_epi64(S::cmpgt_pd(z0, y0)),
    );

    //for i2
    let yx_xz = S::and_epi64(
        S::castpd_epi64(S::cmpge_pd(x0, y0)),
        S::castpd_epi64(S::cmplt_pd(x0, z0)),
    );
    let zx_xy = S::and_epi64(
        S::castpd_epi64(S::cmpge_pd(x0, z0)),
        S::castpd_epi64(S::cmplt_pd(x0, y0)),
    );

    //for j2
    let xy_yz = S::and_epi64(
        S::castpd_epi64(S::cmplt_pd(x0, y0)),
        S::castpd_epi64(S::cmplt_pd(y0, z0)),
    );
    let zy_yx = S::and_epi64(
        S::castpd_epi64(S::cmpge_pd(y0, z0)),
        S::castpd_epi64(S::cmpge_pd(x0, y0)),
    );

    //for k2
    let yz_zx = S::and_epi64(
        S::castpd_epi64(S::cmplt_pd(y0, z0)),
        S::castpd_epi64(S::cmpge_pd(x0, z0)),
    );
    let xz_zy = S::and_epi64(
        S::castpd_epi64(S::cmplt_pd(x0, z0)),
        S::castpd_epi64(S::cmpge_pd(y0, z0)),
    );

    let i2 = S::or_epi64(i1, S::or_epi64(yx_xz, zx_xy));
    let j2 = S::or_epi64(j1, S::or_epi64(xy_yz, zy_yx));
    let k2 = S::or_epi64(k1, S::or_epi64(yz_zx, xz_zy));

    let x1 = S::add_pd(S::add_pd(x0, S::cvtepi64_pd(i1)), S::set1_pd(G3));
    let y1 = S::add_pd(S::add_pd(y0, S::cvtepi64_pd(j1)), S::set1_pd(G3));
    let z1 = S::add_pd(S::add_pd(z0, S::cvtepi64_pd(k1)), S::set1_pd(G3));
    let x2 = S::add_pd(S::add_pd(x0, S::cvtepi64_pd(i2)), S::set1_pd(F3_64));
    let y2 = S::add_pd(S::add_pd(y0, S::cvtepi64_pd(j2)), S::set1_pd(F3_64));
    let z2 = S::add_pd(S::add_pd(z0, S::cvtepi64_pd(k2)), S::set1_pd(F3_64));
    let x3 = S::add_pd(x0, S::set1_pd(-0.5));
    let y3 = S::add_pd(y0, S::set1_pd(-0.5));
    let z3 = S::add_pd(z0, S::set1_pd(-0.5));

    // Wrap indices at 256 so it will fit in the PERM array
    let ii = S::and_epi64(i, S::set1_epi64(0xff));
    let jj = S::and_epi64(j, S::set1_epi64(0xff));
    let kk = S::and_epi64(k, S::set1_epi64(0xff));

    let gi0 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(
            ii,
            S::i64gather_epi64(&PERM64, S::add_epi64(jj, S::i64gather_epi64(&PERM64, kk))),
        ),
    );
    let gi1 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(
            S::sub_epi64(ii, i1),
            S::i64gather_epi64(
                &PERM64,
                S::add_epi64(
                    S::sub_epi64(jj, j1),
                    S::i64gather_epi64(&PERM64, S::sub_epi64(kk, k1)),
                ),
            ),
        ),
    );
    let gi2 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(
            S::sub_epi64(ii, i2),
            S::i64gather_epi64(
                &PERM64,
                S::add_epi64(
                    S::sub_epi64(jj, j2),
                    S::i64gather_epi64(&PERM64, S::sub_epi64(kk, k2)),
                ),
            ),
        ),
    );
    let gi3 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(
            S::sub_epi64(ii, S::set1_epi64(-1)),
            S::i64gather_epi64(
                &PERM64,
                S::add_epi64(
                    S::sub_epi64(jj, S::set1_epi64(-1)),
                    S::i64gather_epi64(&PERM64, S::sub_epi64(kk, S::set1_epi64(-1))),
                ),
            ),
        ),
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y - z*z
    let t0 = S::fnmadd_pd(
        z0,
        z0,
        S::fnmadd_pd(y0, y0, S::fnmadd_pd(x0, x0, S::set1_pd(0.5))),
    );
    let t1 = S::fnmadd_pd(
        z1,
        z1,
        S::fnmadd_pd(y1, y1, S::fnmadd_pd(x1, x1, S::set1_pd(0.5))),
    );
    let t2 = S::fnmadd_pd(
        z2,
        z2,
        S::fnmadd_pd(y2, y2, S::fnmadd_pd(x2, x2, S::set1_pd(0.5))),
    );
    let t3 = S::fnmadd_pd(
        z3,
        z3,
        S::fnmadd_pd(y3, y3, S::fnmadd_pd(x3, x3, S::set1_pd(0.5))),
    );

    //ti*ti*ti*ti
    let mut t0q = S::mul_pd(t0, t0);
    t0q = S::mul_pd(t0q, t0q);
    let mut t1q = S::mul_pd(t1, t1);
    t1q = S::mul_pd(t1q, t1q);
    let mut t2q = S::mul_pd(t2, t2);
    t2q = S::mul_pd(t2q, t2q);
    let mut t3q = S::mul_pd(t3, t3);
    t3q = S::mul_pd(t3q, t3q);

    let mut n0 = S::mul_pd(t0q, grad3d::<S>(seed, gi0, x0, y0, z0));
    let mut n1 = S::mul_pd(t1q, grad3d::<S>(seed, gi1, x1, y1, z1));
    let mut n2 = S::mul_pd(t2q, grad3d::<S>(seed, gi2, x2, y2, z2));
    let mut n3 = S::mul_pd(t3q, grad3d::<S>(seed, gi3, x3, y3, z3));

    //if ti < 0 then 0 else ni
    let mut cond = S::cmplt_pd(t0, S::setzero_pd());
    n0 = S::andnot_pd(cond, n0);
    cond = S::cmplt_pd(t1, S::setzero_pd());
    n1 = S::andnot_pd(cond, n1);
    cond = S::cmplt_pd(t2, S::setzero_pd());
    n2 = S::andnot_pd(cond, n2);
    cond = S::cmplt_pd(t3, S::setzero_pd());
    n3 = S::andnot_pd(cond, n3);

    S::add_pd(n0, S::add_pd(n1, S::add_pd(n2, n3)))
}

/// Samples 4-dimensional simplex noise
///
/// Produces a value -1 ≤ n ≤ 1.
#[inline(always)]
pub unsafe fn simplex_4d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let s = S::mul_pd(
        S::set1_pd(F4_64),
        S::add_pd(x, S::add_pd(y, S::add_pd(z, w))),
    );

    let ipd = S::floor_pd(S::add_pd(x, s));
    let jpd = S::floor_pd(S::add_pd(y, s));
    let kpd = S::floor_pd(S::add_pd(z, s));
    let lpd = S::floor_pd(S::add_pd(w, s));

    let i = S::cvtpd_epi64(ipd);
    let j = S::cvtpd_epi64(jpd);
    let k = S::cvtpd_epi64(kpd);
    let l = S::cvtpd_epi64(lpd);

    let t = S::mul_pd(
        S::cvtepi64_pd(S::add_epi64(i, S::add_epi64(j, S::add_epi64(k, l)))),
        S::set1_pd(G4),
    );
    let x0 = S::sub_pd(x, S::sub_pd(ipd, t));
    let y0 = S::sub_pd(y, S::sub_pd(jpd, t));
    let z0 = S::sub_pd(z, S::sub_pd(kpd, t));
    let w0 = S::sub_pd(w, S::sub_pd(lpd, t));

    let mut rank_x = S::setzero_epi64();
    let mut rank_y = S::setzero_epi64();
    let mut rank_z = S::setzero_epi64();
    let mut rank_w = S::setzero_epi64();

    let cond = S::castpd_epi64(S::cmpgt_pd(x0, y0));
    rank_x = S::add_epi64(rank_x, S::and_epi64(cond, S::set1_epi64(1)));
    rank_y = S::add_epi64(rank_y, S::andnot_epi64(cond, S::set1_epi64(1)));
    let cond = S::castpd_epi64(S::cmpgt_pd(x0, z0));
    rank_x = S::add_epi64(rank_x, S::and_epi64(cond, S::set1_epi64(1)));
    rank_z = S::add_epi64(rank_z, S::andnot_epi64(cond, S::set1_epi64(1)));
    let cond = S::castpd_epi64(S::cmpgt_pd(x0, w0));
    rank_x = S::add_epi64(rank_x, S::and_epi64(cond, S::set1_epi64(1)));
    rank_w = S::add_epi64(rank_w, S::andnot_epi64(cond, S::set1_epi64(1)));
    let cond = S::castpd_epi64(S::cmpgt_pd(y0, z0));
    rank_y = S::add_epi64(rank_y, S::and_epi64(cond, S::set1_epi64(1)));
    rank_z = S::add_epi64(rank_z, S::andnot_epi64(cond, S::set1_epi64(1)));
    let cond = S::castpd_epi64(S::cmpgt_pd(y0, w0));
    rank_y = S::add_epi64(rank_y, S::and_epi64(cond, S::set1_epi64(1)));
    rank_w = S::add_epi64(rank_w, S::andnot_epi64(cond, S::set1_epi64(1)));
    let cond = S::castpd_epi64(S::cmpgt_pd(z0, w0));
    rank_z = S::add_epi64(rank_z, S::and_epi64(cond, S::set1_epi64(1)));
    rank_w = S::add_epi64(rank_w, S::andnot_epi64(cond, S::set1_epi64(1)));

    let cond = S::cmpgt_epi64(rank_x, S::set1_epi64(2));
    let i1 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_y, S::set1_epi64(2));
    let j1 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_z, S::set1_epi64(2));
    let k1 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_w, S::set1_epi64(2));
    let l1 = S::and_epi64(S::set1_epi64(1), cond);

    let cond = S::cmpgt_epi64(rank_x, S::set1_epi64(1));
    let i2 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_y, S::set1_epi64(1));
    let j2 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_z, S::set1_epi64(1));
    let k2 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_w, S::set1_epi64(1));
    let l2 = S::and_epi64(S::set1_epi64(1), cond);

    let cond = S::cmpgt_epi64(rank_x, S::setzero_epi64());
    let i3 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_y, S::setzero_epi64());
    let j3 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_z, S::setzero_epi64());
    let k3 = S::and_epi64(S::set1_epi64(1), cond);
    let cond = S::cmpgt_epi64(rank_w, S::setzero_epi64());
    let l3 = S::and_epi64(S::set1_epi64(1), cond);

    let x1 = S::add_pd(S::sub_pd(x0, S::cvtepi64_pd(i1)), S::set1_pd(G4));
    let y1 = S::add_pd(S::sub_pd(y0, S::cvtepi64_pd(j1)), S::set1_pd(G4));
    let z1 = S::add_pd(S::sub_pd(z0, S::cvtepi64_pd(k1)), S::set1_pd(G4));
    let w1 = S::add_pd(S::sub_pd(w0, S::cvtepi64_pd(l1)), S::set1_pd(G4));
    let x2 = S::add_pd(S::sub_pd(x0, S::cvtepi64_pd(i2)), S::set1_pd(G24));
    let y2 = S::add_pd(S::sub_pd(y0, S::cvtepi64_pd(j2)), S::set1_pd(G24));
    let z2 = S::add_pd(S::sub_pd(z0, S::cvtepi64_pd(k2)), S::set1_pd(G24));
    let w2 = S::add_pd(S::sub_pd(w0, S::cvtepi64_pd(l2)), S::set1_pd(G24));
    let x3 = S::add_pd(S::sub_pd(x0, S::cvtepi64_pd(i3)), S::set1_pd(G34));
    let y3 = S::add_pd(S::sub_pd(y0, S::cvtepi64_pd(j3)), S::set1_pd(G34));
    let z3 = S::add_pd(S::sub_pd(z0, S::cvtepi64_pd(k3)), S::set1_pd(G34));
    let w3 = S::add_pd(S::sub_pd(w0, S::cvtepi64_pd(l3)), S::set1_pd(G34));
    let x4 = S::add_pd(S::sub_pd(x0, S::set1_pd(1.0)), S::set1_pd(G44));
    let y4 = S::add_pd(S::sub_pd(y0, S::set1_pd(1.0)), S::set1_pd(G44));
    let z4 = S::add_pd(S::sub_pd(z0, S::set1_pd(1.0)), S::set1_pd(G44));
    let w4 = S::add_pd(S::sub_pd(w0, S::set1_pd(1.0)), S::set1_pd(G44));

    let ii = S::and_epi64(i, S::set1_epi64(0xff));
    let jj = S::and_epi64(j, S::set1_epi64(0xff));
    let kk = S::and_epi64(k, S::set1_epi64(0xff));
    let ll = S::and_epi64(l, S::set1_epi64(0xff));

    let lp = S::i64gather_epi64(&PERM64, ll);
    let kp = S::i64gather_epi64(&PERM64, S::add_epi64(kk, lp));
    let jp = S::i64gather_epi64(&PERM64, S::add_epi64(jj, kp));
    let gi0 = S::i64gather_epi64(&PERM64, S::add_epi64(ii, jp));

    let lp = S::i64gather_epi64(&PERM64, S::add_epi64(ll, l1));
    let kp = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(kk, k1), lp));
    let jp = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(jj, j1), kp));
    let gi1 = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(ii, i1), jp));

    let lp = S::i64gather_epi64(&PERM64, S::add_epi64(ll, l2));
    let kp = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(kk, k2), lp));
    let jp = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(jj, j2), kp));
    let gi2 = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(ii, i2), jp));

    let lp = S::i64gather_epi64(&PERM64, S::add_epi64(ll, l3));
    let kp = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(kk, k3), lp));
    let jp = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(jj, j3), kp));
    let gi3 = S::i64gather_epi64(&PERM64, S::add_epi64(S::add_epi64(ii, i3), jp));

    let lp = S::i64gather_epi64(&PERM64, S::add_epi64(ll, S::set1_epi64(1)));
    let kp = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(S::add_epi64(kk, S::set1_epi64(1)), lp),
    );
    let jp = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(S::add_epi64(jj, S::set1_epi64(1)), kp),
    );
    let gi4 = S::i64gather_epi64(
        &PERM64,
        S::add_epi64(S::add_epi64(ii, S::set1_epi64(1)), jp),
    );

    let t0 = S::sub_pd(
        S::sub_pd(
            S::sub_pd(
                S::sub_pd(S::set1_pd(0.5), S::mul_pd(x0, x0)),
                S::mul_pd(y0, y0),
            ),
            S::mul_pd(z0, z0),
        ),
        S::mul_pd(w0, w0),
    );
    let t1 = S::sub_pd(
        S::sub_pd(
            S::sub_pd(
                S::sub_pd(S::set1_pd(0.5), S::mul_pd(x1, x1)),
                S::mul_pd(y1, y1),
            ),
            S::mul_pd(z1, z1),
        ),
        S::mul_pd(w1, w1),
    );
    let t2 = S::sub_pd(
        S::sub_pd(
            S::sub_pd(
                S::sub_pd(S::set1_pd(0.5), S::mul_pd(x2, x2)),
                S::mul_pd(y2, y2),
            ),
            S::mul_pd(z2, z2),
        ),
        S::mul_pd(w2, w2),
    );
    let t3 = S::sub_pd(
        S::sub_pd(
            S::sub_pd(
                S::sub_pd(S::set1_pd(0.5), S::mul_pd(x3, x3)),
                S::mul_pd(y3, y3),
            ),
            S::mul_pd(z3, z3),
        ),
        S::mul_pd(w3, w3),
    );
    let t4 = S::sub_pd(
        S::sub_pd(
            S::sub_pd(
                S::sub_pd(S::set1_pd(0.5), S::mul_pd(x4, x4)),
                S::mul_pd(y4, y4),
            ),
            S::mul_pd(z4, z4),
        ),
        S::mul_pd(w4, w4),
    );
    //ti*ti*ti*ti
    let mut t0q = S::mul_pd(t0, t0);
    t0q = S::mul_pd(t0q, t0q);
    let mut t1q = S::mul_pd(t1, t1);
    t1q = S::mul_pd(t1q, t1q);
    let mut t2q = S::mul_pd(t2, t2);
    t2q = S::mul_pd(t2q, t2q);
    let mut t3q = S::mul_pd(t3, t3);
    t3q = S::mul_pd(t3q, t3q);
    let mut t4q = S::mul_pd(t4, t4);
    t4q = S::mul_pd(t4q, t4q);

    let mut n0 = S::mul_pd(t0q, grad4::<S>(seed, gi0, x0, y0, z0, w0));
    let mut n1 = S::mul_pd(t1q, grad4::<S>(seed, gi1, x1, y1, z1, w1));
    let mut n2 = S::mul_pd(t2q, grad4::<S>(seed, gi2, x2, y2, z2, w2));
    let mut n3 = S::mul_pd(t3q, grad4::<S>(seed, gi3, x3, y3, z3, w3));
    let mut n4 = S::mul_pd(t4q, grad4::<S>(seed, gi4, x4, y4, z4, w4));

    //if ti < 0 then 0 else ni
    let mut cond = S::cmplt_pd(t0, S::setzero_pd());
    n0 = S::andnot_pd(cond, n0);
    cond = S::cmplt_pd(t1, S::setzero_pd());
    n1 = S::andnot_pd(cond, n1);
    cond = S::cmplt_pd(t2, S::setzero_pd());
    n2 = S::andnot_pd(cond, n2);
    cond = S::cmplt_pd(t3, S::setzero_pd());
    n3 = S::andnot_pd(cond, n3);
    cond = S::cmplt_pd(t4, S::setzero_pd());
    n4 = S::andnot_pd(cond, n4);

    S::add_pd(n0, S::add_pd(n1, S::add_pd(n2, S::add_pd(n3, n4)))) * S::set1_pd(62.77772078955791)
}

#[cfg(test)]
mod tests {
    use super::*;
    use simdeez::scalar::{F64x1, Scalar};

    fn check_bounds(min: f64, max: f64) {
        assert!(min < -0.75 && min >= -1.0, "min out of range {}", min);
        assert!(max > 0.75 && max <= 1.0, "max out of range: {}", max);
    }

    #[test]
    fn simplex_1d_range() {
        for seed in 0..10 {
            let mut min = f64::INFINITY;
            let mut max = -f64::INFINITY;
            for x in 0..1000 {
                let n = unsafe { simplex_1d::<Scalar>(F64x1(x as f64 / 10.0), seed).0 };
                min = min.min(n);
                max = max.max(n);
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn simplex_2d_range() {
        for seed in 0..10 {
            let mut min = f64::INFINITY;
            let mut max = -f64::INFINITY;
            for y in 0..10 {
                for x in 0..100 {
                    let n = unsafe {
                        simplex_2d::<Scalar>(F64x1(x as f64 / 10.0), F64x1(y as f64 / 10.0), seed).0
                    };
                    min = min.min(n);
                    max = max.max(n);
                }
            }
            check_bounds(min, max);
        }
    }

    #[test]
    fn simplex_4d_range() {
        let mut min = f64::INFINITY;
        let mut max = -f64::INFINITY;
        const SEED: i64 = 0;
        for w in 0..10 {
            for z in 0..10 {
                for y in 0..10 {
                    for x in 0..1000 {
                        let n = unsafe {
                            simplex_4d::<Scalar>(
                                F64x1(x as f64 / 10.0),
                                F64x1(y as f64 / 10.0),
                                F64x1(z as f64 / 10.0),
                                F64x1(w as f64 / 10.0),
                                SEED,
                            )
                            .0
                        };
                        min = min.min(n);
                        max = max.max(n);
                    }
                }
            }
        }
        check_bounds(min, max);
    }
}
