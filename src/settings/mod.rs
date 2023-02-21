pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

pub trait Settings {
    fn default(dim: NoiseDimensions) -> Self;
    fn with_seed(&mut self, seed: i32) -> &mut Self;
    fn with_freq(&mut self, freq: f32) -> &mut Self;
    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut Self;
    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut Self;
    fn with_freq_4d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32, freq_w: f32) -> &mut Self;

    /// If you want to call noise functions by hand, call wrap on the settings
    /// to get back a NoiseType to call the noise functions with
    fn wrap(self) -> NoiseType;

    /// Generate a chunk of noise based on your settings, and the min and max value
    /// generated, so you can scale it as you wish
    fn generate(self) -> (Vec<f32>, f32, f32);
    fn validate(&self);

    /// Generate a chunk of noise with values scaled from min to max
    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32>;
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
