//! f64 golden-fixture tests for cellular / cellular2 noise (x86 engines).
//!
//! Mirrors the f32 cellular tests in `intrinsics_x86.rs` but exercises the f64
//! path (`get_*_noise_64`), closing a coverage gap: the suite had no f64
//! cellular fixtures at all. There are 184 cases here, matching the 184 f32
//! cellular cases (4 x86 engines x 2 dims x {7 cellular, 16 cellular2} combos).
//!
//! The cases are macro-generated to stay compact (the repo already uses test
//! macros, e.g. `tests/emplace.rs`). Regenerate the `.bin` fixtures by running
//! with SIMDNOISE_REGEN set:
//!     SIMDNOISE_REGEN=1 cargo test --test intrinsics_cellular_64
//!
//! NEON f64 2d/3d is intentionally NOT covered (the known NEON f64 2d/3d NaN
//! case; NEON f64 fixtures exist only for 1d/4d), so this file is x86-only.
#![cfg(any(target_arch = "x86", target_arch = "x86_64"))]

use simdnoise::intrinsics::{avx2, scalar, sse2, sse41};
use simdnoise::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, Cellular2Settings, CellularSettings,
    NoiseDimensions, Settings,
};

mod helpers;
use helpers::{read_from_file_f64, save_to_file_f64, ARCH, BIN_PATH};

/// Compare against the stored fixture, or (re)write it when SIMDNOISE_REGEN is set.
fn check(file_name: String, noise: &[f64]) {
    if std::env::var_os("SIMDNOISE_REGEN").is_some() {
        save_to_file_f64(&file_name, noise).unwrap();
    } else {
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, *noise);
    }
}

macro_rules! cell {
    ($(#[$a:meta])* $name:ident, $m:ident, $getfn:ident, $eng:ty, $engname:literal,
     $dim:literal, $combo:literal, $dims:expr, $df:expr, $rt:expr) => {
        #[test]
        fn $name() {
            $(#[$a])*
            unsafe fn run() -> Vec<f64> {
                let nt = CellularSettings::default($dims)
                    .with_seed(1337)
                    .with_distance_function($df)
                    .with_return_type($rt)
                    .wrap();
                $m::$getfn::<$eng>(&nt).0
            }
            let file_name = format!(
                "{}/{}_{}_{}_{}_{}_{}_{}.bin",
                BIN_PATH, "intrinsics", "cellular", "64", $dim, $engname, $combo, ARCH
            );
            let noise = unsafe { run() };
            check(file_name, &noise);
        }
    };
}

macro_rules! cell2 {
    ($(#[$a:meta])* $name:ident, $m:ident, $getfn:ident, $eng:ty, $engname:literal,
     $dim:literal, $combo:literal, $dims:expr, $df:expr, $rt:expr) => {
        #[test]
        fn $name() {
            $(#[$a])*
            unsafe fn run() -> Vec<f64> {
                let nt = Cellular2Settings::default($dims)
                    .with_seed(1337)
                    .with_distance_function($df)
                    .with_return_type($rt)
                    .wrap();
                $m::$getfn::<$eng>(&nt).0
            }
            let file_name = format!(
                "{}/{}_{}_{}_{}_{}_{}_{}.bin",
                BIN_PATH, "intrinsics", "cellular2", "64", $dim, $engname, $combo, ARCH
            );
            let noise = unsafe { run() };
            check(file_name, &noise);
        }
    };
}

// ---- cellular (CellReturnType) ----
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_normal,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_euclidean_cellvalue,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_euclidean_distance,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_manhattan_cellvalue,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_manhattan_distance,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_natural_cellvalue,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_2d_avx2_natural_distance,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_normal,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_euclidean_cellvalue,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_euclidean_distance,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_manhattan_cellvalue,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_manhattan_distance,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_natural_cellvalue,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_2d_sse2_natural_distance,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_normal,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_euclidean_cellvalue,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_euclidean_distance,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_manhattan_cellvalue,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_manhattan_distance,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_natural_cellvalue,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_2d_sse41_natural_distance,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_2d_scalar_normal,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_2d_scalar_euclidean_cellvalue,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    test_cellular_64_2d_scalar_euclidean_distance,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_2d_scalar_manhattan_cellvalue,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    test_cellular_64_2d_scalar_manhattan_distance,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_2d_scalar_natural_cellvalue,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    test_cellular_64_2d_scalar_natural_distance,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_normal,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_euclidean_cellvalue,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_euclidean_distance,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_manhattan_cellvalue,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_manhattan_distance,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_natural_cellvalue,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "avx2")]
    test_cellular_64_3d_avx2_natural_distance,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_normal,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_euclidean_cellvalue,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_euclidean_distance,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_manhattan_cellvalue,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_manhattan_distance,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_natural_cellvalue,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse2")]
    test_cellular_64_3d_sse2_natural_distance,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_normal,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_euclidean_cellvalue,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_euclidean_distance,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_manhattan_cellvalue,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_manhattan_distance,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_natural_cellvalue,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    #[target_feature(enable = "sse4.1")]
    test_cellular_64_3d_sse41_natural_distance,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_3d_scalar_normal,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_3d_scalar_euclidean_cellvalue,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::CellValue
);
cell!(
    test_cellular_64_3d_scalar_euclidean_distance,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_3d_scalar_manhattan_cellvalue,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::CellValue
);
cell!(
    test_cellular_64_3d_scalar_manhattan_distance,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    CellReturnType::Distance
);
cell!(
    test_cellular_64_3d_scalar_natural_cellvalue,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_cellvalue",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::CellValue
);
cell!(
    test_cellular_64_3d_scalar_natural_distance,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_distance",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    CellReturnType::Distance
);

// ---- cellular2 (Cell2ReturnType) ----
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_normal,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_euclidean_distance2,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_euclidean_distance2add,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_euclidean_distance2sub,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_euclidean_distance2mul,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_euclidean_distance2div,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_manhattan_distance2,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_manhattan_distance2add,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_manhattan_distance2sub,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_manhattan_distance2mul,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_manhattan_distance2div,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_natural_distance2,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_natural_distance2add,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_natural_distance2sub,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_natural_distance2mul,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_2d_avx2_natural_distance2div,
    avx2,
    get_2d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "2d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_normal,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_euclidean_distance2,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_euclidean_distance2add,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_euclidean_distance2sub,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_euclidean_distance2mul,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_euclidean_distance2div,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_manhattan_distance2,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_manhattan_distance2add,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_manhattan_distance2sub,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_manhattan_distance2mul,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_manhattan_distance2div,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_natural_distance2,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_natural_distance2add,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_natural_distance2sub,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_natural_distance2mul,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_2d_sse2_natural_distance2div,
    sse2,
    get_2d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "2d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_normal,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_euclidean_distance2,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_euclidean_distance2add,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_euclidean_distance2sub,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_euclidean_distance2mul,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_euclidean_distance2div,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_manhattan_distance2,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_manhattan_distance2add,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_manhattan_distance2sub,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_manhattan_distance2mul,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_manhattan_distance2div,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_natural_distance2,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_natural_distance2add,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_natural_distance2sub,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_natural_distance2mul,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_2d_sse41_natural_distance2div,
    sse41,
    get_2d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "2d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    test_cellular2_64_2d_scalar_normal,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_2d_scalar_euclidean_distance2,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_2d_scalar_euclidean_distance2add,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    test_cellular2_64_2d_scalar_euclidean_distance2sub,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    test_cellular2_64_2d_scalar_euclidean_distance2mul,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    test_cellular2_64_2d_scalar_euclidean_distance2div,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    test_cellular2_64_2d_scalar_manhattan_distance2,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_2d_scalar_manhattan_distance2add,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    test_cellular2_64_2d_scalar_manhattan_distance2sub,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    test_cellular2_64_2d_scalar_manhattan_distance2mul,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    test_cellular2_64_2d_scalar_manhattan_distance2div,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    test_cellular2_64_2d_scalar_natural_distance2,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_2d_scalar_natural_distance2add,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    test_cellular2_64_2d_scalar_natural_distance2sub,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    test_cellular2_64_2d_scalar_natural_distance2mul,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    test_cellular2_64_2d_scalar_natural_distance2div,
    scalar,
    get_2d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "2d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_normal,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_euclidean_distance2,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_euclidean_distance2add,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_euclidean_distance2sub,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_euclidean_distance2mul,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_euclidean_distance2div,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_manhattan_distance2,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_manhattan_distance2add,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_manhattan_distance2sub,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_manhattan_distance2mul,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_manhattan_distance2div,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_natural_distance2,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_natural_distance2add,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_natural_distance2sub,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_natural_distance2mul,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "avx2")]
    test_cellular2_64_3d_avx2_natural_distance2div,
    avx2,
    get_3d_noise_64,
    simdeez::engines::avx2::Avx2,
    "avx2",
    "3d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_normal,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_euclidean_distance2,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_euclidean_distance2add,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_euclidean_distance2sub,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_euclidean_distance2mul,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_euclidean_distance2div,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_manhattan_distance2,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_manhattan_distance2add,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_manhattan_distance2sub,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_manhattan_distance2mul,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_manhattan_distance2div,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_natural_distance2,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_natural_distance2add,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_natural_distance2sub,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_natural_distance2mul,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse2")]
    test_cellular2_64_3d_sse2_natural_distance2div,
    sse2,
    get_3d_noise_64,
    simdeez::engines::sse2::Sse2,
    "sse2",
    "3d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_normal,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_euclidean_distance2,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_euclidean_distance2add,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_euclidean_distance2sub,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_euclidean_distance2mul,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_euclidean_distance2div,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_manhattan_distance2,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_manhattan_distance2add,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_manhattan_distance2sub,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_manhattan_distance2mul,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_manhattan_distance2div,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_natural_distance2,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_natural_distance2add,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_natural_distance2sub,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_natural_distance2mul,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    #[target_feature(enable = "sse4.1")]
    test_cellular2_64_3d_sse41_natural_distance2div,
    sse41,
    get_3d_noise_64,
    simdeez::engines::sse41::Sse41,
    "sse41",
    "3d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
cell2!(
    test_cellular2_64_3d_scalar_normal,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "normal",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_3d_scalar_euclidean_distance2,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_3d_scalar_euclidean_distance2add,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Add
);
cell2!(
    test_cellular2_64_3d_scalar_euclidean_distance2sub,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    test_cellular2_64_3d_scalar_euclidean_distance2mul,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    test_cellular2_64_3d_scalar_euclidean_distance2div,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "euclidean_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Euclidean,
    Cell2ReturnType::Distance2Div
);
cell2!(
    test_cellular2_64_3d_scalar_manhattan_distance2,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_3d_scalar_manhattan_distance2add,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Add
);
cell2!(
    test_cellular2_64_3d_scalar_manhattan_distance2sub,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    test_cellular2_64_3d_scalar_manhattan_distance2mul,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    test_cellular2_64_3d_scalar_manhattan_distance2div,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "manhattan_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Manhattan,
    Cell2ReturnType::Distance2Div
);
cell2!(
    test_cellular2_64_3d_scalar_natural_distance2,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_distance2",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2
);
cell2!(
    test_cellular2_64_3d_scalar_natural_distance2add,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_distance2add",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Add
);
cell2!(
    test_cellular2_64_3d_scalar_natural_distance2sub,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_distance2sub",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Sub
);
cell2!(
    test_cellular2_64_3d_scalar_natural_distance2mul,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_distance2mul",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Mul
);
cell2!(
    test_cellular2_64_3d_scalar_natural_distance2div,
    scalar,
    get_3d_noise_64,
    simdeez::scalar::Scalar,
    "scalar",
    "3d",
    "natural_distance2div",
    NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    },
    CellDistanceFunction::Natural,
    Cell2ReturnType::Distance2Div
);
