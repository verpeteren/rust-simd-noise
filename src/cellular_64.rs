use super::*;
use std::f64;

use simdeez::*;

const X_PRIME: i64 = 1619;
const Y_PRIME: i64 = 31337;
const Z_PRIME: i64 = 6971;
//const BIT_5_MASK : i64 = 31;
const BIT_10_MASK: i64 = 1023;
const HASH_2_FLOAT: f64 = 1.0 / 2147483648.0;

#[inline(always)]
unsafe fn hash_2d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64) -> S::Vi64 {
    let mut hash = S::xor_epi64(x, S::set1_epi64(seed));
    hash = S::xor_epi64(y, hash);
    S::mullo_epi64(
        S::mullo_epi64(S::mullo_epi64(hash, hash), S::set1_epi64(60493)),
        hash,
    )
}

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

    xc = S::mullo_epi64(xc, S::set1_epi64(X_PRIME));
    yc_base = S::mullo_epi64(yc_base, S::set1_epi64(Y_PRIME));
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
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
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
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
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
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
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
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
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
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
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
                            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
                        }
                        xcf = S::add_pd(xcf, S::set1_pd(1.0));
                        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
                    }
                }
            }
            cell_value
        }
    }
}

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
    let mut distance: [S::Vf64; 4] = [S::set1_pd(999999.0); 4];

    let mut xc = S::sub_epi64(S::cvtpd_epi64(x), S::set1_epi64(1));
    let mut yc_base = S::sub_epi64(S::cvtpd_epi64(y), S::set1_epi64(1));

    let mut xcf = S::sub_pd(S::cvtepi64_pd(xc), x);
    let ycf_base = S::sub_pd(S::cvtepi64_pd(yc_base), y);

    xc = S::mullo_epi64(xc, S::set1_epi64(X_PRIME));
    yc_base = S::mullo_epi64(yc_base, S::set1_epi64(Y_PRIME));

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

            let new_distance = match distance_function {
                CellDistanceFunction::Euclidean => S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd)),
                CellDistanceFunction::Manhattan => S::add_pd(S::abs_pd(xd), S::abs_pd(yd)),
                CellDistanceFunction::Natural => {
                    let euc = S::add_pd(S::mul_pd(xd, xd), S::mul_pd(yd, yd));
                    let man = S::add_pd(S::abs_pd(xd), S::abs_pd(yd));
                    S::add_pd(euc, man)
                }
            };
            let mut i = index1;
            while i > 0 {
                distance[i] = S::max_pd(S::min_pd(distance[i], new_distance), distance[i - 1]);
                distance[0] = S::min_pd(distance[0], new_distance);
                i -= 1;
            }
            ycf = S::add_pd(ycf, S::set1_pd(1.0));
            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
        }
        xcf = S::add_pd(xcf, S::set1_pd(1.0));
        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => S::add_pd(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Sub => S::sub_pd(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Mul => S::mul_pd(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Div => S::div_pd(distance[index0], distance[index1]),
    }
}

#[inline(always)]
unsafe fn hash_3d<S: Simd>(seed: i64, x: S::Vi64, y: S::Vi64, z: S::Vi64) -> S::Vi64 {
    let mut hash = S::xor_epi64(x, S::set1_epi64(seed));
    hash = S::xor_epi64(y, hash);
    hash = S::xor_epi64(z, hash);
    S::mullo_epi64(
        S::mullo_epi64(S::mullo_epi64(hash, hash), S::set1_epi64(60493)),
        hash,
    )
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

    xc = S::mullo_epi64(xc, S::set1_epi64(X_PRIME));
    yc_base = S::mullo_epi64(yc_base, S::set1_epi64(Y_PRIME));
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
            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
        }
        xcf = S::add_pd(xcf, S::set1_pd(1.0));
        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
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
    let mut distance: [S::Vf64; 4] = [S::set1_pd(999999.0); 4];

    let mut xc = S::sub_epi64(S::cvtpd_epi64(x), S::set1_epi64(1));
    let mut yc_base = S::sub_epi64(S::cvtpd_epi64(y), S::set1_epi64(1));
    let mut zc_base = S::sub_epi64(S::cvtpd_epi64(z), S::set1_epi64(1));

    let mut xcf = S::sub_pd(S::cvtepi64_pd(xc), x);
    let ycf_base = S::sub_pd(S::cvtepi64_pd(yc_base), y);
    let zcf_base = S::sub_pd(S::cvtepi64_pd(zc_base), z);

    xc = S::mullo_epi64(xc, S::set1_epi64(X_PRIME));
    yc_base = S::mullo_epi64(yc_base, S::set1_epi64(Y_PRIME));
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
                let mut i = index1;
                while i > 0 {
                    distance[i] = S::max_pd(S::min_pd(distance[i], new_distance), distance[i - 1]);
                    distance[0] = S::min_pd(distance[0], new_distance);
                    i -= 1;
                }
                zcf = S::add_pd(ycf, S::set1_pd(1.0));
                zc = S::add_epi64(yc, S::set1_epi64(Z_PRIME));
            }
            ycf = S::add_pd(ycf, S::set1_pd(1.0));
            yc = S::add_epi64(yc, S::set1_epi64(Y_PRIME));
        }
        xcf = S::add_pd(xcf, S::set1_pd(1.0));
        xc = S::add_epi64(xc, S::set1_epi64(X_PRIME));
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => S::add_pd(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Sub => S::sub_pd(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Mul => S::mul_pd(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Div => S::div_pd(distance[index0], distance[index1]),
    }
}
