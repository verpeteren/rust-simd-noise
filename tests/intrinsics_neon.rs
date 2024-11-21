/*
use simdnoise::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, Cellular2Settings, CellularSettings,
    FbmSettings, GradientSettings, NoiseDimensions, NoiseType, RidgeSettings, Settings,
    SimplexSettings, TurbulenceSettings,
};

mod helpers;
use helpers::{
    read_from_file_f32, read_from_file_f64, save_to_file_f32, save_to_file_f64, ARCH, BIN_PATH,
};
#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use core::arch::aarch64::*;
    use simdnoise::intrinsics::neon;

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_euclidean_cellvalue() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(CellReturnType::CellValue)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_euclidean_cellvalue() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "euclidean_cellvalue", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_euclidean_cellvalue();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_euclidean_cellvalue() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(CellReturnType::CellValue)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_euclidean_cellvalue() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "euclidean_cellvalue", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_euclidean_cellvalue();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_euclidean_distance() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(CellReturnType::Distance)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_euclidean_distance() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "euclidean_distance", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_euclidean_distance();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_euclidean_distance() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(CellReturnType::Distance)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_euclidean_distance() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "euclidean_distance", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_euclidean_distance();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_manhattan_cellvalue() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(CellReturnType::CellValue)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_manhattan_cellvalue() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "manhattan_cellvalue", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_manhattan_cellvalue();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_manhattan_cellvalue() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(CellReturnType::CellValue)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_manhattan_cellvalue() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "manhattan_cellvalue", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_manhattan_cellvalue();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_manhattan_distance() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(CellReturnType::Distance)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_manhattan_distance() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "manhattan_distance", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_manhattan_distance();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_manhattan_distance() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(CellReturnType::Distance)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_manhattan_distance() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "manhattan_distance", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_manhattan_distance();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_natural_cellvalue() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(CellReturnType::CellValue)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_natural_cellvalue() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "natural_cellvalue", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_natural_cellvalue();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_natural_cellvalue() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(CellReturnType::CellValue)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_natural_cellvalue() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "natural_cellvalue", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_natural_cellvalue();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_2_neon_32_natural_distance() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(CellReturnType::Distance)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_2_neon_32_natural_distance() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "2d", "neon", "natural_distance", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_2_neon_32_natural_distance();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular_3_neon_32_natural_distance() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = CellularSettings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(CellReturnType::Distance)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular_3_neon_32_natural_distance() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular", "32", "3d", "neon", "natural_distance", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular_3_neon_32_natural_distance();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_euclidean_distance2() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_euclidean_distance2() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "euclidean_distance2", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_euclidean_distance2();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_euclidean_distance2() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_euclidean_distance2() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "euclidean_distance2", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_euclidean_distance2();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_euclidean_distance2add() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Add)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_euclidean_distance2add() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "euclidean_distance2add", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_euclidean_distance2add();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_euclidean_distance2add() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Add)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_euclidean_distance2add() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "euclidean_distance2add", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_euclidean_distance2add();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_euclidean_distance2sub() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Sub)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_euclidean_distance2sub() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "euclidean_distance2sub", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_euclidean_distance2sub();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_euclidean_distance2sub() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Sub)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_euclidean_distance2sub() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "euclidean_distance2sub", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_euclidean_distance2sub();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_euclidean_distance2mul() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Mul)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_euclidean_distance2mul() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "euclidean_distance2mul", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_euclidean_distance2mul();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_euclidean_distance2mul() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Mul)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_euclidean_distance2mul() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "euclidean_distance2mul", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_euclidean_distance2mul();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_euclidean_distance2div() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_euclidean_distance2div() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "euclidean_distance2div", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_euclidean_distance2div();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_euclidean_distance2div() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Euclidean)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_euclidean_distance2div() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "euclidean_distance2div", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_euclidean_distance2div();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_manhattan_distance2() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_manhattan_distance2() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "manhattan_distance2", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_manhattan_distance2();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_manhattan_distance2() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_manhattan_distance2() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "manhattan_distance2", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_manhattan_distance2();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_manhattan_distance2add() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Add)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_manhattan_distance2add() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "manhattan_distance2add", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_manhattan_distance2add();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_manhattan_distance2add() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Add)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_manhattan_distance2add() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "manhattan_distance2add", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_manhattan_distance2add();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_manhattan_distance2sub() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Sub)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_manhattan_distance2sub() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "manhattan_distance2sub", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_manhattan_distance2sub();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_manhattan_distance2sub() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Sub)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_manhattan_distance2sub() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "manhattan_distance2sub", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_manhattan_distance2sub();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_manhattan_distance2mul() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Mul)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_manhattan_distance2mul() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "manhattan_distance2mul", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_manhattan_distance2mul();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_manhattan_distance2mul() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Mul)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_manhattan_distance2mul() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "manhattan_distance2mul", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_manhattan_distance2mul();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_manhattan_distance2div() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_manhattan_distance2div() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "manhattan_distance2div", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_manhattan_distance2div();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_manhattan_distance2div() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Manhattan)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_manhattan_distance2div() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "manhattan_distance2div", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_manhattan_distance2div();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_natural_distance2() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_natural_distance2() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "natural_distance2", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_natural_distance2();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_natural_distance2() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_natural_distance2() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "natural_distance2", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_natural_distance2();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_natural_distance2add() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Add)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_natural_distance2add() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "natural_distance2add", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_natural_distance2add();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_natural_distance2add() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Add)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_natural_distance2add() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "natural_distance2add", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_natural_distance2add();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_natural_distance2sub() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Sub)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_natural_distance2sub() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "natural_distance2sub", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_natural_distance2sub();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_natural_distance2sub() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Sub)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_natural_distance2sub() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "natural_distance2sub", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_natural_distance2sub();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_natural_distance2mul() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Mul)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_natural_distance2mul() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "natural_distance2mul", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_natural_distance2mul();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_natural_distance2mul() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Mul)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_natural_distance2mul() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "natural_distance2mul", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_natural_distance2mul();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_2_neon_32_natural_distance2div() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_2_neon_32_natural_distance2div() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "2d", "neon", "natural_distance2div", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_2_neon_32_natural_distance2div();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_cellular2_3_neon_32_natural_distance2div() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = Cellular2Settings::default(dims)
            .with_seed(1337)
            .with_distance_function(CellDistanceFunction::Natural)
            .with_return_type(Cell2ReturnType::Distance2Div)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_cellular2_3_neon_32_natural_distance2div() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "cellular2", "32", "3d", "neon", "natural_distance2div", ARCH
        );
        unsafe {
            let noise = do_intrinsic_cellular2_3_neon_32_natural_distance2div();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_1_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_1d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_1_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "32", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_1_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_1_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_1d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_1_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "64", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_1_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_2_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_2_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "32", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_2_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_2_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_2_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "64", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_2_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_3_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_3_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "32", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_3_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_3_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_3_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "64", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_3_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_4_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_4d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_4_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "32", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_4_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_ridge_4_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = RidgeSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_4d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_ridge_4_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "ridge", "64", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_ridge_4_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_1_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_1d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_1_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "32", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_1_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_1_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_1d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_1_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "64", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_1_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_2_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_2_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "32", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_2_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_2_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_2_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "64", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_2_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_3_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_3_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "32", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_3_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_3_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_3_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "64", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_3_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_4_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_4d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_4_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "32", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_4_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_fbm_4_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = FbmSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_4d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_fbm_4_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "fbm", "64", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_fbm_4_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_1_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_1d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_1_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "32", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_1_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_1_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_1d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_1_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "64", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_1_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_2_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_2_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "32", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_2_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_2_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_2d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_2_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "64", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_2_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_3_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_3_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "32", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_3_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_3_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_3d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_3_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "64", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_3_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_4_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_4d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_4_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "32", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_4_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_turbulence_4_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = TurbulenceSettings::default(dims)
            .with_seed(1337)
            .with_lacunarity(0.5)
            .with_gain(2.0)
            .with_octaves(5)
            .wrap();
        let (noise, _min, _max) = neon::get_4d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_turbulence_4_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "turbulence", "64", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_turbulence_4_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_1_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_1d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_1_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "32", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_1_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_1_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            ..NoiseDimensions::default(1)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_1d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_1_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "64", "1d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_1_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_2_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_2d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_2_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "32", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_2_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_2_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            ..NoiseDimensions::default(2)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_2d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_2_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "64", "2d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_2_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_3_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_3d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_3_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "32", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_3_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_3_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            ..NoiseDimensions::default(3)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_3d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_3_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "64", "3d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_3_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_4_neon_32_normal() -> Vec<f32> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_4d_noise(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_4_neon_32_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "32", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_4_neon_32_normal();
            //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f32(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }

    #[target_feature(enable = "neon")]
    unsafe fn do_intrinsic_gradient_4_neon_64_normal() -> Vec<f64> {
        let dims = NoiseDimensions {
            width: 64,
            height: 32,
            depth: 16,
            time: 8,
            ..NoiseDimensions::default(4)
        };

        let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
        let (noise, _min, _max) = neon::get_4d_noise_64(&noise_type);
        noise
    }

    #[test]
    fn test_intrinsic_gradient_4_neon_64_normal() {
        let file_name = format!(
            "{}/{}_{}_{}_{}_{}_{}_{}.bin",
            BIN_PATH, "intrinsics", "gradient", "64", "4d", "neon", "normal", ARCH
        );
        unsafe {
            let noise = do_intrinsic_gradient_4_neon_64_normal();
            //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
            let expected = read_from_file_f64(&file_name).unwrap();
            assert_eq!(expected, noise);
        }
    }
}
*/
