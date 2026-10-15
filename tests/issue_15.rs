//! Regression test for issue #15: grid-like diagonal artifacts in turbulence_3d.
//!
//! Root cause (established by audit + measurement): a z-slice of classic 3D
//! simplex noise shows a one-directional (down-right) diagonal streak. It is
//! ~25-40% the biased 12-edge gradient set and ~60-75% the simplex (1,1,1) skew
//! geometry projecting onto the fixed-z plane (made visible by turbulence's
//! abs()). It is NOT a hash-quality bug (a strong finalizer changed nothing) and
//! NOT an axis-asymmetry bug (all x/y/z are handled symmetrically; xy/xz/yz
//! render planes are identical).
//!
//! Fix (issue #15): in the 3D path only,
//!   1. an OpenSimplex2-style "ImproveXY" domain rotation at the top of
//!      `simplex_3d_deriv` maps world-z onto the lattice (1,1,1) diagonal, so
//!      constant-z slices become isotropic cross-sections; and
//!   2. a balanced 12-gradient LUT (`gradient_32::grad3d_vec`) replaces the
//!      biased `hash & 13` edge selection.
//!
//! The trade-off: XY/z-slices become isotropic, the anisotropy moving to the
//! XZ/YZ planes — the right call for the z-slice / 2D-texture / animation use
//! that the issue is about.
//!
//! This test measures the anisotropy directly with the metric that the
//! investigation settled on (the earlier FFT and single-lag autocorrelation
//! attempts could not separate the artifact from the simplex lattice): the
//! SIGNED off-diagonal of the gradient structure tensor, measured after a
//! low-pass to the dominant-octave cell size,
//!     C = 2*E[gx*gy] / (E[gx*gx] + E[gy*gy]).
//! A clean, mirror-symmetric field gives C ~ 0; the streaky z-slice gave
//! C ~ +0.12. Threshold 0.05 separates them with wide margin.

use simdnoise::{NoiseBuilder, Settings, SimplexSettings};

const W: usize = 256;
const H: usize = 256;
const FREQ: f32 = 0.3;
const OCTAVES: u8 = 2;

/// Separable box blur, radius `r`, edge-clamped. Iterated, it approximates a
/// Gaussian; three radius-3 passes give sigma ~= 3.5 px, matching the
/// dominant-octave cell size for FREQ/OCTAVES (lacunarity 0.5).
fn box_blur(src: &[f32], w: usize, h: usize, r: usize) -> Vec<f32> {
    let mut tmp = vec![0.0f32; w * h];
    let n = (2 * r + 1) as f32;
    // horizontal
    for y in 0..h {
        for x in 0..w {
            let mut s = 0.0;
            for d in 0..=2 * r {
                let xx = (x + d).saturating_sub(r).min(w - 1);
                s += src[y * w + xx];
            }
            tmp[y * w + x] = s / n;
        }
    }
    // vertical
    let mut out = vec![0.0f32; w * h];
    for y in 0..h {
        for x in 0..w {
            let mut s = 0.0;
            for d in 0..=2 * r {
                let yy = (y + d).saturating_sub(r).min(h - 1);
                s += tmp[yy * w + x];
            }
            out[y * w + x] = s / n;
        }
    }
    out
}

/// Directional anisotropy C = 2*E[gx*gy]/(E[gx^2]+E[gy^2]) of `field` after a
/// low-pass. ~0 for an isotropic / mirror-symmetric field; large and one-signed
/// for the diagonal streak.
fn anisotropy(field: &[f32], w: usize, h: usize) -> f32 {
    let mean = field.iter().sum::<f32>() / field.len() as f32;
    let mut a: Vec<f32> = field.iter().map(|v| v - mean).collect();
    for _ in 0..3 {
        a = box_blur(&a, w, h, 3);
    }
    let (mut sxy, mut sxx, mut syy) = (0.0f64, 0.0f64, 0.0f64);
    let m = 8; // trim border
    for y in m..h - m {
        for x in m..w - m {
            let gx = (a[y * w + x + 1] - a[y * w + x - 1]) as f64;
            let gy = (a[(y + 1) * w + x] - a[(y - 1) * w + x]) as f64;
            sxy += gx * gy;
            sxx += gx * gx;
            syy += gy * gy;
        }
    }
    (2.0 * sxy / (sxx + syy)) as f32
}

fn turbulence_slice(dims: u8) -> Vec<f32> {
    match dims {
        2 => NoiseBuilder::turbulence_2d_offset(0.0, W, 0.0, H)
            .with_freq(FREQ)
            .with_octaves(OCTAVES)
            .generate()
            .0,
        4 => NoiseBuilder::turbulence_4d_offset(0.0, W, 0.0, H, 0.0, 1, 0.0, 1)
            .with_freq(FREQ)
            .with_octaves(OCTAVES)
            .generate()
            .0,
        _ => NoiseBuilder::turbulence_3d_offset(0.0, W, 0.0, H, 0.0, 1)
            .with_freq(FREQ)
            .with_octaves(OCTAVES)
            .generate()
            .0,
    }
}

/// The 3D z-slice must be about as isotropic as a 2D slice. Before the fix the
/// 3D C was ~+0.12 (far above threshold); after, it sits in the clean band.
#[test]
fn turbulence_3d_zslice_is_isotropic() {
    let c3 = anisotropy(&turbulence_slice(3), W, H);
    assert!(
        c3.abs() < 0.05,
        "turbulence_3d z-slice is anisotropic (C = {:.4}); the 3D diagonal-streak artifact is present",
        c3
    );
}

/// A 2D slice of 4D noise (z, w held constant) must likewise be isotropic. The
/// 4D artifact was even stronger than 3D (C ~ +0.60 before the fix). The
/// ImproveXYZ rotation removes it (grad4's 32-gradient set is already balanced,
/// so no gradient LUT is needed there).
#[test]
fn turbulence_4d_slice_is_isotropic() {
    let c4 = anisotropy(&turbulence_slice(4), W, H);
    assert!(
        c4.abs() < 0.05,
        "turbulence_4d slice is anisotropic (C = {:.4}); the diagonal-streak artifact is present",
        c4
    );
}

/// Sanity: the 2D slice is isotropic too (guards the metric itself).
#[test]
fn turbulence_2d_is_isotropic() {
    let c2 = anisotropy(&turbulence_slice(2), W, H);
    assert!(c2.abs() < 0.05, "turbulence_2d unexpectedly anisotropic (C = {:.4})", c2);
}
