//! Golden-fixture + cross-check tests for the compile-time **AVX-512** engine
//! (`simdnoise::intrinsics::avx512`), covering every noise type across 1D-4D in
//! f32 and f64. Companion to `intrinsics_x86.rs` (scalar/sse2/sse41/avx2) and
//! `intrinsics_cellular_64.rs`.
//!
//! The 2D/3D simplex-family dims match `intrinsics_x86.rs` (so the avx512
//! fixtures are a true parity set); 4D uses small extents to keep fixtures
//! compact. Simplex-family cases also assert avx512 ~= scalar within tolerance
//! (continuous noise), which is the real correctness gate for the new engine;
//! cellular relies on the golden fixture (its approximate `rsqrt` makes exact
//! cross-engine comparison unreliable).
//!
//! The whole file no-ops on hosts without AVX-512. Regenerate the fixtures on an
//! AVX-512 host with:
//!     SIMDNOISE_REGEN=1 cargo test --test intrinsics_avx512
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use simdnoise::intrinsics::{avx512, scalar};
use simdnoise::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, Cellular2Settings, CellularSettings,
    FbmSettings, GradientSettings, NoiseDimensions, NoiseType, RidgeSettings, Settings,
    SimplexSettings, TurbulenceSettings,
};

mod helpers;
use helpers::{
    read_from_file_f32, read_from_file_f64, save_to_file_f32, save_to_file_f64, ARCH, BIN_PATH,
};

const DIMS: [(usize, &str, usize, usize, usize, usize); 4] = [
    (1, "1d", 64, 1, 1, 1),
    (2, "2d", 64, 32, 1, 1),
    (3, "3d", 64, 32, 16, 1),
    (4, "4d", 16, 8, 4, 4),
];

fn dims_of(dim: usize, w: usize, h: usize, d: usize, t: usize) -> NoiseDimensions {
    let mut nd = NoiseDimensions::default(dim);
    nd.width = w;
    nd.height = h;
    nd.depth = d;
    nd.time = t;
    nd
}

fn check32(file_name: String, noise: &[f32]) {
    if std::env::var_os("SIMDNOISE_REGEN").is_some() {
        save_to_file_f32(&file_name, noise).unwrap();
    } else {
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, *noise, "mismatch {}", file_name);
    }
}

fn check64(file_name: String, noise: &[f64]) {
    if std::env::var_os("SIMDNOISE_REGEN").is_some() {
        save_to_file_f64(&file_name, noise).unwrap();
    } else {
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, *noise, "mismatch {}", file_name);
    }
}

const SIMPLEX_KINDS: [&str; 4] = ["gradient", "fbm", "ridge", "turbulence"];

fn simplex_nt(kind: &str, dims: NoiseDimensions) -> NoiseType {
    match kind {
        "gradient" => GradientSettings::default(dims).with_seed(1337).wrap(),
        "fbm" => FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5.0)
            .wrap(),
        "ridge" => RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5.0)
            .wrap(),
        "turbulence" => TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5.0)
            .wrap(),
        _ => unreachable!(),
    }
}

#[target_feature(enable = "avx512f")]
unsafe fn avx512_f32(nt: &NoiseType, dim: usize) -> Vec<f32> {
    type E = simdeez::engines::avx512::Avx512;
    match dim {
        1 => avx512::get_1d_noise::<E>(nt).0,
        2 => avx512::get_2d_noise::<E>(nt).0,
        3 => avx512::get_3d_noise::<E>(nt).0,
        _ => avx512::get_4d_noise::<E>(nt).0,
    }
}

#[target_feature(enable = "avx512f")]
unsafe fn avx512_f64(nt: &NoiseType, dim: usize) -> Vec<f64> {
    type E = simdeez::engines::avx512::Avx512;
    match dim {
        1 => avx512::get_1d_noise_64::<E>(nt).0,
        2 => avx512::get_2d_noise_64::<E>(nt).0,
        3 => avx512::get_3d_noise_64::<E>(nt).0,
        _ => avx512::get_4d_noise_64::<E>(nt).0,
    }
}

unsafe fn scalar_f32(nt: &NoiseType, dim: usize) -> Vec<f32> {
    type E = simdeez::scalar::Scalar;
    match dim {
        1 => scalar::get_1d_noise::<E>(nt).0,
        2 => scalar::get_2d_noise::<E>(nt).0,
        3 => scalar::get_3d_noise::<E>(nt).0,
        _ => scalar::get_4d_noise::<E>(nt).0,
    }
}

unsafe fn scalar_f64(nt: &NoiseType, dim: usize) -> Vec<f64> {
    type E = simdeez::scalar::Scalar;
    match dim {
        1 => scalar::get_1d_noise_64::<E>(nt).0,
        2 => scalar::get_2d_noise_64::<E>(nt).0,
        3 => scalar::get_3d_noise_64::<E>(nt).0,
        _ => scalar::get_4d_noise_64::<E>(nt).0,
    }
}

#[test]
fn avx512_simplex_family() {
    if !is_x86_feature_detected!("avx512f") {
        eprintln!("no avx512f; skipping");
        return;
    }
    for kind in SIMPLEX_KINDS {
        for (dim, ds, w, h, d, t) in DIMS {
            let nt = simplex_nt(kind, dims_of(dim, w, h, d, t));
            let av32 = unsafe { avx512_f32(&nt, dim) };
            let sc32 = unsafe { scalar_f32(&nt, dim) };
            let maxdiff = av32
                .iter()
                .zip(&sc32)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f32, f32::max);
            assert!(
                av32.iter().all(|v| v.is_finite()),
                "{} {} f32 finite",
                kind,
                ds
            );
            assert!(
                maxdiff < 1e-3,
                "{} {} f32 avx512 vs scalar = {}",
                kind,
                ds,
                maxdiff
            );
            check32(
                format!(
                    "{}/intrinsics_{}_32_{}_avx512_normal_{}.bin",
                    BIN_PATH, kind, ds, ARCH
                ),
                &av32,
            );

            let av64 = unsafe { avx512_f64(&nt, dim) };
            let sc64 = unsafe { scalar_f64(&nt, dim) };
            let maxdiff = av64
                .iter()
                .zip(&sc64)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max);
            assert!(
                av64.iter().all(|v| v.is_finite()),
                "{} {} f64 finite",
                kind,
                ds
            );
            assert!(
                maxdiff < 1e-6,
                "{} {} f64 avx512 vs scalar = {}",
                kind,
                ds,
                maxdiff
            );
            check64(
                format!(
                    "{}/intrinsics_{}_64_{}_avx512_normal_{}.bin",
                    BIN_PATH, kind, ds, ARCH
                ),
                &av64,
            );
        }
    }
}

const CELL_DF: [(CellDistanceFunction, &str); 3] = [
    (CellDistanceFunction::Euclidean, "euclidean"),
    (CellDistanceFunction::Manhattan, "manhattan"),
    (CellDistanceFunction::Natural, "natural"),
];
const CELL_RT: [(CellReturnType, &str); 2] = [
    (CellReturnType::Distance, "distance"),
    (CellReturnType::CellValue, "cellvalue"),
];
const CELL2_RT: [(Cell2ReturnType, &str); 5] = [
    (Cell2ReturnType::Distance2, "distance2"),
    (Cell2ReturnType::Distance2Add, "distance2add"),
    (Cell2ReturnType::Distance2Sub, "distance2sub"),
    (Cell2ReturnType::Distance2Mul, "distance2mul"),
    (Cell2ReturnType::Distance2Div, "distance2div"),
];

#[test]
fn avx512_cellular() {
    if !is_x86_feature_detected!("avx512f") {
        eprintln!("no avx512f; skipping");
        return;
    }
    for (dim, ds, w, h, d, t) in DIMS {
        for (df, dfn) in CELL_DF {
            for (rt, rtn) in CELL_RT {
                let nt = CellularSettings::default(dims_of(dim, w, h, d, t))
                    .with_seed(1337)
                    .with_distance_function(df)
                    .with_return_type(rt)
                    .wrap();
                let av32 = unsafe { avx512_f32(&nt, dim) };
                let av64 = unsafe { avx512_f64(&nt, dim) };
                assert!(av32.iter().all(|v| v.is_finite()) && av64.iter().all(|v| v.is_finite()));
                check32(
                    format!(
                        "{}/intrinsics_cellular_32_{}_avx512_{}_{}_{}.bin",
                        BIN_PATH, ds, dfn, rtn, ARCH
                    ),
                    &av32,
                );
                check64(
                    format!(
                        "{}/intrinsics_cellular_64_{}_avx512_{}_{}_{}.bin",
                        BIN_PATH, ds, dfn, rtn, ARCH
                    ),
                    &av64,
                );
            }
        }
        for (rt, rtn) in CELL2_RT {
            for (df, dfn) in CELL_DF {
                let nt = Cellular2Settings::default(dims_of(dim, w, h, d, t))
                    .with_seed(1337)
                    .with_distance_function(df)
                    .with_return_type(rt)
                    .wrap();
                let av32 = unsafe { avx512_f32(&nt, dim) };
                let av64 = unsafe { avx512_f64(&nt, dim) };
                assert!(av32.iter().all(|v| v.is_finite()) && av64.iter().all(|v| v.is_finite()));
                check32(
                    format!(
                        "{}/intrinsics_cellular2_32_{}_avx512_{}_{}_{}.bin",
                        BIN_PATH, ds, dfn, rtn, ARCH
                    ),
                    &av32,
                );
                check64(
                    format!(
                        "{}/intrinsics_cellular2_64_{}_avx512_{}_{}_{}.bin",
                        BIN_PATH, ds, dfn, rtn, ARCH
                    ),
                    &av64,
                );
            }
        }
    }
}
