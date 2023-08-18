use super::cellular_32::{BIT_10_MASK_64, HASH_2_FLOAT_64, X_PRIME_64, Y_PRIME_64, Z_PRIME_64};
use super::cellular_64::{hash_2d, hash_3d};
use crate::{CellDistanceFunction, CellReturnType};

use simdeez::prelude::*;

#[inline(always)]
pub unsafe fn cellular_2d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut distance = S::Vf64::set1(999999.0);
    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;

    xc = xc * S::Vi64::set1(X_PRIME_64);
    yc_base = yc_base * S::Vi64::set1(Y_PRIME_64);
    match return_type {
        CellReturnType::Distance => {
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut xd2 = xd * xd;
                            let inv_mag = (jitter * (xd2 + (yd * yd)).rsqrt());
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;
                            xd2 = (xd * xd);
                            let new_distance = xd2 + (yd * yd);
                            distance = new_distance.min(distance);

                            ycf = ycf + S::Vf64::set1(1.0);
                            yc = yc + S::Vi64::set1(Y_PRIME_64);
                        }
                        xcf = xcf + S::Vf64::set1(1.0);
                        xc = xc + S::Vi64::set1(X_PRIME_64);
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let inv_mag = (jitter * ((xd * xd) + (yd * yd)).rsqrt());
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;

                            let new_distance = xd.abs() + yd.abs();
                            distance = new_distance.min(distance);

                            ycf = ycf + S::Vf64::set1(1.0);
                            yc = yc + S::Vi64::set1(Y_PRIME_64);
                        }
                        xcf = xcf + S::Vf64::set1(1.0);
                        xc = xc + S::Vi64::set1(X_PRIME_64);
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let inv_mag = (jitter * ((xd * xd) + (yd * yd)).rsqrt());
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;

                            let new_distance = {
                                let euc = ((xd * xd) + (yd * yd));
                                let man = xd.abs() + yd.abs();
                                euc + man
                            };
                            distance = new_distance.min(distance);

                            ycf = ycf + S::Vf64::set1(1.0);
                            yc = yc + S::Vi64::set1(Y_PRIME_64);
                        }
                        xcf = xcf + S::Vf64::set1(1.0);
                        xc = xc + S::Vi64::set1(X_PRIME_64);
                    }
                }
            }
            distance
        }
        CellReturnType::CellValue => {
            let mut cell_value = S::Vf64::zeroes();
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let inv_mag = (jitter * ((xd * xd) + (yd * yd)).rsqrt());
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;

                            let new_cell_value = (S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64());
                            let new_distance = (xd * xd) + (yd * yd);
                            let closer = new_distance.cmp_lt(distance);
                            distance = new_distance.min(distance);
                            cell_value = closer.blendv(cell_value, new_cell_value);

                            ycf = ycf + S::Vf64::set1(1.0);
                            yc = yc + S::Vi64::set1(Y_PRIME_64);
                        }
                        xcf = xcf + S::Vf64::set1(1.0);
                        xc = xc + S::Vi64::set1(X_PRIME_64);
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;

                            let new_cell_value = S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64();
                            let new_distance = xd.abs() + yd.abs();
                            let closer = new_distance.cmp_lt(distance);
                            distance = new_distance.min(distance);
                            cell_value = closer.blendv(cell_value, new_cell_value);

                            ycf = ycf + S::Vf64::set1(1.0);
                            yc = yc + S::Vi64::set1(Y_PRIME_64);
                        }
                        xcf = xcf + S::Vf64::set1(1.0);
                        xc = xc + S::Vi64::set1(X_PRIME_64);
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = (hash & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let mut yd = ((hash >> 10) & S::Vi64::set1(BIT_10_MASK_64)).cast_f64()
                                - S::Vf64::set1(511.5);
                            let inv_mag = jitter * ((xd * xd) + (yd * yd)).rsqrt();
                            xd = (xd * inv_mag) + xcf;
                            yd = (yd * inv_mag) + ycf;

                            let new_cell_value = S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64();
                            let new_distance = {
                                let euc = (xd * xd) + (yd * yd);
                                let man = xd.abs() + yd.abs();
                                euc + man
                            };
                            let closer = new_distance.cmp_lt(distance);
                            distance = new_distance.min(distance);
                            cell_value = closer.blendv(cell_value, new_cell_value);

                            ycf = ycf + S::Vf64::set1(1.0);
                            yc = yc + S::Vi64::set1(Y_PRIME_64);
                        }
                        xcf = xcf + S::Vf64::set1(1.0);
                        xc = xc + S::Vi64::set1(X_PRIME_64);
                    }
                }
            }
            cell_value
        }
    }
}

#[inline(always)]
pub unsafe fn cellular_3d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    z: S::Vf64,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut distance = S::Vf64::set1(999999.0);
    let mut cell_value = S::Vf64::zeroes();

    let mut xc = x.cast_i64() - S::Vi64::set1(1);
    let mut yc_base = y.cast_i64() - S::Vi64::set1(1);
    let mut zc_base = z.cast_i64() - S::Vi64::set1(1);

    let mut xcf = xc.cast_f64() - x;
    let ycf_base = yc_base.cast_f64() - y;
    let zcf_base = zc_base.cast_f64() - z;

    xc = xc * S::Vi64::set1(X_PRIME_64);
    yc_base = yc_base * S::Vi64::set1(Y_PRIME_64);
    zc_base = zc_base * S::Vi64::set1(Z_PRIME_64);

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

                let new_cell_value = S::Vf64::set1(HASH_2_FLOAT_64) * hash.cast_f64();
                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => (xd * xd) + ((yd * yd) + (zd * zd)),
                    CellDistanceFunction::Manhattan => ((xd.abs() + yd.abs()) + zd.abs()),
                    CellDistanceFunction::Natural => {
                        let euc = (xd * xd) + ((yd * yd) + (zd * zd));
                        let man = ((xd.abs() + yd.abs()) + zd.abs());
                        euc + man
                    }
                };
                let closer = new_distance.cmp_lt(distance);
                distance = new_distance.min(distance);
                cell_value = closer.blendv(cell_value, new_cell_value);
                zcf = ycf + S::Vf64::set1(1.0);
                zc = yc + S::Vi64::set1(Z_PRIME_64);
            }
            ycf = ycf + S::Vf64::set1(1.0);
            yc = yc + S::Vi64::set1(Y_PRIME_64);
        }
        xcf = xcf + S::Vf64::set1(1.0);
        xc = xc + S::Vi64::set1(X_PRIME_64);
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
    }
}
