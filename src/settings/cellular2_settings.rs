use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::cell2_32::{
    cellular2_1d, cellular2_2d, cellular2_3d, cellular2_4d, cellular2_5d, cellular2_6d,
};
use crate::noise::cell2_64::{
    cellular2_1d as cellular2_1d_f64, cellular2_2d as cellular2_2d_f64,
    cellular2_3d as cellular2_3d_f64, cellular2_4d as cellular2_4d_f64,
    cellular2_5d as cellular2_5d_f64, cellular2_6d as cellular2_6d_f64,
};
pub use crate::noise::cell2_return_type::Cell2ReturnType;
pub use crate::noise::cell_distance_function::CellDistanceFunction;
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
pub use crate::noise_type::NoiseType;
use crate::{
    get_1d_noise, get_1d_scaled_noise, get_2d_cellular2_gradient, get_2d_cellular2_gradient_64,
    get_2d_noise, get_2d_scaled_noise, get_3d_cellular2_gradient, get_3d_cellular2_gradient_64,
    get_3d_noise, get_3d_scaled_noise, get_4d_noise, get_4d_scaled_noise, get_5d_noise,
    get_5d_scaled_noise, get_6d_noise, get_6d_scaled_noise, NoiseAndGradient, NoiseAndGradient64,
};

use super::Settings;

/// Select the monomorphized cellular2 kernel for the runtime (distance, return)
/// configuration. Runs once per `sample_*` call, outside the kernel's inner
/// loops; `index0`/`index1` stay runtime arguments (array indices / `while`
/// bound), so only the distance metric and the final combine are specialized.
macro_rules! dispatch_cell2 {
    ($call:ident, $df:expr, $rt:expr, $($arg:expr),* $(,)?) => {{
        use crate::noise::cell_strategy as cs;
        use Cell2ReturnType::*;
        use CellDistanceFunction::*;
        match ($df, $rt) {
            (Euclidean, Distance2) => $call::<S, cs::Euclidean, cs::Distance2>($($arg),*),
            (Euclidean, Distance2Add) => $call::<S, cs::Euclidean, cs::Distance2Add>($($arg),*),
            (Euclidean, Distance2Sub) => $call::<S, cs::Euclidean, cs::Distance2Sub>($($arg),*),
            (Euclidean, Distance2Mul) => $call::<S, cs::Euclidean, cs::Distance2Mul>($($arg),*),
            (Euclidean, Distance2Div) => $call::<S, cs::Euclidean, cs::Distance2Div>($($arg),*),
            (Manhattan, Distance2) => $call::<S, cs::Manhattan, cs::Distance2>($($arg),*),
            (Manhattan, Distance2Add) => $call::<S, cs::Manhattan, cs::Distance2Add>($($arg),*),
            (Manhattan, Distance2Sub) => $call::<S, cs::Manhattan, cs::Distance2Sub>($($arg),*),
            (Manhattan, Distance2Mul) => $call::<S, cs::Manhattan, cs::Distance2Mul>($($arg),*),
            (Manhattan, Distance2Div) => $call::<S, cs::Manhattan, cs::Distance2Div>($($arg),*),
            (Natural, Distance2) => $call::<S, cs::Natural, cs::Distance2>($($arg),*),
            (Natural, Distance2Add) => $call::<S, cs::Natural, cs::Distance2Add>($($arg),*),
            (Natural, Distance2Sub) => $call::<S, cs::Natural, cs::Distance2Sub>($($arg),*),
            (Natural, Distance2Mul) => $call::<S, cs::Natural, cs::Distance2Mul>($($arg),*),
            (Natural, Distance2Div) => $call::<S, cs::Natural, cs::Distance2Div>($($arg),*),
        }
    }};
}

/// **Second-order cellular noise** — Worley patterns from the *two* nearest cells.
///
/// Where [`CellularSettings`](crate::CellularSettings) looks only at the single nearest feature point,
/// this variant finds the two points you pick with [`index0`](Self::index0) and
/// [`index1`](Self::index1) (by default the 1st- and 2nd-nearest) and **combines
/// their distances** with [`return_type`](Self::return_type). The most useful
/// combination, F2 − F1 ([`Distance2Sub`](Cell2ReturnType::Distance2Sub)), is
/// near zero right at a cell border and larger inside, which draws the crisp
/// cell-edge / crack-network / cracked-glass patterns Worley noise is known for.
///
/// It shares the cell shape ([`distance_function`](Self::distance_function)),
/// density ([`freq_x`](Self::freq_x) …), and [`jitter`](Self::jitter) controls
/// with [`CellularSettings`](crate::CellularSettings). See
/// [Worley / cellular noise](https://en.wikipedia.org/wiki/Worley_noise).
///
/// **Output range:** `Distance2Div` is `[0, 1]`; the other combinations are
/// jitter/metric-dependent with no closed form and so are scaled per-block.
#[derive(Copy, Clone)]
pub struct Cellular2Settings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: sets the **cell density** (higher = smaller,
    /// more numerous cells). See [`GradientSettings::freq_x`](crate::GradientSettings::freq_x).
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
    /// How distance to a feature point is measured (the cell shape): Euclidean,
    /// Manhattan, or Natural. See [`CellularSettings::distance_function`](crate::CellularSettings::distance_function).
    pub distance_function: CellDistanceFunction,
    /// How the two selected distances (F1 at `index0`, F2 at `index1`) are
    /// combined: [`Distance2`](Cell2ReturnType::Distance2) (just F2),
    /// [`Distance2Add`](Cell2ReturnType::Distance2Add) (F1+F2),
    /// [`Distance2Sub`](Cell2ReturnType::Distance2Sub) (F2−F1 — the classic
    /// cell-edge/crack look), [`Distance2Mul`](Cell2ReturnType::Distance2Mul)
    /// (F1·F2), or [`Distance2Div`](Cell2ReturnType::Distance2Div) (F1/F2).
    pub return_type: Cell2ReturnType,
    /// How far each feature point is nudged from its cell centre, `0.0` (regular
    /// grid) to `1.0` (fully random). Default `0.25`. See
    /// [`CellularSettings::jitter`](crate::CellularSettings::jitter).
    pub jitter: f32,
    /// Rank of the **first** feature point to use, 0-based: `0` = nearest (F1),
    /// `1` = 2nd-nearest, `2` = 3rd. Must be less than [`index1`](Self::index1).
    /// Default `0`.
    pub index0: usize,
    /// Rank of the **second** feature point to use: `1` = 2nd-nearest (F2), `2` =
    /// 3rd, `3` = 4th. Must be greater than [`index0`](Self::index0). The two
    /// distances are combined per [`return_type`](Self::return_type). Default `1`.
    pub index1: usize,
}

impl DimensionalBeing for Cellular2Settings {
    fn get_dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for Cellular2Settings {
    fn default(dim: NoiseDimensions) -> Cellular2Settings {
        Cellular2Settings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
            freq_u: 0.02,
            freq_v: 0.02,
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
        self.freq_w = freq;
        self.freq_u = freq;
        self.freq_v = freq;
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
        freq_x: f32,
        freq_y: f32,
        freq_z: f32,
        freq_w: f32,
    ) -> &mut Cellular2Settings {
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
    ) -> &mut Cellular2Settings {
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
    ) -> &mut Cellular2Settings {
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
        NoiseType::Cellular2(self)
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise(&NoiseType::Cellular2(self)),
            2 => get_2d_noise(&NoiseType::Cellular2(self)),
            3 => get_3d_noise(&NoiseType::Cellular2(self)),
            4 => get_4d_noise(&NoiseType::Cellular2(self)),
            5 => get_5d_noise(&NoiseType::Cellular2(self)),
            6 => get_6d_noise(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
        }
    }

    fn validate(&self) {
        if self.index0 > 2 || self.index1 > 3 || self.index0 >= self.index1 {
            panic!("invalid index settings in cellular2 noise");
        }
        // 1D cellular2 visits only 3 candidate cells, so the 4th-nearest slot is
        // never filled; cap index1 accordingly.
        if self.dim.dim == 1 && self.index1 > 2 {
            panic!("1D cellular2 has only 3 candidate cells; index1 must be <= 2");
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        self.validate();
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Cellular2(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Cellular2(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Cellular2(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Cellular2(new_self)),
            5 => get_5d_scaled_noise(&NoiseType::Cellular2(new_self)),
            6 => get_6d_scaled_noise(&NoiseType::Cellular2(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_64(&NoiseType::Cellular2(self)),
            2 => crate::get_2d_noise_64(&NoiseType::Cellular2(self)),
            3 => crate::get_3d_noise_64(&NoiseType::Cellular2(self)),
            4 => crate::get_4d_noise_64(&NoiseType::Cellular2(self)),
            5 => crate::get_5d_noise_64(&NoiseType::Cellular2(self)),
            6 => crate::get_6d_noise_64(&NoiseType::Cellular2(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_64(self, min: f64, max: f64) -> Vec<f64> {
        self.validate();
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_64(&NoiseType::Cellular2(self), min, max),
            2 => crate::get_2d_scaled_noise_64(&NoiseType::Cellular2(self), min, max),
            3 => crate::get_3d_scaled_noise_64(&NoiseType::Cellular2(self), min, max),
            4 => crate::get_4d_scaled_noise_64(&NoiseType::Cellular2(self), min, max),
            5 => crate::get_5d_scaled_noise_64(&NoiseType::Cellular2(self), min, max),
            6 => crate::get_6d_scaled_noise_64(&NoiseType::Cellular2(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into(&NoiseType::Cellular2(self), out),
            2 => crate::get_2d_noise_into(&NoiseType::Cellular2(self), out),
            3 => crate::get_3d_noise_into(&NoiseType::Cellular2(self), out),
            4 => crate::get_4d_noise_into(&NoiseType::Cellular2(self), out),
            5 => crate::get_5d_noise_into(&NoiseType::Cellular2(self), out),
            6 => crate::get_6d_noise_into(&NoiseType::Cellular2(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into(self, min: f32, max: f32, out: &mut [f32]) {
        self.validate();
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::get_1d_scaled_noise_into(&NoiseType::Cellular2(new_self), out),
            2 => crate::get_2d_scaled_noise_into(&NoiseType::Cellular2(new_self), out),
            3 => crate::get_3d_scaled_noise_into(&NoiseType::Cellular2(new_self), out),
            4 => crate::get_4d_scaled_noise_into(&NoiseType::Cellular2(new_self), out),
            5 => crate::get_5d_scaled_noise_into(&NoiseType::Cellular2(new_self), out),
            6 => crate::get_6d_scaled_noise_into(&NoiseType::Cellular2(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into_64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into_64(&NoiseType::Cellular2(self), out),
            2 => crate::get_2d_noise_into_64(&NoiseType::Cellular2(self), out),
            3 => crate::get_3d_noise_into_64(&NoiseType::Cellular2(self), out),
            4 => crate::get_4d_noise_into_64(&NoiseType::Cellular2(self), out),
            5 => crate::get_5d_noise_into_64(&NoiseType::Cellular2(self), out),
            6 => crate::get_6d_noise_into_64(&NoiseType::Cellular2(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into_64(self, min: f64, max: f64, out: &mut [f64]) {
        self.validate();
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_into_64(&NoiseType::Cellular2(self), min, max, out),
            2 => crate::get_2d_scaled_noise_into_64(&NoiseType::Cellular2(self), min, max, out),
            3 => crate::get_3d_scaled_noise_into_64(&NoiseType::Cellular2(self), min, max, out),
            4 => crate::get_4d_scaled_noise_into_64(&NoiseType::Cellular2(self), min, max, out),
            5 => crate::get_5d_scaled_noise_into_64(&NoiseType::Cellular2(self), min, max, out),
            6 => crate::get_6d_scaled_noise_into_64(&NoiseType::Cellular2(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_range_into<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        self.validate();
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_range_into::<T>(
                &NoiseType::Cellular2(self),
                src_min,
                src_max,
                out,
            ),
            2 => crate::get_2d_noise_range_into::<T>(
                &NoiseType::Cellular2(self),
                src_min,
                src_max,
                out,
            ),
            3 => crate::get_3d_noise_range_into::<T>(
                &NoiseType::Cellular2(self),
                src_min,
                src_max,
                out,
            ),
            4 => crate::get_4d_noise_range_into::<T>(
                &NoiseType::Cellular2(self),
                src_min,
                src_max,
                out,
            ),
            5 => crate::get_5d_noise_range_into::<T>(
                &NoiseType::Cellular2(self),
                src_min,
                src_max,
                out,
            ),
            6 => crate::get_6d_noise_range_into::<T>(
                &NoiseType::Cellular2(self),
                src_min,
                src_max,
                out,
            ),
            _ => panic!("not implemented"),
        }
    }
}

impl<S: Simd> Sample32<S> for Cellular2Settings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        dispatch_cell2!(
            cellular2_1d,
            self.distance_function,
            self.return_type,
            x,
            S::Vf32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        dispatch_cell2!(
            cellular2_2d,
            self.distance_function,
            self.return_type,
            x,
            y,
            S::Vf32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        dispatch_cell2!(
            cellular2_3d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            S::Vf32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        dispatch_cell2!(
            cellular2_4d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            S::Vf32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed
        )
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32 {
        dispatch_cell2!(
            cellular2_5d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            S::Vf32::set1(self.jitter),
            self.index0,
            self.index1,
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
        dispatch_cell2!(
            cellular2_6d,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf32::set1(self.jitter),
            self.index0,
            self.index1,
            self.dim.seed
        )
    }
}

impl<S: Simd> Sample64<S> for Cellular2Settings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        dispatch_cell2!(
            cellular2_1d_f64,
            self.distance_function,
            self.return_type,
            x,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        dispatch_cell2!(
            cellular2_2d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        dispatch_cell2!(
            cellular2_3d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        dispatch_cell2!(
            cellular2_4d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into()
        )
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64 {
        dispatch_cell2!(
            cellular2_5d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
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
        dispatch_cell2!(
            cellular2_6d_f64,
            self.distance_function,
            self.return_type,
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf64::set1(self.jitter.into()),
            self.index0,
            self.index1,
            self.dim.seed.into()
        )
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

    /// Value + analytic gradient of cellular2 (Worley F1/F2) noise (issue #24).
    ///
    /// Supported only for the **Euclidean** distance function, where each
    /// selected distance is a squared distance `Fₖ = Σ dₖ²` with gradient
    /// `−2·(sample − feature)`. The configured [`return_type`](Self::return_type)
    /// combination of `F1 = distance[index0]` and `F2 = distance[index1]`
    /// (F2, F1±F2, F1·F2, F1/F2) is differentiated by the product/quotient rule.
    /// The gradient is defined everywhere except on the Voronoi cell borders,
    /// where a feature point switches sorted slots (the same kinks the value
    /// has); `Distance2Div` additionally diverges where `F2 → 0`.
    ///
    /// Only 2D and 3D are implemented.
    ///
    /// # Panics
    /// Panics unless `distance_function` is `Euclidean` and the dimensionality
    /// is 2 or 3.
    pub fn generate_with_derivative(self) -> NoiseAndGradient {
        assert!(
            matches!(self.distance_function, CellDistanceFunction::Euclidean),
            "cellular2 generate_with_derivative requires the Euclidean distance function"
        );
        match self.dim.dim {
            2 => get_2d_cellular2_gradient(&self),
            3 => get_3d_cellular2_gradient(&self),
            d => panic!(
                "cellular2 generate_with_derivative is only implemented for 2D/3D (got {})",
                d
            ),
        }
    }

    /// `f64` twin of [`generate_with_derivative`](Self::generate_with_derivative).
    ///
    /// Same Euclidean restriction, 2D/3D support and border semantics, computed
    /// and returned in double precision ([`NoiseAndGradient64`]).
    ///
    /// # Panics
    /// Panics unless `distance_function` is `Euclidean` and the dimensionality
    /// is 2 or 3.
    pub fn generate_with_derivative_64(self) -> NoiseAndGradient64 {
        assert!(
            matches!(self.distance_function, CellDistanceFunction::Euclidean),
            "cellular2 generate_with_derivative_64 requires the Euclidean distance function"
        );
        match self.dim.dim {
            2 => get_2d_cellular2_gradient_64(&self),
            3 => get_3d_cellular2_gradient_64(&self),
            d => panic!(
                "cellular2 generate_with_derivative_64 is only implemented for 2D/3D (got {})",
                d
            ),
        }
    }
}
