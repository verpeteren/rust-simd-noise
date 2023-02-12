use super::cellular_64::{
    hash_2d, hash_3d, BIT_10_MASK, HASH_2_FLOAT, X_PRIME_64, Y_PRIME_64, Z_PRIME,
};
use crate::{CellDistanceFunction, CellReturnType};

use simdeez::Simd;

#[inline(always)]
pub unsafe fn cellular_2d<S: Simd>(
    x: S::Vf64,
    y: S::Vf64,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf64,
    seed: i64,
) -> S::Vf64 {
    let mut distance = S::set1_pd(999999.0);
    let mut xc = S::sub_epi64(S::cvtpd_epi64(x), S::set1_epi64(1));
    let mut yc_base = S::sub_epi64(S::cvtpd_epi64(y), S::set1_epi64(1));

    let mut xcf = S::sub_pd(S::cvtepi64_pd(xc), x);
    let ycf_base = S::sub_pd(S::cvtepi64_pd(yc_base), y);

    xc = S::mullo_epi64(xc, S::set1_epi64(X_PRIME_64));
    yc_base = S::mullo_epi64(yc_base, S::set1_epi64(Y_PRIME_64));
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
                                S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                                S::set1_pd(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::set1_epi64(BIT_10_MASK),
                                )),
                                S::set1_pd(511.5),
                            );
                            let mut xd2 = S::mul_pd(xd, xd);
                            let inv_mag =
                                S::mul_pd(jitter, S::rsqrt_pd(S::add_pd(xd2, S::mul_pd(yd, yd))));
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);
                            xd2 = S::mul_pd(xd, xd);
                            let new_distance = S::add_pd(xd2, S::mul_pd(yd, yd));
                            distance = S::min_pd(new_distance, distance);

                            ycf = S::add_pd(ycf, S::set1_pd(1.0));
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                                S::set1_pd(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::set1_epi64(BIT_10_MASK),
                                )),
                                S::set1_pd(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::rsqrt_pd(S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd))),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_distance = S::add_pd(S::abs_pd(xd), S::abs_pd(yd));
                            distance = S::min_pd(new_distance, distance);

                            ycf = S::add_pd(ycf, S::set1_pd(1.0));
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                                S::set1_pd(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::set1_epi64(BIT_10_MASK),
                                )),
                                S::set1_pd(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::rsqrt_pd(S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd))),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_distance = {
                                let euc = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                                let man = S::add_pd(S::abs_pd(xd), S::abs_pd(yd));
                                S::add_pd(euc, man)
                            };
                            distance = S::min_pd(new_distance, distance);

                            ycf = S::add_pd(ycf, S::set1_pd(1.0));
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
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
                                S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                                S::set1_pd(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::set1_epi64(BIT_10_MASK),
                                )),
                                S::set1_pd(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::rsqrt_pd(S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd))),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_pd(S::set1_pd(HASH_2_FLOAT), S::cvtepi64_pd(hash));
                            let new_distance = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                            let closer = S::cmplt_pd(new_distance, distance);
                            distance = S::min_pd(new_distance, distance);
                            cell_value = S::blendv_pd(cell_value, new_cell_value, closer);

                            ycf = S::add_pd(ycf, S::set1_pd(1.0));
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                                S::set1_pd(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::set1_epi64(BIT_10_MASK),
                                )),
                                S::set1_pd(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::rsqrt_pd(S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd))),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_pd(S::set1_pd(HASH_2_FLOAT), S::cvtepi64_pd(hash));
                            let new_distance = S::add_pd(S::abs_pd(xd), S::abs_pd(yd));
                            let closer = S::cmplt_pd(new_distance, distance);
                            distance = S::min_pd(new_distance, distance);
                            cell_value = S::blendv_pd(cell_value, new_cell_value, closer);

                            ycf = S::add_pd(ycf, S::set1_pd(1.0));
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                                S::set1_pd(511.5),
                            );
                            let mut yd = S::sub_pd(
                                S::cvtepi64_pd(S::and_epi64(
                                    S::srai_epi64(hash, 10),
                                    S::set1_epi64(BIT_10_MASK),
                                )),
                                S::set1_pd(511.5),
                            );
                            let inv_mag = S::mul_pd(
                                jitter,
                                S::rsqrt_pd(S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd))),
                            );
                            xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                            yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_pd(S::set1_pd(HASH_2_FLOAT), S::cvtepi64_pd(hash));
                            let new_distance = {
                                let euc = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                                let man = S::add_pd(S::abs_pd(xd), S::abs_pd(yd));
                                S::add_pd(euc, man)
                            };
                            let closer = S::cmplt_pd(new_distance, distance);
                            distance = S::min_pd(new_distance, distance);
                            cell_value = S::blendv_pd(cell_value, new_cell_value, closer);

                            ycf = S::add_pd(ycf, S::set1_pd(1.0));
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
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
    let mut distance = S::set1_pd(999999.0);
    let mut cell_value = S::setzero_pd();

    let mut xc = S::sub_epi64(S::cvtpd_epi64(x), S::set1_epi64(1));
    let mut yc_base = S::sub_epi64(S::cvtpd_epi64(y), S::set1_epi64(1));
    let mut zc_base = S::sub_epi64(S::cvtpd_epi64(z), S::set1_epi64(1));

    let mut xcf = S::sub_pd(S::cvtepi64_pd(xc), x);
    let ycf_base = S::sub_pd(S::cvtepi64_pd(yc_base), y);
    let zcf_base = S::sub_pd(S::cvtepi64_pd(zc_base), z);

    xc = S::mullo_epi64(xc, S::set1_epi64(X_PRIME_64));
    yc_base = S::mullo_epi64(yc_base, S::set1_epi64(Y_PRIME_64));
    zc_base = S::mullo_epi64(zc_base, S::set1_epi64(Z_PRIME));

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d::<S>(seed, xc, yc, zc);
                let mut xd = S::sub_pd(
                    S::cvtepi64_pd(S::and_epi64(hash, S::set1_epi64(BIT_10_MASK))),
                    S::set1_pd(511.5),
                );
                let mut yd = S::sub_pd(
                    S::cvtepi64_pd(S::and_epi64(
                        S::srai_epi64(hash, 10),
                        S::set1_epi64(BIT_10_MASK),
                    )),
                    S::set1_pd(511.5),
                );
                let mut zd = S::sub_pd(
                    S::cvtepi64_pd(S::and_epi64(
                        S::srai_epi64(hash, 20),
                        S::set1_epi64(BIT_10_MASK),
                    )),
                    S::set1_pd(511.5),
                );
                let inv_mag = S::mul_pd(
                    jitter,
                    S::rsqrt_pd(S::add_pd(
                        S::mul_pd(xd, xd),
                        S::add_pd(S::mul_pd(yd, yd), S::mul_pd(zd, zd)),
                    )),
                );
                xd = S::add_pd(S::mul_pd(xd, inv_mag), xcf);
                yd = S::add_pd(S::mul_pd(yd, inv_mag), ycf);
                zd = S::add_pd(S::mul_pd(zd, inv_mag), zcf);

                let new_cell_value = S::mul_pd(S::set1_pd(HASH_2_FLOAT), S::cvtepi64_pd(hash));
                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => S::add_pd(
                        S::mul_pd(xd, xd),
                        S::add_pd(S::mul_pd(yd, yd), S::mul_pd(zd, zd)),
                    ),
                    CellDistanceFunction::Manhattan => {
                        S::add_pd(S::add_pd(S::abs_pd(xd), S::abs_pd(yd)), S::abs_pd(zd))
                    }
                    CellDistanceFunction::Natural => {
                        let euc = S::add_pd(
                            S::mul_pd(xd, xd),
                            S::add_pd(S::mul_pd(yd, yd), S::mul_pd(zd, zd)),
                        );
                        let man = S::add_pd(S::add_pd(S::abs_pd(xd), S::abs_pd(yd)), S::abs_pd(zd));
                        S::add_pd(euc, man)
                    }
                };
                let closer = S::cmplt_pd(new_distance, distance);
                distance = S::min_pd(new_distance, distance);
                cell_value = S::blendv_pd(cell_value, new_cell_value, closer);
                zcf = S::add_pd(ycf, S::set1_pd(1.0));
                zc = S::add_epi64(yc, S::set1_epi64(Z_PRIME));
            }
            ycf = S::add_pd(ycf, S::set1_pd(1.0));
            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME_64));
        }
        xcf = S::add_pd(xcf, S::set1_pd(1.0));
        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME_64));
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
    }
}
