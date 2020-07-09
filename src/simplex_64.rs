use self::simdeez::*;
use super::*;
use crate::shared::*;

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

/// Generates a random integer gradient in ±7 inclusive, and returns its product with `x`
///
/// This differs from Gustavson's well-known implementation in that gradients can be zero, and the
/// maximum gradient is 7 rather than 8.
#[inline(always)]
pub unsafe fn grad1<S: Simd>(seed: i64, hash: S::Vi64, x: S::Vf64) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(S::set1_epi64(seed), hash), S::set1_epi64(15));
    let v = S::cvtepi64_pd(S::and_epi64(h, S::set1_epi64(7)));

    let h_and_8 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(8)),
    ));
    let grad = S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_8);
    S::mul_pd(grad, x)
}

#[inline(always)]
pub unsafe fn fbm_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut result = simplex_1d::<S>(x, seed);

    for _ in 1..octaves {
        x = S::mul_pd(x, lacunarity);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(result, simplex_1d::<S>(x, seed));
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut result = S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_1d::<S>(x, seed)));

    for _ in 1..octaves {
        x = S::mul_pd(x, lacunarity);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_1d::<S>(x, seed))),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::set1_pd(1.0);
    let mut result = S::abs_pd(simplex_1d::<S>(x, seed));

    for _ in 1..octaves {
        x = S::mul_pd(x, lacunarity);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(result, S::abs_pd(simplex_1d::<S>(x, seed)));
    }

    result
}

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
pub unsafe fn simplex_2d<S: Simd>(x: S::Vf64, y: S::Vf64, seed: i64) -> S::Vf64 {
    let s = S::mul_pd(S::set1_pd(F2), S::add_pd(x, y));
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

    S::add_pd(n0, S::add_pd(n1, n2))
}

#[inline(always)]
pub unsafe fn fbm_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_2d::<S>(x, y, seed);
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(S::mul_pd(simplex_2d::<S>(x, y, seed), amp), result);
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_2d::<S>(x, y, seed)));
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::fnmadd_pd(S::abs_pd(simplex_2d::<S>(x, y, seed)), amp, S::set1_pd(1.0)),
        );
    }

    result
}
#[inline(always)]
pub unsafe fn turbulence_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::abs_pd(simplex_2d::<S>(x, y, seed));

    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::abs_pd(S::mul_pd(simplex_2d::<S>(x, y, seed), amp)),
        );
    }

    result
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

#[inline(always)]
pub unsafe fn ridge_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::sub_pd(S::set1_pd(1.0), S::abs_pd(simplex_3d::<S>(x, y, z, seed)));
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::fnmadd_pd(
                S::abs_pd(simplex_3d::<S>(x, y, z, seed)),
                amp,
                S::set1_pd(1.0),
            ),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::abs_pd(simplex_3d::<S>(x, y, z, seed));
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::abs_pd(S::mul_pd(simplex_3d::<S>(x, y, z, seed), amp)),
        );
    }

    result
}

#[inline(always)]
unsafe fn grad4<S: Simd>(
    seed: i64,
    hash: S::Vi64,
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    t: S::Vf64,
) -> S::Vf64 {
    let h = S::and_epi64(S::xor_epi64(S::set1_epi64(seed), hash), S::set1_epi64(31));
    let mut mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(24), h));
    let u = S::blendv_pd(y, x, mask);
    mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(16), h));
    let v = S::blendv_pd(z, y, mask);
    mask = S::castepi64_pd(S::cmpgt_epi64(S::set1_epi64(8), h));
    let w = S::blendv_pd(t, z, mask);

    let h_and_1 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(1)),
    ));
    let h_and_2 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(2)),
    ));
    let h_and_4 = S::castepi64_pd(S::cmpeq_epi64(
        S::setzero_epi64(),
        S::and_epi64(h, S::set1_epi64(4)),
    ));

    S::add_pd(
        S::blendv_pd(S::sub_pd(S::setzero_pd(), u), u, h_and_1),
        S::add_pd(
            S::blendv_pd(S::sub_pd(S::setzero_pd(), v), v, h_and_2),
            S::blendv_pd(S::sub_pd(S::setzero_pd(), w), w, h_and_4),
        ),
    )
}
#[inline(always)]
pub unsafe fn simplex_4d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let s = S::mul_pd(S::set1_pd(F4), S::add_pd(x, S::add_pd(y, S::add_pd(z, w))));

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

    S::add_pd(n0, S::add_pd(n1, S::add_pd(n2, S::add_pd(n3, n4))))
}
#[inline(always)]
pub unsafe fn fbm_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed);
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        w = S::mul_pd(w, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(result, S::mul_pd(simplex_4d::<S>(x, y, z, w, seed), amp));
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::sub_pd(
        S::set1_pd(1.0),
        S::abs_pd(simplex_4d::<S>(x, y, z, w, seed)),
    );
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        w = S::mul_pd(w, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::sub_pd(
                S::set1_pd(1.0),
                S::abs_pd(S::mul_pd(simplex_4d::<S>(x, y, z, w, seed), amp)),
            ),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: u8,
    seed: i64,
) -> S::Vf64 {
    let mut result = S::abs_pd(simplex_4d::<S>(x, y, z, w, seed));
    let mut amp = S::set1_pd(1.0);

    for _ in 1..octaves {
        x = S::mul_pd(x, lac);
        y = S::mul_pd(y, lac);
        z = S::mul_pd(z, lac);
        w = S::mul_pd(w, lac);
        amp = S::mul_pd(amp, gain);
        result = S::add_pd(
            result,
            S::abs_pd(S::mul_pd(simplex_4d::<S>(x, y, z, w, seed), amp)),
        );
    }

    result
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
}
