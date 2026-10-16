use super::cellular_32::{
    hash_1d, hash_2d, hash_3d, hash_4d, hash_5d, hash_6d, rehash, BIT_10_MASK_32, U_PRIME_32,
    V_PRIME_32, W_PRIME_32, X_PRIME_32, Y_PRIME_32, Z_PRIME_32,
};
use crate::noise::cell_strategy::{Cell2Return, CellDistance};

use simdeez::prelude::*;

// `#[inline]` rather than `#[inline(always)]`: the call boundary now dispatches
// over the (distance, return) strategy types, so force-inlining would splice
// every instantiation into one frame and overflow small stacks in debug
// (same concern as issue #38). `#[inline]` still inlines in optimized builds.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_2d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf32,
    y: S::Vf32,
    jitter: S::Vf32,
    index0: usize,
    index1: usize,
    seed: i32,
) -> S::Vf32 {
    let mut distance: [S::Vf32; 4] = [S::Vf32::set1(999999.0); 4];

    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut yc_base = y.cast_i32() - S::Vi32::set1(1);

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;

    xc *= S::Vi32::set1(X_PRIME_32);
    yc_base *= S::Vi32::set1(Y_PRIME_32);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let hash = hash_2d::<S>(seed, xc, yc);
            let mut xd = (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
            let mut yd =
                ((hash >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
            xd = (xd * inv_mag) + xcf;
            yd = (yd * inv_mag) + ycf;

            let new_distance = D::new_distance_2d(xd, yd);
            let mut i = index1;
            while i > 0 {
                distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                distance[0] = distance[0].min(new_distance);
                i -= 1;
            }
            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::combine(&distance, index0, index1)
}

// Not `#[inline(always)]`: force-inlining this large function into the noise
// generation loop builds one huge stack frame that overflows small stacks under
// AVX2 (issue #38). `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_3d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    jitter: S::Vf32,
    index0: usize,
    index1: usize,
    seed: i32,
) -> S::Vf32 {
    let mut distance: [S::Vf32; 4] = [S::Vf32::set1(999999.0); 4];

    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut yc_base = y.cast_i32() - S::Vi32::set1(1);
    let mut zc_base = z.cast_i32() - S::Vi32::set1(1);

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;
    let zcf_base = zc_base.cast_f32() - z;

    xc *= S::Vi32::set1(X_PRIME_32);
    yc_base *= S::Vi32::set1(Y_PRIME_32);
    zc_base *= S::Vi32::set1(Z_PRIME_32);

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d::<S>(seed, xc, yc, zc);
                let mut xd =
                    (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
                let mut yd = ((hash >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                    - S::Vf32::set1(511.5);
                let mut zd = ((hash >> 20) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                    - S::Vf32::set1(511.5);
                let inv_mag = jitter * ((xd * xd) + ((yd * yd) + (zd * zd))).rsqrt();
                xd = (xd * inv_mag) + xcf;
                yd = (yd * inv_mag) + ycf;
                zd = (zd * inv_mag) + zcf;

                let new_distance = D::new_distance_3d(xd, yd, zd);
                let mut i = index1;
                while i > 0 {
                    distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                    distance[0] = distance[0].min(new_distance);
                    i -= 1;
                }
                zcf += S::Vf32::set1(1.0);
                zc += S::Vi32::set1(Z_PRIME_32);
            }
            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::combine(&distance, index0, index1)
}

// 1D cellular2: a 3-cell neighbourhood along x. Only 3 candidate cells exist, so
// callers must keep `index1 <= 2` (enforced by `Cellular2Settings::validate`).
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_1d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf32,
    jitter: S::Vf32,
    index0: usize,
    index1: usize,
    seed: i32,
) -> S::Vf32 {
    let mut distance: [S::Vf32; 4] = [S::Vf32::set1(999999.0); 4];

    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut xcf = xc.cast_f32() - x;
    xc *= S::Vi32::set1(X_PRIME_32);

    for _x in 0..3 {
        let hash = hash_1d::<S>(seed, xc);
        let mut xd = (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
        let inv_mag = jitter * (xd * xd).rsqrt();
        xd = (xd * inv_mag) + xcf;

        let new_distance = D::new_distance_1d(xd);
        let mut i = index1;
        while i > 0 {
            distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
            distance[0] = distance[0].min(new_distance);
            i -= 1;
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::combine(&distance, index0, index1)
}

// 4D cellular2: a 3^4 = 81-cell neighbourhood. The 4th jitter field is drawn
// from a second hash round (`rehash`); see `cellular_32::rehash`.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_4d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    jitter: S::Vf32,
    index0: usize,
    index1: usize,
    seed: i32,
) -> S::Vf32 {
    let mut distance: [S::Vf32; 4] = [S::Vf32::set1(999999.0); 4];

    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut yc_base = y.cast_i32() - S::Vi32::set1(1);
    let mut zc_base = z.cast_i32() - S::Vi32::set1(1);
    let mut wc_base = w.cast_i32() - S::Vi32::set1(1);

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;
    let zcf_base = zc_base.cast_f32() - z;
    let wcf_base = wc_base.cast_f32() - w;

    xc *= S::Vi32::set1(X_PRIME_32);
    yc_base *= S::Vi32::set1(Y_PRIME_32);
    zc_base *= S::Vi32::set1(Z_PRIME_32);
    wc_base *= S::Vi32::set1(W_PRIME_32);

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
                        (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
                    let mut yd = ((hash >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                        - S::Vf32::set1(511.5);
                    let mut zd = ((hash >> 20) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                        - S::Vf32::set1(511.5);
                    let mut wd = (rehash::<S>(hash) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                        - S::Vf32::set1(511.5);
                    let inv_mag =
                        jitter * ((xd * xd) + ((yd * yd) + ((zd * zd) + (wd * wd)))).rsqrt();
                    xd = (xd * inv_mag) + xcf;
                    yd = (yd * inv_mag) + ycf;
                    zd = (zd * inv_mag) + zcf;
                    wd = (wd * inv_mag) + wcf;

                    let new_distance = D::new_distance_4d(xd, yd, zd, wd);
                    let mut i = index1;
                    while i > 0 {
                        distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                        distance[0] = distance[0].min(new_distance);
                        i -= 1;
                    }
                    wcf += S::Vf32::set1(1.0);
                    wc += S::Vi32::set1(W_PRIME_32);
                }
                zcf += S::Vf32::set1(1.0);
                zc += S::Vi32::set1(Z_PRIME_32);
            }
            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::combine(&distance, index0, index1)
}

// 5D cellular2: a 3^5 = 243-cell neighbourhood. Jitter fields 4 and 5 come from
// a second hash round (`rehash`); see `cellular_32::rehash`.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_5d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    u: S::Vf32,
    jitter: S::Vf32,
    index0: usize,
    index1: usize,
    seed: i32,
) -> S::Vf32 {
    let mut distance: [S::Vf32; 4] = [S::Vf32::set1(999999.0); 4];

    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut yc_base = y.cast_i32() - S::Vi32::set1(1);
    let mut zc_base = z.cast_i32() - S::Vi32::set1(1);
    let mut wc_base = w.cast_i32() - S::Vi32::set1(1);
    let mut uc_base = u.cast_i32() - S::Vi32::set1(1);

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;
    let zcf_base = zc_base.cast_f32() - z;
    let wcf_base = wc_base.cast_f32() - w;
    let ucf_base = uc_base.cast_f32() - u;

    xc *= S::Vi32::set1(X_PRIME_32);
    yc_base *= S::Vi32::set1(Y_PRIME_32);
    zc_base *= S::Vi32::set1(Z_PRIME_32);
    wc_base *= S::Vi32::set1(W_PRIME_32);
    uc_base *= S::Vi32::set1(U_PRIME_32);

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
                        let mut xd = (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                            - S::Vf32::set1(511.5);
                        let mut yd = ((hash >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                            - S::Vf32::set1(511.5);
                        let mut zd = ((hash >> 20) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                            - S::Vf32::set1(511.5);
                        let mut wd =
                            (rh & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
                        let mut ud = ((rh >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                            - S::Vf32::set1(511.5);
                        let inv_mag = jitter
                            * ((xd * xd) + ((yd * yd) + ((zd * zd) + ((wd * wd) + (ud * ud)))))
                                .rsqrt();
                        xd = (xd * inv_mag) + xcf;
                        yd = (yd * inv_mag) + ycf;
                        zd = (zd * inv_mag) + zcf;
                        wd = (wd * inv_mag) + wcf;
                        ud = (ud * inv_mag) + ucf;

                        let new_distance = D::new_distance_5d(xd, yd, zd, wd, ud);
                        let mut i = index1;
                        while i > 0 {
                            distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                            distance[0] = distance[0].min(new_distance);
                            i -= 1;
                        }
                        ucf += S::Vf32::set1(1.0);
                        uc += S::Vi32::set1(U_PRIME_32);
                    }
                    wcf += S::Vf32::set1(1.0);
                    wc += S::Vi32::set1(W_PRIME_32);
                }
                zcf += S::Vf32::set1(1.0);
                zc += S::Vi32::set1(Z_PRIME_32);
            }
            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::combine(&distance, index0, index1)
}

// 6D cellular2: a 3^6 = 729-cell neighbourhood. Three jitter fields from the
// primary hash and three from a `rehash` round.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_6d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    u: S::Vf32,
    v: S::Vf32,
    jitter: S::Vf32,
    index0: usize,
    index1: usize,
    seed: i32,
) -> S::Vf32 {
    let mut distance: [S::Vf32; 4] = [S::Vf32::set1(999999.0); 4];

    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut yc_base = y.cast_i32() - S::Vi32::set1(1);
    let mut zc_base = z.cast_i32() - S::Vi32::set1(1);
    let mut wc_base = w.cast_i32() - S::Vi32::set1(1);
    let mut uc_base = u.cast_i32() - S::Vi32::set1(1);
    let mut vc_base = v.cast_i32() - S::Vi32::set1(1);

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;
    let zcf_base = zc_base.cast_f32() - z;
    let wcf_base = wc_base.cast_f32() - w;
    let ucf_base = uc_base.cast_f32() - u;
    let vcf_base = vc_base.cast_f32() - v;

    xc *= S::Vi32::set1(X_PRIME_32);
    yc_base *= S::Vi32::set1(Y_PRIME_32);
    zc_base *= S::Vi32::set1(Z_PRIME_32);
    wc_base *= S::Vi32::set1(W_PRIME_32);
    uc_base *= S::Vi32::set1(U_PRIME_32);
    vc_base *= S::Vi32::set1(V_PRIME_32);

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
                            let mut xd = (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                                - S::Vf32::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                                - S::Vf32::set1(511.5);
                            let mut zd = ((hash >> 20) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                                - S::Vf32::set1(511.5);
                            let mut wd = (rh & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                                - S::Vf32::set1(511.5);
                            let mut ud = ((rh >> 10) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                                - S::Vf32::set1(511.5);
                            let mut vd = ((rh >> 20) & S::Vi32::set1(BIT_10_MASK_32)).cast_f32()
                                - S::Vf32::set1(511.5);
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
                            let mut i = index1;
                            while i > 0 {
                                distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                                distance[0] = distance[0].min(new_distance);
                                i -= 1;
                            }
                            vcf += S::Vf32::set1(1.0);
                            vc += S::Vi32::set1(V_PRIME_32);
                        }
                        ucf += S::Vf32::set1(1.0);
                        uc += S::Vi32::set1(U_PRIME_32);
                    }
                    wcf += S::Vf32::set1(1.0);
                    wc += S::Vi32::set1(W_PRIME_32);
                }
                zcf += S::Vf32::set1(1.0);
                zc += S::Vi32::set1(Z_PRIME_32);
            }
            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::combine(&distance, index0, index1)
}
