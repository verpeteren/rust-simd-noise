extern crate simdeez;
use self::simdeez::*;
use super::*;
use shared::*;
use std::f32;

const X_PRIME: i32 = 1619;
const Y_PRIME: i32 = 31337;
const Z_PRIME: i32 = 6971;
const CELL_DIVISOR: f32 = 2147483648.0;

#[inline(always)]
unsafe fn hash_2d<S: Simd>(seed: S::Vi32, x: S::Vi32, y: S::Vi32) -> S::Vi32 {
    let mut hash = S::xor_epi32(seed, S::mullo_epi32(S::set1_epi32(X_PRIME), x));
    hash = S::xor_epi32(hash, S::mullo_epi32(S::set1_epi32(Y_PRIME), y));
    hash = S::mullo_epi32(
        hash,
        S::mullo_epi32(hash, S::mullo_epi32(hash, S::set1_epi32(60493))),
    );
    S::xor_epi32(S::srai_epi32(hash, 13), hash)
}
#[inline(always)]
unsafe fn val_coord_2d<S: Simd>(seed: S::Vi32, x: S::Vi32, y: S::Vi32) -> S::Vf32 {
    let mut hash = S::xor_epi32(seed, S::mullo_epi32(S::set1_epi32(X_PRIME), x));
    hash = S::xor_epi32(hash, S::mullo_epi32(S::set1_epi32(Y_PRIME), y));
    hash = S::mullo_epi32(
        hash,
        S::mullo_epi32(hash, S::mullo_epi32(hash, S::set1_epi32(60493))),
    );
    S::div_ps(S::cvtepi32_ps(hash), S::set1_ps(CELL_DIVISOR))
}
#[inline(always)]
pub unsafe fn cellular_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf32,
) -> S::Vf32 {
    let xr = S::cvtps_epi32(S::round_ps(x));
    let yr = S::cvtps_epi32(S::round_ps(y));
    let mut distance = S::set1_ps(f32::MAX);
    let mut xc = S::set1_epi32(0);
    let mut yc = S::set1_epi32(0);
    match distance_function {
        CellDistanceFunction::Euclidean => {
            for xmod in -1..2 {
                let xi = S::add_epi32(xr, S::set1_epi32(xmod));
                let xisubx = S::sub_ps(S::cvtepi32_ps(xi), x);
                for ymod in -1..2 {
                    let yi = S::add_epi32(yr, S::set1_epi32(ymod));
                    let hi = S::and_epi32(
                        hash_2d::<S>(S::set1_epi32(1337), xi, yi),
                        S::set1_epi32(0xff),
                    );
                    let cellx = S::i32gather_ps(&CELL_2D_X, hi);
                    let celly = S::i32gather_ps(&CELL_2D_Y, hi);

                    let vx = S::add_ps(xisubx, S::mul_ps(cellx, jitter));

                    let vy = S::add_ps(S::sub_ps(S::cvtepi32_ps(yi), y), S::mul_ps(celly, jitter));
                    let new_dist = S::add_ps(S::mul_ps(vx, vx), S::mul_ps(vy, vy));
                    let cond = S::cmplt_ps(new_dist, distance);
                    distance = S::blendv_ps(distance, new_dist, cond);
                    xc = S::blendv_epi32(xc, xi, S::castps_epi32(cond));
                    yc = S::blendv_epi32(yc, yi, S::castps_epi32(cond));
                }
            }
        }
        CellDistanceFunction::Manhattan => {
            for xmod in -1..2 {
                let xi = S::add_epi32(xr, S::set1_epi32(xmod));
                let xisubx = S::sub_ps(S::cvtepi32_ps(xi), x);
                for ymod in -1..2 {
                    let yi = S::add_epi32(yr, S::set1_epi32(ymod));
                    let hi = S::and_epi32(
                        hash_2d::<S>(S::set1_epi32(1337), xi, yi),
                        S::set1_epi32(0xff),
                    );
                    let cellx = S::i32gather_ps(&CELL_2D_X, hi);
                    let celly = S::i32gather_ps(&CELL_2D_Y, hi);

                    let vx = S::add_ps(xisubx, S::mul_ps(cellx, jitter));

                    let vy = S::add_ps(S::sub_ps(S::cvtepi32_ps(yi), y), S::mul_ps(celly, jitter));
                    let new_dist = S::add_ps(S::abs_ps(vx), S::abs_ps(vy));
                    let cond = S::cmplt_ps(new_dist, distance);
                    distance = S::blendv_ps(distance, new_dist, cond);
                    xc = S::blendv_epi32(xc, xi, S::castps_epi32(cond));
                    yc = S::blendv_epi32(yc, yi, S::castps_epi32(cond));
                }
            }
        }
        CellDistanceFunction::Natural => {
            for xmod in -1..2 {
                let xi = S::add_epi32(xr, S::set1_epi32(xmod));
                let xisubx = S::sub_ps(S::cvtepi32_ps(xi), x);
                for ymod in -1..2 {
                    let yi = S::add_epi32(yr, S::set1_epi32(ymod));
                    let hi = S::and_epi32(
                        hash_2d::<S>(S::set1_epi32(1337), xi, yi),
                        S::set1_epi32(0xff),
                    );
                    let cellx = S::i32gather_ps(&CELL_2D_X, hi);
                    let celly = S::i32gather_ps(&CELL_2D_Y, hi);

                    let vx = S::add_ps(xisubx, S::mul_ps(cellx, jitter));

                    let vy = S::add_ps(S::sub_ps(S::cvtepi32_ps(yi), y), S::mul_ps(celly, jitter));
                    let new_dist = S::add_ps(
                        S::add_ps(S::abs_ps(vx), S::abs_ps(vy)),
                        S::add_ps(S::mul_ps(vx, vx), S::mul_ps(vy, vy)),
                    );
                    let cond = S::cmplt_ps(new_dist, distance);
                    distance = S::blendv_ps(distance, new_dist, cond);
                    xc = S::blendv_epi32(xc, xi, S::castps_epi32(cond));
                    yc = S::blendv_epi32(yc, yi, S::castps_epi32(cond));
                }
            }
        }
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => val_coord_2d::<S>(S::set1_epi32(1337), xc, yc),
    }
}

#[inline(always)]
unsafe fn hash_3d<S: Simd>(seed: S::Vi32, x: S::Vi32, y: S::Vi32, z: S::Vi32) -> S::Vi32 {
    let mut hash = S::xor_epi32(seed, S::mullo_epi32(S::set1_epi32(X_PRIME), x));
    hash = S::xor_epi32(hash, S::mullo_epi32(S::set1_epi32(Y_PRIME), y));
    hash = S::xor_epi32(hash, S::mullo_epi32(S::set1_epi32(Z_PRIME), z));
    hash = S::mullo_epi32(
        hash,
        S::mullo_epi32(hash, S::mullo_epi32(hash, S::set1_epi32(60493))),
    );
    S::xor_epi32(S::srai_epi32(hash, 13), hash)
}
#[inline(always)]
unsafe fn val_coord_3d<S: Simd>(seed: S::Vi32, x: S::Vi32, y: S::Vi32, z: S::Vi32) -> S::Vf32 {
    let mut hash = S::xor_epi32(seed, S::mullo_epi32(S::set1_epi32(X_PRIME), x));
    hash = S::xor_epi32(hash, S::mullo_epi32(S::set1_epi32(Y_PRIME), y));
    hash = S::xor_epi32(hash, S::mullo_epi32(S::set1_epi32(Z_PRIME), z));
    hash = S::mullo_epi32(
        hash,
        S::mullo_epi32(hash, S::mullo_epi32(hash, S::set1_epi32(60493))),
    );
    S::div_ps(S::cvtepi32_ps(hash), S::set1_ps(CELL_DIVISOR))
}
#[inline(always)]
pub unsafe fn cellular_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    distance_function: CellDistanceFunction,
    return_type: CellReturnType,
    jitter: S::Vf32,
) -> S::Vf32 {
    let xr = S::cvtps_epi32(S::round_ps(x));
    let yr = S::cvtps_epi32(S::round_ps(y));
    let zr = S::cvtps_epi32(S::round_ps(z));
    let mut distance = S::set1_ps(f32::MAX);
    let mut xc = S::set1_epi32(0);
    let mut yc = S::set1_epi32(0);
    let mut zc = S::set1_epi32(0);

    match distance_function {
        CellDistanceFunction::Euclidean => {
            for xmod in -1..2 {
                let xi = S::add_epi32(xr, S::set1_epi32(xmod));
                let xisubx = S::sub_ps(S::cvtepi32_ps(xi), x);
                for ymod in -1..2 {
                    let yi = S::add_epi32(yr, S::set1_epi32(ymod));
                    for zmod in -1..2 {
                        let zi = S::add_epi32(zr, S::set1_epi32(zmod));
                        let hi = S::and_epi32(
                            hash_3d::<S>(S::set1_epi32(1337), xi, yi, zi),
                            S::set1_epi32(0xff),
                        );
                        let cellx = S::i32gather_ps(&CELL_3D_X, hi);
                        let celly = S::i32gather_ps(&CELL_3D_Y, hi);
                        let cellz = S::i32gather_ps(&CELL_3D_Z, hi);

                        let vx = S::add_ps(xisubx, S::mul_ps(cellx, jitter));

                        let vy =
                            S::add_ps(S::sub_ps(S::cvtepi32_ps(yi), y), S::mul_ps(celly, jitter));
                        let vz =
                            S::add_ps(S::sub_ps(S::cvtepi32_ps(zi), z), S::mul_ps(cellz, jitter));

                        let new_dist = S::add_ps(
                            S::mul_ps(vz, vz),
                            S::add_ps(S::mul_ps(vx, vx), S::mul_ps(vy, vy)),
                        );
                        let cond = S::cmplt_ps(new_dist, distance);
                        distance = S::blendv_ps(distance, new_dist, cond);
                        xc = S::blendv_epi32(xc, xi, S::castps_epi32(cond));
                        yc = S::blendv_epi32(yc, yi, S::castps_epi32(cond));
                        zc = S::blendv_epi32(zc, zi, S::castps_epi32(cond));
                    }
                }
            }
        }
        CellDistanceFunction::Manhattan => {
            for xmod in -1..2 {
                let xi = S::add_epi32(xr, S::set1_epi32(xmod));
                let xisubx = S::sub_ps(S::cvtepi32_ps(xi), x);
                for ymod in -1..2 {
                    let yi = S::add_epi32(yr, S::set1_epi32(ymod));
                    for zmod in -1..2 {
                        let zi = S::add_epi32(zr, S::set1_epi32(zmod));
                        let hi = S::and_epi32(
                            hash_3d::<S>(S::set1_epi32(1337), xi, yi, zi),
                            S::set1_epi32(0xff),
                        );
                        let cellx = S::i32gather_ps(&CELL_3D_X, hi);
                        let celly = S::i32gather_ps(&CELL_3D_Y, hi);
                        let cellz = S::i32gather_ps(&CELL_3D_Z, hi);

                        let vx = S::add_ps(xisubx, S::mul_ps(cellx, jitter));

                        let vy =
                            S::add_ps(S::sub_ps(S::cvtepi32_ps(yi), y), S::mul_ps(celly, jitter));
                        let vz =
                            S::add_ps(S::sub_ps(S::cvtepi32_ps(zi), z), S::mul_ps(cellz, jitter));
                        let new_dist =
                            S::add_ps(S::abs_ps(vz), S::add_ps(S::abs_ps(vx), S::abs_ps(vy)));
                        let cond = S::cmplt_ps(new_dist, distance);
                        distance = S::blendv_ps(distance, new_dist, cond);
                        xc = S::blendv_epi32(xc, xi, S::castps_epi32(cond));
                        yc = S::blendv_epi32(yc, yi, S::castps_epi32(cond));
                        zc = S::blendv_epi32(zc, zi, S::castps_epi32(cond));
                    }
                }
            }
        }
        CellDistanceFunction::Natural => {
            for xmod in -1..2 {
                let xi = S::add_epi32(xr, S::set1_epi32(xmod));
                let xisubx = S::sub_ps(S::cvtepi32_ps(xi), x);
                for ymod in -1..2 {
                    let yi = S::add_epi32(yr, S::set1_epi32(ymod));
                    for zmod in -1..2 {
                        let zi = S::add_epi32(zr, S::set1_epi32(zmod));

                        let hi = S::and_epi32(
                            hash_3d::<S>(S::set1_epi32(1337), xi, yi, zi),
                            S::set1_epi32(0xff),
                        );
                        let cellx = S::i32gather_ps(&CELL_3D_X, hi);
                        let celly = S::i32gather_ps(&CELL_3D_Y, hi);
                        let cellz = S::i32gather_ps(&CELL_3D_Z, hi);

                        let vx = S::add_ps(xisubx, S::mul_ps(cellx, jitter));

                        let vy =
                            S::add_ps(S::sub_ps(S::cvtepi32_ps(yi), y), S::mul_ps(celly, jitter));
                        let vz =
                            S::add_ps(S::sub_ps(S::cvtepi32_ps(zi), z), S::mul_ps(cellz, jitter));
                        let new_dist = S::add_ps(
                            S::add_ps(S::abs_ps(vz), S::add_ps(S::abs_ps(vx), S::abs_ps(vy))),
                            S::add_ps(
                                S::mul_ps(vz, vz),
                                S::add_ps(S::mul_ps(vx, vx), S::mul_ps(vy, vy)),
                            ),
                        );
                        let cond = S::cmplt_ps(new_dist, distance);
                        distance = S::blendv_ps(distance, new_dist, cond);
                        xc = S::blendv_epi32(xc, xi, S::castps_epi32(cond));
                        yc = S::blendv_epi32(yc, yi, S::castps_epi32(cond));
                        zc = S::blendv_epi32(zc, zi, S::castps_epi32(cond));
                    }
                }
            }
        }
    }

    match return_type {
        CellReturnType::Distance => distance,
        CellReturnType::CellValue => val_coord_3d::<S>(S::set1_epi32(1337), xc, yc, zc),
    }
}
