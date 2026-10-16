use super::cellular_32::{
    hash_1d, hash_2d, hash_3d, hash_4d, rehash, BIT_10_MASK_32, HASH_2_FLOAT_32, W_PRIME_32,
    X_PRIME_32, Y_PRIME_32, Z_PRIME_32,
};
use crate::noise::cell_strategy::{CellDistance, CellReturn};

use simdeez::prelude::*;

// `#[inline]` rather than `#[inline(always)]`: the call boundary now dispatches
// over the (distance, return) strategy types, so force-inlining would splice
// every instantiation into one frame and overflow small stacks in debug
// (same concern as issue #38). `#[inline]` still inlines in optimized builds.
#[inline]
pub fn cellular_2d<S: Simd, D: CellDistance, R: CellReturn<S::Vf32>>(
    x: S::Vf32,
    y: S::Vf32,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut acc = R::init();
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
            R::accumulate(&mut acc, new_distance, || {
                S::Vf32::set1(HASH_2_FLOAT_32) * hash.cast_f32()
            });

            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::finish(acc)
}

// Not `#[inline(always)]`: force-inlining this large function into the noise
// generation loop builds one huge stack frame that overflows small stacks under
// AVX2 (issue #38). `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
pub fn cellular_3d<S: Simd, D: CellDistance, R: CellReturn<S::Vf32>>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut acc = R::init();

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
                R::accumulate(&mut acc, new_distance, || {
                    S::Vf32::set1(HASH_2_FLOAT_32) * hash.cast_f32()
                });

                zcf += S::Vf32::set1(1.0);
                zc += S::Vi32::set1(Z_PRIME_32);
            }
            ycf += S::Vf32::set1(1.0);
            yc += S::Vi32::set1(Y_PRIME_32);
        }
        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::finish(acc)
}

// 1D cellular: a 3-cell neighbourhood along x. With a single axis the jittered
// feature point collapses to `±jitter` from each cell centre, but the
// cell/distance selection is still meaningful.
#[inline]
pub fn cellular_1d<S: Simd, D: CellDistance, R: CellReturn<S::Vf32>>(
    x: S::Vf32,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut acc = R::init();
    let mut xc = x.cast_i32() - S::Vi32::set1(1);
    let mut xcf = xc.cast_f32() - x;
    xc *= S::Vi32::set1(X_PRIME_32);

    for _x in 0..3 {
        let hash = hash_1d::<S>(seed, xc);
        let mut xd = (hash & S::Vi32::set1(BIT_10_MASK_32)).cast_f32() - S::Vf32::set1(511.5);
        let inv_mag = jitter * (xd * xd).rsqrt();
        xd = (xd * inv_mag) + xcf;

        let new_distance = D::new_distance_1d(xd);
        R::accumulate(&mut acc, new_distance, || {
            S::Vf32::set1(HASH_2_FLOAT_32) * hash.cast_f32()
        });

        xcf += S::Vf32::set1(1.0);
        xc += S::Vi32::set1(X_PRIME_32);
    }

    R::finish(acc)
}

// 4D cellular: a 3^4 = 81-cell neighbourhood. The 4th jitter field comes from a
// second hash round (`rehash`) because a 32-bit hash only holds three 10-bit
// fields; see `cellular_32::rehash`.
#[inline]
pub fn cellular_4d<S: Simd, D: CellDistance, R: CellReturn<S::Vf32>>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut acc = R::init();

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
                    R::accumulate(&mut acc, new_distance, || {
                        S::Vf32::set1(HASH_2_FLOAT_32) * hash.cast_f32()
                    });

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

    R::finish(acc)
}
