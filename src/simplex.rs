use self::simdeez::*;
use super::*;
use shared::*;
use std::f32;

const F2: f32 = 0.36602540378;
const F3: f32 = 1.0 / 3.0;
const F4: f32 = 0.309016994;
const G2: f32 = 0.2113248654;
const G22: f32 = G2 * 2.0;
const G3: f32 = 1.0 / 6.0;
const G4: f32 = 0.138196601;
const G24: f32 = 2.0 * G4;
const G34: f32 = 3.0 * G4;
const G44: f32 = 4.0 * G4;

#[inline(always)]
pub unsafe fn grad1_simd<S: Simd>(hash: S::Vi32, x: S::Vf32) -> S::Vf32 {
    let h = S::and_si(hash, S::set1_epi32(15));
    let v = S::cvtepi32_ps(S::and_si(h, S::set1_epi32(7)));

    let h_and_8 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(8)),
    ));
    let grad = S::blendv_ps(S::sub_ps(S::setzero_ps(), v), v, h_and_8);
    S::mul_ps(grad, x)
}
#[inline(always)]
pub unsafe fn simplex_1d<S: Simd>(x: S::Vf32) -> S::Vf32 {
    let ips = S::fastfloor_ps(x);
    let mut i0 = S::cvtps_epi32(ips);
    let i1 = S::and_si(S::add_epi32(i0, S::set1_epi32(1)), S::set1_epi32(0xff));

    let x0 = S::sub_ps(x, ips);
    let x1 = S::sub_ps(x0, S::set1_ps(1.0));

    i0 = S::and_si(i0, S::set1_epi32(0xff));
    let gi0 = S::i32gather_epi32(&PERM, i0);
    let gi1 = S::i32gather_epi32(&PERM, i1);

    let mut t0 = S::sub_ps(S::set1_ps(1.0), S::mul_ps(x0, x0));
    t0 = S::mul_ps(t0, t0);
    t0 = S::mul_ps(t0, t0);
    let n0 = S::mul_ps(t0, grad1_simd::<S>(gi0, x0));

    let mut t1 = S::sub_ps(S::set1_ps(1.0), S::mul_ps(x1, x1));
    t1 = S::mul_ps(t1, t1);
    t1 = S::mul_ps(t1, t1);
    let n1 = S::mul_ps(t1, grad1_simd::<S>(gi1, x1));

    S::add_ps(n0, n1)
}

#[inline(always)]
pub unsafe fn fbm_1d<S: Simd>(
    x: S::Vf32,
    freq: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut result = simplex_1d::<S>(xf);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::mul_ps(simplex_1d::<S>(xf), amp));
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_1d<S: Simd>(
    x: S::Vf32,
    freq: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut result = S::sub_ps(S::set1_ps(1.0), S::abs_ps(simplex_1d::<S>(xf)));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::sub_ps(
                S::set1_ps(1.0),
                S::abs_ps(S::mul_ps(simplex_1d::<S>(xf), amp)),
            ),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_1d<S: Simd>(
    x: S::Vf32,
    freq: S::Vf32,
    lacunarity: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut result = S::abs_ps(simplex_1d::<S>(xf));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lacunarity);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::abs_ps(S::mul_ps(simplex_1d::<S>(xf), amp)));
    }

    result
}

#[inline(always)]
unsafe fn grad2<S: Simd>(hash: S::Vi32, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
    let h = S::and_si(hash, S::set1_epi32(7));
    let mask = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(4), h));
    let u = S::blendv_ps(y, x, mask);
    let v = S::mul_ps(S::set1_ps(2.0), S::blendv_ps(x, y, mask));

    let h_and_1 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(1)),
    ));
    let h_and_2 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(2)),
    ));

    S::add_ps(
        S::blendv_ps(S::sub_ps(S::setzero_ps(), u), u, h_and_1),
        S::blendv_ps(S::sub_ps(S::setzero_ps(), v), v, h_and_2),
    )
}

#[inline(always)]
pub unsafe fn simplex_2d<S: Simd>(x: S::Vf32, y: S::Vf32) -> S::Vf32 {
    let s = S::mul_ps(S::set1_ps(F2), S::add_ps(x, y));
    let ips = S::floor_ps(S::add_ps(x, s));
    let jps = S::floor_ps(S::add_ps(y, s));

    let i = S::cvtps_epi32(ips);
    let j = S::cvtps_epi32(jps);

    let t = S::mul_ps(S::cvtepi32_ps(S::add_epi32(i, j)), S::set1_ps(G2));

    let x0 = S::sub_ps(x, S::sub_ps(ips, t));
    let y0 = S::sub_ps(y, S::sub_ps(jps, t));

    let i1 = S::castps_si(S::cmpge_ps(x0, y0));

    let j1 = S::castps_si(S::cmpgt_ps(y0, x0));

    let x1 = S::add_ps(S::add_ps(x0, S::cvtepi32_ps(i1)), S::set1_ps(G2));
    let y1 = S::add_ps(S::add_ps(y0, S::cvtepi32_ps(j1)), S::set1_ps(G2));
    let x2 = S::add_ps(S::add_ps(x0, S::set1_ps(-1.0)), S::set1_ps(G22));
    let y2 = S::add_ps(S::add_ps(y0, S::set1_ps(-1.0)), S::set1_ps(G22));

    let ii = S::and_si(i, S::set1_epi32(0xff));
    let jj = S::and_si(j, S::set1_epi32(0xff));

    let gi0 = S::i32gather_epi32(&PERM, S::add_epi32(ii, S::i32gather_epi32(&PERM, jj)));

    let gi1 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, i1),
            S::i32gather_epi32(&PERM, S::sub_epi32(jj, j1)),
        ),
    );

    let gi2 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, S::set1_epi32(-1)),
            S::i32gather_epi32(&PERM, S::sub_epi32(jj, S::set1_epi32(-1))),
        ),
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y
    let t0 = S::fnmadd_ps(y0, y0, S::fnmadd_ps(x0, x0, S::set1_ps(0.5)));
    let t1 = S::fnmadd_ps(y1, y1, S::fnmadd_ps(x1, x1, S::set1_ps(0.5)));
    let t2 = S::fnmadd_ps(y2, y2, S::fnmadd_ps(x2, x2, S::set1_ps(0.5)));

    let mut t0q = S::mul_ps(t0, t0);
    t0q = S::mul_ps(t0q, t0q);
    let mut t1q = S::mul_ps(t1, t1);
    t1q = S::mul_ps(t1q, t1q);
    let mut t2q = S::mul_ps(t2, t2);
    t2q = S::mul_ps(t2q, t2q);

    let mut n0 = S::mul_ps(t0q, grad2::<S>(gi0, x0, y0));
    let mut n1 = S::mul_ps(t1q, grad2::<S>(gi1, x1, y1));
    let mut n2 = S::mul_ps(t2q, grad2::<S>(gi2, x2, y2));

    let mut cond = S::cmplt_ps(t0, S::setzero_ps());
    n0 = S::andnot_ps(cond, n0);
    cond = S::cmplt_ps(t1, S::setzero_ps());
    n1 = S::andnot_ps(cond, n1);
    cond = S::cmplt_ps(t2, S::setzero_ps());
    n2 = S::andnot_ps(cond, n2);

    S::add_ps(n0, S::add_ps(n1, n2))
}
#[inline(always)]
pub unsafe fn fbm_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut result = simplex_2d::<S>(xf, yf);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::fmadd_ps(simplex_2d::<S>(xf, yf), amp, result);
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut result = S::sub_ps(S::set1_ps(1.0), S::abs_ps(simplex_2d::<S>(xf, yf)));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::fnmadd_ps(S::abs_ps(simplex_2d::<S>(xf, yf)), amp, S::set1_ps(1.0)),
        );
    }

    result
}
#[inline(always)]
pub unsafe fn turbulence_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut result = S::abs_ps(simplex_2d::<S>(xf, yf));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::abs_ps(S::mul_ps(simplex_2d::<S>(xf, yf), amp)));
    }

    result
}

#[inline(always)]
unsafe fn grad3d<S: Simd>(hash: S::Vi32, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
    let h = S::and_si(hash, S::set1_epi32(15));

    let mut u = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(8), h));
    u = S::blendv_ps(y, x, u);

    let mut v = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(4), h));
    let mut h12_or_14 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::or_si(
            S::cmpeq_epi32(h, S::set1_epi32(12)),
            S::cmpeq_epi32(h, S::set1_epi32(14)),
        ),
    ));
    h12_or_14 = S::blendv_ps(x, z, h12_or_14);
    v = S::blendv_ps(h12_or_14, y, v);

    let h_and_1 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(1)),
    ));
    let h_and_2 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(2)),
    ));

    S::add_ps(
        S::blendv_ps(S::sub_ps(S::setzero_ps(), u), u, h_and_1),
        S::blendv_ps(S::sub_ps(S::setzero_ps(), v), v, h_and_2),
    )
}

#[inline(always)]
pub unsafe fn simplex_3d<S: Simd>(x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
    let s = S::mul_ps(S::set1_ps(F3), S::add_ps(x, S::add_ps(y, z)));

    let ips = S::floor_ps(S::add_ps(x, s));
    let jps = S::floor_ps(S::add_ps(y, s));
    let kps = S::floor_ps(S::add_ps(z, s));

    let i = S::cvtps_epi32(ips);
    let j = S::cvtps_epi32(jps);
    let k = S::cvtps_epi32(kps);

    let t = S::mul_ps(
        S::cvtepi32_ps(S::add_epi32(i, S::add_epi32(j, k))),
        S::set1_ps(G3),
    );

    let x0 = S::sub_ps(x, S::sub_ps(ips, t));
    let y0 = S::sub_ps(y, S::sub_ps(jps, t));
    let z0 = S::sub_ps(z, S::sub_ps(kps, t));

    let i1 = S::and_si(
        S::castps_si(S::cmpge_ps(x0, y0)),
        S::castps_si(S::cmpge_ps(x0, z0)),
    );
    let j1 = S::and_si(
        S::castps_si(S::cmpgt_ps(y0, x0)),
        S::castps_si(S::cmpge_ps(y0, z0)),
    );
    let k1 = S::and_si(
        S::castps_si(S::cmpgt_ps(z0, x0)),
        S::castps_si(S::cmpgt_ps(z0, y0)),
    );

    //for i2
    let yx_xz = S::and_si(
        S::castps_si(S::cmpge_ps(x0, y0)),
        S::castps_si(S::cmplt_ps(x0, z0)),
    );
    let zx_xy = S::and_si(
        S::castps_si(S::cmpge_ps(x0, z0)),
        S::castps_si(S::cmplt_ps(x0, y0)),
    );

    //for j2
    let xy_yz = S::and_si(
        S::castps_si(S::cmplt_ps(x0, y0)),
        S::castps_si(S::cmplt_ps(y0, z0)),
    );
    let zy_yx = S::and_si(
        S::castps_si(S::cmpge_ps(y0, z0)),
        S::castps_si(S::cmpge_ps(x0, y0)),
    );

    //for k2
    let yz_zx = S::and_si(
        S::castps_si(S::cmplt_ps(y0, z0)),
        S::castps_si(S::cmpge_ps(x0, z0)),
    );
    let xz_zy = S::and_si(
        S::castps_si(S::cmplt_ps(x0, z0)),
        S::castps_si(S::cmpge_ps(y0, z0)),
    );

    let i2 = S::or_si(i1, S::or_si(yx_xz, zx_xy));
    let j2 = S::or_si(j1, S::or_si(xy_yz, zy_yx));
    let k2 = S::or_si(k1, S::or_si(yz_zx, xz_zy));

    let x1 = S::add_ps(S::add_ps(x0, S::cvtepi32_ps(i1)), S::set1_ps(G3));
    let y1 = S::add_ps(S::add_ps(y0, S::cvtepi32_ps(j1)), S::set1_ps(G3));
    let z1 = S::add_ps(S::add_ps(z0, S::cvtepi32_ps(k1)), S::set1_ps(G3));
    let x2 = S::add_ps(S::add_ps(x0, S::cvtepi32_ps(i2)), S::set1_ps(F3));
    let y2 = S::add_ps(S::add_ps(y0, S::cvtepi32_ps(j2)), S::set1_ps(F3));
    let z2 = S::add_ps(S::add_ps(z0, S::cvtepi32_ps(k2)), S::set1_ps(F3));
    let x3 = S::add_ps(x0, S::set1_ps(-0.5));
    let y3 = S::add_ps(y0, S::set1_ps(-0.5));
    let z3 = S::add_ps(z0, S::set1_ps(-0.5));

    // Wrap indices at 256 so it will fit in the PERM array
    let ii = S::and_si(i, S::set1_epi32(0xff));
    let jj = S::and_si(j, S::set1_epi32(0xff));
    let kk = S::and_si(k, S::set1_epi32(0xff));

    let gi0 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            ii,
            S::i32gather_epi32(&PERM, S::add_epi32(jj, S::i32gather_epi32(&PERM, kk))),
        ),
    );
    let gi1 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, i1),
            S::i32gather_epi32(
                &PERM,
                S::add_epi32(
                    S::sub_epi32(jj, j1),
                    S::i32gather_epi32(&PERM, S::sub_epi32(kk, k1)),
                ),
            ),
        ),
    );
    let gi2 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, i2),
            S::i32gather_epi32(
                &PERM,
                S::add_epi32(
                    S::sub_epi32(jj, j2),
                    S::i32gather_epi32(&PERM, S::sub_epi32(kk, k2)),
                ),
            ),
        ),
    );
    let gi3 = S::i32gather_epi32(
        &PERM,
        S::add_epi32(
            S::sub_epi32(ii, S::set1_epi32(-1)),
            S::i32gather_epi32(
                &PERM,
                S::add_epi32(
                    S::sub_epi32(jj, S::set1_epi32(-1)),
                    S::i32gather_epi32(&PERM, S::sub_epi32(kk, S::set1_epi32(-1))),
                ),
            ),
        ),
    );

    // These FMA operations are equivalent to: let t = 0.5 - x*x - y*y - z*z
    let t0 = S::fnmadd_ps(
        z0,
        z0,
        S::fnmadd_ps(y0, y0, S::fnmadd_ps(x0, x0, S::set1_ps(0.5))),
    );
    let t1 = S::fnmadd_ps(
        z1,
        z1,
        S::fnmadd_ps(y1, y1, S::fnmadd_ps(x1, x1, S::set1_ps(0.5))),
    );
    let t2 = S::fnmadd_ps(
        z2,
        z2,
        S::fnmadd_ps(y2, y2, S::fnmadd_ps(x2, x2, S::set1_ps(0.5))),
    );
    let t3 = S::fnmadd_ps(
        z3,
        z3,
        S::fnmadd_ps(y3, y3, S::fnmadd_ps(x3, x3, S::set1_ps(0.5))),
    );

    //ti*ti*ti*ti
    let mut t0q = S::mul_ps(t0, t0);
    t0q = S::mul_ps(t0q, t0q);
    let mut t1q = S::mul_ps(t1, t1);
    t1q = S::mul_ps(t1q, t1q);
    let mut t2q = S::mul_ps(t2, t2);
    t2q = S::mul_ps(t2q, t2q);
    let mut t3q = S::mul_ps(t3, t3);
    t3q = S::mul_ps(t3q, t3q);

    let mut n0 = S::mul_ps(t0q, grad3d::<S>(gi0, x0, y0, z0));
    let mut n1 = S::mul_ps(t1q, grad3d::<S>(gi1, x1, y1, z1));
    let mut n2 = S::mul_ps(t2q, grad3d::<S>(gi2, x2, y2, z2));
    let mut n3 = S::mul_ps(t3q, grad3d::<S>(gi3, x3, y3, z3));

    //if ti < 0 then 0 else ni
    let mut cond = S::cmplt_ps(t0, S::setzero_ps());
    n0 = S::andnot_ps(cond, n0);
    cond = S::cmplt_ps(t1, S::setzero_ps());
    n1 = S::andnot_ps(cond, n1);
    cond = S::cmplt_ps(t2, S::setzero_ps());
    n2 = S::andnot_ps(cond, n2);
    cond = S::cmplt_ps(t3, S::setzero_ps());
    n3 = S::andnot_ps(cond, n3);

    S::add_ps(n0, S::add_ps(n1, S::add_ps(n2, n3)))
}

#[inline(always)]
pub unsafe fn fbm_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut zf = S::mul_ps(z, freq);
    let mut result = simplex_3d::<S>(xf, yf, zf);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        zf = S::mul_ps(zf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::fmadd_ps(simplex_3d::<S>(xf, yf, zf), amp, result);
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut zf = S::mul_ps(z, freq);
    let mut result = S::sub_ps(S::set1_ps(1.0), S::abs_ps(simplex_3d::<S>(xf, yf, zf)));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        zf = S::mul_ps(zf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::fnmadd_ps(S::abs_ps(simplex_3d::<S>(xf, yf, zf)), amp, S::set1_ps(1.0)),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut zf = S::mul_ps(z, freq);
    let mut result = S::abs_ps(simplex_3d::<S>(xf, yf, zf));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        zf = S::mul_ps(zf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::abs_ps(S::mul_ps(simplex_3d::<S>(xf, yf, zf), amp)),
        );
    }

    result
}

#[inline(always)]
unsafe fn grad4<S: Simd>(hash: S::Vi32, x: S::Vf32, y: S::Vf32, z: S::Vf32, t: S::Vf32) -> S::Vf32 {
    let h = S::and_si(hash, S::set1_epi32(31));
    let mut mask = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(24), h));
    let u = S::blendv_ps(y, x, mask);
    mask = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(16), h));
    let v = S::blendv_ps(z, y, mask);
    mask = S::castsi_ps(S::cmpgt_epi32(S::set1_epi32(8), h));
    let w = S::blendv_ps(t, z, mask);

    let h_and_1 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(1)),
    ));
    let h_and_2 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(2)),
    ));
    let h_and_4 = S::castsi_ps(S::cmpeq_epi32(
        S::setzero_si(),
        S::and_si(h, S::set1_epi32(4)),
    ));

    S::add_ps(
        S::blendv_ps(S::sub_ps(S::setzero_ps(), u), u, h_and_1),
        S::add_ps(
            S::blendv_ps(S::sub_ps(S::setzero_ps(), v), v, h_and_2),
            S::blendv_ps(S::sub_ps(S::setzero_ps(), w), w, h_and_4),
        ),
    )
}
#[inline(always)]
pub unsafe fn simplex_4d<S: Simd>(x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
    let s = S::mul_ps(S::set1_ps(F4), S::add_ps(x, S::add_ps(y, S::add_ps(z, w))));

    let ips = S::floor_ps(S::add_ps(x, s));
    let jps = S::floor_ps(S::add_ps(y, s));
    let kps = S::floor_ps(S::add_ps(z, s));
    let lps = S::floor_ps(S::add_ps(w, s));

    let i = S::cvtps_epi32(ips);
    let j = S::cvtps_epi32(jps);
    let k = S::cvtps_epi32(kps);
    let l = S::cvtps_epi32(lps);

    let t = S::mul_ps(
        S::cvtepi32_ps(S::add_epi32(i, S::add_epi32(j, S::add_epi32(k, l)))),
        S::set1_ps(G4),
    );
    let x0 = S::sub_ps(x, S::sub_ps(ips, t));
    let y0 = S::sub_ps(y, S::sub_ps(jps, t));
    let z0 = S::sub_ps(z, S::sub_ps(kps, t));
    let w0 = S::sub_ps(w, S::sub_ps(lps, t));

    let mut rank_x = S::setzero_si();
    let mut rank_y = S::setzero_si();
    let mut rank_z = S::setzero_si();
    let mut rank_w = S::setzero_si();

    let cond = S::castps_si(S::cmpgt_ps(x0, y0));
    rank_x = S::add_epi32(rank_x, S::and_si(cond, S::set1_epi32(1)));
    rank_y = S::add_epi32(rank_y, S::andnot_si(cond, S::set1_epi32(1)));
    let cond = S::castps_si(S::cmpgt_ps(x0, z0));
    rank_x = S::add_epi32(rank_x, S::and_si(cond, S::set1_epi32(1)));
    rank_z = S::add_epi32(rank_z, S::andnot_si(cond, S::set1_epi32(1)));
    let cond = S::castps_si(S::cmpgt_ps(x0, w0));
    rank_x = S::add_epi32(rank_x, S::and_si(cond, S::set1_epi32(1)));
    rank_w = S::add_epi32(rank_w, S::andnot_si(cond, S::set1_epi32(1)));
    let cond = S::castps_si(S::cmpgt_ps(y0, z0));
    rank_y = S::add_epi32(rank_y, S::and_si(cond, S::set1_epi32(1)));
    rank_z = S::add_epi32(rank_z, S::andnot_si(cond, S::set1_epi32(1)));
    let cond = S::castps_si(S::cmpgt_ps(y0, w0));
    rank_y = S::add_epi32(rank_y, S::and_si(cond, S::set1_epi32(1)));
    rank_w = S::add_epi32(rank_w, S::andnot_si(cond, S::set1_epi32(1)));
    let cond = S::castps_si(S::cmpgt_ps(z0, w0));
    rank_z = S::add_epi32(rank_z, S::and_si(cond, S::set1_epi32(1)));
    rank_w = S::add_epi32(rank_w, S::andnot_si(cond, S::set1_epi32(1)));

    let cond = S::cmpgt_epi32(rank_x, S::set1_epi32(2));
    let i1 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_y, S::set1_epi32(2));
    let j1 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_z, S::set1_epi32(2));
    let k1 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_w, S::set1_epi32(2));
    let l1 = S::and_si(S::set1_epi32(1), cond);

    let cond = S::cmpgt_epi32(rank_x, S::set1_epi32(1));
    let i2 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_y, S::set1_epi32(1));
    let j2 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_z, S::set1_epi32(1));
    let k2 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_w, S::set1_epi32(1));
    let l2 = S::and_si(S::set1_epi32(1), cond);

    let cond = S::cmpgt_epi32(rank_x, S::setzero_si());
    let i3 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_y, S::setzero_si());
    let j3 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_z, S::setzero_si());
    let k3 = S::and_si(S::set1_epi32(1), cond);
    let cond = S::cmpgt_epi32(rank_w, S::setzero_si());
    let l3 = S::and_si(S::set1_epi32(1), cond);

    let x1 = S::add_ps(S::sub_ps(x0, S::cvtepi32_ps(i1)), S::set1_ps(G4));
    let y1 = S::add_ps(S::sub_ps(y0, S::cvtepi32_ps(j1)), S::set1_ps(G4));
    let z1 = S::add_ps(S::sub_ps(z0, S::cvtepi32_ps(k1)), S::set1_ps(G4));
    let w1 = S::add_ps(S::sub_ps(w0, S::cvtepi32_ps(l1)), S::set1_ps(G4));
    let x2 = S::add_ps(S::sub_ps(x0, S::cvtepi32_ps(i2)), S::set1_ps(G24));
    let y2 = S::add_ps(S::sub_ps(y0, S::cvtepi32_ps(j2)), S::set1_ps(G24));
    let z2 = S::add_ps(S::sub_ps(z0, S::cvtepi32_ps(k2)), S::set1_ps(G24));
    let w2 = S::add_ps(S::sub_ps(w0, S::cvtepi32_ps(l2)), S::set1_ps(G24));
    let x3 = S::add_ps(S::sub_ps(x0, S::cvtepi32_ps(i3)), S::set1_ps(G34));
    let y3 = S::add_ps(S::sub_ps(y0, S::cvtepi32_ps(j3)), S::set1_ps(G34));
    let z3 = S::add_ps(S::sub_ps(z0, S::cvtepi32_ps(k3)), S::set1_ps(G34));
    let w3 = S::add_ps(S::sub_ps(w0, S::cvtepi32_ps(l3)), S::set1_ps(G34));
    let x4 = S::add_ps(S::sub_ps(x0, S::set1_ps(1.0)), S::set1_ps(G44));
    let y4 = S::add_ps(S::sub_ps(y0, S::set1_ps(1.0)), S::set1_ps(G44));
    let z4 = S::add_ps(S::sub_ps(z0, S::set1_ps(1.0)), S::set1_ps(G44));
    let w4 = S::add_ps(S::sub_ps(w0, S::set1_ps(1.0)), S::set1_ps(G44));

    let ii = S::and_si(i, S::set1_epi32(0xff));
    let jj = S::and_si(j, S::set1_epi32(0xff));
    let kk = S::and_si(k, S::set1_epi32(0xff));
    let ll = S::and_si(l, S::set1_epi32(0xff));

    let lp = S::i32gather_epi32(&PERM, ll);
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(kk, lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(jj, kp));
    let gi0 = S::i32gather_epi32(&PERM, S::add_epi32(ii, jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, l1));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, k1), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, j1), kp));
    let gi1 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, i1), jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, l2));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, k2), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, j2), kp));
    let gi2 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, i2), jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, l3));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, k3), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, j3), kp));
    let gi3 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, i3), jp));

    let lp = S::i32gather_epi32(&PERM, S::add_epi32(ll, S::set1_epi32(1)));
    let kp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(kk, S::set1_epi32(1)), lp));
    let jp = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(jj, S::set1_epi32(1)), kp));
    let gi4 = S::i32gather_epi32(&PERM, S::add_epi32(S::add_epi32(ii, S::set1_epi32(1)), jp));

    let t0 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x0, x0)),
                S::mul_ps(y0, y0),
            ),
            S::mul_ps(z0, z0),
        ),
        S::mul_ps(w0, w0),
    );
    let t1 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x1, x1)),
                S::mul_ps(y1, y1),
            ),
            S::mul_ps(z1, z1),
        ),
        S::mul_ps(w1, w1),
    );
    let t2 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x2, x2)),
                S::mul_ps(y2, y2),
            ),
            S::mul_ps(z2, z2),
        ),
        S::mul_ps(w2, w2),
    );
    let t3 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x3, x3)),
                S::mul_ps(y3, y3),
            ),
            S::mul_ps(z3, z3),
        ),
        S::mul_ps(w3, w3),
    );
    let t4 = S::sub_ps(
        S::sub_ps(
            S::sub_ps(
                S::sub_ps(S::set1_ps(0.5), S::mul_ps(x4, x4)),
                S::mul_ps(y4, y4),
            ),
            S::mul_ps(z4, z4),
        ),
        S::mul_ps(w4, w4),
    );
    //ti*ti*ti*ti
    let mut t0q = S::mul_ps(t0, t0);
    t0q = S::mul_ps(t0q, t0q);
    let mut t1q = S::mul_ps(t1, t1);
    t1q = S::mul_ps(t1q, t1q);
    let mut t2q = S::mul_ps(t2, t2);
    t2q = S::mul_ps(t2q, t2q);
    let mut t3q = S::mul_ps(t3, t3);
    t3q = S::mul_ps(t3q, t3q);
    let mut t4q = S::mul_ps(t4, t4);
    t4q = S::mul_ps(t4q, t4q);

    let mut n0 = S::mul_ps(t0q, grad4::<S>(gi0, x0, y0, z0, w0));
    let mut n1 = S::mul_ps(t1q, grad4::<S>(gi1, x1, y1, z1, w1));
    let mut n2 = S::mul_ps(t2q, grad4::<S>(gi2, x2, y2, z2, w2));
    let mut n3 = S::mul_ps(t3q, grad4::<S>(gi3, x3, y3, z3, w3));
    let mut n4 = S::mul_ps(t4q, grad4::<S>(gi4, x4, y4, z4, w4));

    //if ti < 0 then 0 else ni
    let mut cond = S::cmplt_ps(t0, S::setzero_ps());
    n0 = S::andnot_ps(cond, n0);
    cond = S::cmplt_ps(t1, S::setzero_ps());
    n1 = S::andnot_ps(cond, n1);
    cond = S::cmplt_ps(t2, S::setzero_ps());
    n2 = S::andnot_ps(cond, n2);
    cond = S::cmplt_ps(t3, S::setzero_ps());
    n3 = S::andnot_ps(cond, n3);
    cond = S::cmplt_ps(t4, S::setzero_ps());
    n4 = S::andnot_ps(cond, n4);

    S::add_ps(n0, S::add_ps(n1, S::add_ps(n2, S::add_ps(n3, n4))))
}
#[inline(always)]
pub unsafe fn fbm_4d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut zf = S::mul_ps(z, freq);
    let mut wf = S::mul_ps(w, freq);
    let mut result = simplex_4d::<S>(xf, yf, zf, wf);
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        zf = S::mul_ps(zf, lac);
        wf = S::mul_ps(wf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(result, S::mul_ps(simplex_4d::<S>(xf, yf, zf, wf), amp));
    }

    result
}

#[inline(always)]
pub unsafe fn ridge_4d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut zf = S::mul_ps(z, freq);
    let mut wf = S::mul_ps(w, freq);
    let mut result = S::sub_ps(S::set1_ps(1.0), S::abs_ps(simplex_4d::<S>(xf, yf, zf, wf)));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        zf = S::mul_ps(zf, lac);
        wf = S::mul_ps(wf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::sub_ps(
                S::set1_ps(1.0),
                S::abs_ps(S::mul_ps(simplex_4d::<S>(xf, yf, zf, wf), amp)),
            ),
        );
    }

    result
}

#[inline(always)]
pub unsafe fn turbulence_4d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    freq: S::Vf32,
    lac: S::Vf32,
    gain: S::Vf32,
    octaves: u8,
) -> S::Vf32 {
    let mut xf = S::mul_ps(x, freq);
    let mut yf = S::mul_ps(y, freq);
    let mut zf = S::mul_ps(z, freq);
    let mut wf = S::mul_ps(w, freq);
    let mut result = S::abs_ps(simplex_4d::<S>(xf, yf, zf, wf));
    let mut amp = S::set1_ps(1.0);

    for _ in 1..octaves {
        xf = S::mul_ps(xf, lac);
        yf = S::mul_ps(yf, lac);
        zf = S::mul_ps(zf, lac);
        wf = S::mul_ps(wf, lac);
        amp = S::mul_ps(amp, gain);
        result = S::add_ps(
            result,
            S::abs_ps(S::mul_ps(simplex_4d::<S>(xf, yf, zf, wf), amp)),
        );
    }

    result
}


