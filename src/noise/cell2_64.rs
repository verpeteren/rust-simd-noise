use super::cellular::{hash_2d, hash_3d, BIT_10_MASK_64, X_PRIME_64, Y_PRIME_64, Z_PRIME_64};
use crate::{Cell2ReturnType, CellDistanceFunction};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn cellular2_2d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    distance_function: CellDistanceFunction,
    return_type: Cell2ReturnType,
    jitter: S::Vf64,
    index0: usize,
    index1: usize,
    seed: i64,
) -> S::Vf64 {
    let mut distance: [S::Vf64; 4] = [S::Vf64::set1(999999.0); 4];

    let mut xc = x.cast_i64() - 1;
    let mut yc_base = y.cast_i64() - 1;

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;

    xc = xc * X_PRIME_64;
    yc_base = yc_base * Y_PRIME_64;

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let hash = hash_2d(seed, xc, yc);
            let mut xd = (hash & BIT_10_MASK_64).cast_f64() - 511.5;
            let mut yd = ((hash >> 10) & BIT_10_MASK_64).cast_f64() - 511.5;
            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
            xd = xd * inv_mag + xcf;
            yd = yd * inv_mag + ycf;

            let new_distance = match distance_function {
                CellDistanceFunction::Euclidean => (xd * xd) + (yd * yd),
                CellDistanceFunction::Manhattan => xd.abs() + yd.abs(),
                CellDistanceFunction::Natural => {
                    let euc = (xd * xd) + (yd * yd);
                    let man = xd.abs() + yd.abs();
                    euc + man
                }
            };
            let mut i = index1;
            while i > 0 {
                distance[i] = new_distance.min(distance[i]).max(distance[i - 1]);
                distance[0] = distance[0].min(new_distance);
                i -= 1;
            }
            ycf = ycf + 1.0;
            yc = yc + Y_PRIME_64;
        }
        xcf = xcf + 1.0;
        xc = xc + X_PRIME_64;
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => distance[index0] + distance[index1],
        Cell2ReturnType::Distance2Sub => distance[index0] - distance[index1],
        Cell2ReturnType::Distance2Mul => distance[index0] * distance[index1],
        Cell2ReturnType::Distance2Div => distance[index0] / distance[index1],
    }
}

#[inline(always)]
pub unsafe fn cellular2_3d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    distance_function: CellDistanceFunction,
    return_type: Cell2ReturnType,
    jitter: S::Vf64,
    index0: usize,
    index1: usize,
    seed: i64,
) -> S::Vf64 {
    let mut distance: [S::Vf64; 4] = [S::Vf64::set1(999999.0); 4];

    let mut xc = x.cast_i64() - 1;
    let mut yc_base = y.cast_i64() - 1;
    let mut zc_base = z.cast_i64() - 1;

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;
    let zcf_base = zc_base.cast_f64() - z;

    xc = xc * X_PRIME_64;
    yc_base = yc_base * Y_PRIME_64;
    zc_base = zc_base * Z_PRIME_64;

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d(seed, xc, yc, zc);
                let mut xd = (hash & BIT_10_MASK_64).cast_f64() - 511.5;
                let mut yd = ((hash >> 10) & BIT_10_MASK_64).cast_f64() - 511.5;
                let mut zd = ((hash >> 20) & BIT_10_MASK_64).cast_f64() - 511.5;
                let inv_mag = jitter * (xd * xd + yd * yd + zd * zd).rsqrt();
                xd = xd * inv_mag + xcf;
                yd = yd * inv_mag + ycf;
                zd = zd * inv_mag + zcf;

                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => (xd * xd) + (yd * yd) + (zd * zd),
                    CellDistanceFunction::Manhattan => xd.abs() + yd.abs() + zd.abs(),
                    CellDistanceFunction::Natural => {
                        let euc = (xd * xd) + (yd * yd) + (zd * zd);
                        let man = xd.abs() + yd.abs() + zd.abs();
                        euc + man
                    }
                };
                let mut i = index1;
                while i > 0 {
                    distance[i] = new_distance.min(distance[i]).max(distance[i - 1]);
                    distance[0] = distance[0].min(new_distance);
                    i -= 1;
                }
                zcf = ycf + 1.0;
                zc = yc + Z_PRIME_64;
            }
            ycf = ycf + 1.0;
            yc = yc + Y_PRIME_64;
        }
        xcf = xcf + 1.0;
        xc = xc + X_PRIME_64;
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => distance[index0] + distance[index1],
        Cell2ReturnType::Distance2Sub => distance[index0] - distance[index1],
        Cell2ReturnType::Distance2Mul => distance[index0] * distance[index1],
        Cell2ReturnType::Distance2Div => distance[index0] / distance[index1],
    }
}
