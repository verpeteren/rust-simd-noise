use super::cellular_32::{
    BIT_10_MASK_64, HASH_2_FLOAT_64, U_PRIME_64, V_PRIME_64, W_PRIME_64, X_PRIME_64, Y_PRIME_64,
    Z_PRIME_64,
};
use super::cellular_64::{hash_1d, hash_2d, hash_3d, hash_4d, hash_5d, hash_6d, rehash};
use crate::noise::cell_strategy::{CellDistance, CellReturn};

use simdeez::prelude::*;

// `#[inline]` rather than `#[inline(always)]`: the call boundary now dispatches
// over the (distance, return) strategy types, so force-inlining would splice
// every instantiation into one frame and overflow small stacks in debug
// (same concern as issue #38). `#[inline]` still inlines in optimized builds.
#[inline]
pub fn cellular_2d<S: Simd, D: CellDistance, R: CellReturn<S::Vf64>>(
    x: S::Vf64,
    y: S::Vf64,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut acc = R::init();
    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;

    xc *= S::Vi64::set1(X_PRIME_64);
    yc_base *= S::Vi64::set1(Y_PRIME_64);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let hash = hash_2d::<S>(seed, xc, yc);
            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64() - S::Vf64::set1(511.5);
            let mut yd =
                ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64() - S::Vf64::set1(511.5);
            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
            xd = (xd * inv_mag) + xcf;
            yd = (yd * inv_mag) + ycf;

            let new_distance = D::new_distance_2d(xd, yd);
            R::accumulate(&mut acc, new_distance, || {
                S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64()
            });

            ycf += S::Vf64::set1(1.0);
            yc += S::Vi64::set1(Y_PRIME_64);
        }
        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::finish(acc)
}

// Not `#[inline(always)]`: force-inlining this large function into the noise
// generation loop builds one huge stack frame that overflows small stacks under
// AVX2 (issue #38). `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
pub fn cellular_3d<S: Simd, D: CellDistance, R: CellReturn<S::Vf64>>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut acc = R::init();

    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);
    let mut zc_base = z.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;
    let zcf_base = zc_base.cast_f64() - z;

    xc *= S::Vi64::set1(X_PRIME_64);
    yc_base *= S::Vi64::set1(Y_PRIME_64);
    zc_base *= S::Vi64::set1(Z_PRIME_64);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d::<S>(seed, xc, yc, zc);
                let mut xd =
                    (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64() - S::Vf64::set1(511.5);
                let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                    - S::Vf64::set1(511.5);
                let mut zd = ((hash >> 20) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                    - S::Vf64::set1(511.5);
                let inv_mag = jitter * ((xd * xd) + ((yd * yd) + (zd * zd))).rsqrt();
                xd = (xd * inv_mag) + xcf;
                yd = (yd * inv_mag) + ycf;
                zd = (zd * inv_mag) + zcf;

                let new_distance = D::new_distance_3d(xd, yd, zd);
                R::accumulate(&mut acc, new_distance, || {
                    S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64()
                });

                zcf += S::Vf64::set1(1.0);
                zc += S::Vi64::set1(Z_PRIME_64);
            }
            ycf += S::Vf64::set1(1.0);
            yc += S::Vi64::set1(Y_PRIME_64);
        }
        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::finish(acc)
}

// 1D cellular: a 3-cell neighbourhood along x. With a single axis the jittered
// feature point collapses to `±jitter` from each cell centre, but the
// cell/distance selection is still meaningful.
#[inline]
pub fn cellular_1d<S: Simd, D: CellDistance, R: CellReturn<S::Vf64>>(
    x: S::Vf64,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut acc = R::init();
    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut xcf = xc.cast_f64() - x;
    xc *= S::Vi64::set1(X_PRIME_64);

    for _x in 0..3 {
        let hash = hash_1d::<S>(seed, xc);
        let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64() - S::Vf64::set1(511.5);
        let inv_mag = jitter * (xd * xd).rsqrt();
        xd = (xd * inv_mag) + xcf;

        let new_distance = D::new_distance_1d(xd);
        R::accumulate(&mut acc, new_distance, || {
            S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64()
        });

        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::finish(acc)
}

// 4D cellular: a 3^4 = 81-cell neighbourhood. The 4th jitter field comes from a
// second hash round (`rehash`), kept identical to the 32-bit kernel; see
// `cellular_64::rehash`.
#[inline]
pub fn cellular_4d<S: Simd, D: CellDistance, R: CellReturn<S::Vf64>>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut acc = R::init();

    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);
    let mut zc_base = z.cast_i64() - S::Vi64::set1(1);
    let mut wc_base = w.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;
    let zcf_base = zc_base.cast_f64() - z;
    let wcf_base = wc_base.cast_f64() - w;

    xc *= S::Vi64::set1(X_PRIME_64);
    yc_base *= S::Vi64::set1(Y_PRIME_64);
    zc_base *= S::Vi64::set1(Z_PRIME_64);
    wc_base *= S::Vi64::set1(W_PRIME_64);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let mut wcf = wcf_base;
                let mut wc = wc_base;
                for _w in 0..3 {
                    let hash = hash_4d::<S>(seed, xc, yc, zc, wc);
                    let mut xd =
                        (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64() - S::Vf64::set1(511.5);
                    let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                        - S::Vf64::set1(511.5);
                    let mut zd = ((hash >> 20) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                        - S::Vf64::set1(511.5);
                    let mut wd = (rehash::<S>(hash) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                        - S::Vf64::set1(511.5);
                    let inv_mag =
                        jitter * ((xd * xd) + ((yd * yd) + ((zd * zd) + (wd * wd)))).rsqrt();
                    xd = (xd * inv_mag) + xcf;
                    yd = (yd * inv_mag) + ycf;
                    zd = (zd * inv_mag) + zcf;
                    wd = (wd * inv_mag) + wcf;

                    let new_distance = D::new_distance_4d(xd, yd, zd, wd);
                    R::accumulate(&mut acc, new_distance, || {
                        S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64()
                    });

                    wcf += S::Vf64::set1(1.0);
                    wc += S::Vi64::set1(W_PRIME_64);
                }
                zcf += S::Vf64::set1(1.0);
                zc += S::Vi64::set1(Z_PRIME_64);
            }
            ycf += S::Vf64::set1(1.0);
            yc += S::Vi64::set1(Y_PRIME_64);
        }
        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::finish(acc)
}

// 5D cellular (f64): mirror of the f32 `cell_32::cellular_5d`. 3^5 = 243 cells;
// three jitter fields from the primary hash and two from a `rehash` round.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular_5d<S: Simd, D: CellDistance, R: CellReturn<S::Vf64>>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    u: S::Vf64,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut acc = R::init();

    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);
    let mut zc_base = z.cast_i64() - S::Vi64::set1(1);
    let mut wc_base = w.cast_i64() - S::Vi64::set1(1);
    let mut uc_base = u.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;
    let zcf_base = zc_base.cast_f64() - z;
    let wcf_base = wc_base.cast_f64() - w;
    let ucf_base = uc_base.cast_f64() - u;

    xc *= S::Vi64::set1(X_PRIME_64);
    yc_base *= S::Vi64::set1(Y_PRIME_64);
    zc_base *= S::Vi64::set1(Z_PRIME_64);
    wc_base *= S::Vi64::set1(W_PRIME_64);
    uc_base *= S::Vi64::set1(U_PRIME_64);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let mut wcf = wcf_base;
                let mut wc = wc_base;
                for _w in 0..3 {
                    let mut ucf = ucf_base;
                    let mut uc = uc_base;
                    for _u in 0..3 {
                        let hash = hash_5d::<S>(seed, xc, yc, zc, wc, uc);
                        let rh = rehash::<S>(hash);
                        let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                            - S::Vf64::set1(511.5);
                        let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                            - S::Vf64::set1(511.5);
                        let mut zd = ((hash >> 20) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                            - S::Vf64::set1(511.5);
                        let mut wd =
                            (rh & S::Vi64::set1(BIT_10_MASK_64)).cast_f64() - S::Vf64::set1(511.5);
                        let mut ud = ((rh >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                            - S::Vf64::set1(511.5);
                        let inv_mag = jitter
                            * ((xd * xd) + ((yd * yd) + ((zd * zd) + ((wd * wd) + (ud * ud)))))
                                .rsqrt();
                        xd = (xd * inv_mag) + xcf;
                        yd = (yd * inv_mag) + ycf;
                        zd = (zd * inv_mag) + zcf;
                        wd = (wd * inv_mag) + wcf;
                        ud = (ud * inv_mag) + ucf;

                        let new_distance = D::new_distance_5d(xd, yd, zd, wd, ud);
                        R::accumulate(&mut acc, new_distance, || {
                            S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64()
                        });

                        ucf += S::Vf64::set1(1.0);
                        uc += S::Vi64::set1(U_PRIME_64);
                    }
                    wcf += S::Vf64::set1(1.0);
                    wc += S::Vi64::set1(W_PRIME_64);
                }
                zcf += S::Vf64::set1(1.0);
                zc += S::Vi64::set1(Z_PRIME_64);
            }
            ycf += S::Vf64::set1(1.0);
            yc += S::Vi64::set1(Y_PRIME_64);
        }
        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::finish(acc)
}

// 6D cellular (f64): mirror of the f32 `cell_32::cellular_6d`. 3^6 = 729 cells;
// three jitter fields from the primary hash and three from a `rehash` round.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular_6d<S: Simd, D: CellDistance, R: CellReturn<S::Vf64>>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    w: S::Vf64,
    u: S::Vf64,
    v: S::Vf64,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut acc = R::init();

    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);
    let mut zc_base = z.cast_i64() - S::Vi64::set1(1);
    let mut wc_base = w.cast_i64() - S::Vi64::set1(1);
    let mut uc_base = u.cast_i64() - S::Vi64::set1(1);
    let mut vc_base = v.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;
    let zcf_base = zc_base.cast_f64() - z;
    let wcf_base = wc_base.cast_f64() - w;
    let ucf_base = uc_base.cast_f64() - u;
    let vcf_base = vc_base.cast_f64() - v;

    xc *= S::Vi64::set1(X_PRIME_64);
    yc_base *= S::Vi64::set1(Y_PRIME_64);
    zc_base *= S::Vi64::set1(Z_PRIME_64);
    wc_base *= S::Vi64::set1(W_PRIME_64);
    uc_base *= S::Vi64::set1(U_PRIME_64);
    vc_base *= S::Vi64::set1(V_PRIME_64);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let mut wcf = wcf_base;
                let mut wc = wc_base;
                for _w in 0..3 {
                    let mut ucf = ucf_base;
                    let mut uc = uc_base;
                    for _u in 0..3 {
                        let mut vcf = vcf_base;
                        let mut vc = vc_base;
                        for _v in 0..3 {
                            let hash = hash_6d::<S>(seed, xc, yc, zc, wc, uc, vc);
                            let rh = rehash::<S>(hash);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut zd = ((hash >> 20) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut wd = (rh & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut ud = ((rh >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut vd = ((rh >> 20) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let inv_mag = jitter
                                * ((xd * xd)
                                    + ((yd * yd)
                                        + ((zd * zd) + ((wd * wd) + ((ud * ud) + (vd * vd))))))
                                    .rsqrt();
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;
                            zd = (zd * inv_mag) + zcf;
                            wd = (wd * inv_mag) + wcf;
                            ud = (ud * inv_mag) + ucf;
                            vd = (vd * inv_mag) + vcf;

                            let new_distance = D::new_distance_6d(xd, yd, zd, wd, ud, vd);
                            R::accumulate(&mut acc, new_distance, || {
                                S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64()
                            });

                            vcf += S::Vf64::set1(1.0);
                            vc += S::Vi64::set1(V_PRIME_64);
                        }
                        ucf += S::Vf64::set1(1.0);
                        uc += S::Vi64::set1(U_PRIME_64);
                    }
                    wcf += S::Vf64::set1(1.0);
                    wc += S::Vi64::set1(W_PRIME_64);
                }
                zcf += S::Vf64::set1(1.0);
                zc += S::Vi64::set1(Z_PRIME_64);
            }
            ycf += S::Vf64::set1(1.0);
            yc += S::Vi64::set1(Y_PRIME_64);
        }
        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::finish(acc)
}
