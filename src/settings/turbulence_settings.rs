use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::turbulence_32::{
    turbulence_1d, turbulence_2d, turbulence_3d, turbulence_4d, turbulence_5d, turbulence_6d,
};
use crate::noise::turbulence_64::{
    turbulence_1d as turbulence_1d_f64, turbulence_2d as turbulence_2d_f64,
    turbulence_3d as turbulence_3d_f64, turbulence_4d as turbulence_4d_f64,
    turbulence_5d as turbulence_5d_f64, turbulence_6d as turbulence_6d_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::Sample32;
use crate::noise_helpers_64::Sample64;
pub use crate::noise_type::NoiseType;
use crate::{
    get_1d_noise, get_1d_scaled_noise, get_2d_noise, get_2d_scaled_noise, get_3d_noise,
    get_3d_scaled_noise, get_4d_noise, get_4d_scaled_noise, get_5d_noise, get_5d_scaled_noise,
    get_6d_noise, get_6d_scaled_noise,
};

use super::{Settings, SimplexSettings};

/// **Turbulence noise** — billowy, puffy noise with soft creases.
///
/// Like [`FbmSettings`](crate::FbmSettings) it sums several [octaves](Self::octaves) of gradient
/// noise, but each octave contributes `|noise|` (its absolute value). Folding the
/// field at every zero-crossing turns the smooth waves into rolling, puffy lobes
/// separated by sharp creases — the classic look of smoke, clouds, fire, and
/// marble veining. (It is the same idea as [`RidgeSettings`](crate::RidgeSettings) without the `1 -`
/// flip, so the creases point down into valleys instead of up into ridges.)
///
/// Output is biased positive, so a `generate_scaled` / scaling pass is usually
/// wanted. See [`FbmSettings`](crate::FbmSettings) for the shared layering knobs.
///
/// **Output range:** `[0, A]`, with `A` the per-octave amplitude sum (as for
/// [`FbmSettings`](crate::FbmSettings)).
#[derive(Copy, Clone)]
pub struct TurbulenceSettings {
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
    /// octave). Default `0.5`. See [`FbmSettings::lacunarity`](crate::FbmSettings::lacunarity) and
    /// [Lacunarity](https://en.wikipedia.org/wiki/Lacunarity).
    pub lacunarity: f32,
    /// Amplitude ratio between successive octaves (*persistence*): the running
    /// amplitude starts at 1 and is multiplied by `gain` each octave, scaling that
    /// octave's noise before the `|…|` fold. Default `2.0`. See
    /// [`FbmSettings::gain`](crate::FbmSettings::gain).
    pub gain: f32,
    /// How many octaves to sum: `1` is a single turbulent layer; more octaves add
    /// finer puffs (and cost more). Fractional values blend a partial last octave.
    /// Default `3`. See [`FbmSettings::octaves`](crate::FbmSettings::octaves).
    pub octaves: f32,
}

impl DimensionalBeing for TurbulenceSettings {
    fn get_dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for TurbulenceSettings {
    fn default(dim: NoiseDimensions) -> TurbulenceSettings {
        TurbulenceSettings {
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

    fn with_seed(&mut self, seed: i32) -> &mut TurbulenceSettings {
        self.dim.seed = seed;
        self
    }

    fn with_freq(&mut self, freq: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq;
        self.freq_y = freq;
        self.freq_z = freq;
        self.freq_w = freq;
        self.freq_u = freq;
        self.freq_v = freq;
        self
    }

    fn with_freq_2d(&mut self, freq_x: f32, freq_y: f32) -> &mut TurbulenceSettings {
        self.freq_x = freq_x;
        self.freq_y = freq_y;
        self
    }

    fn with_freq_3d(&mut self, freq_x: f32, freq_y: f32, freq_z: f32) -> &mut TurbulenceSettings {
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
    ) -> &mut TurbulenceSettings {
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
    ) -> &mut TurbulenceSettings {
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
    ) -> &mut TurbulenceSettings {
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
        NoiseType::Turbulence(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => get_1d_noise(&NoiseType::Turbulence(self)),
            2 => get_2d_noise(&NoiseType::Turbulence(self)),
            3 => get_3d_noise(&NoiseType::Turbulence(self)),
            4 => get_4d_noise(&NoiseType::Turbulence(self)),
            5 => get_5d_noise(&NoiseType::Turbulence(self)),
            6 => get_6d_noise(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => get_1d_scaled_noise(&NoiseType::Turbulence(new_self)),
            2 => get_2d_scaled_noise(&NoiseType::Turbulence(new_self)),
            3 => get_3d_scaled_noise(&NoiseType::Turbulence(new_self)),
            4 => get_4d_scaled_noise(&NoiseType::Turbulence(new_self)),
            5 => get_5d_scaled_noise(&NoiseType::Turbulence(new_self)),
            6 => get_6d_scaled_noise(&NoiseType::Turbulence(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_64(&NoiseType::Turbulence(self)),
            2 => crate::get_2d_noise_64(&NoiseType::Turbulence(self)),
            3 => crate::get_3d_noise_64(&NoiseType::Turbulence(self)),
            4 => crate::get_4d_noise_64(&NoiseType::Turbulence(self)),
            5 => crate::get_5d_noise_64(&NoiseType::Turbulence(self)),
            6 => crate::get_6d_noise_64(&NoiseType::Turbulence(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_64(&NoiseType::Turbulence(self), min, max),
            2 => crate::get_2d_scaled_noise_64(&NoiseType::Turbulence(self), min, max),
            3 => crate::get_3d_scaled_noise_64(&NoiseType::Turbulence(self), min, max),
            4 => crate::get_4d_scaled_noise_64(&NoiseType::Turbulence(self), min, max),
            5 => crate::get_5d_scaled_noise_64(&NoiseType::Turbulence(self), min, max),
            6 => crate::get_6d_scaled_noise_64(&NoiseType::Turbulence(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into(&NoiseType::Turbulence(self), out),
            2 => crate::get_2d_noise_into(&NoiseType::Turbulence(self), out),
            3 => crate::get_3d_noise_into(&NoiseType::Turbulence(self), out),
            4 => crate::get_4d_noise_into(&NoiseType::Turbulence(self), out),
            5 => crate::get_5d_noise_into(&NoiseType::Turbulence(self), out),
            6 => crate::get_6d_noise_into(&NoiseType::Turbulence(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dim;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::get_1d_scaled_noise_into(&NoiseType::Turbulence(new_self), out),
            2 => crate::get_2d_scaled_noise_into(&NoiseType::Turbulence(new_self), out),
            3 => crate::get_3d_scaled_noise_into(&NoiseType::Turbulence(new_self), out),
            4 => crate::get_4d_scaled_noise_into(&NoiseType::Turbulence(new_self), out),
            5 => crate::get_5d_scaled_noise_into(&NoiseType::Turbulence(new_self), out),
            6 => crate::get_6d_scaled_noise_into(&NoiseType::Turbulence(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_into_64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_into_64(&NoiseType::Turbulence(self), out),
            2 => crate::get_2d_noise_into_64(&NoiseType::Turbulence(self), out),
            3 => crate::get_3d_noise_into_64(&NoiseType::Turbulence(self), out),
            4 => crate::get_4d_noise_into_64(&NoiseType::Turbulence(self), out),
            5 => crate::get_5d_noise_into_64(&NoiseType::Turbulence(self), out),
            6 => crate::get_6d_noise_into_64(&NoiseType::Turbulence(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_into_64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_scaled_noise_into_64(&NoiseType::Turbulence(self), min, max, out),
            2 => crate::get_2d_scaled_noise_into_64(&NoiseType::Turbulence(self), min, max, out),
            3 => crate::get_3d_scaled_noise_into_64(&NoiseType::Turbulence(self), min, max, out),
            4 => crate::get_4d_scaled_noise_into_64(&NoiseType::Turbulence(self), min, max, out),
            5 => crate::get_5d_scaled_noise_into_64(&NoiseType::Turbulence(self), min, max, out),
            6 => crate::get_6d_scaled_noise_into_64(&NoiseType::Turbulence(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn generate_range_into<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dim;
        match d {
            1 => crate::get_1d_noise_range_into::<T>(
                &NoiseType::Turbulence(self),
                src_min,
                src_max,
                out,
            ),
            2 => crate::get_2d_noise_range_into::<T>(
                &NoiseType::Turbulence(self),
                src_min,
                src_max,
                out,
            ),
            3 => crate::get_3d_noise_range_into::<T>(
                &NoiseType::Turbulence(self),
                src_min,
                src_max,
                out,
            ),
            4 => crate::get_4d_noise_range_into::<T>(
                &NoiseType::Turbulence(self),
                src_min,
                src_max,
                out,
            ),
            5 => crate::get_5d_noise_range_into::<T>(
                &NoiseType::Turbulence(self),
                src_min,
                src_max,
                out,
            ),
            6 => crate::get_6d_noise_range_into::<T>(
                &NoiseType::Turbulence(self),
                src_min,
                src_max,
                out,
            ),
            _ => panic!("not implemented"),
        }
    }
}

impl SimplexSettings for TurbulenceSettings {
    fn with_lacunarity(&mut self, lacunarity: f32) -> &mut TurbulenceSettings {
        self.lacunarity = lacunarity;
        self
    }

    fn with_gain(&mut self, gain: f32) -> &mut TurbulenceSettings {
        self.gain = gain;
        self
    }

    fn with_octaves(&mut self, octaves: f32) -> &mut TurbulenceSettings {
        self.octaves = octaves;
        self
    }
}

impl<S: Simd> Sample32<S> for TurbulenceSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        turbulence_1d::<S>(
            x,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        turbulence_2d::<S>(
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
        turbulence_3d::<S>(
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
        turbulence_4d::<S>(
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
        turbulence_5d::<S>(
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
        turbulence_6d::<S>(
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

impl<S: Simd> Sample64<S> for TurbulenceSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        turbulence_1d_f64::<S>(
            x,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        turbulence_2d_f64::<S>(
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
        turbulence_3d_f64::<S>(
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
        turbulence_4d_f64::<S>(
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
        turbulence_5d_f64::<S>(
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
        turbulence_6d_f64::<S>(
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

impl TurbulenceSettings {}
