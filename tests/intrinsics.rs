use simdnoise::intrinsics::{avx2, scalar, sse2, sse41};
use simdnoise::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, Cellular2Settings, CellularSettings,
    FbmSettings, GradientSettings, NoiseDimensions, RidgeSettings, Settings, SimplexSettings,
    TurbulenceSettings,
};

mod helpers;
use helpers::{
    read_from_file_f32, read_from_file_f64, /*save_to_file_f32, save_to_file_f64, */ BIN_PATH,
};

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = CellularSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_euclidean_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_euclidean_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "euclidean_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_euclidean_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_euclidean_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_euclidean_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "euclidean_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_euclidean_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_manhattan_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_manhattan_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "manhattan_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_manhattan_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_manhattan_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_manhattan_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "manhattan_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_manhattan_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_natural_cellvalue() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_natural_cellvalue() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "natural_cellvalue"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_natural_cellvalue();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_2_avx2_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_avx2_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "avx2", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_avx2_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_2_scalar_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_scalar_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "scalar", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_scalar_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_2_sse2_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse2_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse2", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse2_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_2_sse41_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_2_sse41_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "2d", "sse41", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_2_sse41_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular_3_avx2_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_avx2_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "avx2", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_avx2_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular_3_scalar_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_scalar_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "scalar", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_scalar_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular_3_sse2_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse2_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse2", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse2_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular_3_sse41_32_natural_distance() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular_3_sse41_32_natural_distance() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular", "32", "3d", "sse41", "natural_distance"
    );
    unsafe {
        let noise = do_intrinsic_cellular_3_sse41_32_natural_distance();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = Cellular2Settings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_euclidean_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_euclidean_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "euclidean_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_euclidean_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_euclidean_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_euclidean_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "euclidean_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_euclidean_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_euclidean_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_euclidean_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "euclidean_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_euclidean_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_euclidean_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_euclidean_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "euclidean_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_euclidean_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_euclidean_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_euclidean_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "euclidean_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_euclidean_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_manhattan_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_manhattan_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "manhattan_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_manhattan_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_manhattan_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_manhattan_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "manhattan_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_manhattan_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_manhattan_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_manhattan_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "manhattan_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_manhattan_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_manhattan_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_manhattan_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "manhattan_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_manhattan_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_manhattan_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_manhattan_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "manhattan_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_manhattan_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_natural_distance2() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_natural_distance2() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "natural_distance2"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_natural_distance2();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_natural_distance2add() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_natural_distance2add() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "natural_distance2add"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_natural_distance2add();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_natural_distance2sub() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_natural_distance2sub() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "natural_distance2sub"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_natural_distance2sub();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_natural_distance2mul() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_natural_distance2mul() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "natural_distance2mul"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_natural_distance2mul();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_2_avx2_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_avx2_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "avx2", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_avx2_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_2_scalar_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_scalar_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "scalar", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_scalar_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_2_sse2_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse2_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse2", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse2_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_2_sse41_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_2_sse41_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "2d", "sse41", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_2_sse41_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_cellular2_3_avx2_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_avx2_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "avx2", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_avx2_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_cellular2_3_scalar_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_scalar_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "scalar", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_scalar_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_cellular2_3_sse2_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse2_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse2", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse2_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_cellular2_3_sse41_32_natural_distance2div() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_cellular2_3_sse41_32_natural_distance2div() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "cellular2", "32", "3d", "sse41", "natural_distance2div"
    );
    unsafe {
        let noise = do_intrinsic_cellular2_3_sse41_32_natural_distance2div();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_1_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_1d_noise::<simdeez::Avx2>(&noise_type);
    noise
}
#[test]
fn test_intrinsic_ridge_1_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_1_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_1d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_1_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_1d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_1_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_1d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_1_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_1d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_1_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_1d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_1_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_1d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_1_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_1d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_1_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_1_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_2_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_2_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_2d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_2_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_2_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_2d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_2_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_2_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_2d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[ignore]
fn test_intrinsic_ridge_2_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_2_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_2_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_2d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_2_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_2_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_3_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_3_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_3d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_ridge_3_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_3_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_3_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_3_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_3d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_ridge_3_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_3_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_3_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_3_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_3d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_ridge_3_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_3_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_3_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_3_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_3d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_ridge_3_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_3_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_4_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_4d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_ridge_4_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_4d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_4_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_4d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_ridge_4_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_4d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_4_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_4d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_ridge_4_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_4d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_4_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_4d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "32", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_ridge_4_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_4d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_ridge_4_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "ridge", "64", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_ridge_4_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_1_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_1d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_1_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_1d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_1_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_1d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_1_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_1d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_1_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_1d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_1_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_1d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_1_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_1d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_1_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_1d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_1_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_1_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}
#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_2_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_2_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_2d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_2_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_2_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_2d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_2_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_2_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_2d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[ignore]
fn test_intrinsic_fbm_2_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_2_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_2_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_2d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_2_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_2_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_3_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_3_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_3d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_fbm_3_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_3_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_3_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_3_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_3d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_fbm_3_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_3_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_3_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_3_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_3d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_fbm_3_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_3_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_3_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_3_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_3d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_fbm_3_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_3_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_4_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_4d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_fbm_4_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_4d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_4_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_4d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_fbm_4_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_4d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_4_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_4d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_fbm_4_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_4d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_4_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_4d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "32", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_fbm_4_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_4d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_fbm_4_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "fbm", "64", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_fbm_4_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_1_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_1d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_1_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_1d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_1_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_1d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_1_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_1d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_1_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_1d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_1_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_1d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_1_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_1d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_1_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_1d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_1_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_1_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_2_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_2_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_2d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_2_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_2_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_2d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_2_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_2_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_2d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[ignore]
fn test_intrinsic_turbulence_2_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_2_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_2_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_2d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_2_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_2_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_3_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_3_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_3d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_turbulence_3_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_3_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_3_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_3_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_3d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_turbulence_3_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_3_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_3_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_3_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_3d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_turbulence_3_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_3_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_3_sse41_2_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_3_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_3d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_turbulence_3_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_3_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_4_avx2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = avx2::get_4d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_turbulence_4_avx2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = avx2::get_4d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_4_scalar_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = scalar::get_4d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_turbulence_4_scalar_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = scalar::get_4d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_4_sse2_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse2::get_4d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_turbulence_4_sse2_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse2::get_4d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_4_sse41_32_normal() -> Vec<f32> {
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
    let (noise, _min, _max) = sse41::get_4d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "32", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_turbulence_4_sse41_64_normal() -> Vec<f64> {
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
    let (noise, _min, _max) = sse41::get_4d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_turbulence_4_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "turbulence", "64", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_turbulence_4_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_1_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_1d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_1_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_1d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "1d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_1_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_1d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_1_scalar_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_1d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "1d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_1_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_1d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_1_sse2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_1d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "1d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_1_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_1d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_1_sse41_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        ..NoiseDimensions::default(1)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_1d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_1_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "1d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_1_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_2_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_2d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_2_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_2d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "2d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_2_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_2d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_2_scalar_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_2d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "2d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_2_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_2d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_2_sse2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_2d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[ignore]
fn test_intrinsic_gradient_2_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "2d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_2_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_2d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_2_sse41_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        ..NoiseDimensions::default(2)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_2d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_2_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "2d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_2_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_3_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_3d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_3_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_3_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_3d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_gradient_3_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "3d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_3_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_3d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_3_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_3_scalar_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_3d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_gradient_3_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "3d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_3_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_3d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_3_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_3_sse2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_3d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_gradient_3_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "3d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_3_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_3d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_3_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_3_sse41_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        ..NoiseDimensions::default(3)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_3d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
#[should_panic(expected = "not implemented")]
fn test_intrinsic_gradient_3_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "3d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_3_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_4_avx2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_4d_noise::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_avx2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_avx2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "avx2")]
unsafe fn do_intrinsic_gradient_4_avx2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = avx2::get_4d_noise_64::<simdeez::Avx2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_avx2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "4d", "avx2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_avx2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_4_scalar_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_4d_noise::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_scalar_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_scalar_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

unsafe fn do_intrinsic_gradient_4_scalar_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = scalar::get_4d_noise_64::<simdeez::scalar::Scalar>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_scalar_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "4d", "scalar", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_scalar_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_4_sse2_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_4d_noise::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_sse2_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_sse2_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse2")]
unsafe fn do_intrinsic_gradient_4_sse2_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse2::get_4d_noise_64::<simdeez::Sse2>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_sse2_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "4d", "sse2", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_sse2_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_4_sse41_32_normal() -> Vec<f32> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_4d_noise::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_sse41_32_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "32", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_sse41_32_normal();
        //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f32(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}

#[target_feature(enable = "sse4.1")]
unsafe fn do_intrinsic_gradient_4_sse41_64_normal() -> Vec<f64> {
    let dims = NoiseDimensions {
        width: 64,
        height: 32,
        depth: 16,
        time: 8,
        ..NoiseDimensions::default(4)
    };

    let noise_type = GradientSettings::default(dims).with_seed(1337).wrap();
    let (noise, _min, _max) = sse41::get_4d_noise_64::<simdeez::Sse41>(&noise_type);
    noise
}

#[test]
fn test_intrinsic_gradient_4_sse41_64_normal() {
    let file_name = format!(
        "{}/{}_{}_{}_{}_{}_{}.bin",
        BIN_PATH, "intrinsics", "gradient", "64", "4d", "sse41", "normal"
    );
    unsafe {
        let noise = do_intrinsic_gradient_4_sse41_64_normal();
        //save_to_file_f64(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f64(&file_name).unwrap();
        assert_eq!(expected, noise);
    }
}
