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
    let mut xc = S::sub_epi64(S::cvtpd_epi64(x), S::Vi64::set1(1));
    let mut yc_base = S::sub_epi64(S::cvtpd_epi64(y), S::Vi64::set1(1));

    let mut xcf = S::sub_pd(S::cvtepi64_pd(xc), x);
    let ycf_base = S::sub_pd(S::cvtepi64_pd(yc_base), y);

    xc = S::mullo_epi64(xc, S::Vi64::set1(X_PRIME_64));
    yc_base = S::mullo_epi64(yc_base, S::Vi64::set1(Y_PRIME_64));
    match return_type {
        CellReturnType::Distance => {
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                                S::Vf64::set1(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::Vi64::set1(BIT_10_MASK_64),
                                )),
                                S::Vf64::set1(511.5),
                            );
                            let mut xd2 = S::mul_pd(xd, xd);
                            let inv_mag =
                                S::mul_pd(jitter, S::add_pd(xd2, S::mul_pd(yd, yd)).rsqrt());
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);
                            xd2 = S::mul_pd(xd, xd);
                            let new_distance = S::add_pd(xd2, S::mul_pd(yd, yd));
                            distance = new_distance.min(distance);

                            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
                            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
                        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                                S::Vf64::set1(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::Vi64::set1(BIT_10_MASK_64),
                                )),
                                S::Vf64::set1(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd)).rsqrt(),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_distance = S::add_pd(xd.abs(), yd.abs());
                            distance = new_distance.min(distance);

                            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
                            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
                        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                                S::Vf64::set1(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::Vi64::set1(BIT_10_MASK_64),
                                )),
                                S::Vf64::set1(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd)).rsqrt(),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_distance = {
                                let euc = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                                let man = S::add_pd(xd.abs(), yd.abs());
                                S::add_pd(euc, man)
                            };
                            distance = new_distance.min(distance);

                            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
                            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
                        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
                    }
                }
            }
            distance
        }
        CellReturnType::CellValue => {
            let mut cell_value = S::setzero_pd();
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                                S::Vf64::set1(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::Vi64::set1(BIT_10_MASK_64),
                                )),
                                S::Vf64::set1(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd)).rsqrt(),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_pd(S::Vf64::set1(HASH_2_FLOAT_64), S::cvtepi64_pd(hash));
                            let new_distance = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                            let closer = S::cmplt_pd(new_distance, distance);
                            distance = new_distance.min(distance);
                            cell_value = S::blendv_pd(cell_value, new_cell_value, closer);

                            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
                            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
                        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                                S::Vf64::set1(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::Vi64::set1(BIT_10_MASK_64),
                                )),
                                S::Vf64::set1(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd)).rsqrt(),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_pd(S::Vf64::set1(HASH_2_FLOAT_64), S::cvtepi64_pd(hash));
                            let new_distance = S::add_pd(xd.abs(), yd.abs());
                            let closer = S::cmplt_pd(new_distance, distance);
                            distance = new_distance.min(distance);
                            cell_value = S::blendv_pd(cell_value, new_cell_value, closer);

                            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
                            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
                        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                                S::Vf64::set1(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::Vi64::set1(BIT_10_MASK_64),
                                )),
                                S::Vf64::set1(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd)).rsqrt(),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_pd(S::Vf64::set1(HASH_2_FLOAT_64), S::cvtepi64_pd(hash));
                            let new_distance = {
                                let euc = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                                let man = S::add_pd(xd.abs(), yd.abs());
                                S::add_pd(euc, man)
                            };
                            let closer = S::cmplt_pd(new_distance, distance);
                            distance = new_distance.min(distance);
                            cell_value = S::blendv_pd(cell_value, new_cell_value, closer);

                            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
                            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
                        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
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
    let mut cell_value = S::setzero_pd();

    let mut xc = S::sub_epi64(S::cvtpd_epi64(x), S::Vi64::set1(1));
    let mut yc_base = S::sub_epi64(S::cvtpd_epi64(y), S::Vi64::set1(1));
    let mut zc_base = S::sub_epi64(S::cvtpd_epi64(z), S::Vi64::set1(1));

    let mut xcf = S::sub_pd(S::cvtepi64_pd(xc), x);
    let ycf_base = S::sub_pd(S::cvtepi64_pd(yc_base), y);
    let zcf_base = S::sub_pd(S::cvtepi64_pd(zc_base), z);

    xc = S::mullo_epi64(xc, S::Vi64::set1(X_PRIME_64));
    yc_base = S::mullo_epi64(yc_base, S::Vi64::set1(Y_PRIME_64));
    zc_base = S::mullo_epi64(zc_base, S::Vi64::set1(Z_PRIME_64));

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d::<S>(seed, xc, yc, zc);
                let mut xd = S::sub_pd(
                    S::cvtepi64_pd(S::and_epi64(hash, S::Vi64::set1(BIT_10_MASK_64))),
                    S::Vf64::set1(511.5),
                );
                let mut yd = S::sub_pd(
                    S::cvtepi64_pd(S::and_epi64(
                        S::srai_epi64(hash, 10),
                        S::Vi64::set1(BIT_10_MASK_64),
                    )),
                    S::Vf64::set1(511.5),
                );
                let mut zd = S::sub_pd(
                    S::cvtepi64_pd(S::and_epi64(
                        S::srai_epi64(hash, 20),
                        S::Vi64::set1(BIT_10_MASK_64),
                    )),
                    S::Vf64::set1(511.5),
                );
                let inv_mag = S::mul_pd(
                    jitter,
                    S::add_pd(
                        S::mul_pd(xd, xd),
                        S::add_pd(S::mul_pd(yd, yd), S::mul_pd(zd, zd)),
                    )
                    .rsqrt(),
                );
                xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);
                zd = S::add_pd(S::mul_pd(zd, inv_mag), zcf);

                let new_cell_value =
                    S::mul_pd(S::Vf64::set1(HASH_2_FLOAT_64), S::cvtepi64_pd(hash));
                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => S::add_pd(
                        S::mul_pd(xd, xd),
                        S::add_pd(S::mul_pd(yd, yd), S::mul_pd(zd, zd)),
                    ),
                    CellDistanceFunction::Manhattan => {
                        S::add_pd(S::add_pd(xd.abs(), yd.abs()), zd.abs())
                    }
                    CellDistanceFunction::Natural => {
                        let euc = S::add_pd(
                            S::mul_pd(xd, xd),
                            S::add_pd(S::mul_pd(yd, yd), S::mul_pd(zd, zd)),
                        );
                        let man = S::add_pd(S::add_pd(xd.abs(), yd.abs()), zd.abs());
                        S::add_pd(euc, man)
                    }
                };
                let closer = S::cmplt_pd(new_distance, distance);
                distance = new_distance.min(distance);
                cell_value = S::blendv_pd(cell_value, new_cell_value, closer);
                zcf = S::add_pd(ycf, S::Vf64::set1(1.0));
                zc = S::add_epi64(yc, S::Vi64::set1(Z_PRIME_64));
            }
            ycf = S::add_pd(ycf, S::Vf64::set1(1.0));
            yc = S::add_epi64(yc, S::Vi64::set1(Y_PRIME_64));
        }
        xcf = S::add_pd(xcf, S::Vf64::set1(1.0));
        xc = S::add_epi64(xc, S::Vi64::set1(X_PRIME_64));
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
    }
}
