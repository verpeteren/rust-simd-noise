use crate::noise_dimensions::NoiseDimensions;

pub trait DimensionalBeing {
    fn dimensions(&self) -> NoiseDimensions;
}
