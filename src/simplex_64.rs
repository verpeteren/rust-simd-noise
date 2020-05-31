use self::simdeez::*;
use super::*;
use crate::shared::*;
use std::f32;

const F2: f64 = 0.36602540378;
const F3: f64 = 1.0 / 3.0;
const F4: f64 = 0.309016994;
const G2: f64 = 0.2113248654;
const G22: f64 = G2 * 2.0;
const G3: f64 = 1.0 / 6.0;
const G4: f64 = 0.138196601;
const G24: f64 = 2.0 * G4;
const G34: f64 = 3.0 * G4;
const G44: f64 = 4.0 * G4;

#[inline(always)]
unsafe fn grad2<S: Simd>(seed: i64, hash: S::Vi64, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(hash, S::set1_epi64(seed)), S::set1_epi64(7));
    let mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(4), h));
    let u = S::blendv_pd(y, x, mask);
    let v = S::mul_pd(S::set1_pd(2.0), S::blendv_pd(x, y, mask));

    let h_and_1 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(1)),
    ));
    let h_and_2 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(2)),
    ));

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
    )
}


#[inline(always)]
unsafe fn grad3d<S: Simd>(seed: i64, hash: S::Vi64, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(S::set1_epi64(seed), hash), S::set1_epi64(15));

    let mut u = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(8), h));
    u = S::blendv_pd(y, x, u);

    let mut v = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(4), h));
    let mut h12_or_14 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::or_epi64(
            S::cmpeq_epi64(h, S::set1_epi64(12)),
            S::cmpeq_epi64(h, S::set1_epi64(14)),
        ),
    ));
    h12_or_14 = S::blendv_pd(x, z, h12_or_14);
    v = S::blendv_pd(h12_or_14, y, v);

    let h_and_1 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(1)),
    ));
    let h_and_2 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(2)),
    ));

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
    )
}

#[inline(always)]
pub unsafe fn simplex_3d<S: Simd>(x: S::Vf64, y: S::Vf64, z: S::Vf64, seed: i64) -> S::Vf64 {
    let s = S::mul_pd(S::set1_pd(F3), S::add_pd(x, S::add_pd(y, z)));

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
    let x2 = S::add_pd(S::add_pd(x0, S::cvtepi64_pd(i2)), S::set1_pd(F3));
    let y2 = S::add_pd(S::add_pd(y0, S::cvtepi64_pd(j2)), S::set1_pd(F3));
    let z2 = S::add_pd(S::add_pd(z0, S::cvtepi64_pd(k2)), S::set1_pd(F3));
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

#[inline(always)]
pub unsafe fn fbm_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_3d::<S>(x, y, z, seed);
    let mut amp = S::set1_pd(1.0);
    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(S::mul_pd(simplex_3d::<S>(x, y, z, seed), amp), result);
    }
    result
}
