use simdeez::prelude::*;

use crate::dimensional_being::DimensionalBeing;
use crate::noise::simplex_32::{
    simplex_1d, simplex_1d_deriv, simplex_2d, simplex_2d_deriv, simplex_3d, simplex_3d_deriv,
    simplex_4d, simplex_4d_deriv, simplex_5d, simplex_5d_deriv, simplex_6d, simplex_6d_deriv,
};
use crate::noise::simplex_64::{
    simplex_1d as simplex_1d_f64, simplex_1d_deriv as simplex_1d_deriv_f64,
    simplex_2d as simplex_2d_f64, simplex_2d_deriv as simplex_2d_deriv_f64,
    simplex_3d as simplex_3d_f64, simplex_3d_deriv as simplex_3d_deriv_f64,
    simplex_4d as simplex_4d_f64, simplex_4d_deriv as simplex_4d_deriv_f64,
    simplex_5d as simplex_5d_f64, simplex_5d_deriv as simplex_5d_deriv_f64,
    simplex_6d as simplex_6d_f64, simplex_6d_deriv as simplex_6d_deriv_f64,
};
pub use crate::noise_dimensions::NoiseDimensions;
use crate::noise_helpers_32::{Sample32, SampleFractalDeriv32};
use crate::noise_helpers_64::{Sample64, SampleFractalDeriv64};
pub use crate::noise_type::NoiseType;
use crate::{
    generate_1d_gradient, generate_1d_noise, generate_1d_scaled_noise, generate_2d_gradient, generate_2d_noise,
    generate_2d_scaled_noise, generate_3d_gradient, generate_3d_noise, generate_3d_scaled_noise, generate_4d_gradient,
    generate_4d_noise, generate_4d_scaled_noise, generate_5d_gradient, generate_5d_noise, generate_5d_scaled_noise,
    generate_6d_gradient, generate_6d_noise, generate_6d_scaled_noise, generate_1d_gradient_f64, generate_2d_gradient_f64, generate_3d_gradient_f64, generate_4d_gradient_f64, generate_5d_gradient_f64, generate_6d_gradient_f64, NoiseAndGradient,
    NoiseAndGradientF64,
};

use crate::settings::Settings;

/// Plain **gradient (simplex) noise** — the smooth, cloud-like "base" noise.
///
/// Gradient noise interpolates between pseudo-random gradients placed on a grid,
/// producing a continuous, organically wavy field with no sharp edges and no
/// visible grid alignment (this crate uses the *simplex* construction, which
/// avoids the directional artifacts of classic Perlin noise). It is the simplest
/// variant and the building block the fractal variants ([`FbmSettings`](crate::FbmSettings),
/// [`RidgeSettings`](crate::RidgeSettings), [`TurbulenceSettings`](crate::TurbulenceSettings)) stack on top of.
///
/// Reach for it when you want one smooth layer of noise — gentle gradients,
/// a single blobby mask, a wobble to offset something by. For richer, more
/// natural detail (terrain, clouds, marble) layer it with a fractal variant.
///
/// See [Gradient noise](https://en.wikipedia.org/wiki/Gradient_noise).
///
/// **Output range:** `[-1, 1]` — a single normalized simplex octave. Query it at
/// runtime with [`output_range`](crate::Settings::output_range).
#[derive(Copy, Clone)]
pub struct GradientSettings {
    dim: NoiseDimensions,
    /// Frequency along the X axis: how quickly the noise varies per sample step.
    /// Higher means finer, busier detail (features packed closer together); lower
    /// means larger, smoother blobs. Think of it as the inverse of feature size.
    pub frequency_x: f32,
    /// Frequency along the Y axis; see [`frequency_x`](Self::frequency_x). Setting the axes
    /// to different values stretches or squashes the pattern in that direction.
    pub frequency_y: f32,
    /// Frequency along the Z axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_z: f32,
    /// Frequency along the 4th (W) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_w: f32,
    /// Frequency along the 5th (U) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_u: f32,
    /// Frequency along the 6th (V) axis; see [`frequency_x`](Self::frequency_x).
    pub frequency_v: f32,
}

impl DimensionalBeing for GradientSettings {
    fn dimensions(&self) -> NoiseDimensions {
        self.dim
    }
}

impl Settings for GradientSettings {
    fn default(dim: NoiseDimensions) -> GradientSettings {
        GradientSettings {
            dim,
            frequency_x: 0.02,
            frequency_y: 0.02,
            frequency_z: 0.02,
            frequency_w: 0.02,
            frequency_u: 0.02,
            frequency_v: 0.02,
        }
    }
    fn with_seed(&mut self, seed: i32) -> &mut GradientSettings {
        self.dim.seed = seed;
        self
    }

    fn with_frequency(&mut self, frequency: f32) -> &mut GradientSettings {
        self.frequency_x = frequency;
        self.frequency_y = frequency;
        self.frequency_z = frequency;
        self.frequency_w = frequency;
        self.frequency_u = frequency;
        self.frequency_v = frequency;
        self
    }

    fn with_frequency_2d(&mut self, frequency_x: f32, frequency_y: f32) -> &mut GradientSettings {
        self.frequency_x = frequency_x;
        self.frequency_y = frequency_y;
        self
    }

    fn with_frequency_3d(&mut self, frequency_x: f32, frequency_y: f32, frequency_z: f32) -> &mut GradientSettings {
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
    ) -> &mut GradientSettings {
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
    ) -> &mut GradientSettings {
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
    ) -> &mut GradientSettings {
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
        NoiseType::Gradient(self)
    }

    fn validate(&self) {
        //todo
    }

    fn generate(self) -> (Vec<f32>, f32, f32) {
        let d = self.dim.dimension;
        match d {
            1 => generate_1d_noise(&NoiseType::Gradient(self)),
            2 => generate_2d_noise(&NoiseType::Gradient(self)),
            3 => generate_3d_noise(&NoiseType::Gradient(self)),
            4 => generate_4d_noise(&NoiseType::Gradient(self)),
            5 => generate_5d_noise(&NoiseType::Gradient(self)),
            6 => generate_6d_noise(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled(self, min: f32, max: f32) -> Vec<f32> {
        let d = self.dim.dimension;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => generate_1d_scaled_noise(&NoiseType::Gradient(new_self)),
            2 => generate_2d_scaled_noise(&NoiseType::Gradient(new_self)),
            3 => generate_3d_scaled_noise(&NoiseType::Gradient(new_self)),
            4 => generate_4d_scaled_noise(&NoiseType::Gradient(new_self)),
            5 => generate_5d_scaled_noise(&NoiseType::Gradient(new_self)),
            6 => generate_6d_scaled_noise(&NoiseType::Gradient(new_self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_f64(self) -> (Vec<f64>, f64, f64) {
        let d = self.dim.dimension;
        match d {
            1 => crate::generate_1d_noise_f64(&NoiseType::Gradient(self)),
            2 => crate::generate_2d_noise_f64(&NoiseType::Gradient(self)),
            3 => crate::generate_3d_noise_f64(&NoiseType::Gradient(self)),
            4 => crate::generate_4d_noise_f64(&NoiseType::Gradient(self)),
            5 => crate::generate_5d_noise_f64(&NoiseType::Gradient(self)),
            6 => crate::generate_6d_noise_f64(&NoiseType::Gradient(self)),
            _ => panic!("not implemented"),
        }
    }

    fn generate_scaled_f64(self, min: f64, max: f64) -> Vec<f64> {
        let d = self.dim.dimension;
        match d {
            1 => crate::generate_1d_scaled_noise_f64(&NoiseType::Gradient(self), min, max),
            2 => crate::generate_2d_scaled_noise_f64(&NoiseType::Gradient(self), min, max),
            3 => crate::generate_3d_scaled_noise_f64(&NoiseType::Gradient(self), min, max),
            4 => crate::generate_4d_scaled_noise_f64(&NoiseType::Gradient(self), min, max),
            5 => crate::generate_5d_scaled_noise_f64(&NoiseType::Gradient(self), min, max),
            6 => crate::generate_6d_scaled_noise_f64(&NoiseType::Gradient(self), min, max),
            _ => panic!("not implemented"),
        }
    }

    fn fill(self, out: &mut [f32]) -> (f32, f32) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise(&NoiseType::Gradient(self), out),
            2 => crate::fill_2d_noise(&NoiseType::Gradient(self), out),
            3 => crate::fill_3d_noise(&NoiseType::Gradient(self), out),
            4 => crate::fill_4d_noise(&NoiseType::Gradient(self), out),
            5 => crate::fill_5d_noise(&NoiseType::Gradient(self), out),
            6 => crate::fill_6d_noise(&NoiseType::Gradient(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_scaled(self, min: f32, max: f32, out: &mut [f32]) {
        let d = self.dim.dimension;
        let mut new_self = self;
        new_self.dim.min = min;
        new_self.dim.max = max;
        match d {
            1 => crate::fill_1d_scaled_noise(&NoiseType::Gradient(new_self), out),
            2 => crate::fill_2d_scaled_noise(&NoiseType::Gradient(new_self), out),
            3 => crate::fill_3d_scaled_noise(&NoiseType::Gradient(new_self), out),
            4 => crate::fill_4d_scaled_noise(&NoiseType::Gradient(new_self), out),
            5 => crate::fill_5d_scaled_noise(&NoiseType::Gradient(new_self), out),
            6 => crate::fill_6d_scaled_noise(&NoiseType::Gradient(new_self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_f64(self, out: &mut [f64]) -> (f64, f64) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise_f64(&NoiseType::Gradient(self), out),
            2 => crate::fill_2d_noise_f64(&NoiseType::Gradient(self), out),
            3 => crate::fill_3d_noise_f64(&NoiseType::Gradient(self), out),
            4 => crate::fill_4d_noise_f64(&NoiseType::Gradient(self), out),
            5 => crate::fill_5d_noise_f64(&NoiseType::Gradient(self), out),
            6 => crate::fill_6d_noise_f64(&NoiseType::Gradient(self), out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_scaled_f64(self, min: f64, max: f64, out: &mut [f64]) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_scaled_noise_f64(&NoiseType::Gradient(self), min, max, out),
            2 => crate::fill_2d_scaled_noise_f64(&NoiseType::Gradient(self), min, max, out),
            3 => crate::fill_3d_scaled_noise_f64(&NoiseType::Gradient(self), min, max, out),
            4 => crate::fill_4d_scaled_noise_f64(&NoiseType::Gradient(self), min, max, out),
            5 => crate::fill_5d_scaled_noise_f64(&NoiseType::Gradient(self), min, max, out),
            6 => crate::fill_6d_scaled_noise_f64(&NoiseType::Gradient(self), min, max, out),
            _ => panic!("not implemented"),
        }
    }

    fn fill_range<T: crate::NoiseInto>(self, src_min: f32, src_max: f32, out: &mut [T]) {
        let d = self.dim.dimension;
        match d {
            1 => crate::fill_1d_noise_range::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            2 => crate::fill_2d_noise_range::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            3 => crate::fill_3d_noise_range::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            4 => crate::fill_4d_noise_range::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            5 => crate::fill_5d_noise_range::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            6 => crate::fill_6d_noise_range::<T>(
                &NoiseType::Gradient(self),
                src_min,
                src_max,
                out,
            ),
            _ => panic!("not implemented"),
        }
    }
}

impl<S: Simd> Sample32<S> for GradientSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf32) -> S::Vf32 {
        simplex_1d::<S>(x, self.dim.seed)
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf32, y: S::Vf32) -> S::Vf32 {
        simplex_2d::<S>(x, y, self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> S::Vf32 {
        simplex_3d::<S>(x, y, z, self.dim.seed)
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32) -> S::Vf32 {
        simplex_4d::<S>(x, y, z, w, self.dim.seed)
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32, w: S::Vf32, u: S::Vf32) -> S::Vf32 {
        simplex_5d::<S>(x, y, z, w, u, self.dim.seed)
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
        simplex_6d::<S>(x, y, z, w, u, t, self.dim.seed)
    }
}

impl<S: Simd> SampleFractalDeriv32<S> for GradientSettings {
    #[inline(always)]
    fn sample_1d_deriv(&self, x: S::Vf32) -> (S::Vf32, S::Vf32) {
        simplex_1d_deriv::<S>(x, self.dim.seed)
    }

    #[inline(always)]
    fn sample_2d_deriv(&self, x: S::Vf32, y: S::Vf32) -> (S::Vf32, [S::Vf32; 2]) {
        simplex_2d_deriv::<S>(x, y, self.dim.seed)
    }

    #[inline(always)]
    fn sample_3d_deriv(&self, x: S::Vf32, y: S::Vf32, z: S::Vf32) -> (S::Vf32, [S::Vf32; 3]) {
        simplex_3d_deriv::<S>(x, y, z, self.dim.seed)
    }

    #[inline(always)]
    fn sample_4d_deriv(
        &self,
        x: S::Vf32,
        y: S::Vf32,
        z: S::Vf32,
        w: S::Vf32,
    ) -> (S::Vf32, [S::Vf32; 4]) {
        simplex_4d_deriv::<S>(x, y, z, w, self.dim.seed)
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
        simplex_5d_deriv::<S>(x, y, z, w, u, self.dim.seed)
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
        simplex_6d_deriv::<S>(x, y, z, w, u, t, self.dim.seed)
    }
}

impl<S: Simd> Sample64<S> for GradientSettings {
    #[inline(always)]
    fn sample_1d(&self, x: S::Vf64) -> S::Vf64 {
        simplex_1d_f64::<S>(x, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_2d(&self, x: S::Vf64, y: S::Vf64) -> S::Vf64 {
        simplex_2d_f64::<S>(x, y, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_3d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> S::Vf64 {
        simplex_3d_f64::<S>(x, y, z, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_4d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64) -> S::Vf64 {
        simplex_4d_f64::<S>(x, y, z, w, self.dim.seed.into())
    }

    #[inline(always)]
    fn sample_5d(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64, w: S::Vf64, u: S::Vf64) -> S::Vf64 {
        simplex_5d_f64::<S>(x, y, z, w, u, self.dim.seed.into())
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
        simplex_6d_f64::<S>(x, y, z, w, u, t, self.dim.seed.into())
    }
}

impl<S: Simd> SampleFractalDeriv64<S> for GradientSettings {
    #[inline(always)]
    fn sample_1d_deriv(&self, x: S::Vf64) -> (S::Vf64, S::Vf64) {
        simplex_1d_deriv_f64::<S>(x, self.dim.seed.into())
    }
    #[inline(always)]
    fn sample_2d_deriv(&self, x: S::Vf64, y: S::Vf64) -> (S::Vf64, [S::Vf64; 2]) {
        simplex_2d_deriv_f64::<S>(x, y, self.dim.seed.into())
    }
    #[inline(always)]
    fn sample_3d_deriv(&self, x: S::Vf64, y: S::Vf64, z: S::Vf64) -> (S::Vf64, [S::Vf64; 3]) {
        simplex_3d_deriv_f64::<S>(x, y, z, self.dim.seed.into())
    }
    #[inline(always)]
    fn sample_4d_deriv(
        &self,
        x: S::Vf64,
        y: S::Vf64,
        z: S::Vf64,
        w: S::Vf64,
    ) -> (S::Vf64, [S::Vf64; 4]) {
        simplex_4d_deriv_f64::<S>(x, y, z, w, self.dim.seed.into())
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
        simplex_5d_deriv_f64::<S>(x, y, z, w, u, self.dim.seed.into())
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
        simplex_6d_deriv_f64::<S>(x, y, z, w, u, t, self.dim.seed.into())
    }
}

impl GradientSettings {
    /// Generate gradient (simplex) noise **together with its analytic gradient**
    /// in a single pass (issue #24).
    ///
    /// The returned [`NoiseAndGradient`] carries the noise `values` (identical to
    /// what [`generate`](Settings::generate) produces for these settings) and a
    /// flattened array of per-sample `gradients`. Each gradient component is the
    /// derivative of the value with respect to the sample position along that
    /// axis — ideal for analytic surface normals or displacement without
    /// finite-difference sampling. See [`NoiseAndGradient`] for the exact layout
    /// and coordinate convention.
    ///
    /// Supported for all dimensions 1D–6D.
    ///
    /// # Panics
    /// Panics if the configured dimensionality is not in 1..=6.
    pub fn generate_with_gradient(self) -> NoiseAndGradient {
        let d = self.dim.dimension;
        match d {
            1 => generate_1d_gradient(&self),
            2 => generate_2d_gradient(&self),
            3 => generate_3d_gradient(&self),
            4 => generate_4d_gradient(&self),
            5 => generate_5d_gradient(&self),
            6 => generate_6d_gradient(&self),
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
            1 => generate_1d_gradient_f64(&self),
            2 => generate_2d_gradient_f64(&self),
            3 => generate_3d_gradient_f64(&self),
            4 => generate_4d_gradient_f64(&self),
            5 => generate_5d_gradient_f64(&self),
            6 => generate_6d_gradient_f64(&self),
            _ => panic!("generate_with_gradient_f64: invalid dimension {}", d),
        }
    }
}
