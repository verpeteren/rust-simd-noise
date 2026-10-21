//! Gabor noise settings and block generation.

use crate::noise_dimensions::NoiseDimensions;
use crate::NoiseAndGradient;

/// Anisotropic Gabor noise.
///
/// Gabor noise is a sparse convolution of random impulses with an oriented
/// Gaussian-windowed cosine. The result is smooth, band-limited and
/// directionally controllable, which makes it useful for brushed metal, wood,
/// fabric and other anisotropic materials.
///
/// Supported dimensions are 2D, 3D and 4D. 4D is 3D plus time, which makes it
/// useful for animated anisotropic materials. The standalone settings type is
/// the first slice; `NoiseType` and the full `Settings` trait integration
/// follow in a second slice.
#[derive(Copy, Clone)]
pub struct GaborSettings {
    dimension: NoiseDimensions,
    /// Frequency along X. The batch loop multiplies the sample position by this
    /// before sampling the kernel, so it controls feature size.
    pub frequency_x: f32,
    /// Frequency along Y; see [`frequency_x`](Self::frequency_x).
    pub frequency_y: f32,
    /// Frequency along Z; see [`frequency_x`](Self::frequency_x).
    pub frequency_z: f32,
    /// Frequency along the 4th (W) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_w: f32,
    /// Orientation of the cosine carrier along X. Normalized in [`validate`](Self::validate).
    pub orientation_x: f32,
    /// Orientation of the cosine carrier along Y.
    pub orientation_y: f32,
    /// Orientation of the cosine carrier along Z.
    pub orientation_z: f32,
    /// Orientation of the cosine carrier along the 4th (W) axis.
    pub orientation_w: f32,
    /// Gaussian envelope width in lattice cells. Larger values make longer,
    /// softer strokes; smaller values make tighter, higher-frequency detail.
    pub bandwidth: f32,
    /// Impulse-search radius in lattice cells, `1..=4`.
    pub radius: i32,
    /// Impulses per unit volume. 1.0 uses one impulse per unit cell; larger
    /// values shrink the cell and add detail.
    pub impulse_density: f32,
    /// Carrier frequency in cycles per lattice cell.
    pub carrier_frequency: f32,
    /// Output gain after normalization. The kernel is normalized by the number
    /// of impulses, so the guaranteed range is `[-|gain|, |gain|]`.
    pub gain: f32,
}

impl GaborSettings {
    /// Default settings for `dimension` dimensions.
    pub fn default(dimension: NoiseDimensions) -> Self {
        Self {
            dimension,
            frequency_x: 0.02,
            frequency_y: 0.02,
            frequency_z: 0.02,
            frequency_w: 0.02,
            orientation_x: 1.0,
            orientation_y: 0.0,
            orientation_z: 0.0,
            orientation_w: 0.0,
            bandwidth: 1.0,
            radius: 1,
            impulse_density: 1.0,
            carrier_frequency: 1.0,
            gain: 1.0,
        }
    }

    /// The block dimensions and seed.
    pub fn dimensions(&self) -> NoiseDimensions {
        self.dimension
    }

    /// Set the seed for the impulse positions and phases.
    pub fn with_seed(&mut self, seed: i32) -> &mut Self {
        self.dimension.seed = seed;
        self
    }

    /// Set the feature frequency on every axis.
    pub fn with_frequency(&mut self, frequency: f32) -> &mut Self {
        self.frequency_x = frequency;
        self.frequency_y = frequency;
        self.frequency_z = frequency;
        self.frequency_w = frequency;
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

    /// Set the X/Y/Z/W feature frequencies.
    pub fn with_frequency_4d(
        &mut self,
        frequency_x: f32,
        frequency_y: f32,
        frequency_z: f32,
        frequency_w: f32,
    ) -> &mut Self {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self.frequency_z = frequency_z;
        self.frequency_w = frequency_w;
        self
    }

    /// Set the carrier orientation. The vector is normalized by
    /// [`validate`](Self::validate).
    pub fn with_orientation(&mut self, x: f32, y: f32, z: f32) -> &mut Self {
        self.orientation_x = x;
        self.orientation_y = y;
        self.orientation_z = z;
        self
    }

    /// Set the 4D carrier orientation. The vector is normalized by
    /// [`validate`](Self::validate).
    pub fn with_orientation_4d(&mut self, x: f32, y: f32, z: f32, w: f32) -> &mut Self {
        self.orientation_x = x;
        self.orientation_y = y;
        self.orientation_z = z;
        self.orientation_w = w;
        self
    }

    /// Set the Gaussian envelope width in lattice cells.
    pub fn with_bandwidth(&mut self, bandwidth: f32) -> &mut Self {
        self.bandwidth = bandwidth;
        self
    }

    /// Set the impulse-search radius in lattice cells.
    pub fn with_radius(&mut self, radius: i32) -> &mut Self {
        self.radius = radius;
        self
    }

    /// Set the impulse density.
    pub fn with_impulse_density(&mut self, density: f32) -> &mut Self {
        self.impulse_density = density;
        self
    }

    /// Set the carrier frequency in cycles per lattice cell.
    pub fn with_carrier_frequency(&mut self, frequency: f32) -> &mut Self {
        self.carrier_frequency = frequency;
        self
    }

    /// Set the output gain.
    pub fn with_gain(&mut self, gain: f32) -> &mut Self {
        self.gain = gain;
        self
    }

    /// Validate the settings. Panics with a specific message on invalid input.
    pub fn validate(&self) {
        if !(2..=4).contains(&self.dimension.dimension) {
            panic!(
                "Gabor noise supports 2D, 3D and 4D only, got {}D",
                self.dimension.dimension
            );
        }
        let finite = |value: f32| value.is_finite();
        if !finite(self.frequency_x) || self.frequency_x <= 0.0 {
            panic!("Gabor frequencies must be finite and positive");
        }
        if self.dimension.dimension >= 2 && (!finite(self.frequency_y) || self.frequency_y <= 0.0) {
            panic!("Gabor frequencies must be finite and positive");
        }
        if self.dimension.dimension >= 3 && (!finite(self.frequency_z) || self.frequency_z <= 0.0) {
            panic!("Gabor frequencies must be finite and positive");
        }
        if self.dimension.dimension >= 4 && (!finite(self.frequency_w) || self.frequency_w <= 0.0) {
            panic!("Gabor frequencies must be finite and positive");
        }
        let mut orientation_len2 = self.orientation_x * self.orientation_x;
        if self.dimension.dimension >= 2 {
            orientation_len2 += self.orientation_y * self.orientation_y;
        }
        if self.dimension.dimension >= 3 {
            orientation_len2 += self.orientation_z * self.orientation_z;
        }
        if self.dimension.dimension >= 4 {
            orientation_len2 += self.orientation_w * self.orientation_w;
        }
        if !orientation_len2.is_finite() || orientation_len2 <= 1.0e-12 {
            panic!("Gabor orientation must be finite and non-zero");
        }
        if !finite(self.bandwidth) || self.bandwidth <= 0.0 {
            panic!("Gabor bandwidth must be finite and positive");
        }
        if !(1..=4).contains(&self.radius) {
            panic!("Gabor radius must be in 1..=4");
        }
        if !finite(self.impulse_density) || self.impulse_density <= 0.0 {
            panic!("Gabor impulse density must be finite and positive");
        }
        if !finite(self.carrier_frequency) || self.carrier_frequency <= 0.0 {
            panic!("Gabor carrier frequency must be finite and positive");
        }
        if !finite(self.gain) {
            panic!("Gabor gain must be finite");
        }
    }

    /// The guaranteed output range after normalization.
    pub fn output_range(&self) -> Option<(f32, f32)> {
        let gain = self.gain.abs();
        Some((-gain, gain))
    }

    /// Generate the block and return `(values, observed_min, observed_max)`.
    pub fn generate(self) -> (Vec<f32>, f32, f32) {
        self.validate();
        match self.dimension.dimension {
            2 => crate::gabor_2d(&self),
            3 => crate::gabor_3d(&self),
            4 => crate::gabor_4d(&self),
            _ => unreachable!("validated above"),
        }
    }

    /// Generate a block scaled to `[min, max]` using the observed block range.
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
    ///
    /// The returned [`NoiseAndGradient`] has one gradient vector per sample,
    /// with `stride` equal to the dimensionality. Each component includes that
    /// axis's frequency factor, matching the sample-space convention used by
    /// the existing gradient API.
    pub fn generate_with_gradient(self) -> NoiseAndGradient {
        self.validate();
        match self.dimension.dimension {
            2 => crate::gabor_2d_gradient(&self),
            3 => crate::gabor_3d_gradient(&self),
            4 => crate::gabor_4d_gradient(&self),
            _ => unreachable!("validated above"),
        }
    }

    /// Fill `out` with the block and return the observed `(min, max)`.
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
