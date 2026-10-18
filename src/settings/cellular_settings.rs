use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::cell_32::{
    cellular_1d, cellular_2d, cellular_3d, cellular_4d, cellular_5d, cellular_6d,
};
use crate::noise::cell_64::{
    cellular_1d as cellular_1d_f64, cellular_2d as cellular_2d_f64, cellular_3d as cellular_3d_f64,
    cellular_4d as cellular_4d_f64, cellular_5d as cellular_5d_f64, cellular_6d as cellular_6d_f64,
};
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise::cell_return_type::CellReturnType;
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
pub use crate::noise_type::NoiseType;
use crate::{
    get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise,
    get_3d_scaled_noise, get_4d_noise, get_4d_scaled_noise, get_5d_noise, get_5d_scaled_noise,
    get_6d_noise, get_6d_scaled_noise,
};

use super::Settings;

/// Select the monomorphized cellular kernel for the runtime (distance, return)
/// configuration. This runs once per `sample_*` call (once per SIMD-vector
/// batch), outside the kernel's inner loops, so the kernel itself — chosen here
/// via the compile-time `cell_strategy` markers — carries no config branch.
macro_rules! dispatch_cell {
    ($call:ident, $df:expr, $rt:expr, $($arg:expr),* $(,)?) => {{
        use crate::noise::cell_strategy as cs;
        match ($df, $rt) {
            (CellDistanceFunction::Euclidean, CellReturnType::Distance) => {
                $call::<S, cs::Euclidean, cs::Distance>($($arg),*)
            }
            (CellDistanceFunction::Euclidean, CellReturnType::CellValue) => {
                $call::<S, cs::Euclidean, cs::CellValue>($($arg),*)
            }
            (CellDistanceFunction::Manhattan, CellReturnType::Distance) => {
                $call::<S, cs::Manhattan, cs::Distance>($($arg),*)
            }
            (CellDistanceFunction::Manhattan, CellReturnType::CellValue) => {
                $call::<S, cs::Manhattan, cs::CellValue>($($arg),*)
            }
            (CellDistanceFunction::Natural, CellReturnType::Distance) => {
                $call::<S, cs::Natural, cs::Distance>($($arg),*)
            }
            (CellDistanceFunction::Natural, CellReturnType::CellValue) => {
                $call::<S, cs::Natural, cs::CellValue>($($arg),*)
            }
        }
    }};
}

/// **Cellular noise** (a.k.a. Worley / Voronoi noise) — cell-based, not wavy.
///
/// Instead of interpolating gradients, cellular noise scatters one random
/// *feature point* in each grid cell and, for every sample, looks at the nearest
/// feature point. Depending on [`return_type`](Self::return_type) it returns the
/// **distance** to that point (giving rounded cell interiors and dark seams along
/// the boundaries) or a flat **per-cell value** (a mosaic of randomly shaded
/// tiles — a Voronoi diagram). It produces the stone, scales, cracked-mud,
/// dried-earth, and biome-region looks that gradient noise can't.
///
/// For the richer Worley patterns that combine the two nearest points (edges,
/// crack networks), use [`Cellular2Settings`](crate::Cellular2Settings). See
/// [Worley / cellular noise](https://en.wikipedia.org/wiki/Worley_noise).
#[derive(Copy, Clone)]
pub struct CellularSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: here it sets the **cell density** — higher
    /// means smaller, more numerous cells; lower means fewer, larger cells. See
    /// [`GradientSettings::freq_x`](crate::GradientSettings::freq_x).
    pub freq_x: f32,
    /// Frequency along the Y axis; see [`freq_x`](Self::freq_x).
    pub freq_y: f32,
    /// Frequency along the Z axis; see [`freq_x`](Self::freq_x).
    pub freq_z: f32,
    /// Frequency along the 4th (W) axis; see [`freq_x`](Self::freq_x).
    pub freq_w: f32,
    /// Frequency along the 5th (U) axis; see [`freq_x`](Self::freq_x).
    pub freq_u: f32,
    /// Frequency along the 6th (V) axis; see [`freq_x`](Self::freq_x).
    pub freq_v: f32,
    /// How distance to a feature point is measured, which sets the **shape of the
    /// cells**: [`Euclidean`](CellDistanceFunction::Euclidean) (straight-line →
    /// round cells), [`Manhattan`](CellDistanceFunction::Manhattan) (sum of axis
    /// distances → diamond/blocky cells), or
    /// [`Natural`](CellDistanceFunction::Natural) (a blend of the two → more
    /// organic cells).
    pub distance_function: CellDistanceFunction,
    /// What each sample returns:
    /// [`Distance`](CellReturnType::Distance) — distance to the nearest feature
    /// point (bumps/seams), or [`CellValue`](CellReturnType::CellValue) — one
    /// random value shared by the whole cell (flat Voronoi tiles).
    pub return_type: CellReturnType,
    /// How far each feature point is randomly nudged from its cell centre, `0.0`
    /// (perfectly regular grid of points) to `1.0` (fully randomized, most
    /// organic). Lower values look more lattice-like. Default `0.25`.
    pub jitter: f32,
}

impl DimensionalBeing for CellularSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for CellularSettings {
    fn default(dim: NoiseDimensions) -> CellularSettings {
        CellularSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
            freq_u: 0.02,
            freq_v: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }

    fn with_seed(&mut self, seed: i32) -> &mut CellularSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut CellularSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self.freq_u = freq;
        self.freq_v = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self
    }

    fn with_freq_4d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
    ) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self
    }

    fn with_freq_5d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
    ) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self.freq_u = freq_u;
        self
    }

    fn with_freq_6d(
        &mut self,
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
        freq_u: f32,
        freq_v: f32,
    ) -> &mut CellularSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self.freq_z = freq_z;
        self.freq_w = freq_w;
        self.freq_u = freq_u;
        self.freq_v = freq_v;
        self
    }

    fn get_freq_x(&self) -> f32 {
        self.freq_x
    }

    fn get_freq_y(&self) -> f32 {
        self.freq_y
    }

    fn get_freq_z(&self) -> f32 {
        self.freq_z
    }

    fn get_freq_w(&self) -> f32 {
        self.freq_w
    }

    fn get_freq_u(&self) -> f32 {
        self.freq_u
    }

    fn get_freq_v(&self) -> f32 {
        self.freq_v
    }

    fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Cellular(self)
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise(&NoiseType::Cellular(self)),
            2 => get_2d_noise(&NoiseType::Cellular(self)),
            3 => get_3d_noise(&NoiseType::Cellular(self)),
            4 => get_4d_noise(&NoiseType::Cellular(self)),
            5 => get_5d_noise(&NoiseType::Cellular(self)),
            6 => get_6d_noise(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {
        //todo
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Cellular(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Cellular(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Cellular(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Cellular(new_self)),
            5 => get_5d_scaled_noise(&NoiseType::Cellular(new_self)),
            6 => get_6d_scaled_noise(&NoiseType::Cellular(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_64(&NoiseType::Cellular(self)),
            2 => crate::get_2d_noise_64(&NoiseType::Cellular(self)),
            3 => crate::get_3d_noise_64(&NoiseType::Cellular(self)),
            4 => crate::get_4d_noise_64(&NoiseType::Cellular(self)),
            5 => crate::get_5d_noise_64(&NoiseType::Cellular(self)),
            6 => crate::get_6d_noise_64(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_64(&NoiseType::Cellular(self), min, max),
            2 => crate::get_2d_scaled_noise_64(&NoiseType::Cellular(self), min, max),
            3 => crate::get_3d_scaled_noise_64(&NoiseType::Cellular(self), min, max),
            4 => crate::get_4d_scaled_noise_64(&NoiseType::Cellular(self), min, max),
            5 => crate::get_5d_scaled_noise_64(&NoiseType::Cellular(self), min, max),
            6 => crate::get_6d_scaled_noise_64(&NoiseType::Cellular(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into(&NoiseType::Cellular(self), out),
            2 => crate::get_2d_noise_into(&NoiseType::Cellular(self), out),
            3 => crate::get_3d_noise_into(&NoiseType::Cellular(self), out),
            4 => crate::get_4d_noise_into(&NoiseType::Cellular(self), out),
            5 => crate::get_5d_noise_into(&NoiseType::Cellular(self), out),
            6 => crate::get_6d_noise_into(&NoiseType::Cellular(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::get_1d_scaled_noise_into(&NoiseType::Cellular(new_self), out),
            2 => crate::get_2d_scaled_noise_into(&NoiseType::Cellular(new_self), out),
            3 => crate::get_3d_scaled_noise_into(&NoiseType::Cellular(new_self), out),
            4 => crate::get_4d_scaled_noise_into(&NoiseType::Cellular(new_self), out),
            5 => crate::get_5d_scaled_noise_into(&NoiseType::Cellular(new_self), out),
            6 => crate::get_6d_scaled_noise_into(&NoiseType::Cellular(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into_64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into_64(&NoiseType::Cellular(self), out),
            2 => crate::get_2d_noise_into_64(&NoiseType::Cellular(self), out),
            3 => crate::get_3d_noise_into_64(&NoiseType::Cellular(self), out),
            4 => crate::get_4d_noise_into_64(&NoiseType::Cellular(self), out),
            5 => crate::get_5d_noise_into_64(&NoiseType::Cellular(self), out),
            6 => crate::get_6d_noise_into_64(&NoiseType::Cellular(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into_64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_into_64(&NoiseType::Cellular(self), min, max, out),
            2 => crate::get_2d_scaled_noise_into_64(&NoiseType::Cellular(self), min, max, out),
            3 => crate::get_3d_scaled_noise_into_64(&NoiseType::Cellular(self), min, max, out),
            4 => crate::get_4d_scaled_noise_into_64(&NoiseType::Cellular(self), min, max, out),
            5 => crate::get_5d_scaled_noise_into_64(&NoiseType::Cellular(self), min, max, out),
            6 => crate::get_6d_scaled_noise_into_64(&NoiseType::Cellular(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_range_into<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_range_into::<T>(
                &NoiseType::Cellular(self),
                src_min,
                src_max,
                out,
            ),
            2 => crate::get_2d_noise_range_into::<T>(
                &NoiseType::Cellular(self),
                src_min,
                src_max,
                out,
            ),
            3 => crate::get_3d_noise_range_into::<T>(
                &NoiseType::Cellular(self),
                src_min,
                src_max,
                out,
            ),
            4 => crate::get_4d_noise_range_into::<T>(
                &NoiseType::Cellular(self),
                src_min,
                src_max,
                out,
            ),
            5 => crate::get_5d_noise_range_into::<T>(
                &NoiseType::Cellular(self),
                src_min,
                src_max,
                out,
            ),
            6 => crate::get_6d_noise_range_into::<T>(
                &NoiseType::Cellular(self),
                src_min,
                src_max,
                out,
            ),
            _ => panic!("not implemented"),
        }
    }
}

impl<S: Simd> Sample32<S> for CellularSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        dispatch_cell!(
            cellular_1d,
            self.distance_function,
            self.return_type,
            x,
            S::Vf32::set1(self.jitter),
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        dispatch_cell!(
            cellular_2d,
            self.distance_function,
            self.return_type,
            x,
            y,
            S::Vf32::set1(self.jitter),
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        dispatch_cell!(
            cellular_3d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            S::Vf32::set1(self.jitter),
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        dispatch_cell!(
            cellular_4d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            S::Vf32::set1(self.jitter),
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32 {
        dispatch_cell!(
            cellular_5d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            S::Vf32::set1(self.jitter),
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_6d(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
        t: S::Vf32,
    ) -> S::Vf32 {
        dispatch_cell!(
            cellular_6d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf32::set1(self.jitter),
            self.dim.seed
        )
    }
}

impl<S: Simd> Sample64<S> for CellularSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        dispatch_cell!(
            cellular_1d_f64,
            self.distance_function,
            self.return_type,
            x,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        dispatch_cell!(
            cellular_2d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        dispatch_cell!(
            cellular_3d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        dispatch_cell!(
            cellular_4d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64 {
        dispatch_cell!(
            cellular_5d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_6d(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
        t: S::Vf64,
    ) -> S::Vf64 {
        dispatch_cell!(
            cellular_6d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf64::set1(self.jitter.into()),
            self.dim.seed.into()
        )
    }
}

impl CellularSettings {
    pub fn with_distance_function(&mut self, dist: CellDistanceFunction) -> &mut CellularSettings {
        self.distance_function = dist;
        self
    }

    pub fn with_return_type(&mut self, return_type: CellReturnType) -> &mut CellularSettings {
        self.return_type = return_type;
        self
    }

    pub fn with_jitter(&mut self, jitter: f32) -> &mut CellularSettings {
        self.jitter = jitter;
        self
    }
}
