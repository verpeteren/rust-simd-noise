use super::cellular::{hash_2d, hash_3d, BIT_10_MASK, X_PRIME, Y_PRIME, Z_PRIME};
use crate::{Cell2ReturnType, CellDistanceFunction};

use simdeez::Simd;

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
