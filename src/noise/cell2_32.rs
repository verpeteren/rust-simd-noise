use super::cellular_32::{hash_2d, hash_3d, BIT_10_MASK_32, X_PRIME_32, Y_PRIME_32, Z_PRIME_32};
use crate::{Cell2ReturnType, CellDistanceFunction};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn cellular2_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: Cell2ReturnType,
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

    xc = xc * S::Vi32::set1(X_PRIME_32);
    yc_base = yc_base * S::Vi32::set1(Y_PRIME_32);

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
                distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                distance[0] = distance[0].min(new_distance);
                i -= 1;
            }
            ycf = ycf + S::Vf32::set1(1.0);
            yc = yc + S::Vi32::set1(Y_PRIME_32);
        }
        xcf = xcf + S::Vf32::set1(1.0);
        xc = xc + S::Vi32::set1(X_PRIME_32);
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
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: Cell2ReturnType,
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

    xc = xc * S::Vi32::set1(X_PRIME_32);
    yc_base = yc_base * S::Vi32::set1(Y_PRIME_32);
    zc_base = zc_base * S::Vi32::set1(Z_PRIME_32);

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

                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => (xd * xd) + ((yd * yd) + (zd * zd)),
                    CellDistanceFunction::Manhattan => (xd.abs() + yd.abs()) + zd.abs(),
                    CellDistanceFunction::Natural => {
                        let euc = (xd * xd) + ((yd * yd) + (zd * zd));
                        let man = (xd.abs() + yd.abs()) + zd.abs();
                        euc + man
                    }
                };
                let mut i = index1;
                while i > 0 {
                    distance[i] = distance[i].min(new_distance).max(distance[i - 1]);
                    distance[0] = distance[0].min(new_distance);
                    i -= 1;
                }
                zcf = ycf + S::Vf32::set1(1.0);
                zc = yc + S::Vi32::set1(Z_PRIME_32);
            }
            ycf = ycf + S::Vf32::set1(1.0);
            yc = yc + S::Vi32::set1(Y_PRIME_32);
        }
        xcf = xcf + S::Vf32::set1(1.0);
        xc = xc + S::Vi32::set1(X_PRIME_32);
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => distance[index0] + distance[index1],
        Cell2ReturnType::Distance2Sub => distance[index0] - distance[index1],
        Cell2ReturnType::Distance2Mul => distance[index0] * distance[index1],
        Cell2ReturnType::Distance2Div => distance[index0] / distance[index1],
    }
}
