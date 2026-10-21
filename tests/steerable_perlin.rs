//! Steerable Perlin: range, determinism, anisotropy and API validation.

use simdnoise::{NoiseDimensions, SteerablePerlinSettings};

fn settings2() -> SteerablePerlinSettings {
    let mut settings = SteerablePerlinSettings::default(NoiseDimensions::default(2));
    settings
        .with_frequency_2d(0.05, 0.03)
        .with_seed(7)
        .with_direction(1.0, 0.25, 0.0)
        .with_metric(4.0, 0.51)
        .with_gain(1.2);
    settings
}

fn settings3() -> SteerablePerlinSettings {
    let mut settings = SteerablePerlinSettings::default(NoiseDimensions::default(3));
    settings
        .with_frequency_3d(0.05, 0.03, 0.04)
        .with_seed(11)
        .with_direction(1.0, -0.25, 0.5)
        .with_metric(4.0, 0.51)
        .with_gain(1.1);
    settings
}

fn block2(mut settings: SteerablePerlinSettings) -> (Vec<f32>, f32, f32) {
    settings.dimensions_mut().extent_x = 32;
    settings.dimensions_mut().extent_y = 24;
    simdnoise::steerable_perlin_2d(&settings)
}

fn block3(mut settings: SteerablePerlinSettings) -> (Vec<f32>, f32, f32) {
    settings.dimensions_mut().extent_x = 16;
    settings.dimensions_mut().extent_y = 16;
    settings.dimensions_mut().extent_z = 16;
    simdnoise::steerable_perlin_3d(&settings)
}

#[test]
fn steerable_perlin_2d_range_and_bounds() {
    let settings = settings2();
    let (values, min, max) = block2(settings);
    assert_eq!(values.len(), 32 * 24);
    assert!(values.iter().all(|value| value.is_finite()));
    assert!(min <= max);
    let (low, high) = settings.output_range().unwrap();
    assert!(
        min >= low && max <= high,
        "range [{min}, {max}] outside [{low}, {high}]"
    );
}

#[test]
fn steerable_perlin_3d_range_and_bounds() {
    let settings = settings3();
    let (values, min, max) = block3(settings);
    assert_eq!(values.len(), 16 * 16 * 16);
    assert!(values.iter().all(|value| value.is_finite()));
    assert!(min <= max);
    let (low, high) = settings.output_range().unwrap();
    assert!(
        min >= low && max <= high,
        "range [{min}, {max}] outside [{low}, {high}]"
    );
}

#[test]
fn steerable_perlin_is_deterministic() {
    assert_eq!(block2(settings2()).0, block2(settings2()).0);
    assert_eq!(block3(settings3()).0, block3(settings3()).0);
}

#[test]
fn steerable_perlin_seed_changes_the_field() {
    let mut a = settings2();
    a.dimensions_mut().extent_x = 32;
    a.dimensions_mut().extent_y = 24;
    let mut b = a;
    b.with_seed(99);
    let (left, _, _) = simdnoise::steerable_perlin_2d(&a);
    let (right, _, _) = simdnoise::steerable_perlin_2d(&b);
    assert!(left.iter().zip(&right).any(|(a, b)| (a - b).abs() > 1.0e-4));
}

#[test]
fn steerable_perlin_direction_changes_the_field() {
    let mut a = settings2();
    a.dimensions_mut().extent_x = 32;
    a.dimensions_mut().extent_y = 24;
    let mut b = a;
    b.with_direction(0.0, 1.0, 0.0);
    let (left, _, _) = simdnoise::steerable_perlin_2d(&a);
    let (right, _, _) = simdnoise::steerable_perlin_2d(&b);
    assert!(left.iter().zip(&right).any(|(a, b)| (a - b).abs() > 1.0e-4));
}

#[test]
fn steerable_perlin_fill_matches_generate() {
    let mut settings = settings2();
    settings.dimensions_mut().extent_x = 8;
    settings.dimensions_mut().extent_y = 8;
    let (expected, min, max) = simdnoise::steerable_perlin_2d(&settings);
    let mut actual = vec![0.0f32; expected.len()];
    let (actual_min, actual_max) = settings.fill(&mut actual);
    assert_eq!(actual, expected);
    assert!((actual_min - min).abs() < 1.0e-6);
    assert!((actual_max - max).abs() < 1.0e-6);
}

#[test]
#[should_panic(expected = "Steerable Perlin supports 2D and 3D only")]
fn steerable_perlin_rejects_1d() {
    let output = simdnoise::steerable_perlin_2d(&SteerablePerlinSettings::default(
        NoiseDimensions::default(1),
    ));
    let _ = output;
}

#[test]
#[should_panic(expected = "Steerable Perlin direction must be finite and non-zero")]
fn steerable_perlin_rejects_zero_direction() {
    let mut settings = settings2();
    settings.with_direction(0.0, 0.0, 0.0);
    let _ = simdnoise::steerable_perlin_2d(&settings);
}

#[test]
#[should_panic(expected = "Steerable Perlin metric requires eigenvalue_sum > k > 0")]
fn steerable_perlin_rejects_bad_metric() {
    let mut settings = settings2();
    settings.with_metric(0.5, 0.51);
    let _ = simdnoise::steerable_perlin_2d(&settings);
}

#[test]
fn steerable_perlin_gradient_values_match_generate() {
    let mut settings = settings2();
    settings.dimensions_mut().extent_x = 12;
    settings.dimensions_mut().extent_y = 10;
    let (values, _, _) = simdnoise::steerable_perlin_2d(&settings);
    let gradient = simdnoise::steerable_perlin_2d_gradient(&settings);
    assert_eq!(gradient.values.len(), values.len());
    for (a, b) in gradient.values.iter().zip(&values) {
        assert!((a - b).abs() <= 1.0e-6);
    }
    assert_eq!(gradient.stride, 2);
    assert_eq!(gradient.gradients.len(), values.len() * 2);
}

fn sample_2d(settings: &SteerablePerlinSettings, point: [f32; 2]) -> f32 {
    let mut local = *settings;
    local.dimensions_mut().offset_x = point[0];
    local.dimensions_mut().offset_y = point[1];
    local.dimensions_mut().extent_x = 1;
    local.dimensions_mut().extent_y = 1;
    simdnoise::steerable_perlin_2d(&local).0[0]
}

#[test]
fn steerable_perlin_2d_gradient_matches_finite_difference() {
    let mut settings = settings2();
    let point = [3.25f32, -1.5];
    settings.dimensions_mut().offset_x = point[0];
    settings.dimensions_mut().offset_y = point[1];
    settings.dimensions_mut().extent_x = 1;
    settings.dimensions_mut().extent_y = 1;
    let gradient = simdnoise::steerable_perlin_2d_gradient(&settings);
    let index = 0;
    let step = 1.0e-3;
    for axis in 0..2 {
        let mut plus = point;
        let mut minus = point;
        plus[axis] += step;
        minus[axis] -= step;
        let difference = (sample_2d(&settings, plus) - sample_2d(&settings, minus)) / (2.0 * step);
        let analytic = gradient.gradients[index * 2 + axis];
        assert!(
            (analytic - difference).abs() <= 5.0e-2,
            "axis {axis}: analytic {analytic}, finite difference {difference}"
        );
    }
}

#[test]
fn steerable_perlin_3d_gradient_matches_finite_difference() {
    let mut settings = settings3();
    let point = [3.25f32, -1.5, 0.75];
    settings.dimensions_mut().offset_x = point[0];
    settings.dimensions_mut().offset_y = point[1];
    settings.dimensions_mut().offset_z = point[2];
    settings.dimensions_mut().extent_x = 1;
    settings.dimensions_mut().extent_y = 1;
    settings.dimensions_mut().extent_z = 1;
    let gradient = simdnoise::steerable_perlin_3d_gradient(&settings);
    assert_eq!(gradient.stride, 3);
    assert_eq!(gradient.gradients.len(), gradient.values.len() * 3);
    let step = 1.0e-3;
    for axis in 0..3 {
        let mut plus = point;
        let mut minus = point;
        plus[axis] += step;
        minus[axis] -= step;
        let mut local = settings;
        local.dimensions_mut().extent_x = 1;
        local.dimensions_mut().extent_y = 1;
        local.dimensions_mut().extent_z = 1;
        local.dimensions_mut().offset_x = plus[0];
        local.dimensions_mut().offset_y = plus[1];
        local.dimensions_mut().offset_z = plus[2];
        let plus_value = simdnoise::steerable_perlin_3d(&local).0[0];
        local.dimensions_mut().offset_x = minus[0];
        local.dimensions_mut().offset_y = minus[1];
        local.dimensions_mut().offset_z = minus[2];
        let minus_value = simdnoise::steerable_perlin_3d(&local).0[0];
        let difference = (plus_value - minus_value) / (2.0 * step);
        let analytic = gradient.gradients[axis];
        assert!(
            (analytic - difference).abs() <= 5.0e-2,
            "axis {axis}: analytic {analytic}, finite difference {difference}"
        );
    }
}
