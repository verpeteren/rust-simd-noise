//! Arbitrary-point evaluation: values, analytic gradients and Hessians.
//!
//! The block API (`Settings::generate*`) evaluates noise on a regular grid.
//! Many consumers -- path tracers, per-vertex displacement, shader-side
//! evaluation -- need values at arbitrary points, optionally with derivatives.
//! The functions in this module pack arbitrary points into SIMD lanes and call
//! the same kernels the block helpers use.
//!
//! Naming: the unqualified `sample_*` functions return values only; the
//! `*_with_gradient` twins return the value plus its analytic first derivative.
//! `*_with_hessian` adds a finite-difference Hessian.
//!
//! Coordinates and gradients are in the settings' own coordinate space: the
//! gradient includes the configured per-axis frequency, matching
//! [`NoiseAndGradient`](crate::NoiseAndGradient) from the block API. The Hessian
//! variants are finite-difference approximations of that analytic gradient, not
//! analytic second derivatives.
//!
//! The batch length must equal the output length. Points are processed in
//! `S::Vf32::WIDTH`/`S::Vf64::WIDTH` chunks; callers can split a large batch into
//! independent ranges and evaluate them in parallel.
//!
//! AIDEV-NOTE: This module is the arbitrary-point counterpart of the grid
//! helpers in `noise_helpers_32/64`. Keep the per-family math in sync with
//! those helpers: the frequency scaling, the cellular return-type combination
//! and the Euclidean-only gradient restrictions are intentional and tested
//! against the block gradient API. The Hessian functions are central
//! differences of the analytic gradient, not analytic second derivatives.
//! AIDEV-TODO: add 1D and 5D/6D point batches if a consumer needs them.
//! 2D-4D cover geometry, textures, shading and animation.

use crate::dimensional_being::DimensionalBeing;
use crate::noise::{
    cell2_32, cell2_64, cell_32, cell_64, fbm_32, fbm_64, gabor_32, ridge_32, ridge_64, simplex_32,
    simplex_64, turbulence_32, turbulence_64,
};
use crate::noise_helpers_32::{Sample32, SampleFractalDeriv32};
use crate::noise_helpers_64::{Sample64, SampleFractalDeriv64};
use crate::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, GaborSettings, NoiseType, Settings,
};
use simdeez::prelude::*;

/// Widest SIMD lane count used by the f32 point batches (AVX-512 is 16).
const MAX_WIDTH_32: usize = 16;
/// Widest SIMD lane count used by the f64 point batches (AVX-512 is 8).
const MAX_WIDTH_64: usize = 8;

/// A 2D point sample: value plus the first derivative with respect to `(x, y)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSample2 {
    /// Noise value at the point.
    pub value: f32,
    /// `[∂value/∂x, ∂value/∂y]`, including the configured frequency.
    pub gradient: [f32; 2],
}

/// A 3D point sample: value plus the first derivative with respect to `(x, y, z)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSample3 {
    /// Noise value at the point.
    pub value: f32,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z]`, including the configured frequency.
    pub gradient: [f32; 3],
}

/// f64 twin of [`PointSample3`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSample3F64 {
    /// Noise value at the point.
    pub value: f64,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z]`, including the configured frequency.
    pub gradient: [f64; 3],
}

/// A 4D point sample: value plus the first derivative with respect to `(x, y, z, w)`.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSample4 {
    /// Noise value at the point.
    pub value: f32,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z, ∂value/∂w]`, including the configured frequency.
    pub gradient: [f32; 4],
}

/// f64 twin of [`PointSample4`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSample4F64 {
    /// Noise value at the point.
    pub value: f64,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z, ∂value/∂w]`, including the configured frequency.
    pub gradient: [f64; 4],
}

/// A 2D complex Gabor (phasor) value without derivatives.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhasorValue2 {
    /// Real part, equal to the ordinary Gabor value.
    pub real: f32,
    /// Imaginary quadrature part.
    pub imag: f32,
}

impl PhasorValue2 {
    /// Complex modulus.
    pub fn modulus(&self) -> f32 {
        self.real.hypot(self.imag)
    }

    /// Complex phase in radians.
    pub fn phase(&self) -> f32 {
        self.imag.atan2(self.real)
    }
}

/// A 2D complex Gabor (phasor) point sample.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhasorSample2 {
    /// Real part, equal to the ordinary Gabor value.
    pub real: f32,
    /// Imaginary quadrature part.
    pub imag: f32,
    /// `[∂real/∂x, ∂real/∂y]`, including the configured frequency.
    pub real_gradient: [f32; 2],
    /// `[∂imag/∂x, ∂imag/∂y]`, including the configured frequency.
    pub imag_gradient: [f32; 2],
}

impl PhasorSample2 {
    /// Complex modulus.
    pub fn modulus(&self) -> f32 {
        self.real.hypot(self.imag)
    }

    /// Complex phase in radians.
    pub fn phase(&self) -> f32 {
        self.imag.atan2(self.real)
    }

    /// Gradient of [`modulus`](Self::modulus), or zero where the field vanishes.
    pub fn modulus_gradient(&self) -> [f32; 2] {
        let modulus = self.modulus();
        if modulus <= f32::EPSILON {
            return [0.0; 2];
        }
        [
            (self.real * self.real_gradient[0] + self.imag * self.imag_gradient[0]) / modulus,
            (self.real * self.real_gradient[1] + self.imag * self.imag_gradient[1]) / modulus,
        ]
    }

    /// Gradient of [`phase`](Self::phase), or zero where the field vanishes.
    pub fn phase_gradient(&self) -> [f32; 2] {
        let denominator = self.real * self.real + self.imag * self.imag;
        if denominator <= f32::EPSILON {
            return [0.0; 2];
        }
        [
            (self.real * self.imag_gradient[0] - self.imag * self.real_gradient[0]) / denominator,
            (self.real * self.imag_gradient[1] - self.imag * self.real_gradient[1]) / denominator,
        ]
    }
}

/// A 3D complex Gabor (phasor) value without derivatives.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhasorValue3 {
    /// Real part, equal to the ordinary Gabor value.
    pub real: f32,
    /// Imaginary quadrature part.
    pub imag: f32,
}

impl PhasorValue3 {
    /// Complex modulus.
    pub fn modulus(&self) -> f32 {
        self.real.hypot(self.imag)
    }

    /// Complex phase in radians.
    pub fn phase(&self) -> f32 {
        self.imag.atan2(self.real)
    }
}

/// A 3D complex Gabor (phasor) point sample.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhasorSample3 {
    /// Real part, equal to the ordinary Gabor value.
    pub real: f32,
    /// Imaginary quadrature part.
    pub imag: f32,
    /// `[∂real/∂x, ∂real/∂y, ∂real/∂z]`, including the configured frequency.
    pub real_gradient: [f32; 3],
    /// `[∂imag/∂x, ∂imag/∂y, ∂imag/∂z]`, including the configured frequency.
    pub imag_gradient: [f32; 3],
}

impl PhasorSample3 {
    /// Complex modulus.
    pub fn modulus(&self) -> f32 {
        self.real.hypot(self.imag)
    }

    /// Complex phase in radians.
    pub fn phase(&self) -> f32 {
        self.imag.atan2(self.real)
    }

    /// Gradient of [`modulus`](Self::modulus), or zero where the field vanishes.
    pub fn modulus_gradient(&self) -> [f32; 3] {
        let modulus = self.modulus();
        if modulus <= f32::EPSILON {
            return [0.0; 3];
        }
        [
            (self.real * self.real_gradient[0] + self.imag * self.imag_gradient[0]) / modulus,
            (self.real * self.real_gradient[1] + self.imag * self.imag_gradient[1]) / modulus,
            (self.real * self.real_gradient[2] + self.imag * self.imag_gradient[2]) / modulus,
        ]
    }

    /// Gradient of [`phase`](Self::phase), or zero where the field vanishes.
    pub fn phase_gradient(&self) -> [f32; 3] {
        let denominator = self.real * self.real + self.imag * self.imag;
        if denominator <= f32::EPSILON {
            return [0.0; 3];
        }
        [
            (self.real * self.imag_gradient[0] - self.imag * self.real_gradient[0]) / denominator,
            (self.real * self.imag_gradient[1] - self.imag * self.real_gradient[1]) / denominator,
            (self.real * self.imag_gradient[2] - self.imag * self.real_gradient[2]) / denominator,
        ]
    }
}

/// A 2D point sample with a finite-difference Hessian.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSampleHessian2 {
    /// Noise value at the point.
    pub value: f32,
    /// `[∂value/∂x, ∂value/∂y]`, including the configured frequency.
    pub gradient: [f32; 2],
    /// `[[∂²value/∂x², ∂²value/∂x∂y], [∂²value/∂y∂x, ∂²value/∂y²]]`.
    pub hessian: [[f32; 2]; 2],
}

/// A 3D point sample with a finite-difference Hessian.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSampleHessian3 {
    /// Noise value at the point.
    pub value: f32,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z]`, including the configured frequency.
    pub gradient: [f32; 3],
    /// `[[∂²value/∂x², ∂²value/∂x∂y, ∂²value/∂x∂z], ...]`.
    pub hessian: [[f32; 3]; 3],
}

/// f64 twin of [`PointSampleHessian3`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSampleHessian3F64 {
    /// Noise value at the point.
    pub value: f64,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z]`, including the configured frequency.
    pub gradient: [f64; 3],
    /// `[[∂²value/∂x², ∂²value/∂x∂y, ∂²value/∂x∂z], ...]`.
    pub hessian: [[f64; 3]; 3],
}

/// A 4D point sample with a finite-difference Hessian.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSampleHessian4 {
    /// Noise value at the point.
    pub value: f32,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z, ∂value/∂w]`, including the configured frequency.
    pub gradient: [f32; 4],
    /// `[[∂²value/∂x², ∂²value/∂x∂y, ...], ...]`.
    pub hessian: [[f32; 4]; 4],
}

/// f64 twin of [`PointSampleHessian4`].
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointSampleHessian4F64 {
    /// Noise value at the point.
    pub value: f64,
    /// `[∂value/∂x, ∂value/∂y, ∂value/∂z, ∂value/∂w]`, including the configured frequency.
    pub gradient: [f64; 4],
    /// `[[∂²value/∂x², ∂²value/∂x∂y, ...], ...]`.
    pub hessian: [[f64; 4]; 4],
}

#[inline(always)]
fn expect_dim<T: DimensionalBeing>(value: &T, dimension: usize) {
    let actual = value.dimensions().dimension;
    assert_eq!(
        actual, dimension,
        "point sampling: this entry point requires {dimension}D settings, got {actual}D"
    );
}

#[inline(always)]
fn max_frequency_2d(noise_type: &NoiseType) -> f32 {
    match noise_type {
        NoiseType::Gradient(s) => s.frequency_x().abs().max(s.frequency_y().abs()),
        NoiseType::Fbm(s) => s.frequency_x().abs().max(s.frequency_y().abs()),
        NoiseType::Ridge(s) => s.frequency_x().abs().max(s.frequency_y().abs()),
        NoiseType::Turbulence(s) => s.frequency_x().abs().max(s.frequency_y().abs()),
        NoiseType::Cellular(s) => s.frequency_x().abs().max(s.frequency_y().abs()),
        NoiseType::Cellular2(s) => s.frequency_x().abs().max(s.frequency_y().abs()),
    }
}

#[inline(always)]
fn max_frequency_3d(noise_type: &NoiseType) -> f32 {
    match noise_type {
        NoiseType::Gradient(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs()),
        NoiseType::Fbm(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs()),
        NoiseType::Ridge(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs()),
        NoiseType::Turbulence(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs()),
        NoiseType::Cellular(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs()),
        NoiseType::Cellular2(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs()),
    }
}

#[inline(always)]
fn max_frequency_4d(noise_type: &NoiseType) -> f32 {
    match noise_type {
        NoiseType::Gradient(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs())
            .max(s.frequency_w().abs()),
        NoiseType::Fbm(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs())
            .max(s.frequency_w().abs()),
        NoiseType::Ridge(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs())
            .max(s.frequency_w().abs()),
        NoiseType::Turbulence(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs())
            .max(s.frequency_w().abs()),
        NoiseType::Cellular(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs())
            .max(s.frequency_w().abs()),
        NoiseType::Cellular2(s) => s
            .frequency_x()
            .abs()
            .max(s.frequency_y().abs())
            .max(s.frequency_z().abs())
            .max(s.frequency_w().abs()),
    }
}

#[inline(always)]
fn hessian_step_2d(noise_type: &NoiseType) -> f32 {
    let max_frequency = max_frequency_2d(noise_type);
    if max_frequency > 0.0 {
        (1.0e-3 / max_frequency).clamp(1.0e-6, 1.0e-2)
    } else {
        1.0e-3
    }
}

#[inline(always)]
fn hessian_step_3d(noise_type: &NoiseType) -> f32 {
    let max_frequency = max_frequency_3d(noise_type);
    if max_frequency > 0.0 {
        (1.0e-3 / max_frequency).clamp(1.0e-6, 1.0e-2)
    } else {
        1.0e-3
    }
}

#[inline(always)]
fn hessian_step_4d(noise_type: &NoiseType) -> f32 {
    let max_frequency = max_frequency_4d(noise_type);
    if max_frequency > 0.0 {
        (1.0e-3 / max_frequency).clamp(1.0e-6, 1.0e-2)
    } else {
        1.0e-3
    }
}

#[inline(always)]
fn hessian_step_4d_64(noise_type: &NoiseType) -> f64 {
    let max_frequency = max_frequency_4d(noise_type) as f64;
    if max_frequency > 0.0 {
        (1.0e-4 / max_frequency).clamp(1.0e-8, 1.0e-3)
    } else {
        1.0e-4
    }
}

#[inline(always)]
fn hessian_step_3d_64(noise_type: &NoiseType) -> f64 {
    let max_frequency = max_frequency_3d(noise_type) as f64;
    if max_frequency > 0.0 {
        (1.0e-4 / max_frequency).clamp(1.0e-8, 1.0e-3)
    } else {
        1.0e-4
    }
}

// ---------------------------------------------------------------------------
// f32 batch loading
// ---------------------------------------------------------------------------

#[inline(always)]
unsafe fn point_batch_2d<S, F>(points: &[[f32; 2]], out: &mut [PointSample2], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32) -> (S::Vf32, [S::Vf32; 2]),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let (value, gradient) = kernel(x, y);
        for lane in 0..count {
            out[i + lane] = PointSample2 {
                value: value[lane],
                gradient: [gradient[0][lane], gradient[1][lane]],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn point_batch_3d<S, F>(points: &[[f32; 3]], out: &mut [PointSample3], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32, S::Vf32) -> (S::Vf32, [S::Vf32; 3]),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut zb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
            zb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
            zb[lane] = points[i + lane][2];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let z = S::Vf32::load_from_ptr_unaligned(zb.as_ptr());
        let (value, gradient) = kernel(x, y, z);
        for lane in 0..count {
            out[i + lane] = PointSample3 {
                value: value[lane],
                gradient: [gradient[0][lane], gradient[1][lane], gradient[2][lane]],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn point_batch_4d<S, F>(points: &[[f32; 4]], out: &mut [PointSample4], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32, S::Vf32, S::Vf32) -> (S::Vf32, [S::Vf32; 4]),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut zb = [0.0f32; MAX_WIDTH_32];
    let mut wb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
            zb[lane] = 0.0;
            wb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
            zb[lane] = points[i + lane][2];
            wb[lane] = points[i + lane][3];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let z = S::Vf32::load_from_ptr_unaligned(zb.as_ptr());
        let w = S::Vf32::load_from_ptr_unaligned(wb.as_ptr());
        let (value, gradient) = kernel(x, y, z, w);
        for lane in 0..count {
            out[i + lane] = PointSample4 {
                value: value[lane],
                gradient: [
                    gradient[0][lane],
                    gradient[1][lane],
                    gradient[2][lane],
                    gradient[3][lane],
                ],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn phasor_batch_2d<S, F>(points: &[[f32; 2]], out: &mut [PhasorSample2], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32) -> (S::Vf32, S::Vf32, [S::Vf32; 2], [S::Vf32; 2]),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let (real, imag, real_gradient, imag_gradient) = kernel(x, y);
        for lane in 0..count {
            out[i + lane] = PhasorSample2 {
                real: real[lane],
                imag: imag[lane],
                real_gradient: [real_gradient[0][lane], real_gradient[1][lane]],
                imag_gradient: [imag_gradient[0][lane], imag_gradient[1][lane]],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn phasor_batch_3d<S, F>(points: &[[f32; 3]], out: &mut [PhasorSample3], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32, S::Vf32) -> (S::Vf32, S::Vf32, [S::Vf32; 3], [S::Vf32; 3]),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut zb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
            zb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
            zb[lane] = points[i + lane][2];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let z = S::Vf32::load_from_ptr_unaligned(zb.as_ptr());
        let (real, imag, real_gradient, imag_gradient) = kernel(x, y, z);
        for lane in 0..count {
            out[i + lane] = PhasorSample3 {
                real: real[lane],
                imag: imag[lane],
                real_gradient: [
                    real_gradient[0][lane],
                    real_gradient[1][lane],
                    real_gradient[2][lane],
                ],
                imag_gradient: [
                    imag_gradient[0][lane],
                    imag_gradient[1][lane],
                    imag_gradient[2][lane],
                ],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn phasor_value_batch_2d<S, F>(points: &[[f32; 2]], out: &mut [PhasorValue2], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32) -> (S::Vf32, S::Vf32),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let (real, imag) = kernel(x, y);
        for lane in 0..count {
            out[i + lane] = PhasorValue2 {
                real: real[lane],
                imag: imag[lane],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn phasor_value_batch_3d<S, F>(points: &[[f32; 3]], out: &mut [PhasorValue3], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf32, S::Vf32, S::Vf32) -> (S::Vf32, S::Vf32),
{
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut xb = [0.0f32; MAX_WIDTH_32];
    let mut yb = [0.0f32; MAX_WIDTH_32];
    let mut zb = [0.0f32; MAX_WIDTH_32];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
            zb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
            zb[lane] = points[i + lane][2];
        }
        let x = S::Vf32::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf32::load_from_ptr_unaligned(yb.as_ptr());
        let z = S::Vf32::load_from_ptr_unaligned(zb.as_ptr());
        let (real, imag) = kernel(x, y, z);
        for lane in 0..count {
            out[i + lane] = PhasorValue3 {
                real: real[lane],
                imag: imag[lane],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn point_batch_3d_64<S, F>(points: &[[f64; 3]], out: &mut [PointSample3F64], mut kernel: F)
where
    S: Simd,
    F: FnMut(S::Vf64, S::Vf64, S::Vf64) -> (S::Vf64, [S::Vf64; 3]),
{
    let width = S::Vf64::WIDTH;
    debug_assert!(width <= MAX_WIDTH_64);
    let mut xb = [0.0f64; MAX_WIDTH_64];
    let mut yb = [0.0f64; MAX_WIDTH_64];
    let mut zb = [0.0f64; MAX_WIDTH_64];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for lane in 0..width {
            xb[lane] = 0.0;
            yb[lane] = 0.0;
            zb[lane] = 0.0;
        }
        for lane in 0..count {
            xb[lane] = points[i + lane][0];
            yb[lane] = points[i + lane][1];
            zb[lane] = points[i + lane][2];
        }
        let x = S::Vf64::load_from_ptr_unaligned(xb.as_ptr());
        let y = S::Vf64::load_from_ptr_unaligned(yb.as_ptr());
        let z = S::Vf64::load_from_ptr_unaligned(zb.as_ptr());
        let (value, gradient) = kernel(x, y, z);
        for lane in 0..count {
            out[i + lane] = PointSample3F64 {
                value: value[lane],
                gradient: [gradient[0][lane], gradient[1][lane], gradient[2][lane]],
            };
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn point_value_batch_f32<const N: usize, S, F>(
    points: &[[f32; N]],
    out: &mut [f32],
    mut kernel: F,
) where
    S: Simd,
    F: FnMut([S::Vf32; N]) -> S::Vf32,
{
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    let width = S::Vf32::WIDTH;
    debug_assert!(width <= MAX_WIDTH_32);
    let mut lanes = [[0.0f32; MAX_WIDTH_32]; N];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for row in &mut lanes {
            for slot in row.iter_mut().take(width) {
                *slot = 0.0;
            }
        }
        for lane in 0..count {
            for axis in 0..N {
                lanes[axis][lane] = points[i + lane][axis];
            }
        }
        let coords =
            std::array::from_fn(|axis| S::Vf32::load_from_ptr_unaligned(lanes[axis].as_ptr()));
        let value = kernel(coords);
        for lane in 0..count {
            out[i + lane] = value[lane];
        }
        i += count;
    }
}

#[inline(always)]
unsafe fn point_value_batch_f64<const N: usize, S, F>(
    points: &[[f64; N]],
    out: &mut [f64],
    mut kernel: F,
) where
    S: Simd,
    F: FnMut([S::Vf64; N]) -> S::Vf64,
{
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    let width = S::Vf64::WIDTH;
    debug_assert!(width <= MAX_WIDTH_64);
    let mut lanes = [[0.0f64; MAX_WIDTH_64]; N];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for row in &mut lanes {
            for slot in row.iter_mut().take(width) {
                *slot = 0.0;
            }
        }
        for lane in 0..count {
            for axis in 0..N {
                lanes[axis][lane] = points[i + lane][axis];
            }
        }
        let coords =
            std::array::from_fn(|axis| S::Vf64::load_from_ptr_unaligned(lanes[axis].as_ptr()));
        let value = kernel(coords);
        for lane in 0..count {
            out[i + lane] = value[lane];
        }
        i += count;
    }
}

unsafe fn point_value_batch_2d<S: Simd, T: Sample32<S>>(
    settings: &T,
    points: &[[f32; 2]],
    out: &mut [f32],
) {
    let frequency_x = S::Vf32::set1(settings.frequency_x());
    let frequency_y = S::Vf32::set1(settings.frequency_y());
    point_value_batch_f32::<2, S, _>(points, out, |p| {
        settings.sample_2d(p[0] * frequency_x, p[1] * frequency_y)
    });
}

unsafe fn point_value_batch_3d<S: Simd, T: Sample32<S>>(
    settings: &T,
    points: &[[f32; 3]],
    out: &mut [f32],
) {
    let frequency_x = S::Vf32::set1(settings.frequency_x());
    let frequency_y = S::Vf32::set1(settings.frequency_y());
    let frequency_z = S::Vf32::set1(settings.frequency_z());
    point_value_batch_f32::<3, S, _>(points, out, |p| {
        settings.sample_3d(p[0] * frequency_x, p[1] * frequency_y, p[2] * frequency_z)
    });
}

unsafe fn point_value_batch_4d<S: Simd, T: Sample32<S>>(
    settings: &T,
    points: &[[f32; 4]],
    out: &mut [f32],
) {
    let frequency_x = S::Vf32::set1(settings.frequency_x());
    let frequency_y = S::Vf32::set1(settings.frequency_y());
    let frequency_z = S::Vf32::set1(settings.frequency_z());
    let frequency_w = S::Vf32::set1(settings.frequency_w());
    point_value_batch_f32::<4, S, _>(points, out, |p| {
        settings.sample_4d(
            p[0] * frequency_x,
            p[1] * frequency_y,
            p[2] * frequency_z,
            p[3] * frequency_w,
        )
    });
}

unsafe fn point_value_batch_3d_64<S: Simd, T: Sample64<S>>(
    settings: &T,
    points: &[[f64; 3]],
    out: &mut [f64],
) {
    let frequency_x = S::Vf64::set1(settings.frequency_x() as f64);
    let frequency_y = S::Vf64::set1(settings.frequency_y() as f64);
    let frequency_z = S::Vf64::set1(settings.frequency_z() as f64);
    point_value_batch_f64::<3, S, _>(points, out, |p| {
        settings.sample_3d(p[0] * frequency_x, p[1] * frequency_y, p[2] * frequency_z)
    });
}

unsafe fn point_value_batch_4d_64<S: Simd, T: Sample64<S>>(
    settings: &T,
    points: &[[f64; 4]],
    out: &mut [f64],
) {
    let frequency_x = S::Vf64::set1(settings.frequency_x() as f64);
    let frequency_y = S::Vf64::set1(settings.frequency_y() as f64);
    let frequency_z = S::Vf64::set1(settings.frequency_z() as f64);
    let frequency_w = S::Vf64::set1(settings.frequency_w() as f64);
    point_value_batch_f64::<4, S, _>(points, out, |p| {
        settings.sample_4d(
            p[0] * frequency_x,
            p[1] * frequency_y,
            p[2] * frequency_z,
            p[3] * frequency_w,
        )
    });
}

unsafe fn point_gradient_batch_4d<S: Simd, T: SampleFractalDeriv32<S>>(
    settings: &T,
    points: &[[f32; 4]],
    out: &mut [PointSample4],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    let frequency_x = settings.frequency_x();
    let frequency_y = settings.frequency_y();
    let frequency_z = settings.frequency_z();
    let frequency_w = settings.frequency_w();
    let fx = S::Vf32::set1(frequency_x);
    let fy = S::Vf32::set1(frequency_y);
    let fz = S::Vf32::set1(frequency_z);
    let fw = S::Vf32::set1(frequency_w);
    point_batch_4d::<S, _>(points, out, move |x, y, z, w| {
        let (value, d) = settings.sample_4d_deriv(x * fx, y * fy, z * fz, w * fw);
        (
            value,
            [
                d[0] * frequency_x,
                d[1] * frequency_y,
                d[2] * frequency_z,
                d[3] * frequency_w,
            ],
        )
    });
}

unsafe fn point_gradient_batch_4d_64<S: Simd, T: SampleFractalDeriv64<S>>(
    settings: &T,
    points: &[[f64; 4]],
    out: &mut [PointSample4F64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    let frequency_x = settings.frequency_x() as f64;
    let frequency_y = settings.frequency_y() as f64;
    let frequency_z = settings.frequency_z() as f64;
    let frequency_w = settings.frequency_w() as f64;
    let fx = S::Vf64::set1(frequency_x);
    let fy = S::Vf64::set1(frequency_y);
    let fz = S::Vf64::set1(frequency_z);
    let fw = S::Vf64::set1(frequency_w);
    let width = S::Vf64::WIDTH;
    debug_assert!(width <= MAX_WIDTH_64);
    let mut lanes = [[0.0f64; MAX_WIDTH_64]; 4];
    let mut i = 0;
    while i < points.len() {
        let count = (points.len() - i).min(width);
        for row in &mut lanes {
            for slot in row.iter_mut().take(width) {
                *slot = 0.0;
            }
        }
        for lane in 0..count {
            for axis in 0..4 {
                lanes[axis][lane] = points[i + lane][axis];
            }
        }
        let x = S::Vf64::load_from_ptr_unaligned(lanes[0].as_ptr());
        let y = S::Vf64::load_from_ptr_unaligned(lanes[1].as_ptr());
        let z = S::Vf64::load_from_ptr_unaligned(lanes[2].as_ptr());
        let w = S::Vf64::load_from_ptr_unaligned(lanes[3].as_ptr());
        let (value, d) = settings.sample_4d_deriv(x * fx, y * fy, z * fz, w * fw);
        for lane in 0..count {
            out[i + lane] = PointSample4F64 {
                value: value[lane],
                gradient: [
                    d[0][lane] * frequency_x,
                    d[1][lane] * frequency_y,
                    d[2][lane] * frequency_z,
                    d[3][lane] * frequency_w,
                ],
            };
        }
        i += count;
    }
}

// ---------------------------------------------------------------------------
// 2D f32 dispatch
// ---------------------------------------------------------------------------

unsafe fn sample_points_2d_gradient_impl<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f32; 2]],
    out: &mut [PointSample2],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 2);
    match noise_type {
        NoiseType::Gradient(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            point_batch_2d::<S, _>(points, out, move |x, y| {
                let (value, d) = simplex_32::simplex_2d_deriv::<S>(x * fx, y * fy, seed);
                (value, [d[0] * fx, d[1] * fy])
            });
        }
        NoiseType::Fbm(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let lac = S::Vf32::set1(s.lacunarity);
            let gain = S::Vf32::set1(s.gain);
            let octaves = s.octaves;
            point_batch_2d::<S, _>(points, out, move |x, y| {
                let (value, d) =
                    fbm_32::fbm_2d_deriv::<S>(x * fx, y * fy, lac, gain, octaves, seed);
                (value, [d[0] * fx, d[1] * fy])
            });
        }
        NoiseType::Ridge(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let lac = S::Vf32::set1(s.lacunarity);
            let gain = S::Vf32::set1(s.gain);
            let octaves = s.octaves;
            point_batch_2d::<S, _>(points, out, move |x, y| {
                let (value, d) =
                    ridge_32::ridge_2d_deriv::<S>(x * fx, y * fy, lac, gain, octaves, seed);
                (value, [d[0] * fx, d[1] * fy])
            });
        }
        NoiseType::Turbulence(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let lac = S::Vf32::set1(s.lacunarity);
            let gain = S::Vf32::set1(s.gain);
            let octaves = s.octaves;
            point_batch_2d::<S, _>(points, out, move |x, y| {
                let (value, d) = turbulence_32::turbulence_2d_deriv::<S>(
                    x * fx,
                    y * fy,
                    lac,
                    gain,
                    octaves,
                    seed,
                );
                (value, [d[0] * fx, d[1] * fy])
            });
        }
        NoiseType::Cellular(s) => {
            assert!(
                matches!(s.distance_function, CellDistanceFunction::Euclidean),
                "cellular point gradients require the Euclidean distance function"
            );
            assert!(
                matches!(s.return_type, CellReturnType::Distance),
                "cellular point gradients require CellReturnType::Distance"
            );
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let jitter = S::Vf32::set1(s.jitter);
            point_batch_2d::<S, _>(points, out, move |x, y| {
                let (value, d) = cell_32::cellular_2d_deriv::<S>(x * fx, y * fy, jitter, seed);
                (value, [d[0] * fx, d[1] * fy])
            });
        }
        NoiseType::Cellular2(s) => {
            assert!(
                matches!(s.distance_function, CellDistanceFunction::Euclidean),
                "cellular2 point gradients require the Euclidean distance function"
            );
            assert!(
                s.index0 < 4 && s.index1 < 4,
                "cellular2 point gradients require index0/index1 below 4"
            );
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let jitter = S::Vf32::set1(s.jitter);
            let i0 = s.index0;
            let i1 = s.index1;
            let ret = s.return_type;
            point_batch_2d::<S, _>(points, out, move |x, y| {
                let (dist, delta) =
                    cell2_32::cellular2_2d_deriv::<S>(x * fx, y * fy, jitter, i1, seed);
                let a = dist[i0];
                let b = dist[i1];
                let value = cellular2_value::<S>(ret, a, b);
                let neg_two = S::Vf32::set1(-2.0);
                let gx =
                    cellular2_grad::<S>(ret, a, b, neg_two * delta[0][i0], neg_two * delta[0][i1]);
                let gy =
                    cellular2_grad::<S>(ret, a, b, neg_two * delta[1][i0], neg_two * delta[1][i1]);
                (value, [gx * fx, gy * fy])
            });
        }
    }
}

unsafe fn sample_points_2d_values_impl<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f32; 2]],
    out: &mut [f32],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 2);
    match noise_type {
        NoiseType::Gradient(s) => point_value_batch_2d::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_value_batch_2d::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_value_batch_2d::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_value_batch_2d::<S, _>(s, points, out),
        NoiseType::Cellular(s) => point_value_batch_2d::<S, _>(s, points, out),
        NoiseType::Cellular2(s) => point_value_batch_2d::<S, _>(s, points, out),
    }
}

// ---------------------------------------------------------------------------
// 3D f32 dispatch
// ---------------------------------------------------------------------------

unsafe fn sample_points_3d_gradient_impl<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f32; 3]],
    out: &mut [PointSample3],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 3);
    match noise_type {
        NoiseType::Gradient(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let fz = S::Vf32::set1(s.frequency_z());
            point_batch_3d::<S, _>(points, out, move |x, y, z| {
                let (value, d) = simplex_32::simplex_3d_deriv::<S>(x * fx, y * fy, z * fz, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Fbm(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let fz = S::Vf32::set1(s.frequency_z());
            let lac = S::Vf32::set1(s.lacunarity);
            let gain = S::Vf32::set1(s.gain);
            let octaves = s.octaves;
            point_batch_3d::<S, _>(points, out, move |x, y, z| {
                let (value, d) =
                    fbm_32::fbm_3d_deriv::<S>(x * fx, y * fy, z * fz, lac, gain, octaves, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Ridge(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let fz = S::Vf32::set1(s.frequency_z());
            let lac = S::Vf32::set1(s.lacunarity);
            let gain = S::Vf32::set1(s.gain);
            let octaves = s.octaves;
            point_batch_3d::<S, _>(points, out, move |x, y, z| {
                let (value, d) =
                    ridge_32::ridge_3d_deriv::<S>(x * fx, y * fy, z * fz, lac, gain, octaves, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Turbulence(s) => {
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let fz = S::Vf32::set1(s.frequency_z());
            let lac = S::Vf32::set1(s.lacunarity);
            let gain = S::Vf32::set1(s.gain);
            let octaves = s.octaves;
            point_batch_3d::<S, _>(points, out, move |x, y, z| {
                let (value, d) = turbulence_32::turbulence_3d_deriv::<S>(
                    x * fx,
                    y * fy,
                    z * fz,
                    lac,
                    gain,
                    octaves,
                    seed,
                );
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Cellular(s) => {
            assert!(
                matches!(s.distance_function, CellDistanceFunction::Euclidean),
                "cellular point gradients require the Euclidean distance function"
            );
            assert!(
                matches!(s.return_type, CellReturnType::Distance),
                "cellular point gradients require CellReturnType::Distance"
            );
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let fz = S::Vf32::set1(s.frequency_z());
            let jitter = S::Vf32::set1(s.jitter);
            point_batch_3d::<S, _>(points, out, move |x, y, z| {
                let (value, d) =
                    cell_32::cellular_3d_deriv::<S>(x * fx, y * fy, z * fz, jitter, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Cellular2(s) => {
            assert!(
                matches!(s.distance_function, CellDistanceFunction::Euclidean),
                "cellular2 point gradients require the Euclidean distance function"
            );
            assert!(
                s.index0 < 4 && s.index1 < 4,
                "cellular2 point gradients require index0/index1 below 4"
            );
            let seed = s.dimensions().seed;
            let fx = S::Vf32::set1(s.frequency_x());
            let fy = S::Vf32::set1(s.frequency_y());
            let fz = S::Vf32::set1(s.frequency_z());
            let jitter = S::Vf32::set1(s.jitter);
            let i0 = s.index0;
            let i1 = s.index1;
            let ret = s.return_type;
            point_batch_3d::<S, _>(points, out, move |x, y, z| {
                let (dist, delta) =
                    cell2_32::cellular2_3d_deriv::<S>(x * fx, y * fy, z * fz, jitter, i1, seed);
                let a = dist[i0];
                let b = dist[i1];
                let value = cellular2_value::<S>(ret, a, b);
                let neg_two = S::Vf32::set1(-2.0);
                let gx =
                    cellular2_grad::<S>(ret, a, b, neg_two * delta[0][i0], neg_two * delta[0][i1]);
                let gy =
                    cellular2_grad::<S>(ret, a, b, neg_two * delta[1][i0], neg_two * delta[1][i1]);
                let gz =
                    cellular2_grad::<S>(ret, a, b, neg_two * delta[2][i0], neg_two * delta[2][i1]);
                (value, [gx * fx, gy * fy, gz * fz])
            });
        }
    }
}

unsafe fn sample_points_3d_values_impl<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f32; 3]],
    out: &mut [f32],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 3);
    match noise_type {
        NoiseType::Gradient(s) => point_value_batch_3d::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_value_batch_3d::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_value_batch_3d::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_value_batch_3d::<S, _>(s, points, out),
        NoiseType::Cellular(s) => point_value_batch_3d::<S, _>(s, points, out),
        NoiseType::Cellular2(s) => point_value_batch_3d::<S, _>(s, points, out),
    }
}

// ---------------------------------------------------------------------------
// 3D f64 dispatch
// ---------------------------------------------------------------------------

unsafe fn sample_points_3d_gradient_impl_64<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f64; 3]],
    out: &mut [PointSample3F64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 3);
    match noise_type {
        NoiseType::Gradient(s) => {
            let seed = s.dimensions().seed as i64;
            let fx = S::Vf64::set1(s.frequency_x() as f64);
            let fy = S::Vf64::set1(s.frequency_y() as f64);
            let fz = S::Vf64::set1(s.frequency_z() as f64);
            point_batch_3d_64::<S, _>(points, out, move |x, y, z| {
                let (value, d) = simplex_64::simplex_3d_deriv::<S>(x * fx, y * fy, z * fz, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Fbm(s) => {
            let seed = s.dimensions().seed as i64;
            let fx = S::Vf64::set1(s.frequency_x() as f64);
            let fy = S::Vf64::set1(s.frequency_y() as f64);
            let fz = S::Vf64::set1(s.frequency_z() as f64);
            let lac = S::Vf64::set1(s.lacunarity as f64);
            let gain = S::Vf64::set1(s.gain as f64);
            let octaves = s.octaves;
            point_batch_3d_64::<S, _>(points, out, move |x, y, z| {
                let (value, d) =
                    fbm_64::fbm_3d_deriv::<S>(x * fx, y * fy, z * fz, lac, gain, octaves, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Ridge(s) => {
            let seed = s.dimensions().seed as i64;
            let fx = S::Vf64::set1(s.frequency_x() as f64);
            let fy = S::Vf64::set1(s.frequency_y() as f64);
            let fz = S::Vf64::set1(s.frequency_z() as f64);
            let lac = S::Vf64::set1(s.lacunarity as f64);
            let gain = S::Vf64::set1(s.gain as f64);
            let octaves = s.octaves;
            point_batch_3d_64::<S, _>(points, out, move |x, y, z| {
                let (value, d) =
                    ridge_64::ridge_3d_deriv::<S>(x * fx, y * fy, z * fz, lac, gain, octaves, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Turbulence(s) => {
            let seed = s.dimensions().seed as i64;
            let fx = S::Vf64::set1(s.frequency_x() as f64);
            let fy = S::Vf64::set1(s.frequency_y() as f64);
            let fz = S::Vf64::set1(s.frequency_z() as f64);
            let lac = S::Vf64::set1(s.lacunarity as f64);
            let gain = S::Vf64::set1(s.gain as f64);
            let octaves = s.octaves;
            point_batch_3d_64::<S, _>(points, out, move |x, y, z| {
                let (value, d) = turbulence_64::turbulence_3d_deriv::<S>(
                    x * fx,
                    y * fy,
                    z * fz,
                    lac,
                    gain,
                    octaves,
                    seed,
                );
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Cellular(s) => {
            assert!(
                matches!(s.distance_function, CellDistanceFunction::Euclidean),
                "cellular point gradients require the Euclidean distance function"
            );
            assert!(
                matches!(s.return_type, CellReturnType::Distance),
                "cellular point gradients require CellReturnType::Distance"
            );
            let seed = s.dimensions().seed as i64;
            let fx = S::Vf64::set1(s.frequency_x() as f64);
            let fy = S::Vf64::set1(s.frequency_y() as f64);
            let fz = S::Vf64::set1(s.frequency_z() as f64);
            let jitter = S::Vf64::set1(s.jitter as f64);
            point_batch_3d_64::<S, _>(points, out, move |x, y, z| {
                let (value, d) =
                    cell_64::cellular_3d_deriv::<S>(x * fx, y * fy, z * fz, jitter, seed);
                (value, [d[0] * fx, d[1] * fy, d[2] * fz])
            });
        }
        NoiseType::Cellular2(s) => {
            assert!(
                matches!(s.distance_function, CellDistanceFunction::Euclidean),
                "cellular2 point gradients require the Euclidean distance function"
            );
            assert!(
                s.index0 < 4 && s.index1 < 4,
                "cellular2 point gradients require index0/index1 below 4"
            );
            let seed = s.dimensions().seed as i64;
            let fx = S::Vf64::set1(s.frequency_x() as f64);
            let fy = S::Vf64::set1(s.frequency_y() as f64);
            let fz = S::Vf64::set1(s.frequency_z() as f64);
            let jitter = S::Vf64::set1(s.jitter as f64);
            let i0 = s.index0;
            let i1 = s.index1;
            let ret = s.return_type;
            point_batch_3d_64::<S, _>(points, out, move |x, y, z| {
                let (dist, delta) =
                    cell2_64::cellular2_3d_deriv::<S>(x * fx, y * fy, z * fz, jitter, i1, seed);
                let a = dist[i0];
                let b = dist[i1];
                let value = cellular2_value_64::<S>(ret, a, b);
                let neg_two = S::Vf64::set1(-2.0);
                let gx = cellular2_grad_64::<S>(
                    ret,
                    a,
                    b,
                    neg_two * delta[0][i0],
                    neg_two * delta[0][i1],
                );
                let gy = cellular2_grad_64::<S>(
                    ret,
                    a,
                    b,
                    neg_two * delta[1][i0],
                    neg_two * delta[1][i1],
                );
                let gz = cellular2_grad_64::<S>(
                    ret,
                    a,
                    b,
                    neg_two * delta[2][i0],
                    neg_two * delta[2][i1],
                );
                (value, [gx * fx, gy * fy, gz * fz])
            });
        }
    }
}

unsafe fn sample_points_3d_values_impl_64<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f64; 3]],
    out: &mut [f64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 3);
    match noise_type {
        NoiseType::Gradient(s) => point_value_batch_3d_64::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_value_batch_3d_64::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_value_batch_3d_64::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_value_batch_3d_64::<S, _>(s, points, out),
        NoiseType::Cellular(s) => point_value_batch_3d_64::<S, _>(s, points, out),
        NoiseType::Cellular2(s) => point_value_batch_3d_64::<S, _>(s, points, out),
    }
}

// ---------------------------------------------------------------------------
// 4D f32 dispatch
// ---------------------------------------------------------------------------

unsafe fn sample_points_4d_values_impl<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f32; 4]],
    out: &mut [f32],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 4);
    match noise_type {
        NoiseType::Gradient(s) => point_value_batch_4d::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_value_batch_4d::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_value_batch_4d::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_value_batch_4d::<S, _>(s, points, out),
        NoiseType::Cellular(s) => point_value_batch_4d::<S, _>(s, points, out),
        NoiseType::Cellular2(s) => point_value_batch_4d::<S, _>(s, points, out),
    }
}

unsafe fn sample_points_4d_gradient_impl<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f32; 4]],
    out: &mut [PointSample4],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 4);
    match noise_type {
        NoiseType::Gradient(s) => point_gradient_batch_4d::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_gradient_batch_4d::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_gradient_batch_4d::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_gradient_batch_4d::<S, _>(s, points, out),
        NoiseType::Cellular(_) | NoiseType::Cellular2(_) => {
            panic!("4D cellular point gradients are not implemented")
        }
    }
}

// ---------------------------------------------------------------------------
// 4D f64 dispatch
// ---------------------------------------------------------------------------

unsafe fn sample_points_4d_values_impl_64<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f64; 4]],
    out: &mut [f64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 4);
    match noise_type {
        NoiseType::Gradient(s) => point_value_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_value_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_value_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_value_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Cellular(s) => point_value_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Cellular2(s) => point_value_batch_4d_64::<S, _>(s, points, out),
    }
}

unsafe fn sample_points_4d_gradient_impl_64<S: Simd>(
    noise_type: &NoiseType,
    points: &[[f64; 4]],
    out: &mut [PointSample4F64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    expect_dim(noise_type, 4);
    match noise_type {
        NoiseType::Gradient(s) => point_gradient_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Fbm(s) => point_gradient_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Ridge(s) => point_gradient_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Turbulence(s) => point_gradient_batch_4d_64::<S, _>(s, points, out),
        NoiseType::Cellular(_) | NoiseType::Cellular2(_) => {
            panic!("4D cellular point gradients are not implemented")
        }
    }
}

// ---------------------------------------------------------------------------
// Gabor point batches (2D/3D/4D f32)
// ---------------------------------------------------------------------------

unsafe fn sample_gabor_2d_values_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [f32],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        2,
        "gabor point sampling: this entry point requires 2D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let cell_scale = settings.impulse_density.powf(0.5);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    point_value_batch_f32::<2, S, _>(points, out, |p| {
        gabor_32::gabor_2d::<S>(
            p[0] * fx,
            p[1] * fy,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        )
    });
}

unsafe fn sample_gabor_3d_values_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [f32],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        3,
        "gabor point sampling: this entry point requires 3D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let fz = S::Vf32::set1(settings.frequency_z);
    let cell_scale = settings.impulse_density.powf(1.0 / 3.0);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y
        + settings.orientation_z * settings.orientation_z)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
        settings.orientation_z / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    point_value_batch_f32::<3, S, _>(points, out, |p| {
        gabor_32::gabor_3d::<S>(
            p[0] * fx,
            p[1] * fy,
            p[2] * fz,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        )
    });
}

unsafe fn sample_gabor_4d_values_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 4]],
    out: &mut [f32],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        4,
        "gabor point sampling: this entry point requires 4D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let fz = S::Vf32::set1(settings.frequency_z);
    let fw = S::Vf32::set1(settings.frequency_w);
    let cell_scale = settings.impulse_density.powf(1.0 / 4.0);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y
        + settings.orientation_z * settings.orientation_z
        + settings.orientation_w * settings.orientation_w)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
        settings.orientation_z / orientation_len,
        settings.orientation_w / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    point_value_batch_f32::<4, S, _>(points, out, |p| {
        gabor_32::gabor_4d::<S>(
            p[0] * fx,
            p[1] * fy,
            p[2] * fz,
            p[3] * fw,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        )
    });
}

unsafe fn sample_gabor_2d_gradient_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [PointSample2],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        2,
        "gabor point sampling: this entry point requires 2D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let cell_scale = settings.impulse_density.powf(0.5);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    point_batch_2d::<S, _>(points, out, move |x, y| {
        let (value, d) = gabor_32::gabor_2d_deriv::<S>(
            x * fx,
            y * fy,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        );
        (value, [d[0] * fx, d[1] * fy])
    });
}

unsafe fn sample_gabor_3d_gradient_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [PointSample3],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        3,
        "gabor point sampling: this entry point requires 3D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let fz = S::Vf32::set1(settings.frequency_z);
    let cell_scale = settings.impulse_density.powf(1.0 / 3.0);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y
        + settings.orientation_z * settings.orientation_z)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
        settings.orientation_z / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    point_batch_3d::<S, _>(points, out, move |x, y, z| {
        let (value, d) = gabor_32::gabor_3d_deriv::<S>(
            x * fx,
            y * fy,
            z * fz,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        );
        (value, [d[0] * fx, d[1] * fy, d[2] * fz])
    });
}

unsafe fn sample_gabor_4d_gradient_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 4]],
    out: &mut [PointSample4],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        4,
        "gabor point sampling: this entry point requires 4D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let fz = S::Vf32::set1(settings.frequency_z);
    let fw = S::Vf32::set1(settings.frequency_w);
    let cell_scale = settings.impulse_density.powf(1.0 / 4.0);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y
        + settings.orientation_z * settings.orientation_z
        + settings.orientation_w * settings.orientation_w)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
        settings.orientation_z / orientation_len,
        settings.orientation_w / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    point_batch_4d::<S, _>(points, out, move |x, y, z, w| {
        let (value, d) = gabor_32::gabor_4d_deriv::<S>(
            x * fx,
            y * fy,
            z * fz,
            w * fw,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        );
        (value, [d[0] * fx, d[1] * fy, d[2] * fz, d[3] * fw])
    });
}

unsafe fn sample_phasor_2d_values_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [PhasorValue2],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        2,
        "phasor point sampling: this entry point requires 2D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let cell_scale = settings.impulse_density.powf(0.5);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    phasor_value_batch_2d::<S, _>(points, out, move |x, y| {
        gabor_32::phasor_2d_value::<S>(
            x * fx,
            y * fy,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        )
    });
}

unsafe fn sample_phasor_3d_values_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [PhasorValue3],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        3,
        "phasor point sampling: this entry point requires 3D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let fz = S::Vf32::set1(settings.frequency_z);
    let cell_scale = settings.impulse_density.powf(1.0 / 3.0);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y
        + settings.orientation_z * settings.orientation_z)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
        settings.orientation_z / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    phasor_value_batch_3d::<S, _>(points, out, move |x, y, z| {
        gabor_32::phasor_3d_value::<S>(
            x * fx,
            y * fy,
            z * fz,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        )
    });
}

unsafe fn sample_phasor_2d_gradient_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [PhasorSample2],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        2,
        "phasor point sampling: this entry point requires 2D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let cell_scale = settings.impulse_density.powf(0.5);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    phasor_batch_2d::<S, _>(points, out, move |x, y| {
        let (real, imag, real_gradient, imag_gradient) = gabor_32::phasor_2d_deriv::<S>(
            x * fx,
            y * fy,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        );
        (
            real,
            imag,
            [real_gradient[0] * fx, real_gradient[1] * fy],
            [imag_gradient[0] * fx, imag_gradient[1] * fy],
        )
    });
}

unsafe fn sample_phasor_3d_gradient_impl<S: Simd>(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [PhasorSample3],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    settings.validate();
    assert_eq!(
        settings.dimensions().dimension,
        3,
        "phasor point sampling: this entry point requires 3D settings"
    );
    let seed = settings.dimensions().seed;
    let fx = S::Vf32::set1(settings.frequency_x);
    let fy = S::Vf32::set1(settings.frequency_y);
    let fz = S::Vf32::set1(settings.frequency_z);
    let cell_scale = settings.impulse_density.powf(1.0 / 3.0);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y
        + settings.orientation_z * settings.orientation_z)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
        settings.orientation_z / orientation_len,
    ];
    let carrier_frequency = settings.carrier_frequency;
    let bandwidth = settings.bandwidth;
    let radius = settings.radius;
    let gain = settings.gain;
    phasor_batch_3d::<S, _>(points, out, move |x, y, z| {
        let (real, imag, real_gradient, imag_gradient) = gabor_32::phasor_3d_deriv::<S>(
            x * fx,
            y * fy,
            z * fz,
            seed,
            cell_scale,
            carrier_frequency,
            bandwidth,
            orientation,
            radius,
            gain,
        );
        (
            real,
            imag,
            [
                real_gradient[0] * fx,
                real_gradient[1] * fy,
                real_gradient[2] * fz,
            ],
            [
                imag_gradient[0] * fx,
                imag_gradient[1] * fy,
                imag_gradient[2] * fz,
            ],
        )
    });
}

#[inline(always)]
fn cellular2_value<S: Simd>(ret: Cell2ReturnType, a: S::Vf32, b: S::Vf32) -> S::Vf32 {
    match ret {
        Cell2ReturnType::Distance2 => b,
        Cell2ReturnType::Distance2Add => a + b,
        Cell2ReturnType::Distance2Sub => a - b,
        Cell2ReturnType::Distance2Mul => a * b,
        Cell2ReturnType::Distance2Div => a / b,
    }
}

#[inline(always)]
fn cellular2_grad<S: Simd>(
    ret: Cell2ReturnType,
    a: S::Vf32,
    b: S::Vf32,
    ga: S::Vf32,
    gb: S::Vf32,
) -> S::Vf32 {
    match ret {
        Cell2ReturnType::Distance2 => gb,
        Cell2ReturnType::Distance2Add => ga + gb,
        Cell2ReturnType::Distance2Sub => ga - gb,
        Cell2ReturnType::Distance2Mul => b * ga + a * gb,
        Cell2ReturnType::Distance2Div => (ga * b - a * gb) / (b * b),
    }
}

#[inline(always)]
fn cellular2_value_64<S: Simd>(ret: Cell2ReturnType, a: S::Vf64, b: S::Vf64) -> S::Vf64 {
    match ret {
        Cell2ReturnType::Distance2 => b,
        Cell2ReturnType::Distance2Add => a + b,
        Cell2ReturnType::Distance2Sub => a - b,
        Cell2ReturnType::Distance2Mul => a * b,
        Cell2ReturnType::Distance2Div => a / b,
    }
}

#[inline(always)]
fn cellular2_grad_64<S: Simd>(
    ret: Cell2ReturnType,
    a: S::Vf64,
    b: S::Vf64,
    ga: S::Vf64,
    gb: S::Vf64,
) -> S::Vf64 {
    match ret {
        Cell2ReturnType::Distance2 => gb,
        Cell2ReturnType::Distance2Add => ga + gb,
        Cell2ReturnType::Distance2Sub => ga - gb,
        Cell2ReturnType::Distance2Mul => b * ga + a * gb,
        Cell2ReturnType::Distance2Div => (ga * b - a * gb) / (b * b),
    }
}

// ---------------------------------------------------------------------------
// Public runtime entry points
// ---------------------------------------------------------------------------

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 2D points and return only the noise values.
    pub fn sample_points_2d(noise_type: &NoiseType, points: &[[f32; 2]], out: &mut [f32]) {
        unsafe { sample_points_2d_values_impl::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_2d_with_gradient`].
#[cfg(feature = "scalar")]
pub fn sample_points_2d(noise_type: &NoiseType, points: &[[f32; 2]], out: &mut [f32]) {
    unsafe { sample_points_2d_values_impl::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 2D points with analytic first derivatives.
    ///
    /// `points` and `out` must have the same length. The settings must be 2D.
    ///
    /// # Panics
    ///
    /// Panics if the lengths differ, the settings are not 2D, or cellular
    /// settings use a non-Euclidean distance function or a return type without
    /// a gradient.
    pub fn sample_points_2d_with_gradient(
        noise_type: &NoiseType,
        points: &[[f32; 2]],
        out: &mut [PointSample2],
    ) {
        unsafe { sample_points_2d_gradient_impl::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_2d_with_gradient`].
#[cfg(feature = "scalar")]
pub fn sample_points_2d_with_gradient(
    noise_type: &NoiseType,
    points: &[[f32; 2]],
    out: &mut [PointSample2],
) {
    unsafe { sample_points_2d_gradient_impl::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points and return only the noise values.
    pub fn sample_points_3d(noise_type: &NoiseType, points: &[[f32; 3]], out: &mut [f32]) {
        unsafe { sample_points_3d_values_impl::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_3d_with_gradient`].
#[cfg(feature = "scalar")]
pub fn sample_points_3d(noise_type: &NoiseType, points: &[[f32; 3]], out: &mut [f32]) {
    unsafe { sample_points_3d_values_impl::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points with analytic first derivatives.
    ///
    /// `points` and `out` must have the same length. The settings must be 3D.
    ///
    /// # Panics
    ///
    /// Panics if the lengths differ, the settings are not 3D, or cellular
    /// settings use a non-Euclidean distance function or a return type without
    /// a gradient.
    pub fn sample_points_3d_with_gradient(
        noise_type: &NoiseType,
        points: &[[f32; 3]],
        out: &mut [PointSample3],
    ) {
        unsafe { sample_points_3d_gradient_impl::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_3d_with_gradient`].
#[cfg(feature = "scalar")]
pub fn sample_points_3d_with_gradient(
    noise_type: &NoiseType,
    points: &[[f32; 3]],
    out: &mut [PointSample3],
) {
    unsafe { sample_points_3d_gradient_impl::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points in f64 and return only the noise values.
    pub fn sample_points_3d_f64(noise_type: &NoiseType, points: &[[f64; 3]], out: &mut [f64]) {
        unsafe { sample_points_3d_values_impl_64::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_3d_with_gradient_f64`].
#[cfg(feature = "scalar")]
pub fn sample_points_3d_f64(noise_type: &NoiseType, points: &[[f64; 3]], out: &mut [f64]) {
    unsafe { sample_points_3d_values_impl_64::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// f64 twin of [`sample_points_3d_with_gradient`].
    ///
    /// # Panics
    ///
    /// Panics if the lengths differ, the settings are not 3D, or cellular
    /// settings use a non-Euclidean distance function or a return type without
    /// a gradient.
    pub fn sample_points_3d_with_gradient_f64(
        noise_type: &NoiseType,
        points: &[[f64; 3]],
        out: &mut [PointSample3F64],
    ) {
        unsafe { sample_points_3d_gradient_impl_64::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_3d_with_gradient_f64`].
#[cfg(feature = "scalar")]
pub fn sample_points_3d_with_gradient_f64(
    noise_type: &NoiseType,
    points: &[[f64; 3]],
    out: &mut [PointSample3F64],
) {
    unsafe { sample_points_3d_gradient_impl_64::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 4D points and return only the noise values.
    pub fn sample_points_4d(noise_type: &NoiseType, points: &[[f32; 4]], out: &mut [f32]) {
        unsafe { sample_points_4d_values_impl::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_4d`].
#[cfg(feature = "scalar")]
pub fn sample_points_4d(noise_type: &NoiseType, points: &[[f32; 4]], out: &mut [f32]) {
    unsafe { sample_points_4d_values_impl::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 4D points with analytic first derivatives.
    ///
    /// Cellular 4D gradients are not implemented; use the value-only entry
    /// point for cellular noise in 4D.
    pub fn sample_points_4d_with_gradient(
        noise_type: &NoiseType,
        points: &[[f32; 4]],
        out: &mut [PointSample4],
    ) {
        unsafe { sample_points_4d_gradient_impl::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_4d_with_gradient`].
#[cfg(feature = "scalar")]
pub fn sample_points_4d_with_gradient(
    noise_type: &NoiseType,
    points: &[[f32; 4]],
    out: &mut [PointSample4],
) {
    unsafe { sample_points_4d_gradient_impl::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 4D points in f64 and return only the noise values.
    pub fn sample_points_4d_f64(noise_type: &NoiseType, points: &[[f64; 4]], out: &mut [f64]) {
        unsafe { sample_points_4d_values_impl_64::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_4d_f64`].
#[cfg(feature = "scalar")]
pub fn sample_points_4d_f64(noise_type: &NoiseType, points: &[[f64; 4]], out: &mut [f64]) {
    unsafe { sample_points_4d_values_impl_64::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// f64 twin of [`sample_points_4d_with_gradient`].
    pub fn sample_points_4d_with_gradient_f64(
        noise_type: &NoiseType,
        points: &[[f64; 4]],
        out: &mut [PointSample4F64],
    ) {
        unsafe { sample_points_4d_gradient_impl_64::<S>(noise_type, points, out) }
    }
);

/// Scalar-only twin of [`sample_points_4d_with_gradient_f64`].
#[cfg(feature = "scalar")]
pub fn sample_points_4d_with_gradient_f64(
    noise_type: &NoiseType,
    points: &[[f64; 4]],
    out: &mut [PointSample4F64],
) {
    unsafe { sample_points_4d_gradient_impl_64::<simdeez::scalar::Scalar>(noise_type, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 2D points of Gabor noise and return only the values.
    pub fn sample_gabor_points_2d(settings: &GaborSettings, points: &[[f32; 2]], out: &mut [f32]) {
        unsafe { sample_gabor_2d_values_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_gabor_points_2d`].
#[cfg(feature = "scalar")]
pub fn sample_gabor_points_2d(settings: &GaborSettings, points: &[[f32; 2]], out: &mut [f32]) {
    unsafe { sample_gabor_2d_values_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 2D points of Gabor noise with analytic derivatives.
    ///
    /// `points` and `out` must have the same length. The settings must be 2D.
    ///
    /// # Panics
    ///
    /// Panics if the lengths differ, the settings are not 2D, or the Gabor
    /// settings are invalid.
    pub fn sample_gabor_points_2d_with_gradient(
        settings: &GaborSettings,
        points: &[[f32; 2]],
        out: &mut [PointSample2],
    ) {
        unsafe { sample_gabor_2d_gradient_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_gabor_points_2d`].
#[cfg(feature = "scalar")]
pub fn sample_gabor_points_2d_with_gradient(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [PointSample2],
) {
    unsafe { sample_gabor_2d_gradient_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points of Gabor noise and return only the values.
    pub fn sample_gabor_points_3d(settings: &GaborSettings, points: &[[f32; 3]], out: &mut [f32]) {
        unsafe { sample_gabor_3d_values_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_gabor_points_3d`].
#[cfg(feature = "scalar")]
pub fn sample_gabor_points_3d(settings: &GaborSettings, points: &[[f32; 3]], out: &mut [f32]) {
    unsafe { sample_gabor_3d_values_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points of Gabor noise with analytic derivatives.
    ///
    /// `points` and `out` must have the same length. The settings must be 3D.
    ///
    /// # Panics
    ///
    /// Panics if the lengths differ, the settings are not 3D, or the Gabor
    /// settings are invalid.
    pub fn sample_gabor_points_3d_with_gradient(
        settings: &GaborSettings,
        points: &[[f32; 3]],
        out: &mut [PointSample3],
    ) {
        unsafe { sample_gabor_3d_gradient_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_gabor_points_3d`].
#[cfg(feature = "scalar")]
pub fn sample_gabor_points_3d_with_gradient(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [PointSample3],
) {
    unsafe { sample_gabor_3d_gradient_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 4D points of Gabor noise and return only the values.
    pub fn sample_gabor_points_4d(settings: &GaborSettings, points: &[[f32; 4]], out: &mut [f32]) {
        unsafe { sample_gabor_4d_values_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_gabor_points_4d`].
#[cfg(feature = "scalar")]
pub fn sample_gabor_points_4d(settings: &GaborSettings, points: &[[f32; 4]], out: &mut [f32]) {
    unsafe { sample_gabor_4d_values_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 4D points of Gabor noise with analytic derivatives.
    ///
    /// `points` and `out` must have the same length. The settings must be 4D.
    ///
    /// # Panics
    ///
    /// Panics if the lengths differ, the settings are not 4D, or the Gabor
    /// settings are invalid.
    pub fn sample_gabor_points_4d_with_gradient(
        settings: &GaborSettings,
        points: &[[f32; 4]],
        out: &mut [PointSample4],
    ) {
        unsafe { sample_gabor_4d_gradient_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_gabor_points_4d`].
#[cfg(feature = "scalar")]
pub fn sample_gabor_points_4d_with_gradient(
    settings: &GaborSettings,
    points: &[[f32; 4]],
    out: &mut [PointSample4],
) {
    unsafe { sample_gabor_4d_gradient_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 2D points of complex Gabor (phasor) noise.
    ///
    /// Returns only the complex value; use
    /// [`sample_phasor_points_2d_with_gradient`] for derivatives.
    pub fn sample_phasor_points_2d(
        settings: &GaborSettings,
        points: &[[f32; 2]],
        out: &mut [PhasorValue2],
    ) {
        unsafe { sample_phasor_2d_values_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_phasor_points_2d`].
#[cfg(feature = "scalar")]
pub fn sample_phasor_points_2d(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [PhasorValue2],
) {
    unsafe { sample_phasor_2d_values_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 2D points of complex Gabor (phasor) noise.
    ///
    /// `real` is the ordinary Gabor value; `imag` is its quadrature part.
    /// Both carry analytic gradients. The settings must be 2D.
    pub fn sample_phasor_points_2d_with_gradient(
        settings: &GaborSettings,
        points: &[[f32; 2]],
        out: &mut [PhasorSample2],
    ) {
        unsafe { sample_phasor_2d_gradient_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_phasor_points_2d`].
#[cfg(feature = "scalar")]
pub fn sample_phasor_points_2d_with_gradient(
    settings: &GaborSettings,
    points: &[[f32; 2]],
    out: &mut [PhasorSample2],
) {
    unsafe { sample_phasor_2d_gradient_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points of complex Gabor (phasor) noise.
    ///
    /// Returns only the complex value; use
    /// [`sample_phasor_points_3d_with_gradient`] for derivatives.
    pub fn sample_phasor_points_3d(
        settings: &GaborSettings,
        points: &[[f32; 3]],
        out: &mut [PhasorValue3],
    ) {
        unsafe { sample_phasor_3d_values_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_phasor_points_3d`].
#[cfg(feature = "scalar")]
pub fn sample_phasor_points_3d(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [PhasorValue3],
) {
    unsafe { sample_phasor_3d_values_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

#[cfg(not(feature = "scalar"))]
simdeez::simd_runtime_generate!(
    /// Evaluate arbitrary 3D points of complex Gabor (phasor) noise.
    ///
    /// `real` is the ordinary Gabor value; `imag` is its quadrature part.
    /// Both carry analytic gradients. The settings must be 3D.
    pub fn sample_phasor_points_3d_with_gradient(
        settings: &GaborSettings,
        points: &[[f32; 3]],
        out: &mut [PhasorSample3],
    ) {
        unsafe { sample_phasor_3d_gradient_impl::<S>(settings, points, out) }
    }
);

/// Scalar-only twin of [`sample_phasor_points_3d`].
#[cfg(feature = "scalar")]
pub fn sample_phasor_points_3d_with_gradient(
    settings: &GaborSettings,
    points: &[[f32; 3]],
    out: &mut [PhasorSample3],
) {
    unsafe { sample_phasor_3d_gradient_impl::<simdeez::scalar::Scalar>(settings, points, out) }
}

/// Evaluate arbitrary 2D points and a finite-difference Hessian.
///
/// The Hessian is a central difference of the analytic gradient. It costs five
/// batch evaluations per call (one for the value/gradient and four for the
/// Hessian columns), so use it only when curvature is required.
///
/// # Panics
///
/// Same as [`sample_points_2d_with_gradient`].
pub fn sample_points_2d_with_hessian(
    noise_type: &NoiseType,
    points: &[[f32; 2]],
    out: &mut [PointSampleHessian2],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    if points.is_empty() {
        return;
    }
    let step = hessian_step_2d(noise_type);
    let mut base = vec![PointSample2::default(); points.len()];
    sample_points_2d_with_gradient(noise_type, points, &mut base);
    for (sample, base) in out.iter_mut().zip(base.iter()) {
        sample.value = base.value;
        sample.gradient = base.gradient;
    }

    let mut plus = vec![[0.0f32; 2]; points.len()];
    let mut minus = vec![[0.0f32; 2]; points.len()];
    let mut plus_samples = vec![PointSample2::default(); points.len()];
    let mut minus_samples = vec![PointSample2::default(); points.len()];
    for axis in 0..2 {
        for (index, point) in points.iter().enumerate() {
            plus[index] = *point;
            minus[index] = *point;
            plus[index][axis] += step;
            minus[index][axis] -= step;
        }
        sample_points_2d_with_gradient(noise_type, &plus, &mut plus_samples);
        sample_points_2d_with_gradient(noise_type, &minus, &mut minus_samples);
        for index in 0..points.len() {
            for row in 0..2 {
                out[index].hessian[row][axis] = (plus_samples[index].gradient[row]
                    - minus_samples[index].gradient[row])
                    / (2.0 * step);
            }
        }
    }
}

/// Evaluate arbitrary 3D points and a finite-difference Hessian.
///
/// The Hessian is a central difference of the analytic gradient. It costs seven
/// batch evaluations per call (one for the value/gradient and six for the
/// Hessian columns), so use it only when curvature is required.
///
/// # Panics
///
/// Same as [`sample_points_3d_with_gradient`].
pub fn sample_points_3d_with_hessian(
    noise_type: &NoiseType,
    points: &[[f32; 3]],
    out: &mut [PointSampleHessian3],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    if points.is_empty() {
        return;
    }
    let step = hessian_step_3d(noise_type);
    let mut base = vec![PointSample3::default(); points.len()];
    sample_points_3d_with_gradient(noise_type, points, &mut base);
    for (sample, base) in out.iter_mut().zip(base.iter()) {
        sample.value = base.value;
        sample.gradient = base.gradient;
    }

    let mut plus = vec![[0.0f32; 3]; points.len()];
    let mut minus = vec![[0.0f32; 3]; points.len()];
    let mut plus_samples = vec![PointSample3::default(); points.len()];
    let mut minus_samples = vec![PointSample3::default(); points.len()];
    for axis in 0..3 {
        for (index, point) in points.iter().enumerate() {
            plus[index] = *point;
            minus[index] = *point;
            plus[index][axis] += step;
            minus[index][axis] -= step;
        }
        sample_points_3d_with_gradient(noise_type, &plus, &mut plus_samples);
        sample_points_3d_with_gradient(noise_type, &minus, &mut minus_samples);
        for index in 0..points.len() {
            for row in 0..3 {
                out[index].hessian[row][axis] = (plus_samples[index].gradient[row]
                    - minus_samples[index].gradient[row])
                    / (2.0 * step);
            }
        }
    }
}

/// f64 twin of [`sample_points_3d_with_hessian`].
///
/// # Panics
///
/// Same as [`sample_points_3d_with_gradient_f64`].
pub fn sample_points_3d_with_hessian_f64(
    noise_type: &NoiseType,
    points: &[[f64; 3]],
    out: &mut [PointSampleHessian3F64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    if points.is_empty() {
        return;
    }
    let step = hessian_step_3d_64(noise_type);
    let mut base = vec![PointSample3F64::default(); points.len()];
    sample_points_3d_with_gradient_f64(noise_type, points, &mut base);
    for (sample, base) in out.iter_mut().zip(base.iter()) {
        sample.value = base.value;
        sample.gradient = base.gradient;
    }

    let mut plus = vec![[0.0f64; 3]; points.len()];
    let mut minus = vec![[0.0f64; 3]; points.len()];
    let mut plus_samples = vec![PointSample3F64::default(); points.len()];
    let mut minus_samples = vec![PointSample3F64::default(); points.len()];
    for axis in 0..3 {
        for (index, point) in points.iter().enumerate() {
            plus[index] = *point;
            minus[index] = *point;
            plus[index][axis] += step;
            minus[index][axis] -= step;
        }
        sample_points_3d_with_gradient_f64(noise_type, &plus, &mut plus_samples);
        sample_points_3d_with_gradient_f64(noise_type, &minus, &mut minus_samples);
        for index in 0..points.len() {
            for row in 0..3 {
                out[index].hessian[row][axis] = (plus_samples[index].gradient[row]
                    - minus_samples[index].gradient[row])
                    / (2.0 * step);
            }
        }
    }
}

/// Evaluate arbitrary 4D points and a finite-difference Hessian.
///
/// The Hessian is a central difference of the analytic gradient. It costs nine
/// batch evaluations per call (one for the value/gradient and eight for the
/// Hessian columns), so use it only when curvature is required.
///
/// # Panics
///
/// Same as [`sample_points_4d_with_gradient`].
pub fn sample_points_4d_with_hessian(
    noise_type: &NoiseType,
    points: &[[f32; 4]],
    out: &mut [PointSampleHessian4],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    if points.is_empty() {
        return;
    }
    let step = hessian_step_4d(noise_type);
    let mut base = vec![PointSample4::default(); points.len()];
    sample_points_4d_with_gradient(noise_type, points, &mut base);
    for (sample, base) in out.iter_mut().zip(base.iter()) {
        sample.value = base.value;
        sample.gradient = base.gradient;
    }

    let mut plus = vec![[0.0f32; 4]; points.len()];
    let mut minus = vec![[0.0f32; 4]; points.len()];
    let mut plus_samples = vec![PointSample4::default(); points.len()];
    let mut minus_samples = vec![PointSample4::default(); points.len()];
    for axis in 0..4 {
        for (index, point) in points.iter().enumerate() {
            plus[index] = *point;
            minus[index] = *point;
            plus[index][axis] += step;
            minus[index][axis] -= step;
        }
        sample_points_4d_with_gradient(noise_type, &plus, &mut plus_samples);
        sample_points_4d_with_gradient(noise_type, &minus, &mut minus_samples);
        for index in 0..points.len() {
            for row in 0..4 {
                out[index].hessian[row][axis] = (plus_samples[index].gradient[row]
                    - minus_samples[index].gradient[row])
                    / (2.0 * step);
            }
        }
    }
}

/// f64 twin of [`sample_points_4d_with_hessian`].
///
/// # Panics
///
/// Same as [`sample_points_4d_with_gradient_f64`].
pub fn sample_points_4d_with_hessian_f64(
    noise_type: &NoiseType,
    points: &[[f64; 4]],
    out: &mut [PointSampleHessian4F64],
) {
    assert_eq!(points.len(), out.len(), "point and output lengths differ");
    if points.is_empty() {
        return;
    }
    let step = hessian_step_4d_64(noise_type);
    let mut base = vec![PointSample4F64::default(); points.len()];
    sample_points_4d_with_gradient_f64(noise_type, points, &mut base);
    for (sample, base) in out.iter_mut().zip(base.iter()) {
        sample.value = base.value;
        sample.gradient = base.gradient;
    }

    let mut plus = vec![[0.0f64; 4]; points.len()];
    let mut minus = vec![[0.0f64; 4]; points.len()];
    let mut plus_samples = vec![PointSample4F64::default(); points.len()];
    let mut minus_samples = vec![PointSample4F64::default(); points.len()];
    for axis in 0..4 {
        for (index, point) in points.iter().enumerate() {
            plus[index] = *point;
            minus[index] = *point;
            plus[index][axis] += step;
            minus[index][axis] -= step;
        }
        sample_points_4d_with_gradient_f64(noise_type, &plus, &mut plus_samples);
        sample_points_4d_with_gradient_f64(noise_type, &minus, &mut minus_samples);
        for index in 0..points.len() {
            for row in 0..4 {
                out[index].hessian[row][axis] = (plus_samples[index].gradient[row]
                    - minus_samples[index].gradient[row])
                    / (2.0 * step);
            }
        }
    }
}
