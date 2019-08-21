extern crate simdeez;
use self::simdeez::*;
use super::*;
use std::f32;

const X_PRIME: i32 = 1619;
const Y_PRIME: i32 = 31337;
const Z_PRIME: i32 = 6971;
//const BIT_5_MASK : i32 = 31;
const BIT_10_MASK: i32 = 1023;
const HASH_2_FLOAT: f32 = 1.0 / 2147483648.0;

#[inline(always)]
unsafe fn hash_2d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32) -> S::Vi32 {
    let mut hash = S::xor_epi32(x, S::set1_epi32(seed));
    hash = S::xor_epi32(y, hash);
    S::mullo_epi32(
        S::mullo_epi32(S::mullo_epi32(hash, hash), S::set1_epi32(60493)),
        hash,
    )
}

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

    xc = S::mullo_epi32(xc, S::set1_epi32(X_PRIME));
    yc_base = S::mullo_epi32(yc_base, S::set1_epi32(Y_PRIME));
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
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
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
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
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
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
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
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
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
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
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
                            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
                        }
                        xcf = S::add_ps(xcf, S::set1_ps(1.0));
                        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
                    }
                }
            }
            cell_value
        }
    }
}

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
    let mut distance: [S::Vf32; 4] = [S::set1_ps(999999.0); 4];

    let mut xc = S::sub_epi32(S::cvtps_epi32(x), S::set1_epi32(1));
    let mut yc_base = S::sub_epi32(S::cvtps_epi32(y), S::set1_epi32(1));

    let mut xcf = S::sub_ps(S::cvtepi32_ps(xc), x);
    let ycf_base = S::sub_ps(S::cvtepi32_ps(yc_base), y);

    xc = S::mullo_epi32(xc, S::set1_epi32(X_PRIME));
    yc_base = S::mullo_epi32(yc_base, S::set1_epi32(Y_PRIME));

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

            let new_distance = match distance_function {
                CellDistanceFunction::Euclidean => S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd)),
                CellDistanceFunction::Manhattan => S::add_ps(S::abs_ps(xd), S::abs_ps(yd)),
                CellDistanceFunction::Natural => {
                    let euc = S::add_ps(S::mul_ps(xd, xd), S::mul_ps(yd, yd));
                    let man = S::add_ps(S::abs_ps(xd), S::abs_ps(yd));
                    S::add_ps(euc, man)
                }
            };
            let mut i = index1;
            while i > 0 {
                distance[i] = S::max_ps(S::min_ps(distance[i], new_distance), distance[i - 1]);
                distance[0] = S::min_ps(distance[0], new_distance);
                i -= 1;
            }
            ycf = S::add_ps(ycf, S::set1_ps(1.0));
            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
        }
        xcf = S::add_ps(xcf, S::set1_ps(1.0));
        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => S::add_ps(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Sub => S::sub_ps(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Mul => S::mul_ps(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Div => S::div_ps(distance[index0], distance[index1]),
    }
}

#[inline(always)]
unsafe fn hash_3d<S: Simd>(seed: i32, x: S::Vi32, y: S::Vi32, z: S::Vi32) -> S::Vi32 {
    let mut hash = S::xor_epi32(x, S::set1_epi32(seed));
    hash = S::xor_epi32(y, hash);
    hash = S::xor_epi32(z, hash);
    S::mullo_epi32(
        S::mullo_epi32(S::mullo_epi32(hash, hash), S::set1_epi32(60493)),
        hash,
    )
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

    xc = S::mullo_epi32(xc, S::set1_epi32(X_PRIME));
    yc_base = S::mullo_epi32(yc_base, S::set1_epi32(Y_PRIME));
    zc_base = S::mullo_epi32(zc_base, S::set1_epi32(Z_PRIME));

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
                zc = S::add_epi32(yc, S::set1_epi32(Z_PRIME));
            }
            ycf = S::add_ps(ycf, S::set1_ps(1.0));
            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
        }
        xcf = S::add_ps(xcf, S::set1_ps(1.0));
        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => cell_value,
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
    let mut distance: [S::Vf32; 4] = [S::set1_ps(999999.0); 4];

    let mut xc = S::sub_epi32(S::cvtps_epi32(x), S::set1_epi32(1));
    let mut yc_base = S::sub_epi32(S::cvtps_epi32(y), S::set1_epi32(1));
    let mut zc_base = S::sub_epi32(S::cvtps_epi32(z), S::set1_epi32(1));

    let mut xcf = S::sub_ps(S::cvtepi32_ps(xc), x);
    let ycf_base = S::sub_ps(S::cvtepi32_ps(yc_base), y);
    let zcf_base = S::sub_ps(S::cvtepi32_ps(zc_base), z);

    xc = S::mullo_epi32(xc, S::set1_epi32(X_PRIME));
    yc_base = S::mullo_epi32(yc_base, S::set1_epi32(Y_PRIME));
    zc_base = S::mullo_epi32(zc_base, S::set1_epi32(Z_PRIME));

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
                let mut i = index1;
                while i > 0 {
                    distance[i] = S::max_ps(S::min_ps(distance[i], new_distance), distance[i - 1]);
                    distance[0] = S::min_ps(distance[0], new_distance);
                    i -= 1;
                }
                zcf = S::add_ps(ycf, S::set1_ps(1.0));
                zc = S::add_epi32(yc, S::set1_epi32(Z_PRIME));
            }
            ycf = S::add_ps(ycf, S::set1_ps(1.0));
            yc = S::add_epi32(yc, S::set1_epi32(Y_PRIME));
        }
        xcf = S::add_ps(xcf, S::set1_ps(1.0));
        xc = S::add_epi32(xc, S::set1_epi32(X_PRIME));
    }

    match return_type {
        Cell2ReturnType::Distance2 => distance[index1],
        Cell2ReturnType::Distance2Add => S::add_ps(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Sub => S::sub_ps(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Mul => S::mul_ps(distance[index0], distance[index1]),
        Cell2ReturnType::Distance2Div => S::div_ps(distance[index0], distance[index1]),
    }
}
