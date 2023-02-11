use super::{
    Cellular2Settings, CellularSettings, DimensionalBeing, FbmSettings, GradientSettings,
    NoiseDimensions, RidgeSettings, TurbulenceSettings,
};

/// Specifies what type of noise to generate and contains any relevant settings.
#[derive(Copy, Clone)]
pub enum NoiseType {
    Fbm(FbmSettings),
    Ridge(RidgeSettings),
    Turbulence(TurbulenceSettings),
    Gradient(GradientSettings),
    Cellular(CellularSettings),
    Cellular2(Cellular2Settings),
}

impl DimensionalBeing for NoiseType {
    fn get_dimensions(&self) -> NoiseDimensions {
        match self {
            NoiseType::Fbm(s) => s.get_dimensions(),
            NoiseType::Ridge(s) => s.get_dimensions(),
            NoiseType::Turbulence(s) => s.get_dimensions(),
            NoiseType::Gradient(s) => s.get_dimensions(),
            NoiseType::Cellular(s) => s.get_dimensions(),
            NoiseType::Cellular2(s) => s.get_dimensions(),
        }
    }
}
