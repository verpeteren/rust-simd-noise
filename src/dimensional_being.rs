use crate::noise_dimensions::NoiseDimensions;

pub trait DimensionalBeing {
    fn get_dimensions(&self) -> NoiseDimensions;
}
