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
    generate_1d_noise, generate_1d_scaled_noise, generate_2d_cellular_gradient,
    generate_2d_cellular_gradient_f64, generate_2d_noise, generate_2d_scaled_noise,
    generate_3d_cellular_gradient, generate_3d_cellular_gradient_f64, generate_3d_noise,
    generate_3d_scaled_noise, generate_4d_noise, generate_4d_scaled_noise, generate_5d_noise,
    generate_5d_scaled_noise, generate_6d_noise, generate_6d_scaled_noise, NoiseAndGradient,
    NoiseAndGradientF64,
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
///
/// **Output range:** `CellValue` is `[-1, 1)` (jitter-independent); `Distance`
/// has no closed form (it depends on jitter and the distance function) and so is
/// scaled per-block. See [`output_range`](crate::Settings::output_range).
#[derive(Copy, Clone)]
pub struct CellularSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: here it sets the **cell density** — higher
    /// means smaller, more numerous cells; lower means fewer, larger cells. See
    /// [`GradientSettings::frequency_x`](crate::GradientSettings::frequency_x).
    pub frequency_x: f32,
    /// Frequency along the Y axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_y: f32,
    /// Frequency along the Z axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_z: f32,
    /// Frequency along the 4th (W) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_w: f32,
    /// Frequency along the 5th (U) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_u: f32,
    /// Frequency along the 6th (V) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_v: f32,
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
    fn dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for CellularSettings {
    fn default(dim: NoiseDimensions) -> CellularSettings {
        CellularSettings {
            dim,
            frequency_x: 0.02,
            frequency_y: 0.02,
            frequency_z: 0.02,
            frequency_w: 0.02,
            frequency_u: 0.02,
            frequency_v: 0.02,
            distance_function: CellDistanceFunction::Euclidean,
            return_type: CellReturnType::Distance,
            jitter: 0.25,
        }
    }

    fn with_seed(&mut self, seed: i32) -> &mut CellularSettings {
        self.dim.seed = seed;
        self
    }

    fn with_frequency(&mut self, frequency: f32) -> &mut CellularSettings {
        self.frequency_x = frequency;
        self.frequency_y = frequency;
        self.frequency_z = frequency;
        self.frequency_w = frequency;
        self.frequency_u = frequency;
        self.frequency_v = frequency;
        self
    }

    fn with_frequency_2d(&mut self, frequency_x: f32, frequency_y: f32) -> &mut CellularSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self
    }

    fn with_frequency_3d(
        &mut self,
        frequency_x: f32,
        frequency_y: f32,
        frequency_z: f32,
    ) -> &mut CellularSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self.frequency_z = frequency_z;
        self
    }

    fn with_frequency_4d(
        &mut self,
        frequency_x: f32,
        frequency_y: f32,
        frequency_z: f32,
        frequency_w: f32,
    ) -> &mut CellularSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self.frequency_z = frequency_z;
        self.frequency_w = frequency_w;
        self
    }

    fn with_frequency_5d(
        &mut self,
        frequency_x: f32,
        frequency_y: f32,
        frequency_z: f32,
        frequency_w: f32,
        frequency_u: f32,
    ) -> &mut CellularSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self.frequency_z = frequency_z;
        self.frequency_w = frequency_w;
        self.frequency_u = frequency_u;
        self
    }

    fn with_frequency_6d(
        &mut self,
        frequency_x: f32,
        frequency_y: f32,
        frequency_z: f32,
        frequency_w: f32,
        frequency_u: f32,
        frequency_v: f32,
    ) -> &mut CellularSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self.frequency_z = frequency_z;
        self.frequency_w = frequency_w;
        self.frequency_u = frequency_u;
        self.frequency_v = frequency_v;
        self
    }

    fn frequency_x(&self) -> f32 {
        self.frequency_x
    }

    fn frequency_y(&self) -> f32 {
        self.frequency_y
    }

    fn frequency_z(&self) -> f32 {
        self.frequency_z
    }

    fn frequency_w(&self) -> f32 {
        self.frequency_w
    }

    fn frequency_u(&self) -> f32 {
        self.frequency_u
    }

    fn frequency_v(&self) -> f32 {
        self.frequency_v
    }

    fn wrap(self) -> NoiseType {
        self.validate();
        NoiseType::Cellular(self)
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dimension;
        match d {
            1 => generate_1d_noise(&NoiseType::Cellular(self)),
            2 => generate_2d_noise(&NoiseType::Cellular(self)),
            3 => generate_3d_noise(&NoiseType::Cellular(self)),
            4 => generate_4d_noise(&NoiseType::Cellular(self)),
            5 => generate_5d_noise(&NoiseType::Cellular(self)),
            6 => generate_6d_noise(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {
        //todo
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dimension;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => generate_1d_scaled_noise(&NoiseType::Cellular(new_self)),
            2 => generate_2d_scaled_noise(&NoiseType::Cellular(new_self)),
            3 => generate_3d_scaled_noise(&NoiseType::Cellular(new_self)),
            4 => generate_4d_scaled_noise(&NoiseType::Cellular(new_self)),
            5 => generate_5d_scaled_noise(&NoiseType::Cellular(new_self)),
            6 => generate_6d_scaled_noise(&NoiseType::Cellular(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_f64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dimension;
        match d {
            1 => crate::generate_1d_noise_f64(&NoiseType::Cellular(self)),
            2 => crate::generate_2d_noise_f64(&NoiseType::Cellular(self)),
            3 => crate::generate_3d_noise_f64(&NoiseType::Cellular(self)),
            4 => crate::generate_4d_noise_f64(&NoiseType::Cellular(self)),
            5 => crate::generate_5d_noise_f64(&NoiseType::Cellular(self)),
            6 => crate::generate_6d_noise_f64(&NoiseType::Cellular(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_f64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dimension;
        match d {
            1 => crate::generate_1d_scaled_noise_f64(&NoiseType::Cellular(self), min, max),
            2 => crate::generate_2d_scaled_noise_f64(&NoiseType::Cellular(self), min, max),
            3 => crate::generate_3d_scaled_noise_f64(&NoiseType::Cellular(self), min, max),
            4 => crate::generate_4d_scaled_noise_f64(&NoiseType::Cellular(self), min, max),
            5 => crate::generate_5d_scaled_noise_f64(&NoiseType::Cellular(self), min, max),
            6 => crate::generate_6d_scaled_noise_f64(&NoiseType::Cellular(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn fill(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise(&NoiseType::Cellular(self), out),
            2 => crate::fill_2d_noise(&NoiseType::Cellular(self), out),
            3 => crate::fill_3d_noise(&NoiseType::Cellular(self), out),
            4 => crate::fill_4d_noise(&NoiseType::Cellular(self), out),
            5 => crate::fill_5d_noise(&NoiseType::Cellular(self), out),
            6 => crate::fill_6d_noise(&NoiseType::Cellular(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_scaled(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dimension;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::fill_1d_scaled_noise(&NoiseType::Cellular(new_self), out),
            2 => crate::fill_2d_scaled_noise(&NoiseType::Cellular(new_self), out),
            3 => crate::fill_3d_scaled_noise(&NoiseType::Cellular(new_self), out),
            4 => crate::fill_4d_scaled_noise(&NoiseType::Cellular(new_self), out),
            5 => crate::fill_5d_scaled_noise(&NoiseType::Cellular(new_self), out),
            6 => crate::fill_6d_scaled_noise(&NoiseType::Cellular(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_f64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise_f64(&NoiseType::Cellular(self), out),
            2 => crate::fill_2d_noise_f64(&NoiseType::Cellular(self), out),
            3 => crate::fill_3d_noise_f64(&NoiseType::Cellular(self), out),
            4 => crate::fill_4d_noise_f64(&NoiseType::Cellular(self), out),
            5 => crate::fill_5d_noise_f64(&NoiseType::Cellular(self), out),
            6 => crate::fill_6d_noise_f64(&NoiseType::Cellular(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_scaled_f64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_scaled_noise_f64(&NoiseType::Cellular(self), min, max, out),
            2 => crate::fill_2d_scaled_noise_f64(&NoiseType::Cellular(self), min, max, out),
            3 => crate::fill_3d_scaled_noise_f64(&NoiseType::Cellular(self), min, max, out),
            4 => crate::fill_4d_scaled_noise_f64(&NoiseType::Cellular(self), min, max, out),
            5 => crate::fill_5d_scaled_noise_f64(&NoiseType::Cellular(self), min, max, out),
            6 => crate::fill_6d_scaled_noise_f64(&NoiseType::Cellular(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_range<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise_range::<T>(&NoiseType::Cellular(self), src_min, src_max, out),
            2 => crate::fill_2d_noise_range::<T>(&NoiseType::Cellular(self), src_min, src_max, out),
            3 => crate::fill_3d_noise_range::<T>(&NoiseType::Cellular(self), src_min, src_max, out),
            4 => crate::fill_4d_noise_range::<T>(&NoiseType::Cellular(self), src_min, src_max, out),
            5 => crate::fill_5d_noise_range::<T>(&NoiseType::Cellular(self), src_min, src_max, out),
            6 => crate::fill_6d_noise_range::<T>(&NoiseType::Cellular(self), src_min, src_max, out),
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

    /// Value + analytic gradient of cellular (Worley) noise (issue #24).
    ///
    /// Only the **Euclidean `Distance`** configuration has a usable smooth
    /// gradient: the value is the squared distance to the nearest feature point
    /// (F1) and the gradient is `-2·(sample − feature)`, defined everywhere
    /// except on the Voronoi cell borders (where the nearest point switches).
    /// The `CellValue` return is piecewise-constant (gradient ~0) and the
    /// Manhattan/Natural metrics are non-smooth, so they are not supported here.
    ///
    /// Only 2D and 3D are implemented.
    ///
    /// # Panics
    /// Panics unless `distance_function` is `Euclidean`, `return_type` is
    /// `Distance`, and the dimensionality is 2 or 3.
    pub fn generate_with_gradient(self) -> NoiseAndGradient {
        assert!(
            matches!(self.distance_function, CellDistanceFunction::Euclidean),
            "cellular generate_with_gradient requires the Euclidean distance function"
        );
        assert!(
            matches!(self.return_type, CellReturnType::Distance),
            "cellular generate_with_gradient requires the Distance return type"
        );
        match self.dim.dimension {
            2 => generate_2d_cellular_gradient(&self),
            3 => generate_3d_cellular_gradient(&self),
            d => panic!(
                "cellular generate_with_gradient is only implemented for 2D/3D (got {})",
                d
            ),
        }
    }

    /// `f64` twin of [`generate_with_gradient`](Self::generate_with_gradient).
    ///
    /// Same Euclidean `Distance`, 2D/3D restriction and border semantics, but
    /// computed and returned in double precision ([`NoiseAndGradientF64`]).
    ///
    /// # Panics
    /// Panics unless `distance_function` is `Euclidean`, `return_type` is
    /// `Distance`, and the dimensionality is 2 or 3.
    pub fn generate_with_gradient_f64(self) -> NoiseAndGradientF64 {
        assert!(
            matches!(self.distance_function, CellDistanceFunction::Euclidean),
            "cellular generate_with_gradient_f64 requires the Euclidean distance function"
        );
        assert!(
            matches!(self.return_type, CellReturnType::Distance),
            "cellular generate_with_gradient_f64 requires the Distance return type"
        );
        match self.dim.dimension {
            2 => generate_2d_cellular_gradient_f64(&self),
            3 => generate_3d_cellular_gradient_f64(&self),
            d => panic!(
                "cellular generate_with_gradient_f64 is only implemented for 2D/3D (got {})",
                d
            ),
        }
    }
}
