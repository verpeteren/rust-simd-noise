//! Steerable Perlin noise settings and block generation.

use crate::noise::steerable_perlin_32::{metric_2d, metric_3d};
use crate::noise_dimensions::NoiseDimensions;
use simdeez::prelude::*;

/// Anisotropic Perlin noise with a steerable metric (Rice, SIGGRAPH 2025).
///
/// The direction controls the anisotropy axis. `eigenvalue_sum` and `k`
/// control the metric's eigenvalue spread; the defaults match the reference
/// implementation.
#[derive(Copy, Clone)]
pub struct SteerablePerlinSettings {
    dimension: NoiseDimensions,
    /// Feature frequency along X.
    pub frequency_x: f32,
    /// Feature frequency along Y.
    pub frequency_y: f32,
    /// Feature frequency along Z.
    pub frequency_z: f32,
    /// Anisotropy direction X.
    pub direction_x: f32,
    /// Anisotropy direction Y.
    pub direction_y: f32,
    /// Anisotropy direction Z.
    pub direction_z: f32,
    /// Sum of the mapped metric eigenvalues.
    pub eigenvalue_sum: f32,
    /// Large eigenvalue of the steerable metric.
    pub k: f32,
    /// Output gain.
    pub gain: f32,
}

impl SteerablePerlinSettings {
    /// Default settings for 2D or 3D.
    pub fn default(dimension: NoiseDimensions) -> Self {
        Self {
            dimension,
            frequency_x: 0.02,
            frequency_y: 0.02,
            frequency_z: 0.02,
            direction_x: 1.0,
            direction_y: 0.0,
            direction_z: 0.0,
            eigenvalue_sum: 4.0,
            k: 0.51,
            gain: 1.0,
        }
    }

    /// The block dimensions and seed.
    pub fn dimensions(&self) -> NoiseDimensions {
        self.dimension
    }

    /// Mutable block dimensions, for setting extents/offsets directly.
    pub fn dimensions_mut(&mut self) -> &mut NoiseDimensions {
        &mut self.dimension
    }

    /// Set the seed.
    pub fn with_seed(&mut self, seed: i32) -> &mut Self {
        self.dimension.seed = seed;
        self
    }

    /// Set the feature frequency on every axis.
    pub fn with_frequency(&mut self, frequency: f32) -> &mut Self {
        self.frequency_x = frequency;
        self.frequency_y = frequency;
        self.frequency_z = frequency;
        self
    }

    /// Set the X/Y feature frequencies.
    pub fn with_frequency_2d(&mut self, frequency_x: f32, frequency_y: f32) -> &mut Self {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self
    }

    /// Set the X/Y/Z feature frequencies.
    pub fn with_frequency_3d(
        &mut self,
        frequency_x: f32,
        frequency_y: f32,
        frequency_z: f32,
    ) -> &mut Self {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self.frequency_z = frequency_z;
        self
    }

    /// Set the anisotropy direction.
    pub fn with_direction(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.direction_x = x;
        self.direction_y = y;
        self.direction_z = z;
        self
    }

    /// Set the metric eigenvalue spread.
    pub fn with_metric(&mut self, eigenvalue_sum: f32, k: f32) -> &mut Self {
        self.eigenvalue_sum = eigenvalue_sum;
        self.k = k;
        self
    }

    /// Set the output gain.
    pub fn with_gain(&mut self, gain: f32) -> &mut Self {
        self.gain = gain;
        self
    }

    /// Validate the settings. Panics on invalid input.
    pub fn validate(&self) {
        if !(2..=3).contains(&self.dimension.dimension) {
            panic!(
                "Steerable Perlin supports 2D and 3D only, got {}D",
                self.dimension.dimension
            );
        }
        let frequencies = [self.frequency_x, self.frequency_y, self.frequency_z];
        let active = self.dimension.dimension;
        if frequencies[..active]
            .iter()
            .any(|value| !value.is_finite() || *value <= 0.0)
        {
            panic!("Steerable Perlin frequencies must be finite and positive");
        }
        let direction = [self.direction_x, self.direction_y, self.direction_z];
        if direction[..active].iter().all(|value| *value == 0.0)
            || direction.iter().any(|value| !value.is_finite())
        {
            panic!("Steerable Perlin direction must be finite and non-zero");
        }
        if !self.eigenvalue_sum.is_finite()
            || !self.k.is_finite()
            || self.eigenvalue_sum <= self.k
            || self.k <= 0.0
        {
            panic!("Steerable Perlin metric requires eigenvalue_sum > k > 0");
        }
        if !self.gain.is_finite() {
            panic!("Steerable Perlin gain must be finite");
        }
    }

    /// Conservative output bound.
    pub fn output_range(&self) -> Option<(f32, f32)> {
        let dimension = self.dimension.dimension as f32;
        let low = (self.eigenvalue_sum - self.k) / (dimension - 1.0) - 1.0e-4;
        let corners = 2f32.powi(self.dimension.dimension as i32);
        let bound = self.gain.abs() * corners * low * dimension.sqrt();
        Some((-bound, bound))
    }

    /// Generate the block and return `(values, observed_min, observed_max)`.
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        self.validate();
        match self.dimension.dimension {
            2 => crate::steerable_perlin_2d(&self),
            3 => crate::steerable_perlin_3d(&self),
            _ => unreachable!("validated above"),
        }
    }

    /// Generate a block scaled to `[min, max]` using the observed range.
    pub fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let (values, observed_min, observed_max) = self.generate();
        let span = observed_max - observed_min;
        if span <= 0.0 {
            return vec![min; values.len()];
        }
        let scale = (max - min) / span;
        values
            .into_iter()
            .map(|value| min + (value - observed_min) * scale)
            .collect()
    }

    /// Generate the block together with its analytic gradient.
    pub fn generate_with_gradient(self) -> crate::NoiseAndGradient {
        self.validate();
        match self.dimension.dimension {
            2 => crate::steerable_perlin_2d_gradient(&self),
            3 => crate::steerable_perlin_3d_gradient(&self),
            _ => unreachable!("validated above"),
        }
    }

    /// Fill `out` and return the observed `(min, max)`.
    pub fn fill(self, out: &mut [f32]) -> (f32, f32) {
        let (values, min, max) = self.generate();
        assert_eq!(
            out.len(),
            values.len(),
            "noise output slice length must match the configured dimensions"
        );
        out.copy_from_slice(&values);
        (min, max)
    }
}

/// 2D steerable Perlin block.
pub(crate) fn steerable_perlin_2d_block<S: Simd>(
    settings: &SteerablePerlinSettings,
) -> (Vec<f32>, f32, f32) {
    settings.validate();
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let metric = metric_2d(
        [settings.direction_x, settings.direction_y],
        settings.eigenvalue_sum,
        settings.k,
    );
    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let mut values = vec![0.0f32; width * height];
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let lane_width = S::Vf32::WIDTH;
    let mut x_lanes = vec![0.0f32; lane_width];
    for (lane, value) in x_lanes.iter_mut().enumerate() {
        *value = dimension.offset_x + lane as f32;
    }
    let mut index = 0;
    for row in 0..height {
        let y = S::Vf32::set1(dimension.offset_y + row as f32);
        let mut x = unsafe { S::Vf32::load_from_ptr_unaligned(x_lanes.as_ptr()) };
        for _ in 0..width / lane_width {
            let value = crate::noise::steerable_perlin_32::steerable_perlin_2d::<S>(
                x * frequency_x,
                y * frequency_y,
                dimension.seed,
                metric,
                settings.gain,
            );
            for lane in 0..lane_width {
                let sample = value[lane];
                values[index] = sample;
                min = min.min(sample);
                max = max.max(sample);
                index += 1;
            }
            x += S::Vf32::set1(lane_width as f32);
        }
        for lane in 0..width % lane_width {
            let sample = crate::noise::steerable_perlin_32::steerable_perlin_2d::<S>(
                S::Vf32::set1(dimension.offset_x + (width - width % lane_width + lane) as f32)
                    * frequency_x,
                y * frequency_y,
                dimension.seed,
                metric,
                settings.gain,
            )[0];
            values[index] = sample;
            min = min.min(sample);
            max = max.max(sample);
            index += 1;
        }
    }
    (values, min, max)
}

/// 3D steerable Perlin block.
pub(crate) fn steerable_perlin_3d_block<S: Simd>(
    settings: &SteerablePerlinSettings,
) -> (Vec<f32>, f32, f32) {
    settings.validate();
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let depth = dimension.extent_z;
    let metric = metric_3d(
        [
            settings.direction_x,
            settings.direction_y,
            settings.direction_z,
        ],
        settings.eigenvalue_sum,
        settings.k,
    );
    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let frequency_z = S::Vf32::set1(settings.frequency_z);
    let mut values = vec![0.0f32; width * height * depth];
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let lane_width = S::Vf32::WIDTH;
    let mut x_lanes = vec![0.0f32; lane_width];
    for (lane, value) in x_lanes.iter_mut().enumerate() {
        *value = dimension.offset_x + lane as f32;
    }
    let mut index = 0;
    for layer in 0..depth {
        let z = S::Vf32::set1(dimension.offset_z + layer as f32);
        for row in 0..height {
            let y = S::Vf32::set1(dimension.offset_y + row as f32);
            let mut x = unsafe { S::Vf32::load_from_ptr_unaligned(x_lanes.as_ptr()) };
            for _ in 0..width / lane_width {
                let value = crate::noise::steerable_perlin_32::steerable_perlin_3d::<S>(
                    x * frequency_x,
                    y * frequency_y,
                    z * frequency_z,
                    dimension.seed,
                    metric,
                    settings.gain,
                );
                for lane in 0..lane_width {
                    let sample = value[lane];
                    values[index] = sample;
                    min = min.min(sample);
                    max = max.max(sample);
                    index += 1;
                }
                x += S::Vf32::set1(lane_width as f32);
            }
            for lane in 0..width % lane_width {
                let sample = crate::noise::steerable_perlin_32::steerable_perlin_3d::<S>(
                    S::Vf32::set1(dimension.offset_x + (width - width % lane_width + lane) as f32)
                        * frequency_x,
                    y * frequency_y,
                    z * frequency_z,
                    dimension.seed,
                    metric,
                    settings.gain,
                )[0];
                values[index] = sample;
                min = min.min(sample);
                max = max.max(sample);
                index += 1;
            }
        }
    }
    (values, min, max)
}

/// 2D steerable Perlin block with analytic gradients.
pub(crate) fn steerable_perlin_2d_gradient_block<S: Simd>(
    settings: &SteerablePerlinSettings,
) -> crate::NoiseAndGradient {
    settings.validate();
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let metric = metric_2d(
        [settings.direction_x, settings.direction_y],
        settings.eigenvalue_sum,
        settings.k,
    );
    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let mut values = vec![0.0f32; width * height];
    let mut gradients = vec![0.0f32; width * height * 2];
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let lane_width = S::Vf32::WIDTH;
    let mut x_lanes = vec![0.0f32; lane_width];
    for (lane, value) in x_lanes.iter_mut().enumerate() {
        *value = dimension.offset_x + lane as f32;
    }
    let mut index = 0;
    for row in 0..height {
        let y = S::Vf32::set1(dimension.offset_y + row as f32);
        let mut x = unsafe { S::Vf32::load_from_ptr_unaligned(x_lanes.as_ptr()) };
        for _ in 0..width / lane_width {
            let (value, gradient) =
                crate::noise::steerable_perlin_32::steerable_perlin_2d_gradient::<S>(
                    x * frequency_x,
                    y * frequency_y,
                    dimension.seed,
                    metric,
                    settings.gain,
                );
            for lane in 0..lane_width {
                let sample = value[lane];
                values[index] = sample;
                gradients[index * 2] = gradient[0][lane] * settings.frequency_x;
                gradients[index * 2 + 1] = gradient[1][lane] * settings.frequency_y;
                min = min.min(sample);
                max = max.max(sample);
                index += 1;
            }
            x += S::Vf32::set1(lane_width as f32);
        }
        for lane in 0..width % lane_width {
            let (value, gradient) =
                crate::noise::steerable_perlin_32::steerable_perlin_2d_gradient::<S>(
                    S::Vf32::set1(dimension.offset_x + (width - width % lane_width + lane) as f32)
                        * frequency_x,
                    y * frequency_y,
                    dimension.seed,
                    metric,
                    settings.gain,
                );
            let sample = value[0];
            values[index] = sample;
            gradients[index * 2] = gradient[0][0] * settings.frequency_x;
            gradients[index * 2 + 1] = gradient[1][0] * settings.frequency_y;
            min = min.min(sample);
            max = max.max(sample);
            index += 1;
        }
    }
    crate::NoiseAndGradient {
        values,
        gradients,
        stride: 2,
        min,
        max,
    }
}

/// 3D steerable Perlin block with analytic gradients.
pub(crate) fn steerable_perlin_3d_gradient_block<S: Simd>(
    settings: &SteerablePerlinSettings,
) -> crate::NoiseAndGradient {
    settings.validate();
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let depth = dimension.extent_z;
    let metric = metric_3d(
        [
            settings.direction_x,
            settings.direction_y,
            settings.direction_z,
        ],
        settings.eigenvalue_sum,
        settings.k,
    );
    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let frequency_z = S::Vf32::set1(settings.frequency_z);
    let mut values = vec![0.0f32; width * height * depth];
    let mut gradients = vec![0.0f32; width * height * depth * 3];
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;
    let lane_width = S::Vf32::WIDTH;
    let mut x_lanes = vec![0.0f32; lane_width];
    for (lane, value) in x_lanes.iter_mut().enumerate() {
        *value = dimension.offset_x + lane as f32;
    }
    let mut index = 0;
    for layer in 0..depth {
        let z = S::Vf32::set1(dimension.offset_z + layer as f32);
        for row in 0..height {
            let y = S::Vf32::set1(dimension.offset_y + row as f32);
            let mut x = unsafe { S::Vf32::load_from_ptr_unaligned(x_lanes.as_ptr()) };
            for _ in 0..width / lane_width {
                let (value, gradient) =
                    crate::noise::steerable_perlin_32::steerable_perlin_3d_gradient::<S>(
                        x * frequency_x,
                        y * frequency_y,
                        z * frequency_z,
                        dimension.seed,
                        metric,
                        settings.gain,
                    );
                for lane in 0..lane_width {
                    let sample = value[lane];
                    values[index] = sample;
                    gradients[index * 3] = gradient[0][lane] * settings.frequency_x;
                    gradients[index * 3 + 1] = gradient[1][lane] * settings.frequency_y;
                    gradients[index * 3 + 2] = gradient[2][lane] * settings.frequency_z;
                    min = min.min(sample);
                    max = max.max(sample);
                    index += 1;
                }
                x += S::Vf32::set1(lane_width as f32);
            }
            for lane in 0..width % lane_width {
                let (value, gradient) =
                    crate::noise::steerable_perlin_32::steerable_perlin_3d_gradient::<S>(
                        S::Vf32::set1(
                            dimension.offset_x + (width - width % lane_width + lane) as f32,
                        ) * frequency_x,
                        y * frequency_y,
                        z * frequency_z,
                        dimension.seed,
                        metric,
                        settings.gain,
                    );
                let sample = value[0];
                values[index] = sample;
                gradients[index * 3] = gradient[0][0] * settings.frequency_x;
                gradients[index * 3 + 1] = gradient[1][0] * settings.frequency_y;
                gradients[index * 3 + 2] = gradient[2][0] * settings.frequency_z;
                min = min.min(sample);
                max = max.max(sample);
                index += 1;
            }
        }
    }
    crate::NoiseAndGradient {
        values,
        gradients,
        stride: 3,
        min,
        max,
    }
}
