use simdnoise::{NoiseBuilder, Settings, SimplexSettings};

mod helpers;
use helpers::{read_from_file_f32, /*save_to_file_f32,*/ BIN_PATH};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
const arch: &'static str = "x86";
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
const arch: &'static str = "neon";

mod noise {
    use super::*;

    // Cellular noise is the only noise type that uses `rsqrt` (see src/noise/cell_32.rs).
    // `rsqrt` is an approximate instruction whose result is backend-specific: x86 uses the
    // hardware `rsqrtps` approximation, aarch64 NEON uses `vrsqrteq_f32`, and the scalar
    // backend computes an exact `1.0/sqrt`. The stored noisebuilder cellular fixtures were
    // generated on x86 and therefore encode the x86 `rsqrtps` output. `generate()` dispatches
    // at runtime via simdeez, which picks NEON on aarch64, so byte-exact comparison against the
    // x86 fixtures cannot pass off x86. (Every non-cellular noisebuilder case, which avoids
    // rsqrt, does match NEON byte-for-byte.) The scalar cellular algorithm itself is still
    // covered byte-exactly by tests/intrinsics.rs against dedicated scalar fixtures. So these
    // byte-exact cross-arch regressions are gated to x86, matching the rest of the branch.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    mod cellular {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_cellular_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "nooffset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "nooffset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "offset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular", "offset", "32", "3d", arch
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
    // See the comment on `mod cellular`: cellular2 noise also relies on the approximate `rsqrt`,
    // so its byte-exact fixtures are x86-specific and the tests are gated to x86.
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    mod cellular2 {
        use super::*;
        mod f32 {
            use super::*;

            mod nooffset {
                use super::*;

                #[test]
                fn test_noisebuilder_cellular2_nooffset_f32_2d() {
                    let file_name = format!(
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "nooffset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "nooffset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "offset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "cellular2", "offset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "nooffset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "ridge", "offset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "nooffset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "turbulence", "offset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "nooffset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "fbm", "offset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "nooffset", "32", "4d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "1d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "2d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "3d", arch
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
                        "{}/{}_{}_{}_{}_{}_{}.bin",
                        BIN_PATH, "noisebuilder", "gradient", "offset", "32", "4d", arch
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
