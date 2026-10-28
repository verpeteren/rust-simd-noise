use simdnoise::point::{
    sample_points_2d, sample_points_2d_with_gradient, sample_points_2d_with_hessian,
    sample_points_3d, sample_points_3d_f64, sample_points_3d_with_gradient,
    sample_points_3d_with_gradient_f64, sample_points_3d_with_hessian,
    sample_points_3d_with_hessian_f64, sample_points_4d, sample_points_4d_f64,
    sample_points_4d_with_gradient, sample_points_4d_with_gradient_f64,
    sample_points_4d_with_hessian, sample_points_4d_with_hessian_f64, PointSample2, PointSample3,
    PointSample3F64, PointSample4, PointSample4F64, PointSampleHessian2, PointSampleHessian3,
    PointSampleHessian3F64, PointSampleHessian4, PointSampleHessian4F64,
};
use simdnoise::{Cell2ReturnType, NoiseBuilder, Settings};

const P: [f32; 3] = [3.25, -1.5, 0.75];
const P2: [f32; 2] = [3.25, -1.5];

fn close(a: f32, b: f32, tolerance: f32) {
    assert!(
        (a - b).abs() <= tolerance,
        "values differ: {} vs {} (tolerance {})",
        a,
        b,
        tolerance
    );
}

trait BlockGradient {
    fn block_gradient(self) -> simdnoise::NoiseAndGradient;
}

macro_rules! impl_block_gradient {
    ($($settings:ty),* $(,)?) => {
        $(
            impl BlockGradient for $settings {
                fn block_gradient(self) -> simdnoise::NoiseAndGradient {
                    self.generate_with_gradient()
                }
            }
        )*
    };
}

impl_block_gradient!(
    simdnoise::GradientSettings,
    simdnoise::FbmSettings,
    simdnoise::RidgeSettings,
    simdnoise::TurbulenceSettings,
    simdnoise::CellularSettings,
    simdnoise::Cellular2Settings,
);

fn check_3d<T: Settings + BlockGradient + Copy>(settings: T, point: [f32; 3]) {
    let noise_type = settings.wrap();
    let mut sample = [PointSample3::default(); 1];
    sample_points_3d_with_gradient(&noise_type, &[point], &mut sample);
    let mut value = [0.0f32; 1];
    sample_points_3d(&noise_type, &[point], &mut value);
    close(value[0], sample[0].value, 1.0e-6);
    let block = settings.block_gradient();
    close(sample[0].value, block.values[0], 1.0e-5);
    for axis in 0..3 {
        close(sample[0].gradient[axis], block.gradients[axis], 1.0e-5);
    }
}

fn check_2d<T: Settings + BlockGradient + Copy>(settings: T, point: [f32; 2]) {
    let noise_type = settings.wrap();
    let mut sample = [PointSample2::default(); 1];
    sample_points_2d_with_gradient(&noise_type, &[point], &mut sample);
    let mut value = [0.0f32; 1];
    sample_points_2d(&noise_type, &[point], &mut value);
    close(value[0], sample[0].value, 1.0e-6);
    let block = settings.block_gradient();
    close(sample[0].value, block.values[0], 1.0e-5);
    for axis in 0..2 {
        close(sample[0].gradient[axis], block.gradients[axis], 1.0e-5);
    }
}

trait BlockGradient64 {
    fn block_gradient_64(self) -> simdnoise::NoiseAndGradientF64;
}

macro_rules! impl_block_gradient_64 {
    ($($settings:ty),* $(,)?) => {
        $(
            impl BlockGradient64 for $settings {
                fn block_gradient_64(self) -> simdnoise::NoiseAndGradientF64 {
                    self.generate_with_gradient_f64()
                }
            }
        )*
    };
}

impl_block_gradient_64!(
    simdnoise::GradientSettings,
    simdnoise::FbmSettings,
    simdnoise::RidgeSettings,
    simdnoise::TurbulenceSettings,
    simdnoise::CellularSettings,
    simdnoise::Cellular2Settings,
);

fn check_3d_64<T: Settings + BlockGradient64 + Copy>(settings: T, point: [f64; 3]) {
    let noise_type = settings.wrap();
    let mut sample = [PointSample3F64::default(); 1];
    sample_points_3d_with_gradient_f64(&noise_type, &[point], &mut sample);
    let mut value = [0.0f64; 1];
    sample_points_3d_f64(&noise_type, &[point], &mut value);
    assert!((value[0] - sample[0].value).abs() <= 1.0e-12);
    let block = settings.block_gradient_64();
    assert!((sample[0].value - block.values[0]).abs() <= 1.0e-9);
    for axis in 0..3 {
        assert!((sample[0].gradient[axis] - block.gradients[axis]).abs() <= 1.0e-9);
    }
}

trait BlockGradient4 {
    fn block_gradient_4d(self) -> simdnoise::NoiseAndGradient;
}

impl BlockGradient4 for simdnoise::GradientSettings {
    fn block_gradient_4d(self) -> simdnoise::NoiseAndGradient {
        simdnoise::generate_4d_gradient(&self)
    }
}
impl BlockGradient4 for simdnoise::FbmSettings {
    fn block_gradient_4d(self) -> simdnoise::NoiseAndGradient {
        simdnoise::generate_4d_fbm_gradient(&self)
    }
}
impl BlockGradient4 for simdnoise::RidgeSettings {
    fn block_gradient_4d(self) -> simdnoise::NoiseAndGradient {
        simdnoise::generate_4d_ridge_gradient(&self)
    }
}
impl BlockGradient4 for simdnoise::TurbulenceSettings {
    fn block_gradient_4d(self) -> simdnoise::NoiseAndGradient {
        simdnoise::generate_4d_turbulence_gradient(&self)
    }
}

fn check_4d_values<T: Settings + Copy>(settings: T, point: [f32; 4]) {
    let block = settings.generate();
    let noise_type = settings.wrap();
    let mut value = [0.0f32; 1];
    sample_points_4d(&noise_type, &[point], &mut value);
    close(value[0], block.0[0], 1.0e-5);
}

fn check_4d_gradient<T: Settings + BlockGradient4 + Copy>(settings: T, point: [f32; 4]) {
    let noise_type = settings.wrap();
    let mut sample = [PointSample4::default(); 1];
    sample_points_4d_with_gradient(&noise_type, &[point], &mut sample);
    let mut value = [0.0f32; 1];
    sample_points_4d(&noise_type, &[point], &mut value);
    close(value[0], sample[0].value, 1.0e-6);
    let block = settings.block_gradient_4d();
    close(sample[0].value, block.values[0], 1.0e-5);
    for axis in 0..4 {
        close(sample[0].gradient[axis], block.gradients[axis], 1.0e-5);
    }
}

trait BlockGradient4F64 {
    fn block_gradient_4d_f64(self) -> simdnoise::NoiseAndGradientF64;
}

impl BlockGradient4F64 for simdnoise::GradientSettings {
    fn block_gradient_4d_f64(self) -> simdnoise::NoiseAndGradientF64 {
        simdnoise::generate_4d_gradient_f64(&self)
    }
}
impl BlockGradient4F64 for simdnoise::FbmSettings {
    fn block_gradient_4d_f64(self) -> simdnoise::NoiseAndGradientF64 {
        simdnoise::generate_4d_fbm_gradient_f64(&self)
    }
}
impl BlockGradient4F64 for simdnoise::RidgeSettings {
    fn block_gradient_4d_f64(self) -> simdnoise::NoiseAndGradientF64 {
        simdnoise::generate_4d_ridge_gradient_f64(&self)
    }
}
impl BlockGradient4F64 for simdnoise::TurbulenceSettings {
    fn block_gradient_4d_f64(self) -> simdnoise::NoiseAndGradientF64 {
        simdnoise::generate_4d_turbulence_gradient_f64(&self)
    }
}

fn check_4d_values_64<T: Settings + Copy>(settings: T, point: [f64; 4]) {
    let block = settings.generate_f64();
    let noise_type = settings.wrap();
    let mut value = [0.0f64; 1];
    sample_points_4d_f64(&noise_type, &[point], &mut value);
    assert!((value[0] - block.0[0]).abs() <= 1.0e-9);
}

fn check_4d_gradient_64<T: Settings + BlockGradient4F64 + Copy>(settings: T, point: [f64; 4]) {
    let noise_type = settings.wrap();
    let mut sample = [PointSample4F64::default(); 1];
    sample_points_4d_with_gradient_f64(&noise_type, &[point], &mut sample);
    let mut value = [0.0f64; 1];
    sample_points_4d_f64(&noise_type, &[point], &mut value);
    assert!((value[0] - sample[0].value).abs() <= 1.0e-12);
    let block = settings.block_gradient_4d_f64();
    assert!((sample[0].value - block.values[0]).abs() <= 1.0e-9);
    for axis in 0..4 {
        assert!((sample[0].gradient[axis] - block.gradients[axis]).abs() <= 1.0e-9);
    }
}

fn with_3d(mut settings: simdnoise::GradientSettings) -> simdnoise::GradientSettings {
    settings.with_seed(42).with_frequency_3d(0.02, 0.03, 0.04);
    settings
}

#[test]
fn gradient_3d_matches_block() {
    let settings = with_3d(NoiseBuilder::gradient_3d_offset(P[0], 1, P[1], 1, P[2], 1));
    check_3d(settings, P);
}

#[test]
fn fbm_3d_matches_block() {
    let mut settings = NoiseBuilder::fbm_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(7).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d(settings, P);
}

#[test]
fn ridge_3d_matches_block() {
    let mut settings = NoiseBuilder::ridge_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(11).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d(settings, P);
}

#[test]
fn turbulence_3d_matches_block() {
    let mut settings = NoiseBuilder::turbulence_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(13).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d(settings, P);
}

#[test]
fn cellular_3d_matches_block() {
    let mut settings = NoiseBuilder::cellular_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(17).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d(settings, P);
}

#[test]
fn cellular2_3d_matches_block() {
    for return_type in [
        Cell2ReturnType::Distance2,
        Cell2ReturnType::Distance2Add,
        Cell2ReturnType::Distance2Sub,
        Cell2ReturnType::Distance2Mul,
        Cell2ReturnType::Distance2Div,
    ] {
        let mut settings = NoiseBuilder::cellular2_3d_offset(P[0], 1, P[1], 1, P[2], 1);
        settings.with_seed(19).with_frequency_3d(0.02, 0.03, 0.04);
        settings.return_type = return_type;
        check_3d(settings, P);
    }
}

#[test]
fn gradient_2d_matches_block() {
    let mut settings = NoiseBuilder::gradient_2d_offset(P2[0], 1, P2[1], 1);
    settings.with_seed(23).with_frequency_2d(0.02, 0.03);
    check_2d(settings, P2);
}

#[test]
fn fbm_2d_matches_block() {
    let mut settings = NoiseBuilder::fbm_2d_offset(P2[0], 1, P2[1], 1);
    settings.with_seed(53).with_frequency_2d(0.02, 0.03);
    check_2d(settings, P2);
}

#[test]
fn ridge_2d_matches_block() {
    let mut settings = NoiseBuilder::ridge_2d_offset(P2[0], 1, P2[1], 1);
    settings.with_seed(59).with_frequency_2d(0.02, 0.03);
    check_2d(settings, P2);
}

#[test]
fn turbulence_2d_matches_block() {
    let mut settings = NoiseBuilder::turbulence_2d_offset(P2[0], 1, P2[1], 1);
    settings.with_seed(61).with_frequency_2d(0.02, 0.03);
    check_2d(settings, P2);
}

#[test]
fn cellular_2d_matches_block() {
    let mut settings = NoiseBuilder::cellular_2d_offset(P2[0], 1, P2[1], 1);
    settings.with_seed(67).with_frequency_2d(0.02, 0.03);
    check_2d(settings, P2);
}

#[test]
fn cellular2_2d_matches_block() {
    for return_type in [
        Cell2ReturnType::Distance2,
        Cell2ReturnType::Distance2Add,
        Cell2ReturnType::Distance2Sub,
        Cell2ReturnType::Distance2Mul,
        Cell2ReturnType::Distance2Div,
    ] {
        let mut settings = NoiseBuilder::cellular2_2d_offset(P2[0], 1, P2[1], 1);
        settings.with_seed(71).with_frequency_2d(0.02, 0.03);
        settings.return_type = return_type;
        check_2d(settings, P2);
    }
}

#[test]
fn fbm_3d_64_matches_block() {
    let mut settings = NoiseBuilder::fbm_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(73).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d_64(settings, [P[0] as f64, P[1] as f64, P[2] as f64]);
}

#[test]
fn ridge_3d_64_matches_block() {
    let mut settings = NoiseBuilder::ridge_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(79).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d_64(settings, [P[0] as f64, P[1] as f64, P[2] as f64]);
}

#[test]
fn turbulence_3d_64_matches_block() {
    let mut settings = NoiseBuilder::turbulence_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(83).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d_64(settings, [P[0] as f64, P[1] as f64, P[2] as f64]);
}

#[test]
fn cellular_3d_64_matches_block() {
    let mut settings = NoiseBuilder::cellular_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(89).with_frequency_3d(0.02, 0.03, 0.04);
    check_3d_64(settings, [P[0] as f64, P[1] as f64, P[2] as f64]);
}

#[test]
fn cellular2_3d_64_matches_block() {
    for return_type in [
        Cell2ReturnType::Distance2,
        Cell2ReturnType::Distance2Add,
        Cell2ReturnType::Distance2Sub,
        Cell2ReturnType::Distance2Mul,
        Cell2ReturnType::Distance2Div,
    ] {
        let mut settings = NoiseBuilder::cellular2_3d_offset(P[0], 1, P[1], 1, P[2], 1);
        settings.with_seed(97).with_frequency_3d(0.02, 0.03, 0.04);
        settings.return_type = return_type;
        check_3d_64(settings, [P[0] as f64, P[1] as f64, P[2] as f64]);
    }
}

#[test]
fn arbitrary_points_match_one_point_blocks() {
    let points = [
        [0.0, 0.0, 0.0],
        [0.125, -0.25, 3.5],
        [-7.75, 2.5, 0.125],
        [100.0, -50.0, 12.5],
        [0.0, 5.0, -0.0],
    ];
    let mut settings = NoiseBuilder::gradient_3d(1, 1, 1);
    settings.with_seed(29).with_frequency_3d(0.02, 0.03, 0.04);
    let mut samples = [PointSample3::default(); 5];
    sample_points_3d_with_gradient(&settings.wrap(), &points, &mut samples);

    for (point, sample) in points.iter().zip(samples.iter()) {
        let mut block_settings =
            NoiseBuilder::gradient_3d_offset(point[0], 1, point[1], 1, point[2], 1);
        block_settings
            .with_seed(29)
            .with_frequency_3d(0.02, 0.03, 0.04);
        let block = block_settings.generate_with_gradient();
        close(sample.value, block.values[0], 1.0e-5);
        for axis in 0..3 {
            close(sample.gradient[axis], block.gradients[axis], 1.0e-5);
        }
    }
}

#[test]
fn gradient_3d_64_matches_block() {
    let mut settings = NoiseBuilder::gradient_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(31).with_frequency_3d(0.02, 0.03, 0.04);
    let mut sample = [PointSample3F64::default(); 1];
    sample_points_3d_with_gradient_f64(
        &settings.wrap(),
        &[[P[0] as f64, P[1] as f64, P[2] as f64]],
        &mut sample,
    );
    let block = settings.generate_with_gradient_f64();
    assert!((sample[0].value - block.values[0]).abs() <= 1.0e-12);
    for axis in 0..3 {
        assert!((sample[0].gradient[axis] - block.gradients[axis]).abs() <= 1.0e-12);
    }
}

#[test]
fn hessian_matches_gradient_difference() {
    let mut settings = NoiseBuilder::gradient_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(37).with_frequency_3d(0.02, 0.03, 0.04);
    let noise_type = settings.wrap();
    let mut hessian = [PointSampleHessian3::default(); 1];
    sample_points_3d_with_hessian(&noise_type, &[P], &mut hessian);

    let step = 1.0e-4;
    let mut plus = [PointSample3::default(); 1];
    let mut minus = [PointSample3::default(); 1];
    for axis in 0..3 {
        let mut p_plus = P;
        let mut p_minus = P;
        p_plus[axis] += step;
        p_minus[axis] -= step;
        sample_points_3d_with_gradient(&noise_type, &[p_plus], &mut plus);
        sample_points_3d_with_gradient(&noise_type, &[p_minus], &mut minus);
        for row in 0..3 {
            let difference = (plus[0].gradient[row] - minus[0].gradient[row]) / (2.0 * step);
            close(hessian[0].hessian[row][axis], difference, 5.0e-2);
        }
    }

    for row in 0..3 {
        for column in 0..3 {
            close(
                hessian[0].hessian[row][column],
                hessian[0].hessian[column][row],
                5.0e-2,
            );
        }
    }
}

#[test]
fn hessian_2d_matches_gradient_difference() {
    let mut settings = NoiseBuilder::gradient_2d_offset(P2[0], 1, P2[1], 1);
    settings.with_seed(101).with_frequency_2d(0.02, 0.03);
    let noise_type = settings.wrap();
    let mut hessian = [PointSampleHessian2::default(); 1];
    sample_points_2d_with_hessian(&noise_type, &[P2], &mut hessian);

    let step = 1.0e-4;
    let mut plus = [PointSample2::default(); 1];
    let mut minus = [PointSample2::default(); 1];
    for axis in 0..2 {
        let mut p_plus = P2;
        let mut p_minus = P2;
        p_plus[axis] += step;
        p_minus[axis] -= step;
        sample_points_2d_with_gradient(&noise_type, &[p_plus], &mut plus);
        sample_points_2d_with_gradient(&noise_type, &[p_minus], &mut minus);
        for row in 0..2 {
            let difference = (plus[0].gradient[row] - minus[0].gradient[row]) / (2.0 * step);
            close(hessian[0].hessian[row][axis], difference, 5.0e-2);
        }
    }
}

#[test]
fn hessian_3d_64_matches_gradient_difference() {
    let mut settings = NoiseBuilder::gradient_3d_offset(P[0], 1, P[1], 1, P[2], 1);
    settings.with_seed(103).with_frequency_3d(0.02, 0.03, 0.04);
    let noise_type = settings.wrap();
    let point = [P[0] as f64, P[1] as f64, P[2] as f64];
    let mut hessian = [PointSampleHessian3F64::default(); 1];
    sample_points_3d_with_hessian_f64(&noise_type, &[point], &mut hessian);

    let step = 1.0e-5;
    let mut plus = [PointSample3F64::default(); 1];
    let mut minus = [PointSample3F64::default(); 1];
    for axis in 0..3 {
        let mut p_plus = point;
        let mut p_minus = point;
        p_plus[axis] += step;
        p_minus[axis] -= step;
        sample_points_3d_with_gradient_f64(&noise_type, &[p_plus], &mut plus);
        sample_points_3d_with_gradient_f64(&noise_type, &[p_minus], &mut minus);
        for row in 0..3 {
            let difference = (plus[0].gradient[row] - minus[0].gradient[row]) / (2.0 * step);
            assert!((hessian[0].hessian[row][axis] - difference).abs() <= 5.0e-2);
        }
    }
}

#[test]
fn empty_batch_is_a_no_op() {
    let mut settings = NoiseBuilder::gradient_3d(1, 1, 1);
    settings.with_seed(41);
    let noise_type = settings.wrap();
    let mut samples: [PointSample3; 0] = [];
    sample_points_3d_with_gradient(&noise_type, &[], &mut samples);
    sample_points_3d_with_hessian(&noise_type, &[], &mut []);
}

#[test]
#[should_panic(expected = "point and output lengths differ")]
fn length_mismatch_panics() {
    let mut settings = NoiseBuilder::gradient_3d(1, 1, 1);
    settings.with_seed(43);
    let mut samples = [PointSample3::default(); 2];
    sample_points_3d_with_gradient(&settings.wrap(), &[P], &mut samples);
}

#[test]
#[should_panic(expected = "requires 3D settings")]
fn dimension_mismatch_panics() {
    let mut settings = NoiseBuilder::gradient_2d(1, 1);
    settings.with_seed(47);
    let mut samples = [PointSample3::default(); 1];
    sample_points_3d_with_gradient(&settings.wrap(), &[P], &mut samples);
}

const P4: [f32; 4] = [3.25, -1.5, 0.75, 0.5];
const P4_64: [f64; 4] = [3.25, -1.5, 0.75, 0.5];

#[test]
fn gradient_4d_matches_block() {
    let mut settings = NoiseBuilder::gradient_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(71)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient(settings, P4);
}

#[test]
fn fbm_4d_matches_block() {
    let mut settings = NoiseBuilder::fbm_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(73)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient(settings, P4);
}

#[test]
fn ridge_4d_matches_block() {
    let mut settings = NoiseBuilder::ridge_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(79)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient(settings, P4);
}

#[test]
fn turbulence_4d_matches_block() {
    let mut settings = NoiseBuilder::turbulence_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(83)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient(settings, P4);
}

#[test]
fn cellular_4d_values_match_block() {
    let mut settings = NoiseBuilder::cellular_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(89)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_values(settings, P4);
}

#[test]
fn cellular2_4d_values_match_block() {
    for return_type in [
        Cell2ReturnType::Distance2,
        Cell2ReturnType::Distance2Add,
        Cell2ReturnType::Distance2Sub,
        Cell2ReturnType::Distance2Mul,
        Cell2ReturnType::Distance2Div,
    ] {
        let mut settings =
            NoiseBuilder::cellular2_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
        settings
            .with_seed(97)
            .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
        settings.return_type = return_type;
        check_4d_values(settings, P4);
    }
}

#[test]
#[should_panic(expected = "4D cellular point gradients are not implemented")]
fn cellular_4d_gradient_panics() {
    let mut settings = NoiseBuilder::cellular_4d(1, 1, 1, 1);
    settings.with_seed(101);
    let mut samples = [PointSample4::default(); 1];
    sample_points_4d_with_gradient(&settings.wrap(), &[P4], &mut samples);
}

#[test]
fn gradient_4d_64_matches_block() {
    let mut settings = NoiseBuilder::gradient_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(103)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient_64(settings, P4_64);
}

#[test]
fn fbm_4d_64_matches_block() {
    let mut settings = NoiseBuilder::fbm_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(107)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient_64(settings, P4_64);
}

#[test]
fn ridge_4d_64_matches_block() {
    let mut settings = NoiseBuilder::ridge_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(109)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient_64(settings, P4_64);
}

#[test]
fn turbulence_4d_64_matches_block() {
    let mut settings = NoiseBuilder::turbulence_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(113)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_gradient_64(settings, P4_64);
}

#[test]
fn cellular_4d_values_64_match_block() {
    let mut settings = NoiseBuilder::cellular_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(127)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_values_64(settings, P4_64);
}

#[test]
fn cellular2_4d_values_64_match_block() {
    let mut settings = NoiseBuilder::cellular2_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(131)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    check_4d_values_64(settings, P4_64);
}

#[test]
fn hessian_4d_matches_gradient_difference() {
    let mut settings = NoiseBuilder::gradient_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(137)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    let noise_type = settings.wrap();
    let mut hessian = [PointSampleHessian4::default(); 1];
    sample_points_4d_with_hessian(&noise_type, &[P4], &mut hessian);

    let step = 1.0e-5;
    let mut plus = [PointSample4::default(); 1];
    let mut minus = [PointSample4::default(); 1];
    for axis in 0..4 {
        let mut p_plus = P4;
        let mut p_minus = P4;
        p_plus[axis] += step;
        p_minus[axis] -= step;
        sample_points_4d_with_gradient(&noise_type, &[p_plus], &mut plus);
        sample_points_4d_with_gradient(&noise_type, &[p_minus], &mut minus);
        for row in 0..4 {
            let difference = (plus[0].gradient[row] - minus[0].gradient[row]) / (2.0 * step);
            assert!((hessian[0].hessian[row][axis] - difference).abs() <= 5.0e-2);
        }
    }
}

#[test]
fn hessian_4d_64_matches_gradient_difference() {
    let mut settings = NoiseBuilder::gradient_4d_offset(P4[0], 1, P4[1], 1, P4[2], 1, P4[3], 1);
    settings
        .with_seed(139)
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05);
    let noise_type = settings.wrap();
    let mut hessian = [PointSampleHessian4F64::default(); 1];
    sample_points_4d_with_hessian_f64(&noise_type, &[P4_64], &mut hessian);

    let step = 1.0e-5;
    let mut plus = [PointSample4F64::default(); 1];
    let mut minus = [PointSample4F64::default(); 1];
    for axis in 0..4 {
        let mut p_plus = P4_64;
        let mut p_minus = P4_64;
        p_plus[axis] += step;
        p_minus[axis] -= step;
        sample_points_4d_with_gradient_f64(&noise_type, &[p_plus], &mut plus);
        sample_points_4d_with_gradient_f64(&noise_type, &[p_minus], &mut minus);
        for row in 0..4 {
            let difference = (plus[0].gradient[row] - minus[0].gradient[row]) / (2.0 * step);
            assert!((hessian[0].hessian[row][axis] - difference).abs() <= 5.0e-2);
        }
    }
}
