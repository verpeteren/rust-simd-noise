pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

pub trait Settings {
    fn default(dim: NoiseDimensions) -> Self;
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
