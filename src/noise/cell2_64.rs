use super::cellular_32::{BIT_10_MASK_64, X_PRIME_64, Y_PRIME_64, Z_PRIME_64};
use super::cellular_64::{hash_2d, hash_3d};
use crate::noise::cell_strategy::{Cell2Return, CellDistance};

use simdeez::prelude::*;

// `#[inline]` rather than `#[inline(always)]`: the call boundary now dispatches
// over the (distance, return) strategy types, so force-inlining would splice
// every instantiation into one frame and overflow small stacks in debug
// (same concern as issue #38). `#[inline]` still inlines in optimized builds.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_2d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf64,
    y: S::Vf64,
    jitter: S::Vf64,
    index0: usize,
    index1: usize,
    seed: i64,
) -> S::Vf64 {
    let mut distance: [S::Vf64; 4] = [S::Vf64::set1(999999.0); 4];

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
            let mut i = index1;
            while i > 0 {
                distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                distance[0] = distance[0].min(new_distance);
                i -= 1;
            }
            ycf += S::Vf64::set1(1.0);
            yc += S::Vi64::set1(Y_PRIME_64);
        }
        xcf += S::Vf64::set1(1.0);
        xc += S::Vi64::set1(X_PRIME_64);
    }

    R::combine(&distance, index0, index1)
}

// Not `#[inline(always)]`: force-inlining this large function into the noise
// generation loop builds one huge stack frame that overflows small stacks under
// AVX2 (issue #38). `#[inline]` still lets LLVM inline it in optimized builds.
#[inline]
#[allow(clippy::too_many_arguments)]
pub fn cellular2_3d<S: Simd, D: CellDistance, R: Cell2Return>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    jitter: S::Vf64,
    index0: usize,
    index1: usize,
    seed: i64,
) -> S::Vf64 {
    let mut distance: [S::Vf64; 4] = [S::Vf64::set1(999999.0); 4];

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
                let mut i = index1;
                while i > 0 {
                    distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                    distance[0] = distance[0].min(new_distance);
                    i -= 1;
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

    R::combine(&distance, index0, index1)
}
