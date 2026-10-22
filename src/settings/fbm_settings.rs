use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::fbm_32::{
    fbm_1d, fbm_1d_deriv, fbm_2d, fbm_2d_deriv, fbm_3d, fbm_3d_deriv, fbm_4d, fbm_4d_deriv, fbm_5d,
    fbm_5d_deriv, fbm_6d, fbm_6d_deriv,
};
use crate::noise::fbm_64::{
    fbm_1d as fbm_1d_f64, fbm_1d_deriv as fbm_1d_deriv_f64, fbm_2d as fbm_2d_f64,
    fbm_2d_deriv as fbm_2d_deriv_f64, fbm_3d as fbm_3d_f64, fbm_3d_deriv as fbm_3d_deriv_f64,
    fbm_4d as fbm_4d_f64, fbm_4d_deriv as fbm_4d_deriv_f64, fbm_5d as fbm_5d_f64,
    fbm_5d_deriv as fbm_5d_deriv_f64, fbm_6d as fbm_6d_f64, fbm_6d_deriv as fbm_6d_deriv_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::{Sample32, SampleFractalDeriv32};
use crate::noise_helpers_64::{Sample64, SampleFractalDeriv64};
pub use crate::noise_type::NoiseType;
use crate::{
    generate_1d_fbm_gradient, generate_1d_noise, generate_1d_scaled_noise, generate_2d_fbm_gradient, generate_2d_noise,
    generate_2d_scaled_noise, generate_3d_fbm_gradient, generate_3d_noise, generate_3d_scaled_noise,
    generate_4d_fbm_gradient, generate_4d_noise, generate_4d_scaled_noise, generate_5d_fbm_gradient, generate_5d_noise,
    generate_5d_scaled_noise, generate_6d_fbm_gradient, generate_6d_noise, generate_6d_scaled_noise,
    generate_1d_fbm_gradient_f64, generate_2d_fbm_gradient_f64, generate_3d_fbm_gradient_f64, generate_4d_fbm_gradient_f64, generate_5d_fbm_gradient_f64, generate_6d_fbm_gradient_f64, NoiseAndGradient, NoiseAndGradientF64,
};

use super::{Settings, SimplexSettings};

/// **Fractional Brownian motion (fBm)** — the classic layered "natural" noise.
///
/// fBm sums several [octaves](Self::octaves) of plain gradient noise
/// ([`GradientSettings`](crate::GradientSettings)), each at a different frequency and amplitude, into one
/// field. The big low-frequency octaves give the overall shape and the small
/// high-frequency ones add fine detail, the way real terrain, clouds, and many
/// textures combine features at many scales. It is the most common general-purpose
/// noise — a good default when you want something that simply looks natural.
///
/// The three knobs below ([`octaves`](Self::octaves), [`lacunarity`](Self::lacunarity),
/// [`gain`](Self::gain)) control that layering. See
/// [fractional Brownian motion](https://en.wikipedia.org/wiki/Fractional_Brownian_motion).
///
/// **Output range:** `[-A, A]`, where `A` is the sum of the per-octave amplitudes
/// (`1 + gain + … + gain^(octaves-1)`, plus the fractional top octave). The bound
/// is loose — the field rarely reaches the extremes. Query it with
/// [`output_range`](crate::Settings::output_range).
#[derive(Copy, Clone)]
pub struct FbmSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: how quickly the base noise varies per sample
    /// step (this is the frequency of the *first* octave). Higher = finer detail,
    /// lower = larger features. See [`GradientSettings::frequency_x`](crate::GradientSettings::frequency_x).
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
    /// Frequency ratio between successive octaves: each octave multiplies the
    /// sample coordinates by this factor (`coord *= lacunarity` per octave), so it
    /// controls how much finer (or coarser) each layer is than the last. The
    /// conventional choice is `> 1` (e.g. `2.0`, doubling the frequency each
    /// octave); this crate's default is `0.5`. Larger gaps between octave
    /// frequencies look rougher. See [Lacunarity](https://en.wikipedia.org/wiki/Lacunarity).
    pub lacunarity: f32,
    /// Amplitude ratio between successive octaves, a.k.a. *persistence*: each
    /// octave's contribution is multiplied by this factor (`amplitude *= gain` per
    /// octave). The conventional choice is `< 1` (e.g. `0.5`) so each finer octave
    /// adds less, keeping the result smooth; this crate's default is `2.0`.
    /// Together with `lacunarity` it sets how rough vs. smooth the field looks.
    pub gain: f32,
    /// How many layers of noise to sum. `1` is plain gradient noise; each extra
    /// octave adds another, more detailed layer (and costs another noise
    /// evaluation per sample). A fractional value blends in a partial last octave.
    /// Typical values are `1`–`8`; the default is `3`.
    pub octaves: f32,
}

impl DimensionalBeing for FbmSettings {
    fn dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for FbmSettings {
    fn default(dim: NoiseDimensions) -> FbmSettings {
        FbmSettings {
            dim,
            frequency_x: 0.02,
            frequency_y: 0.02,
            frequency_z: 0.02,
            frequency_w: 0.02,
            frequency_u: 0.02,
            frequency_v: 0.02,
            lacunarity: 0.5,
            gain: 2.0,
            octaves: 3.0,
        }
    }
    fn with_seed(&mut self, seed: i32) -> &mut FbmSettings {
        self.dim.seed = seed;
        self
    }

    fn with_frequency(&mut self, frequency: f32) -> &mut FbmSettings {
        self.frequency_x = frequency;
        self.frequency_y = frequency;
        self.frequency_z = frequency;
        self.frequency_w = frequency;
        self.frequency_u = frequency;
        self.frequency_v = frequency;
        self
    }

    fn with_frequency_2d(&mut self, frequency_x: f32, frequency_y: f32) -> &mut FbmSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self
    }

    fn with_frequency_3d(&mut self, frequency_x: f32, frequency_y: f32, frequency_z: f32) -> &mut FbmSettings {
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
    ) -> &mut FbmSettings {
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
    ) -> &mut FbmSettings {
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
    ) -> &mut FbmSettings {
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
        NoiseType::Fbm(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dimension;
        match d {
            1 => generate_1d_noise(&NoiseType::Fbm(self)),
            2 => generate_2d_noise(&NoiseType::Fbm(self)),
            3 => generate_3d_noise(&NoiseType::Fbm(self)),
            4 => generate_4d_noise(&NoiseType::Fbm(self)),
            5 => generate_5d_noise(&NoiseType::Fbm(self)),
            6 => generate_6d_noise(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dimension;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => generate_1d_scaled_noise(&NoiseType::Fbm(new_self)),
            2 => generate_2d_scaled_noise(&NoiseType::Fbm(new_self)),
            3 => generate_3d_scaled_noise(&NoiseType::Fbm(new_self)),
            4 => generate_4d_scaled_noise(&NoiseType::Fbm(new_self)),
            5 => generate_5d_scaled_noise(&NoiseType::Fbm(new_self)),
            6 => generate_6d_scaled_noise(&NoiseType::Fbm(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_f64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dimension;
        match d {
            1 => crate::generate_1d_noise_f64(&NoiseType::Fbm(self)),
            2 => crate::generate_2d_noise_f64(&NoiseType::Fbm(self)),
            3 => crate::generate_3d_noise_f64(&NoiseType::Fbm(self)),
            4 => crate::generate_4d_noise_f64(&NoiseType::Fbm(self)),
            5 => crate::generate_5d_noise_f64(&NoiseType::Fbm(self)),
            6 => crate::generate_6d_noise_f64(&NoiseType::Fbm(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_f64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dimension;
        match d {
            1 => crate::generate_1d_scaled_noise_f64(&NoiseType::Fbm(self), min, max),
            2 => crate::generate_2d_scaled_noise_f64(&NoiseType::Fbm(self), min, max),
            3 => crate::generate_3d_scaled_noise_f64(&NoiseType::Fbm(self), min, max),
            4 => crate::generate_4d_scaled_noise_f64(&NoiseType::Fbm(self), min, max),
            5 => crate::generate_5d_scaled_noise_f64(&NoiseType::Fbm(self), min, max),
            6 => crate::generate_6d_scaled_noise_f64(&NoiseType::Fbm(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn fill(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise(&NoiseType::Fbm(self), out),
            2 => crate::fill_2d_noise(&NoiseType::Fbm(self), out),
            3 => crate::fill_3d_noise(&NoiseType::Fbm(self), out),
            4 => crate::fill_4d_noise(&NoiseType::Fbm(self), out),
            5 => crate::fill_5d_noise(&NoiseType::Fbm(self), out),
            6 => crate::fill_6d_noise(&NoiseType::Fbm(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_scaled(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dimension;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::fill_1d_scaled_noise(&NoiseType::Fbm(new_self), out),
            2 => crate::fill_2d_scaled_noise(&NoiseType::Fbm(new_self), out),
            3 => crate::fill_3d_scaled_noise(&NoiseType::Fbm(new_self), out),
            4 => crate::fill_4d_scaled_noise(&NoiseType::Fbm(new_self), out),
            5 => crate::fill_5d_scaled_noise(&NoiseType::Fbm(new_self), out),
            6 => crate::fill_6d_scaled_noise(&NoiseType::Fbm(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_f64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise_f64(&NoiseType::Fbm(self), out),
            2 => crate::fill_2d_noise_f64(&NoiseType::Fbm(self), out),
            3 => crate::fill_3d_noise_f64(&NoiseType::Fbm(self), out),
            4 => crate::fill_4d_noise_f64(&NoiseType::Fbm(self), out),
            5 => crate::fill_5d_noise_f64(&NoiseType::Fbm(self), out),
            6 => crate::fill_6d_noise_f64(&NoiseType::Fbm(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_scaled_f64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_scaled_noise_f64(&NoiseType::Fbm(self), min, max, out),
            2 => crate::fill_2d_scaled_noise_f64(&NoiseType::Fbm(self), min, max, out),
            3 => crate::fill_3d_scaled_noise_f64(&NoiseType::Fbm(self), min, max, out),
            4 => crate::fill_4d_scaled_noise_f64(&NoiseType::Fbm(self), min, max, out),
            5 => crate::fill_5d_scaled_noise_f64(&NoiseType::Fbm(self), min, max, out),
            6 => crate::fill_6d_scaled_noise_f64(&NoiseType::Fbm(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_range<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise_range::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            2 => crate::fill_2d_noise_range::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            3 => crate::fill_3d_noise_range::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            4 => crate::fill_4d_noise_range::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            5 => crate::fill_5d_noise_range::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            6 => crate::fill_6d_noise_range::<T>(&NoiseType::Fbm(self), src_min, src_max, out),
            _ => panic!("not implemented"),
        }
    }
}

impl SimplexSettings for FbmSettings {
    fn with_lacunarity(&mut self, lacunarity: f32) -> &mut FbmSettings {
        self.lacunarity = lacunarity;
        self
    }

    fn with_gain(&mut self, gain: f32) -> &mut FbmSettings {
        self.gain = gain;
        self
    }

    fn with_octaves(&mut self, octaves: f32) -> &mut FbmSettings {
        self.octaves = octaves;
        self
    }
}

impl<S: Simd> Sample32<S> for FbmSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        fbm_1d::<S>(
            x,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        fbm_2d::<S>(
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
        fbm_3d::<S>(
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
        fbm_4d::<S>(
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
        fbm_5d::<S>(
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
        fbm_6d::<S>(
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

impl<S: Simd> SampleFractalDeriv32<S> for FbmSettings {
    #[inline(always)]
    fn sample_1d_deriv(&self, x: S::Vf32) -> (S::Vf32, S::Vf32) {
        fbm_1d_deriv::<S>(
            x,
            S::Vf32::set1(self.lacunarity),
            S::Vf32::set1(self.gain),
            self.octaves,
            self.dim.seed,
        )
    }

    #[inline(always)]
    fn sample_2d_deriv(&self, x: S::Vf32, y: S::Vf32) -> (S::Vf32, [S::Vf32; 2]) {
        fbm_2d_deriv::<S>(
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
        fbm_3d_deriv::<S>(
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
        fbm_4d_deriv::<S>(
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
        fbm_5d_deriv::<S>(
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
        fbm_6d_deriv::<S>(
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

impl<S: Simd> Sample64<S> for FbmSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        fbm_1d_f64::<S>(
            x,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        fbm_2d_f64::<S>(
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
        fbm_3d_f64::<S>(
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
        fbm_4d_f64::<S>(
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
        fbm_5d_f64::<S>(
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
        fbm_6d_f64::<S>(
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

impl<S: Simd> SampleFractalDeriv64<S> for FbmSettings {
    #[inline(always)]
    fn sample_1d_deriv(&self, x: S::Vf64) -> (S::Vf64, S::Vf64) {
        fbm_1d_deriv_f64::<S>(
            x,
            S::Vf64::set1(self.lacunarity.into()),
            S::Vf64::set1(self.gain.into()),
            self.octaves,
            self.dim.seed.into(),
        )
    }
    #[inline(always)]
    fn sample_2d_deriv(&self, x: S::Vf64, y: S::Vf64) -> (S::Vf64, [S::Vf64; 2]) {
        fbm_2d_deriv_f64::<S>(
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
        fbm_3d_deriv_f64::<S>(
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
        fbm_4d_deriv_f64::<S>(
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
        fbm_5d_deriv_f64::<S>(
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
        fbm_6d_deriv_f64::<S>(
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

impl FbmSettings {
    /// Generate fBm (fractal) noise **together with its analytic gradient** in a
    /// single pass (issue #24).
    ///
    /// Like [`GradientSettings::generate_with_gradient`](crate::GradientSettings::generate_with_gradient),
    /// but for the full octave stack: the returned [`NoiseAndGradient`] carries
    /// the fBm `values` (identical to [`generate`](Settings::generate)) and the
    /// matching per-sample `gradients`, obtained by differentiating every octave
    /// and summing (each octave contributes `gain^k * lacunarity^k` times its
    /// simplex gradient). This gives smooth analytic normals/slopes for layered
    /// noise without finite differencing. See [`NoiseAndGradient`] for the exact
    /// layout and coordinate convention.
    ///
    /// Supported for all dimensions 1D–6D.
    ///
    /// # Panics
    /// Panics if the configured dimensionality is not in 1..=6.
    pub fn generate_with_gradient(self) -> NoiseAndGradient {
        let d = self.dim.dimension;
        match d {
            1 => generate_1d_fbm_gradient(&self),
            2 => generate_2d_fbm_gradient(&self),
            3 => generate_3d_fbm_gradient(&self),
            4 => generate_4d_fbm_gradient(&self),
            5 => generate_5d_fbm_gradient(&self),
            6 => generate_6d_fbm_gradient(&self),
            _ => panic!("generate_with_gradient: invalid dimension {}", d),
        }
    }

    /// f64 counterpart of [`generate_with_gradient`](Self::generate_with_gradient),
    /// returning a [`NoiseAndGradientF64`]. Supported for all dimensions 1D–6D.
    ///
    /// # Panics
    /// Panics if the configured dimensionality is not in 1..=6.
    pub fn generate_with_gradient_f64(self) -> NoiseAndGradientF64 {
        let d = self.dim.dimension;
        match d {
            1 => generate_1d_fbm_gradient_f64(&self),
            2 => generate_2d_fbm_gradient_f64(&self),
            3 => generate_3d_fbm_gradient_f64(&self),
            4 => generate_4d_fbm_gradient_f64(&self),
            5 => generate_5d_fbm_gradient_f64(&self),
            6 => generate_6d_fbm_gradient_f64(&self),
            _ => panic!("generate_with_gradient_f64: invalid dimension {}", d),
        }
    }
}
