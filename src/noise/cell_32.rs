use super::cellular::{
    hash_2d, hash_3d, BIT_10_MASK_32, HASH_2_FLOAT_32, X_PRIME_32, Y_PRIME_32, Z_PRIME_32,
};
use crate::{CellDistanceFunction, CellReturnType};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn cellular_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut distance = S::Vf32::set1(999999.0);
    let mut xc = x.cast_i32() - 1;
    let mut yc_base = y.cast_i32() - 1;

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;

    xc = xc * X_PRIME_32;
    yc_base = yc_base * Y_PRIME_32;

    match return_type {
        CellReturnType::Distance => {
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d(seed, xc, yc);
                            let mut xd = (hash & BIT_10_MASK_32).cast_f32() - 511.5;
                            let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - 511.5;

                            let mut xd2 = xd * xd;
                            let inv_mag = jitter * (xd2 + yd * yd).rsqrt();
                            xd = xd * inv_mag + xcf;
                            yd = yd * inv_mag + ycf;
                            xd2 = xd * xd;
                            let new_distance = xd2 + yd * yd;
                            distance = new_distance.min(distance);

                            ycf = ycf + 1.0;
                            yc = yc + Y_PRIME_32;
                        }
                        xcf = xcf + 1.0;
                        xc = xc + X_PRIME_32;
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d(seed, xc, yc);
                            let mut xd = (hash & BIT_10_MASK_32).cast_f32() - 511.5;
                            let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - 511.5;

                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = xd * inv_mag + xcf;
                            yd = yd * inv_mag + ycf;

                            let new_distance = xd.abs() + yd.abs();
                            distance = new_distance.min(distance);

                            ycf = ycf + 1.0;
                            yc = yc + Y_PRIME_32;
                        }
                        xcf = xcf + 1.0;
                        xc = xc + X_PRIME_32;
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d(seed, xc, yc);
                            let mut xd = (hash & BIT_10_MASK_32).cast_f32() - 511.5;
                            let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - 511.5;

                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = xd * inv_mag + xcf;
                            yd = yd * inv_mag + ycf;

                            let new_distance = {
                                let euc = (xd * xd) + (yd * yd);
                                let man = xd.abs() + yd.abs();
                                euc + man
                            };
                            distance = new_distance.min(distance);

                            ycf = ycf + 1.0;
                            yc = yc + Y_PRIME_32;
                        }
                        xcf = xcf + 1.0;
                        xc = xc + X_PRIME_32;
                    }
                }
            }
            distance
        }
        CellReturnType::CellValue => {
            let mut cell_value = S::Vf32::zeroes();
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d(seed, xc, yc);
                            let mut xd = (hash & BIT_10_MASK_32).cast_f32() - 511.5;
                            let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - 511.5;

                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = xd * inv_mag + xcf;
                            yd = yd * inv_mag + ycf;

                            let new_cell_value = hash.cast_f32() * HASH_2_FLOAT_32;
                            let new_distance = (xd * xd) + (yd * yd);
                            let closer = new_distance.cmp_lt(distance);
                            distance = new_distance.min(distance);
                            cell_value = closer.blendv(cell_value, new_cell_value);

                            ycf = ycf + 1.0;
                            yc = yc + Y_PRIME_32;
                        }
                        xcf = xcf + 1.0;
                        xc = xc + X_PRIME_32;
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d(seed, xc, yc);
                            let mut xd = (hash & BIT_10_MASK_32).cast_f32() - 511.5;
                            let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - 511.5;

                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = xd * inv_mag + xcf;
                            yd = yd * inv_mag + ycf;

                            let new_cell_value = hash.cast_f32() * HASH_2_FLOAT_32;
                            let new_distance = xd.abs() + yd.abs();
                            let closer = new_distance.cmp_lt(distance);
                            distance = new_distance.min(distance);
                            cell_value = closer.blendv(cell_value, new_cell_value);

                            ycf = ycf + 1.0;
                            yc = yc + Y_PRIME_32;
                        }
                        xcf = xcf + 1.0;
                        xc = xc + X_PRIME_32;
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d(seed, xc, yc);
                            let mut xd = (hash & BIT_10_MASK_32).cast_f32() - 511.5;
                            let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - 511.5;

                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = xd * inv_mag + xcf;
                            yd = yd * inv_mag + ycf;

                            let new_cell_value = hash.cast_f32() * HASH_2_FLOAT_32;
                            let euc = xd * xd + yd * yd;
                            let man = xd.abs() + yd.abs();
                            let new_distance = euc + man;

                            let closer = new_distance.cmp_lt(distance);
                            distance = new_distance.min(distance);
                            cell_value = closer.blendv(cell_value, new_cell_value);

                            ycf = ycf + 1.0;
                            yc = yc + Y_PRIME_32;
                        }
                        xcf = xcf + 1.0;
                        xc = xc + X_PRIME_32;
                    }
                }
            }
            cell_value
        }
    }
}

#[inline(always)]
pub unsafe fn cellular_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut distance = S::Vf32::set1(999999.0);
    let mut cell_value = S::Vf32::zeroes();

    let mut xc = (x.cast_i32() - S::Vi32::set1(1)) * S::Vi32::set1(X_PRIME_32);
    let yc_base = (y.cast_i32() - S::Vi32::set1(1)) * S::Vi32::set1(Y_PRIME_32);
    let zc_base = (z.cast_i32() - S::Vi32::set1(1)) * S::Vi32::set1(Z_PRIME_32);

    let mut xcf = xc.cast_f32() - x;
    let ycf_base = yc_base.cast_f32() - y;
    let zcf_base = zc_base.cast_f32() - z;

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d(seed, xc, yc, zc);
                let mut xd = (hash & BIT_10_MASK_32).cast_f32() - S::Vf32::set1(511.5);
                let mut yd = ((hash >> 10) & BIT_10_MASK_32).cast_f32() - S::Vf32::set1(511.5);
                let mut zd = ((hash >> 20) & BIT_10_MASK_32).cast_f32() - S::Vf32::set1(511.5);
                let inv_mag = jitter * ((xd * xd) + (yd * yd) + (zd * zd)).rsqrt();
                xd = (xd * inv_mag) + xcf;
                yd = (yd * inv_mag) + ycf;
                zd = (zd * inv_mag) + zcf;

                let new_cell_value = S::Vf32::set1(HASH_2_FLOAT_32) * hash.cast_f32();
                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => (xd * xd) + (yd * yd) + (zd * zd),
                    CellDistanceFunction::Manhattan => xd.abs() + yd.abs() + zd.abs(),
                    CellDistanceFunction::Natural => {
                        let euc = (xd * xd) + (yd * yd) + (zd * zd);
                        let man = xd.abs() + yd.abs() + zd.abs();
                        euc + man
                    }
                };

                let closer = new_distance.cmp_lt(distance);
                distance = new_distance.min(distance);
                cell_value = closer.blendv(cell_value, new_cell_value);
                zcf = zcf + S::Vf32::set1(1.0);
                zc = zc + S::Vi32::set1(Z_PRIME_32);
            }
            ycf = ycf + 1.0;
            yc = yc + Y_PRIME_32;
        }
        xcf = xcf + 1.0;
        xc = xc + X_PRIME_32;
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
    }
}
