use crate::dimensional_being::DimensionalBeing;
use crate::intrinsics::{avx2, scalar, sse2, sse41};
pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_builder::NoiseBuilder;
pub use crate::noise_dimensions::NoiseDimensions;
pub use crate::noise_type::NoiseType;

use super::Settings;

#[derive(Copy, Clone)]
pub struct Cellular2Settings {
    dim: NoiseDimensions,
    pub freq_x: f32,
    pub freq_y: f32,
    pub freq_z: f32,
    pub distance_function: CellDistanceFunction,
    pub return_type: Cell2ReturnType,
    pub jitter: f32,
    pub index0: usize,
    pub index1: usize,
}

impl DimensionalBeing for Cellular2Settings {
    fn get_dimensions(&self) -> NoiseDimensions {
        return self.dim;
    }
}

impl Settings for Cellular2Settings {
    fn default(dim: NoiseDimensions) -> Cellular2Settings {
        Cellular2Settings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: Cell2ReturnType::Distance2,
            jitter: 0.25,
            index0: 0,
            index1: 1,
        }
    }
    fn with_seed(&mut self, seed: i32) -> &mut Cellular2Settings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut Cellular2Settings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut Cellular2Settings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut Cellular2Settings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }
    fn with_freq_4d(
        &mut self,
        _freq_x: f32,
        _freq_y: f32,
        _freq_z: f32,
        _freq_w: f32,
    ) -> &mut Cellular2Settings {
        unimplemented!()
    }

    fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Cellular2(self)
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            2 => get_2d_noise!(&NoiseType::Cellular2(self)),
            3 => get_3d_noise!(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {
        if self.index0 > 2 || self.index1 > 3 || self.index0 >= self.index1 {
            panic!("invalid index settings in cellular2 noise");
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        self.validate();
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            2 => get_2d_scaled_noise!(&NoiseType::Cellular2(new_self)),
            3 => get_3d_scaled_noise!(&NoiseType::Cellular2(new_self)),
            _ => panic!("not implemented"),
        }
    }
}

impl Cellular2Settings {
    pub fn with_distance_function(&mut self, dist: CellDistanceFunction) -> &mut Cellular2Settings {
        self.distance_function = dist;
        self
    }

    pub fn with_return_type(&mut self, return_type: Cell2ReturnType) -> &mut Cellular2Settings {
        self.return_type = return_type;
        self
    }

    pub fn with_jitter(&mut self, jitter: f32) -> &mut Cellular2Settings {
        self.jitter = jitter;
        self
    }

    pub fn with_index0(&mut self, i: usize) -> &mut Cellular2Settings {
        self.index0 = i;
        self
    }

    pub fn with_index1(&mut self, i: usize) -> &mut Cellular2Settings {
        self.index1 = i;
        self
    }
}
