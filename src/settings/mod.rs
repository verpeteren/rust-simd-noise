pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

pub trait Settings {
    fn default(dim: NoiseDimensions) -> Self;
    fn with_seed(&mut self, seed: i32) -> &mut Self;
    fn with_freq(&mut self, freq: f32) -> &mut Self;
    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut Self;
    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut Self;
    fn with_freq_4d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32, freq_w: f32) -> &mut Self;
    fn with_freq_5d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
    ) -> &mut Self;
    fn with_freq_6d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
        freq_v: f32,
    ) -> &mut Self;

    fn freq_x(&self) -> f32;
    fn freq_y(&self) -> f32;
    fn freq_z(&self) -> f32;
    fn freq_w(&self) -> f32;
    fn freq_u(&self) -> f32;
    fn freq_v(&self) -> f32;

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    fn wrap(self) -> NoiseType;

    /// The analytic, **offset-independent** output range `(min, max)` of the
    /// unscaled noise for these settings, or `None` when it has no closed form
    /// (the jitter/metric-dependent cellular returns).
    ///
    /// This is the range [`generate_scaled`](Settings::generate_scaled) maps onto
    /// your target range, so it is identical regardless of a block's offset/size
    /// — which is what makes adjacent blocks line up. It differs from the
    /// per-block observed `(min, max)` returned by [`generate`](Settings::generate),
    /// which varies from block to block. See each settings type's docs for the
    /// formula.
    fn output_range(&self) -> Option<(f32, f32)>
    where
        Self: Copy,
    {
        (*self).wrap().theoretical_min_max()
    }

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    fn generate(self) -> (Vec<f32>, f32, f32);
    fn validate(&self);

    /// Generate a chunk of noise with values scaled from min to max
    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32>;

    /// f64 counterpart of [`generate`](Settings::generate).
    fn generate_f64(self) -> (Vec<f64>, f64, f64);

    /// f64 counterpart of [`generate_scaled`](Settings::generate_scaled); the
    /// target range is given as f64.
    fn generate_scaled_f64(self, min: f64, max: f64) -> Vec<f64>;

    /// Emplace unscaled noise into `out`, whose length must equal the product of
    /// the configured dimensions. Returns the slice's `(min, max)`.
    fn fill(self, out: &mut [f32]) -> (f32, f32);

    /// Emplace noise scaled to `[min, max]` into `out` (same length rule).
    fn fill_scaled(self, min: f32, max: f32, out: &mut [f32]);

    /// f64 counterpart of [`fill`](Settings::fill).
    fn fill_f64(self, out: &mut [f64]) -> (f64, f64);

    /// f64 counterpart of [`fill_scaled`](Settings::fill_scaled).
    fn fill_scaled_f64(self, min: f64, max: f64, out: &mut [f64]);

    /// Emplace noise quantized to an integer type into `out`, mapping
    /// `[src_min, src_max]` onto the target type's full range (see
    /// [`NoiseInto`](crate::NoiseInto)). `out.len()` must equal the dimension
    /// product.
    fn fill_range<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]);
}

pub trait SimplexSettings {
    fn with_lacunarity(&mut self, lacunarity: f32) -> &mut Self;
    fn with_gain(&mut self, gain: f32) -> &mut Self;
    /// Number of octaves summed by the fractal.
    ///
    /// Intentionally an `f32`, not an integer: a **fractional** octave count is
    /// supported (issue #14). The whole part contributes full octaves and the
    /// fractional part fades the next octave in smoothly, which keeps the output
    /// continuous as the count is animated across integer boundaries instead of
    /// jumping when a new octave appears. Pass e.g. `5.0` for five octaves, or
    /// `5.5` to blend halfway into the sixth.
    fn with_octaves(&mut self, octaves: f32) -> &mut Self;
}

mod cellular2_settings;
mod cellular_settings;
mod fbm_settings;
mod gradient_settings;
mod ridge_settings;
mod turbulence_settings;

pub use cellular2_settings::Cellular2Settings;
pub use cellular_settings::CellularSettings;
pub use fbm_settings::FbmSettings;
pub use gradient_settings::GradientSettings;
pub use ridge_settings::RidgeSettings;
pub use turbulence_settings::TurbulenceSettings;
