use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::ridge_32::{
    ridge_1d, ridge_1d_deriv, ridge_2d, ridge_2d_deriv, ridge_3d, ridge_3d_deriv, ridge_4d,
    ridge_4d_deriv, ridge_5d, ridge_5d_deriv, ridge_6d, ridge_6d_deriv,
};
use crate::noise::ridge_64::{
    ridge_1d as ridge_1d_f64, ridge_1d_deriv as ridge_1d_deriv_f64, ridge_2d as ridge_2d_f64,
    ridge_2d_deriv as ridge_2d_deriv_f64, ridge_3d as ridge_3d_f64,
    ridge_3d_deriv as ridge_3d_deriv_f64, ridge_4d as ridge_4d_f64,
    ridge_4d_deriv as ridge_4d_deriv_f64, ridge_5d as ridge_5d_f64,
    ridge_5d_deriv as ridge_5d_deriv_f64, ridge_6d as ridge_6d_f64,
    ridge_6d_deriv as ridge_6d_deriv_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::{Sample32, SampleFractalDeriv32};
use crate::noise_helpers_64::{Sample64, SampleFractalDeriv64};
pub use crate::noise_type::NoiseType;
use crate::{
    get_1d_noise, get_1d_ridge_gradient, get_1d_scaled_noise, get_2d_noise, get_2d_ridge_gradient,
    get_2d_scaled_noise, get_3d_noise, get_3d_ridge_gradient, get_3d_scaled_noise, get_4d_noise,
    get_4d_ridge_gradient, get_4d_scaled_noise, get_5d_noise, get_5d_ridge_gradient,
    get_5d_scaled_noise, get_6d_noise, get_6d_ridge_gradient, get_6d_scaled_noise,
    get_ridge_gradient_64, NoiseAndGradient, NoiseAndGradient64,
};

use super::{Settings, SimplexSettings};

/// **Ridged noise** — fBm's sharp-crested cousin, for ridgelines and creases.
///
/// Like [`FbmSettings`](crate::FbmSettings) it sums several [octaves](Self::octaves) of gradient
/// noise, but each octave contributes `1 - |noise|` instead of the raw value.
/// Taking the absolute value folds the field at every zero-crossing, and the
/// `1 -` flips those folds into **sharp peaks with smooth valleys** between them.
/// The result looks like mountain ridgelines, canyon networks, river/vein
/// patterns, or cracks — anywhere you want crisp lines rather than soft blobs.
///
/// Note that (unlike fBm) output is biased positive and can exceed 1, so you will
/// usually want `generate_scaled` / a scaling pass. See [`FbmSettings`](crate::FbmSettings) for the
/// shared layering knobs.
///
/// **Output range:** `[octaves - A, octaves]`, with `A` the per-octave amplitude
/// sum (as for [`FbmSettings`](crate::FbmSettings)); the maximum is `octaves`.
#[derive(Copy, Clone)]
pub struct RidgeSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis of the first octave: higher = finer detail,
    /// lower = larger features. See [`GradientSettings::freq_x`](crate::GradientSettings::freq_x).
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
    /// Frequency ratio between successive octaves (`coord *= lacunarity` per
    /// octave): how much finer/coarser each layer is than the last. Default `0.5`.
    /// See [`FbmSettings::lacunarity`](crate::FbmSettings::lacunarity) and
    /// [Lacunarity](https://en.wikipedia.org/wiki/Lacunarity).
    pub lacunarity: f32,
    /// Amplitude ratio between successive octaves (*persistence*): the running
    /// amplitude starts at 1 and is multiplied by `gain` each octave, scaling that
    /// octave's noise before the `1 - |…|` fold. Default `2.0`. See
    /// [`FbmSettings::gain`](crate::FbmSettings::gain).
    pub gain: f32,
    /// How many octaves to sum: `1` is a single ridged layer; more octaves add
    /// finer ridges (and cost more). Fractional values blend a partial last
    /// octave. Default `3`. See [`FbmSettings::octaves`](crate::FbmSettings::octaves).
    pub octaves: f32,
}

impl DimensionalBeing for RidgeSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for RidgeSettings {
    fn default(dim: NoiseDimensions) -> RidgeSettings {
        RidgeSettings {
            dim,
            freq_x: 0.02,
            freq_y: 0.02,
            freq_z: 0.02,
            freq_w: 0.02,
            freq_u: 0.02,
            freq_v: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3.0,
        }
    }

    fn with_seed(&mut self, seed: i32) -> &mut RidgeSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut RidgeSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self.freq_u = freq;
        self.freq_v = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut RidgeSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut RidgeSettings {
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
    ) -> &mut RidgeSettings {
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
    ) -> &mut RidgeSettings {
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
    ) -> &mut RidgeSettings {
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
        NoiseType::Ridge(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise(&NoiseType::Ridge(self)),
            2 => get_2d_noise(&NoiseType::Ridge(self)),
            3 => get_3d_noise(&NoiseType::Ridge(self)),
            4 => get_4d_noise(&NoiseType::Ridge(self)),
            5 => get_5d_noise(&NoiseType::Ridge(self)),
            6 => get_6d_noise(&NoiseType::Ridge(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Ridge(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Ridge(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Ridge(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Ridge(new_self)),
            5 => get_5d_scaled_noise(&NoiseType::Ridge(new_self)),
            6 => get_6d_scaled_noise(&NoiseType::Ridge(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_64(&NoiseType::Ridge(self)),
            2 => crate::get_2d_noise_64(&NoiseType::Ridge(self)),
            3 => crate::get_3d_noise_64(&NoiseType::Ridge(self)),
            4 => crate::get_4d_noise_64(&NoiseType::Ridge(self)),
            5 => crate::get_5d_noise_64(&NoiseType::Ridge(self)),
            6 => crate::get_6d_noise_64(&NoiseType::Ridge(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_64(&NoiseType::Ridge(self), min, max),
            2 => crate::get_2d_scaled_noise_64(&NoiseType::Ridge(self), min, max),
            3 => crate::get_3d_scaled_noise_64(&NoiseType::Ridge(self), min, max),
            4 => crate::get_4d_scaled_noise_64(&NoiseType::Ridge(self), min, max),
            5 => crate::get_5d_scaled_noise_64(&NoiseType::Ridge(self), min, max),
            6 => crate::get_6d_scaled_noise_64(&NoiseType::Ridge(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into(&NoiseType::Ridge(self), out),
            2 => crate::get_2d_noise_into(&NoiseType::Ridge(self), out),
            3 => crate::get_3d_noise_into(&NoiseType::Ridge(self), out),
            4 => crate::get_4d_noise_into(&NoiseType::Ridge(self), out),
            5 => crate::get_5d_noise_into(&NoiseType::Ridge(self), out),
            6 => crate::get_6d_noise_into(&NoiseType::Ridge(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::get_1d_scaled_noise_into(&NoiseType::Ridge(new_self), out),
            2 => crate::get_2d_scaled_noise_into(&NoiseType::Ridge(new_self), out),
            3 => crate::get_3d_scaled_noise_into(&NoiseType::Ridge(new_self), out),
            4 => crate::get_4d_scaled_noise_into(&NoiseType::Ridge(new_self), out),
            5 => crate::get_5d_scaled_noise_into(&NoiseType::Ridge(new_self), out),
            6 => crate::get_6d_scaled_noise_into(&NoiseType::Ridge(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into_64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into_64(&NoiseType::Ridge(self), out),
            2 => crate::get_2d_noise_into_64(&NoiseType::Ridge(self), out),
            3 => crate::get_3d_noise_into_64(&NoiseType::Ridge(self), out),
            4 => crate::get_4d_noise_into_64(&NoiseType::Ridge(self), out),
            5 => crate::get_5d_noise_into_64(&NoiseType::Ridge(self), out),
            6 => crate::get_6d_noise_into_64(&NoiseType::Ridge(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into_64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_into_64(&NoiseType::Ridge(self), min, max, out),
            2 => crate::get_2d_scaled_noise_into_64(&NoiseType::Ridge(self), min, max, out),
            3 => crate::get_3d_scaled_noise_into_64(&NoiseType::Ridge(self), min, max, out),
            4 => crate::get_4d_scaled_noise_into_64(&NoiseType::Ridge(self), min, max, out),
            5 => crate::get_5d_scaled_noise_into_64(&NoiseType::Ridge(self), min, max, out),
            6 => crate::get_6d_scaled_noise_into_64(&NoiseType::Ridge(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_range_into<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dim;
        match d {
            1 => {
                crate::get_1d_noise_range_into::<T>(&NoiseType::Ridge(self), src_min, src_max, out)
            }
            2 => {
                crate::get_2d_noise_range_into::<T>(&NoiseType::Ridge(self), src_min, src_max, out)
            }
            3 => {
                crate::get_3d_noise_range_into::<T>(&NoiseType::Ridge(self), src_min, src_max, out)
            }
            4 => {
                crate::get_4d_noise_range_into::<T>(&NoiseType::Ridge(self), src_min, src_max, out)
            }
            5 => {
                crate::get_5d_noise_range_into::<T>(&NoiseType::Ridge(self), src_min, src_max, out)
            }
            6 => {
                crate::get_6d_noise_range_into::<T>(&NoiseType::Ridge(self), src_min, src_max, out)
            }
            _ => panic!("not implemented"),
        }
    }
}

impl SimplexSettings for RidgeSettings {
    fn with_lacunarity(&mut self, lacunarity: f32) -> &mut RidgeSettings {
        self.lacunarity = lacunarity;
        self
    }

    fn with_gain(&mut self, gain: f32) -> &mut RidgeSettings {
        self.gain = gain;
        self
    }

    fn with_octaves(&mut self, octaves: f32) -> &mut RidgeSettings {
        self.octaves = octaves;
        self
    }
}

impl<S: Simd> Sample32<S> for RidgeSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        ridge_1d::<S>(
            x,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        ridge_2d::<S>(
            x,
            y,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        ridge_3d::<S>(
            x,
            y,
            z,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        ridge_4d::<S>(
            x,
            y,
            z,
            w,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32 {
        ridge_5d::<S>(
            x,
            y,
            z,
            w,
            u,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
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
        ridge_6d::<S>(
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }
}

impl<S: Simd> Sample64<S> for RidgeSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        ridge_1d_f64::<S>(
            x,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        ridge_2d_f64::<S>(
            x,
            y,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        ridge_3d_f64::<S>(
            x,
            y,
            z,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        ridge_4d_f64::<S>(
            x,
            y,
            z,
            w,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64 {
        ridge_5d_f64::<S>(
            x,
            y,
            z,
            w,
            u,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
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
        ridge_6d_f64::<S>(
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
}

impl<S: Simd> SampleFractalDeriv64<S> for RidgeSettings {
    #[inline(always)]
    fn sample_1d_deriv(&self, x: S::Vf64) -> (S::Vf64, S::Vf64) {
        ridge_1d_deriv_f64::<S>(
            x,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
    #[inline(always)]
    fn sample_2d_deriv(&self, x: S::Vf64, y: S::Vf64) -> (S::Vf64, [S::Vf64; 2]) {
        ridge_2d_deriv_f64::<S>(
            x,
            y,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
    #[inline(always)]
    fn sample_3d_deriv(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> (S::Vf64, [S::Vf64; 3]) {
        ridge_3d_deriv_f64::<S>(
            x,
            y,
            z,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
    #[inline(always)]
    fn sample_4d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 4]) {
        ridge_4d_deriv_f64::<S>(
            x,
            y,
            z,
            w,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
    #[inline(always)]
    fn sample_5d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 5]) {
        ridge_5d_deriv_f64::<S>(
            x,
            y,
            z,
            w,
            u,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
    #[inline(always)]
    fn sample_6d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
        u: S::Vf64,
        t: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 6]) {
        ridge_6d_deriv_f64::<S>(
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
}

impl RidgeSettings {
    /// Generate ridge noise **together with its analytic gradient** in a single
    /// pass (issue #24).
    ///
    /// Like [`GradientSettings::generate_with_derivative`](crate::GradientSettings::generate_with_derivative),
    /// but for ridge noise. Ridge stacks `1 - |simplex|` octaves, so its gradient
    /// uses the sign of each octave's simplex value (the derivative of `|·|`);
    /// the returned [`NoiseAndGradient`] carries values identical to
    /// [`generate`](Settings::generate) plus the matching per-sample gradients.
    /// The gradient is continuous except at the ridge crests, where `|·|` has a
    /// kink. See [`NoiseAndGradient`] for the layout and coordinate convention.
    ///
    /// Supported for all dimensions 1D–6D.
    ///
    /// # Panics
    /// Panics if the configured dimensionality is not in 1..=6.
    pub fn generate_with_derivative(self) -> NoiseAndGradient {
        let d = self.dim.dim;
        match d {
            1 => get_1d_ridge_gradient(&self),
            2 => get_2d_ridge_gradient(&self),
            3 => get_3d_ridge_gradient(&self),
            4 => get_4d_ridge_gradient(&self),
            5 => get_5d_ridge_gradient(&self),
            6 => get_6d_ridge_gradient(&self),
            _ => panic!("generate_with_derivative: invalid dimension {}", d),
        }
    }

    /// f64 counterpart of [`generate_with_derivative`](Self::generate_with_derivative),
    /// returning a [`NoiseAndGradient64`]. Supported for all dimensions 1D–6D.
    ///
    /// # Panics
    /// Panics if the configured dimensionality is not in 1..=6.
    pub fn generate_with_derivative_64(self) -> NoiseAndGradient64 {
        get_ridge_gradient_64(&self)
    }
}

impl<S: Simd> SampleFractalDeriv32<S> for RidgeSettings {
    #[inline(always)]
    fn sample_1d_deriv(&self, x: S::Vf32) -> (S::Vf32, S::Vf32) {
        ridge_1d_deriv::<S>(
            x,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_2d_deriv(&self, x: S::Vf32, y: S::Vf32) -> (S::Vf32, [S::Vf32; 2]) {
        ridge_2d_deriv::<S>(
            x,
            y,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_3d_deriv(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> (S::Vf32, [S::Vf32; 3]) {
        ridge_3d_deriv::<S>(
            x,
            y,
            z,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_4d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 4]) {
        ridge_4d_deriv::<S>(
            x,
            y,
            z,
            w,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_5d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 5]) {
        ridge_5d_deriv::<S>(
            x,
            y,
            z,
            w,
            u,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_6d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
        u: S::Vf32,
        t: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 6]) {
        ridge_6d_deriv::<S>(
            x,
            y,
            z,
            w,
            u,
            t,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }
}
