use simdnoise::{NoiseBuilder, Settings};

mod helpers;
use helpers::{read_from_file_f32, save_to_file_f32, BIN_PATH};

mod noise {
    use super::*;

    mod cellular {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_cellular_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::cellular_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_cellular_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "nooffset", "32", "3d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::cellular_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_cellular_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "offset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::cellular_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_cellular_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::cellular_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
    mod cellular2 {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_cellular2_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::cellular2_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_cellular2_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "nooffset", "32", "3d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::cellular2_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_cellular2_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "offset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::cellular2_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_cellular2_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::cellular2_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
    mod ridge {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_ridge_nooffset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::ridge_1d(64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_ridge_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::ridge_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_ridge_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "3d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::ridge_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_ridge_nooffset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "4d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::ridge_4d(64, 32, 16, 8)
                        .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_ridge_offset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::ridge_1d_offset(16.0, 64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_ridge_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::ridge_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_ridge_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::ridge_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_ridge_offset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "4d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::ridge_4d_offset(16.0, 64, 32.0, 32, 64.0, 16, 128.0, 8)
                            .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
    mod turbulence {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_turbulence_nooffset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::turbulence_1d(64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_turbulence_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::turbulence_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_turbulence_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "3d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::turbulence_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_turbulence_nooffset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "4d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::turbulence_4d(64, 32, 16, 8)
                        .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_turbulence_offset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::turbulence_1d_offset(16.0, 64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_turbulence_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "2d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::turbulence_2d_offset(16.0, 64, 32.0, 32)
                            .with_freq_2d(0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_turbulence_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::turbulence_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_turbulence_offset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "4d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::turbulence_4d_offset(16.0, 64, 32.0, 32, 64.0, 16, 128.0, 8)
                            .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
    mod fbm {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_1d(64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "3d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_nooffset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "4d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_4d(64, 32, 16, 8)
                        .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_fbm_offset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_1d_offset(16.0, 64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::fbm_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .with_octaves(5)
                        .with_gain(2.0)
                        .with_lacunarity(0.5)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::fbm_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_fbm_offset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "4d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::fbm_4d_offset(16.0, 648, 32.0, 32, 64.0, 16, 128.0, 8)
                            .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .with_octaves(5)
                            .with_gain(2.0)
                            .with_lacunarity(0.5)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
    mod gradient {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::gradient_1d(64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::gradient_2d(64, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "3d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::gradient_3d(64, 32, 16)
                        .with_freq_3d(0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_nooffset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "4d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::gradient_4d(64, 32, 16, 8)
                        .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }

            mod offset {
                use super::*;

                #[test]
                fn test_noisebuilder_gradient_offset_f32_1d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "1d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::gradient_1d_offset(16.0, 64)
                        .with_freq(0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_offset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "2d"
                    );
                    let (noise, _min, _max) = NoiseBuilder::gradient_2d_offset(16.0, 64, 32.0, 32)
                        .with_freq_2d(0.04, 0.01)
                        .with_seed(1337)
                        .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_offset_f32_3d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "3d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::gradient_3d_offset(16.0, 64, 32.0, 32, 64.0, 16)
                            .with_freq_3d(0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
                #[test]
                fn test_noisebuilder_gradient_offset_f32_4d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "4d"
                    );
                    let (noise, _min, _max) =
                        NoiseBuilder::gradient_4d_offset(16.0, 64, 32.0, 32, 64.0, 16, 128.0, 8)
                            .with_freq_4d(0.10, 0.05, 0.04, 0.01)
                            .with_seed(1337)
                            .generate();

                    //save_to_file_f32(&file_name, noise.as_slice()).unwrap();
                    let expected = read_from_file_f32(&file_name).unwrap();
                    assert_eq!(expected, noise);
                }
            }
        }
    }
}
