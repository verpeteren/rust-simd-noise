//! Gabor noise: range, determinism and public API tests.

use simdnoise::{GaborSettings, NoiseBuilder, NoiseDimensions};

#[test]
fn gabor_2d_range() {
    let (values, min, max) = NoiseBuilder::gabor_2d(128, 128)
        .with_frequency(0.05)
        .with_seed(7)
        .with_gain(1.0)
        .generate();

    assert_eq!(values.len(), 128 * 128);
    assert!(values.iter().all(|value| value.is_finite()));
    assert!(
        values.iter().all(|value| value.abs() <= 1.01),
        "normalized Gabor 2D exceeded its guaranteed range: min {}, max {}",
        min,
        max
    );
    assert!(min <= 0.0 && max >= 0.0, "Gabor 2D should be signed");
}

#[test]
fn gabor_3d_range() {
    let (values, min, max) = NoiseBuilder::gabor_3d(24, 24, 24)
        .with_frequency(0.05)
        .with_seed(11)
        .with_gain(1.0)
        .generate();

    assert_eq!(values.len(), 24 * 24 * 24);
    assert!(values.iter().all(|value| value.is_finite()));
    assert!(
        values.iter().all(|value| value.abs() <= 1.01),
        "normalized Gabor 3D exceeded its guaranteed range: min {}, max {}",
        min,
        max
    );
}

#[test]
fn gabor_seed_changes_the_field() {
    let a = NoiseBuilder::gabor_2d(64, 64)
        .with_frequency(0.05)
        .with_seed(1)
        .generate()
        .0;
    let b = NoiseBuilder::gabor_2d(64, 64)
        .with_frequency(0.05)
        .with_seed(2)
        .generate()
        .0;

    assert!(
        a.iter().zip(&b).any(|(a, b)| (a - b).abs() > 1.0e-6),
        "different seeds produced the same field"
    );
}

#[test]
fn gabor_orientation_changes_the_field() {
    let x = NoiseBuilder::gabor_2d(64, 64)
        .with_frequency(0.05)
        .with_seed(3)
        .with_orientation(1.0, 0.0, 0.0)
        .generate()
        .0;
    let y = NoiseBuilder::gabor_2d(64, 64)
        .with_frequency(0.05)
        .with_seed(3)
        .with_orientation(0.0, 1.0, 0.0)
        .generate()
        .0;

    assert!(
        x.iter().zip(&y).any(|(x, y)| (x - y).abs() > 1.0e-6),
        "different orientations produced the same field"
    );
}

#[test]
fn gabor_scaled_range() {
    let values = NoiseBuilder::gabor_2d(32, 32)
        .with_frequency(0.05)
        .with_seed(5)
        .generate_scaled(0.0, 1.0);

    assert_eq!(values.len(), 32 * 32);
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    assert!((min - 0.0).abs() < 1.0e-5, "scaled min was {}", min);
    assert!((max - 1.0).abs() < 1.0e-5, "scaled max was {}", max);
}

#[test]
fn gabor_fill_matches_generate() {
    let mut settings = NoiseBuilder::gabor_2d(16, 16);
    settings.with_frequency(0.05).with_seed(9);
    let (expected, expected_min, expected_max) = settings.generate();
    let mut actual = vec![0.0; 16 * 16];
    let (actual_min, actual_max) = settings.fill(&mut actual);

    assert_eq!(actual, expected);
    assert!((actual_min - expected_min).abs() < 1.0e-6);
    assert!((actual_max - expected_max).abs() < 1.0e-6);
}

#[test]
#[should_panic(expected = "Gabor noise supports 2D, 3D and 4D only")]
fn gabor_rejects_1d() {
    let settings = GaborSettings::default(NoiseDimensions::default(1));
    let _ = settings.generate();
}

#[test]
#[should_panic(expected = "Gabor orientation must be finite and non-zero")]
fn gabor_rejects_zero_orientation() {
    let mut settings = NoiseBuilder::gabor_2d(8, 8);
    settings.with_orientation(0.0, 0.0, 0.0);
    let _ = settings.generate();
}

#[test]
fn gabor_2d_gradient_matches_finite_difference() {
    let mut settings = NoiseBuilder::gabor_2d(3, 3);
    settings
        .with_frequency(0.05)
        .with_seed(7)
        .with_bandwidth(2.0)
        .with_radius(2);
    let sampled = settings.generate_with_gradient();

    assert_eq!(sampled.stride, 2);
    assert_eq!(sampled.gradients.len(), sampled.values.len() * 2);

    let center = 4;
    let fd_x = (sampled.values[5] - sampled.values[3]) * 0.5;
    let fd_y = (sampled.values[7] - sampled.values[1]) * 0.5;
    let gx = sampled.gradients[center * 2];
    let gy = sampled.gradients[center * 2 + 1];

    assert!(
        (gx - fd_x).abs() < 0.05,
        "2D gx analytic {:.6} vs finite difference {:.6}",
        gx,
        fd_x
    );
    assert!(
        (gy - fd_y).abs() < 0.05,
        "2D gy analytic {:.6} vs finite difference {:.6}",
        gy,
        fd_y
    );
}

#[test]
fn gabor_3d_gradient_matches_finite_difference() {
    let mut settings = NoiseBuilder::gabor_3d(3, 3, 3);
    settings
        .with_frequency(0.05)
        .with_seed(7)
        .with_bandwidth(2.0)
        .with_radius(2);
    let sampled = settings.generate_with_gradient();

    assert_eq!(sampled.stride, 3);
    assert_eq!(sampled.gradients.len(), sampled.values.len() * 3);

    let center = 13;
    let fd_x = (sampled.values[14] - sampled.values[12]) * 0.5;
    let fd_y = (sampled.values[16] - sampled.values[10]) * 0.5;
    let fd_z = (sampled.values[22] - sampled.values[4]) * 0.5;
    let gx = sampled.gradients[center * 3];
    let gy = sampled.gradients[center * 3 + 1];
    let gz = sampled.gradients[center * 3 + 2];

    assert!(
        (gx - fd_x).abs() < 0.05,
        "3D gx analytic {:.6} vs finite difference {:.6}",
        gx,
        fd_x
    );
    assert!(
        (gy - fd_y).abs() < 0.05,
        "3D gy analytic {:.6} vs finite difference {:.6}",
        gy,
        fd_y
    );
    assert!(
        (gz - fd_z).abs() < 0.05,
        "3D gz analytic {:.6} vs finite difference {:.6}",
        gz,
        fd_z
    );
}

#[test]
fn gabor_gradient_values_match_generate() {
    let mut settings = NoiseBuilder::gabor_2d(8, 8);
    settings.with_frequency(0.07).with_seed(13);
    let (expected, _, _) = settings.generate();
    let sampled = settings.generate_with_gradient();

    assert_eq!(sampled.values.len(), expected.len());
    assert!(
        sampled
            .values
            .iter()
            .zip(&expected)
            .all(|(a, b)| (a - b).abs() < 1.0e-6),
        "gradient path changed the value output"
    );
}

#[test]
fn gabor_4d_range() {
    let (values, min, max) = NoiseBuilder::gabor_4d(8, 8, 8, 8)
        .with_frequency(0.05)
        .with_seed(17)
        .with_gain(1.0)
        .generate();

    assert_eq!(values.len(), 8 * 8 * 8 * 8);
    assert!(values.iter().all(|value| value.is_finite()));
    assert!(
        values.iter().all(|value| value.abs() <= 1.01),
        "normalized Gabor 4D exceeded its guaranteed range: min {}, max {}",
        min,
        max
    );
}

#[test]
fn gabor_4d_gradient_matches_finite_difference() {
    let mut settings = NoiseBuilder::gabor_4d(3, 3, 3, 3);
    settings
        .with_frequency(0.05)
        .with_seed(7)
        .with_bandwidth(2.0)
        .with_radius(2);
    let sampled = settings.generate_with_gradient();

    assert_eq!(sampled.stride, 4);
    assert_eq!(sampled.gradients.len(), sampled.values.len() * 4);

    // Index layout: (((w * 3 + z) * 3 + y) * 3 + x).
    let center = 40;
    let fd_x = (sampled.values[41] - sampled.values[39]) * 0.5;
    let fd_y = (sampled.values[43] - sampled.values[37]) * 0.5;
    let fd_z = (sampled.values[49] - sampled.values[31]) * 0.5;
    let fd_w = (sampled.values[67] - sampled.values[13]) * 0.5;
    let gx = sampled.gradients[center * 4];
    let gy = sampled.gradients[center * 4 + 1];
    let gz = sampled.gradients[center * 4 + 2];
    let gw = sampled.gradients[center * 4 + 3];

    assert!(
        (gx - fd_x).abs() < 0.05,
        "4D gx analytic {:.6} vs finite difference {:.6}",
        gx,
        fd_x
    );
    assert!(
        (gy - fd_y).abs() < 0.05,
        "4D gy analytic {:.6} vs finite difference {:.6}",
        gy,
        fd_y
    );
    assert!(
        (gz - fd_z).abs() < 0.05,
        "4D gz analytic {:.6} vs finite difference {:.6}",
        gz,
        fd_z
    );
    assert!(
        (gw - fd_w).abs() < 0.05,
        "4D gw analytic {:.6} vs finite difference {:.6}",
        gw,
        fd_w
    );
}

#[test]
fn gabor_2d_point_matches_block() {
    use simdnoise::point::{
        sample_gabor_points_2d, sample_gabor_points_2d_with_gradient, PointSample2,
    };

    let point = [3.25, -1.5];
    let mut settings = NoiseBuilder::gabor_2d_offset(point[0], 1, point[1], 1);
    settings
        .with_frequency_2d(0.02, 0.03)
        .with_seed(7)
        .with_bandwidth(2.0)
        .with_radius(2);

    let mut sample = [PointSample2::default(); 1];
    sample_gabor_points_2d_with_gradient(&settings, &[point], &mut sample);
    let mut value = [0.0f32; 1];
    sample_gabor_points_2d(&settings, &[point], &mut value);
    assert!((value[0] - sample[0].value).abs() < 1.0e-6);
    let block = settings.generate_with_gradient();

    assert!((sample[0].value - block.values[0]).abs() < 1.0e-6);
    assert!((sample[0].gradient[0] - block.gradients[0]).abs() < 1.0e-6);
    assert!((sample[0].gradient[1] - block.gradients[1]).abs() < 1.0e-6);
}

#[test]
fn gabor_3d_point_matches_block() {
    use simdnoise::point::{
        sample_gabor_points_3d, sample_gabor_points_3d_with_gradient, PointSample3,
    };

    let point = [3.25, -1.5, 0.75];
    let mut settings = NoiseBuilder::gabor_3d_offset(point[0], 1, point[1], 1, point[2], 1);
    settings
        .with_frequency_3d(0.02, 0.03, 0.04)
        .with_seed(11)
        .with_bandwidth(2.0)
        .with_radius(2);

    let mut sample = [PointSample3::default(); 1];
    sample_gabor_points_3d_with_gradient(&settings, &[point], &mut sample);
    let mut value = [0.0f32; 1];
    sample_gabor_points_3d(&settings, &[point], &mut value);
    assert!((value[0] - sample[0].value).abs() < 1.0e-6);
    let block = settings.generate_with_gradient();

    assert!((sample[0].value - block.values[0]).abs() < 1.0e-6);
    for axis in 0..3 {
        assert!((sample[0].gradient[axis] - block.gradients[axis]).abs() < 1.0e-6);
    }
}

#[test]
fn gabor_3d_arbitrary_points_match_blocks() {
    use simdnoise::point::{
        sample_gabor_points_3d, sample_gabor_points_3d_with_gradient, PointSample3,
    };

    let points = [[0.0, 0.0, 0.0], [0.125, -0.25, 3.5], [-7.75, 2.5, 0.125]];
    let mut settings = NoiseBuilder::gabor_3d(1, 1, 1);
    settings
        .with_frequency_3d(0.02, 0.03, 0.04)
        .with_seed(13)
        .with_bandwidth(2.0)
        .with_radius(2);

    let mut samples = [PointSample3::default(); 3];
    sample_gabor_points_3d_with_gradient(&settings, &points, &mut samples);
    let mut values = [0.0f32; 3];
    sample_gabor_points_3d(&settings, &points, &mut values);

    for (point, (sample, value)) in points.iter().zip(samples.iter().zip(values.iter())) {
        assert!((value - sample.value).abs() < 1.0e-6);
        let mut block_settings =
            NoiseBuilder::gabor_3d_offset(point[0], 1, point[1], 1, point[2], 1);
        block_settings
            .with_frequency_3d(0.02, 0.03, 0.04)
            .with_seed(13)
            .with_bandwidth(2.0)
            .with_radius(2);
        let block = block_settings.generate_with_gradient();
        assert!((sample.value - block.values[0]).abs() < 1.0e-6);
        for axis in 0..3 {
            assert!((sample.gradient[axis] - block.gradients[axis]).abs() < 1.0e-6);
        }
    }
}

#[test]
fn gabor_4d_point_matches_block() {
    use simdnoise::point::{
        sample_gabor_points_4d, sample_gabor_points_4d_with_gradient, PointSample4,
    };

    let point = [3.25, -1.5, 0.75, 0.125];
    let mut settings =
        NoiseBuilder::gabor_4d_offset(point[0], 1, point[1], 1, point[2], 1, point[3], 1);
    settings
        .with_frequency_4d(0.02, 0.03, 0.04, 0.05)
        .with_seed(17)
        .with_bandwidth(2.0)
        .with_radius(2);

    let mut sample = [PointSample4::default(); 1];
    sample_gabor_points_4d_with_gradient(&settings, &[point], &mut sample);
    let mut value = [0.0f32; 1];
    sample_gabor_points_4d(&settings, &[point], &mut value);
    assert!((value[0] - sample[0].value).abs() < 1.0e-6);
    let block = settings.generate_with_gradient();

    assert!((sample[0].value - block.values[0]).abs() < 1.0e-6);
    for axis in 0..4 {
        assert!((sample[0].gradient[axis] - block.gradients[axis]).abs() < 1.0e-6);
    }
}

#[test]
fn phasor_2d_matches_gabor_real_and_gradients() {
    use simdnoise::point::{
        sample_gabor_points_2d_with_gradient, sample_phasor_points_2d,
        sample_phasor_points_2d_with_gradient, PhasorSample2, PointSample2,
    };

    let point = [3.25, -1.5];
    let mut settings = NoiseBuilder::gabor_2d_offset(point[0], 1, point[1], 1);
    settings
        .with_frequency_2d(0.02, 0.03)
        .with_seed(7)
        .with_bandwidth(2.0)
        .with_radius(2);

    let mut phasor = [PhasorSample2::default(); 1];
    sample_phasor_points_2d_with_gradient(&settings, &[point], &mut phasor);
    let mut phasor_value = [simdnoise::point::PhasorValue2::default(); 1];
    sample_phasor_points_2d(&settings, &[point], &mut phasor_value);
    assert!((phasor_value[0].real - phasor[0].real).abs() < 1.0e-6);
    assert!((phasor_value[0].imag - phasor[0].imag).abs() < 1.0e-6);
    let mut gabor = [PointSample2::default(); 1];
    sample_gabor_points_2d_with_gradient(&settings, &[point], &mut gabor);

    assert!((phasor[0].real - gabor[0].value).abs() < 1.0e-6);
    assert!((phasor[0].real_gradient[0] - gabor[0].gradient[0]).abs() < 1.0e-6);
    assert!((phasor[0].real_gradient[1] - gabor[0].gradient[1]).abs() < 1.0e-6);
    assert!(phasor[0].imag.is_finite());
    assert!(phasor[0].modulus().is_finite());
    assert!(phasor[0].phase().is_finite());

    let step = 1.0e-4;
    let mut plus = [PhasorSample2::default(); 1];
    let mut minus = [PhasorSample2::default(); 1];
    for axis in 0..2 {
        let mut p_plus = point;
        let mut p_minus = point;
        p_plus[axis] += step;
        p_minus[axis] -= step;
        sample_phasor_points_2d_with_gradient(&settings, &[p_plus], &mut plus);
        sample_phasor_points_2d_with_gradient(&settings, &[p_minus], &mut minus);
        let real_difference = (plus[0].real - minus[0].real) / (2.0 * step);
        let imag_difference = (plus[0].imag - minus[0].imag) / (2.0 * step);
        assert!((phasor[0].real_gradient[axis] - real_difference).abs() < 1.0e-2);
        assert!((phasor[0].imag_gradient[axis] - imag_difference).abs() < 1.0e-2);
    }
}

#[test]
fn phasor_3d_real_matches_gabor() {
    use simdnoise::point::{
        sample_gabor_points_3d_with_gradient, sample_phasor_points_3d,
        sample_phasor_points_3d_with_gradient, PhasorSample3, PointSample3,
    };

    let point = [3.25, -1.5, 0.75];
    let mut settings = NoiseBuilder::gabor_3d_offset(point[0], 1, point[1], 1, point[2], 1);
    settings
        .with_frequency_3d(0.02, 0.03, 0.04)
        .with_seed(11)
        .with_bandwidth(2.0)
        .with_radius(2);

    let mut phasor = [PhasorSample3::default(); 1];
    sample_phasor_points_3d_with_gradient(&settings, &[point], &mut phasor);
    let mut phasor_value = [simdnoise::point::PhasorValue3::default(); 1];
    sample_phasor_points_3d(&settings, &[point], &mut phasor_value);
    assert!((phasor_value[0].real - phasor[0].real).abs() < 1.0e-6);
    assert!((phasor_value[0].imag - phasor[0].imag).abs() < 1.0e-6);
    let mut gabor = [PointSample3::default(); 1];
    sample_gabor_points_3d_with_gradient(&settings, &[point], &mut gabor);

    assert!((phasor[0].real - gabor[0].value).abs() < 1.0e-6);
    for axis in 0..3 {
        assert!((phasor[0].real_gradient[axis] - gabor[0].gradient[axis]).abs() < 1.0e-6);
        assert!(phasor[0].imag_gradient[axis].is_finite());
    }
    assert!(phasor[0].modulus().is_finite());
    assert!(phasor[0].phase().is_finite());
}
