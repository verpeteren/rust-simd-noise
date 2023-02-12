use super::cellular::{
    hash_2d, hash_3d, BIT_10_MASK, HASH_2_FLOAT, X_PRIME_32, Y_PRIME_32, Z_PRIME_32,
};
use crate::{CellDistanceFunction, CellReturnType};

use simdeez::Simd;

#[inline(always)]
pub unsafe fn cellular_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf32,
    seed: i32,
) -> S::Vf32 {
    let mut distance = S::set1_ps(999999.0);
    let mut xc = S::sub_epi32(S::cvtps_epi32(x), S::set1_epi32(1));
    let mut yc_base = S::sub_epi32(S::cvtps_epi32(y), S::set1_epi32(1));

    let mut xcf = S::sub_ps(S::cvtepi32_ps(xc), x);
    let ycf_base = S::sub_ps(S::cvtepi32_ps(yc_base), y);

    xc = S::mullo_epi32(xc, S::set1_epi32(X_PRIME_32));
    yc_base = S::mullo_epi32(yc_base, S::set1_epi32(Y_PRIME_32));
    match return_type {
        CellReturnType::Distance => {
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                                S::set1_ps(511.5),
                            );
                            let mut yd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(
                                    S::srai_epi32(hash, 10),
                                    S::set1_epi32(BIT_10_MASK),
                                )),
                                S::set1_ps(511.5),
                            );
                            let mut xd2 = S::mul_ps(xd, xd);
                            let inv_mag =
                                S::mul_ps(jitter, S::rsqrt_ps(S::add_ps(xd2, S::mul_ps(yd, yd))));
                            xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                            yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);
                            xd2 = S::mul_ps(xd, xd);
                            let new_distance = S::add_ps(xd2, S::mul_ps(yd, yd));
                            distance = S::min_ps(new_distance, distance);

                            ycf = S::add_ps(ycf, S::set1_ps(1.0));
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                                S::set1_ps(511.5),
                            );
                            let mut yd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(
                                    S::srai_epi32(hash, 10),
                                    S::set1_epi32(BIT_10_MASK),
                                )),
                                S::set1_ps(511.5),
                            );
                            let inv_mag = S::mul_ps(
                                jitter,
                                S::rsqrt_ps(S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd))),
                            );
                            xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                            yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);

                            let new_distance = S::add_ps(S::abs_ps(xd), S::abs_ps(yd));
                            distance = S::min_ps(new_distance, distance);

                            ycf = S::add_ps(ycf, S::set1_ps(1.0));
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                                S::set1_ps(511.5),
                            );
                            let mut yd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(
                                    S::srai_epi32(hash, 10),
                                    S::set1_epi32(BIT_10_MASK),
                                )),
                                S::set1_ps(511.5),
                            );
                            let inv_mag = S::mul_ps(
                                jitter,
                                S::rsqrt_ps(S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd))),
                            );
                            xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                            yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);

                            let new_distance = {
                                let euc = S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd));
                                let man = S::add_ps(S::abs_ps(xd), S::abs_ps(yd));
                                S::add_ps(euc, man)
                            };
                            distance = S::min_ps(new_distance, distance);

                            ycf = S::add_ps(ycf, S::set1_ps(1.0));
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
                    }
                }
            }
            distance
        }
        CellReturnType::CellValue => {
            let mut cell_value = S::setzero_ps();
            match distance_function {
                CellDistanceFunction::Euclidean => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                                S::set1_ps(511.5),
                            );
                            let mut yd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(
                                    S::srai_epi32(hash, 10),
                                    S::set1_epi32(BIT_10_MASK),
                                )),
                                S::set1_ps(511.5),
                            );
                            let inv_mag = S::mul_ps(
                                jitter,
                                S::rsqrt_ps(S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd))),
                            );
                            xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                            yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_ps(S::set1_ps(HASH_2_FLOAT), S::cvtepi32_ps(hash));
                            let new_distance = S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd));
                            let closer = S::cmplt_ps(new_distance, distance);
                            distance = S::min_ps(new_distance, distance);
                            cell_value = S::blendv_ps(cell_value, new_cell_value, closer);

                            ycf = S::add_ps(ycf, S::set1_ps(1.0));
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
                    }
                }
                CellDistanceFunction::Manhattan => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                                S::set1_ps(511.5),
                            );
                            let mut yd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(
                                    S::srai_epi32(hash, 10),
                                    S::set1_epi32(BIT_10_MASK),
                                )),
                                S::set1_ps(511.5),
                            );
                            let inv_mag = S::mul_ps(
                                jitter,
                                S::rsqrt_ps(S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd))),
                            );
                            xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                            yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_ps(S::set1_ps(HASH_2_FLOAT), S::cvtepi32_ps(hash));
                            let new_distance = S::add_ps(S::abs_ps(xd), S::abs_ps(yd));
                            let closer = S::cmplt_ps(new_distance, distance);
                            distance = S::min_ps(new_distance, distance);
                            cell_value = S::blendv_ps(cell_value, new_cell_value, closer);

                            ycf = S::add_ps(ycf, S::set1_ps(1.0));
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
                    }
                }
                CellDistanceFunction::Natural => {
                    for _x in 0..3 {
                        let mut ycf = ycf_base;
                        let mut yc = yc_base;
                        for _y in 0..3 {
                            let hash = hash_2d::<S>(seed, xc, yc);
                            let mut xd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                                S::set1_ps(511.5),
                            );
                            let mut yd = S::sub_ps(
                                S::cvtepi32_ps(S::and_epi32(
                                    S::srai_epi32(hash, 10),
                                    S::set1_epi32(BIT_10_MASK),
                                )),
                                S::set1_ps(511.5),
                            );
                            let inv_mag = S::mul_ps(
                                jitter,
                                S::rsqrt_ps(S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd))),
                            );
                            xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                            yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);

                            let new_cell_value =
                                S::mul_ps(S::set1_ps(HASH_2_FLOAT), S::cvtepi32_ps(hash));
                            let new_distance = {
                                let euc = S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd));
                                let man = S::add_ps(S::abs_ps(xd), S::abs_ps(yd));
                                S::add_ps(euc, man)
                            };
                            let closer = S::cmplt_ps(new_distance, distance);
                            distance = S::min_ps(new_distance, distance);
                            cell_value = S::blendv_ps(cell_value, new_cell_value, closer);

                            ycf = S::add_ps(ycf, S::set1_ps(1.0));
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
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
    let mut distance = S::set1_ps(999999.0);
    let mut cell_value = S::setzero_ps();

    let mut xc = S::sub_epi32(S::cvtps_epi32(x), S::set1_epi32(1));
    let mut yc_base = S::sub_epi32(S::cvtps_epi32(y), S::set1_epi32(1));
    let mut zc_base = S::sub_epi32(S::cvtps_epi32(z), S::set1_epi32(1));

    let mut xcf = S::sub_ps(S::cvtepi32_ps(xc), x);
    let ycf_base = S::sub_ps(S::cvtepi32_ps(yc_base), y);
    let zcf_base = S::sub_ps(S::cvtepi32_ps(zc_base), z);

    xc = S::mullo_epi32(xc, S::set1_epi32(X_PRIME_32));
    yc_base = S::mullo_epi32(yc_base, S::set1_epi32(Y_PRIME_32));
    zc_base = S::mullo_epi32(zc_base, S::set1_epi32(Z_PRIME_32));

    for _x in 0..3 {
        let mut ycf = ycf_base;
        let mut yc = yc_base;
        for _y in 0..3 {
            let mut zcf = zcf_base;
            let mut zc = zc_base;
            for _z in 0..3 {
                let hash = hash_3d::<S>(seed, xc, yc, zc);
                let mut xd = S::sub_ps(
                    S::cvtepi32_ps(S::and_epi32(hash, S::set1_epi32(BIT_10_MASK))),
                    S::set1_ps(511.5),
                );
                let mut yd = S::sub_ps(
                    S::cvtepi32_ps(S::and_epi32(
                        S::srai_epi32(hash, 10),
                        S::set1_epi32(BIT_10_MASK),
                    )),
                    S::set1_ps(511.5),
                );
                let mut zd = S::sub_ps(
                    S::cvtepi32_ps(S::and_epi32(
                        S::srai_epi32(hash, 20),
                        S::set1_epi32(BIT_10_MASK),
                    )),
                    S::set1_ps(511.5),
                );
                let inv_mag = S::mul_ps(
                    jitter,
                    S::rsqrt_ps(S::add_ps(
                        S::mul_ps(xd, xd),
                        S::add_ps(S::mul_ps(yd, yd), S::mul_ps(zd, zd)),
                    )),
                );
                xd = S::add_ps(S::mul_ps(xd, inv_mag), xcf);
                yd = S::add_ps(S::mul_ps(yd, inv_mag), ycf);
                zd = S::add_ps(S::mul_ps(zd, inv_mag), zcf);

                let new_cell_value = S::mul_ps(S::set1_ps(HASH_2_FLOAT), S::cvtepi32_ps(hash));
                let new_distance = match distance_function {
                    CellDistanceFunction::Euclidean => S::add_ps(
                        S::mul_ps(xd, xd),
                        S::add_ps(S::mul_ps(yd, yd), S::mul_ps(zd, zd)),
                    ),
                    CellDistanceFunction::Manhattan => {
                        S::add_ps(S::add_ps(S::abs_ps(xd), S::abs_ps(yd)), S::abs_ps(zd))
                    }
                    CellDistanceFunction::Natural => {
                        let euc = S::add_ps(
                            S::mul_ps(xd, xd),
                            S::add_ps(S::mul_ps(yd, yd), S::mul_ps(zd, zd)),
                        );
                        let man = S::add_ps(S::add_ps(S::abs_ps(xd), S::abs_ps(yd)), S::abs_ps(zd));
                        S::add_ps(euc, man)
                    }
                };
                let closer = S::cmplt_ps(new_distance, distance);
                distance = S::min_ps(new_distance, distance);
                cell_value = S::blendv_ps(cell_value, new_cell_value, closer);
                zcf = S::add_ps(ycf, S::set1_ps(1.0));
                zc = S::add_epi32(yc, S::set1_epi32(Z_PRIME_32));
            }
            ycf = S::add_ps(ycf, S::set1_ps(1.0));
            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME_32));
        }
        xcf = S::add_ps(xcf, S::set1_ps(1.0));
        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME_32));
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
    }
}
