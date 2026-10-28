//! Gabor noise 2D/3D: a sparse convolution of hash-placed impulses with a
//! Gaussian envelope and an oriented cosine carrier.
//!
//! Each integer lattice cell contributes one impulse. The impulse position,
//! phase and amplitude come from the cell hash, so the field is deterministic
//! for a given seed. The kernel is normalized by the number of impulses, which
//! guarantees `|value| <= gain` for a non-negative `gain`.

use crate::noise::hash3d_32::hash3d_i32;
use crate::settings::GaborSettings;
use crate::NoiseAndGradient;
use simdeez::math::SimdMathF32Core;
use simdeez::prelude::*;
use std::f32::consts::{PI, TAU};

/// Ten random bits per component.
const RAND_MASK: i32 = 0x3FF;
const RAND_SCALE: f32 = 1.0 / 1024.0;

/// `(real, imag, real_grad, imag_grad)` for 2D phasor noise.
type Phasor2<S> = (
    <S as Simd>::Vf32,
    <S as Simd>::Vf32,
    [<S as Simd>::Vf32; 2],
    [<S as Simd>::Vf32; 2],
);
/// `(real, imag, real_grad, imag_grad)` for 3D phasor noise.
type Phasor3<S> = (
    <S as Simd>::Vf32,
    <S as Simd>::Vf32,
    [<S as Simd>::Vf32; 3],
    [<S as Simd>::Vf32; 3],
);

#[inline(always)]
fn rand01<S: Simd>(hash: S::Vi32, shift: i32) -> S::Vf32 {
    ((hash >> shift) & S::Vi32::set1(RAND_MASK)).cast_f32() * S::Vf32::set1(RAND_SCALE)
}

/// 2D Gabor noise at `(x, y)`, in lattice-cell space.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn gabor_2d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 2],
    radius: i32,
    gain: f32,
) -> S::Vf32 {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();

    let inv_sigma2_pi = S::Vf32::set1(PI / (bandwidth * bandwidth));
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];

    let mut sum = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let cx = ix + S::Vi32::set1(dx);
            let cy = iy + S::Vi32::set1(dy);
            let zero = S::Vi32::zeroes();
            let h0 = hash3d_i32::<S>(seed, cx, cy, zero);
            let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, zero);

            let rx0: S::Vf32 = rand01::<S>(h0, 0);
            let ry0: S::Vf32 = rand01::<S>(h0, 10);
            let qx: S::Vf32 = cx.cast_f32() + rx0;
            let qy: S::Vf32 = cy.cast_f32() + ry0;
            let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

            let rx = x - qx;
            let ry = y - qy;
            let r2 = rx * rx + ry * ry;
            let exponent: S::Vf32 = r2 * inv_sigma2_pi;
            let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
            let projection = rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy);
            let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
            let carrier: S::Vf32 = angle.cos_u35();
            sum += envelope * carrier;

            count += 1.0;
            dy += 1;
        }
        dx += 1;
    }

    sum * S::Vf32::set1(gain / count)
}

/// 3D Gabor noise at `(x, y, z)`, in lattice-cell space.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn gabor_3d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 3],
    radius: i32,
    gain: f32,
) -> S::Vf32 {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let z = z * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();
    let iz = z.floor().cast_i32();

    let inv_sigma2_pi = S::Vf32::set1(PI / (bandwidth * bandwidth));
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];
    let oz = orientation[2];

    let mut sum = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let mut dz = -radius;
            while dz <= radius {
                let cx = ix + S::Vi32::set1(dx);
                let cy = iy + S::Vi32::set1(dy);
                let cz = iz + S::Vi32::set1(dz);
                let h0 = hash3d_i32::<S>(seed, cx, cy, cz);
                let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, cz);

                let rx0: S::Vf32 = rand01::<S>(h0, 0);
                let ry0: S::Vf32 = rand01::<S>(h0, 10);
                let rz0: S::Vf32 = rand01::<S>(h0, 20);
                let qx: S::Vf32 = cx.cast_f32() + rx0;
                let qy: S::Vf32 = cy.cast_f32() + ry0;
                let qz: S::Vf32 = cz.cast_f32() + rz0;
                let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

                let rx = x - qx;
                let ry = y - qy;
                let rz = z - qz;
                let r2 = rx * rx + ry * ry + rz * rz;
                let exponent: S::Vf32 = r2 * inv_sigma2_pi;
                let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
                let projection =
                    rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy) + rz * S::Vf32::set1(oz);
                let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
                let carrier: S::Vf32 = angle.cos_u35();
                sum += envelope * carrier;

                count += 1.0;
                dz += 1;
            }
            dy += 1;
        }
        dx += 1;
    }

    sum * S::Vf32::set1(gain / count)
}

/// Fill a 2D block with Gabor noise. `result` must be `width * height` long.
#[inline(always)]
pub unsafe fn fill_gabor_2d<S: Simd>(settings: &GaborSettings, result: &mut [f32]) -> (f32, f32) {
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    assert_eq!(
        result.len(),
        width * height,
        "noise output slice length must equal width * height"
    );

    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let cell_scale = settings.impulse_density.powf(0.5);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
    ];

    let mut min_s = S::Vf32::set1(f32::INFINITY);
    let mut max_s = S::Vf32::set1(f32::NEG_INFINITY);
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(dimension.offset_x + i as f32);
    }
    x_arr.set_len(vector_width);

    for row in 0..height {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let y = S::Vf32::set1(dimension.offset_y + row as f32);
        for _ in 0..width / vector_width {
            let f = gabor_2d::<S>(
                x * frequency_x,
                y * frequency_y,
                dimension.seed,
                cell_scale,
                settings.carrier_frequency,
                settings.bandwidth,
                orientation,
                settings.radius,
                settings.gain,
            );
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(result_ptr.add(i));
            i += vector_width;
            x += S::Vf32::set1(vector_width as f32);
        }
        if remainder != 0 {
            let f = gabor_2d::<S>(
                x * frequency_x,
                y * frequency_y,
                dimension.seed,
                cell_scale,
                settings.carrier_frequency,
                settings.bandwidth,
                orientation,
                settings.radius,
                settings.gain,
            );
            for j in 0..remainder {
                let n = f[j];
                result_ptr.add(i).write(n);
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

/// Fill a 3D block with Gabor noise. `result` must be `width * height * depth`
/// long.
#[inline(always)]
pub unsafe fn fill_gabor_3d<S: Simd>(settings: &GaborSettings, result: &mut [f32]) -> (f32, f32) {
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let depth = dimension.extent_z;
    assert_eq!(
        result.len(),
        width * height * depth,
        "noise output slice length must equal width * height * depth"
    );

    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let frequency_z = S::Vf32::set1(settings.frequency_z);
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

    let mut min_s = S::Vf32::set1(f32::INFINITY);
    let mut max_s = S::Vf32::set1(f32::NEG_INFINITY);
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(dimension.offset_x + i as f32);
    }
    x_arr.set_len(vector_width);

    for layer in 0..depth {
        let z = S::Vf32::set1(dimension.offset_z + layer as f32);
        for row in 0..height {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let y = S::Vf32::set1(dimension.offset_y + row as f32);
            for _ in 0..width / vector_width {
                let f = gabor_3d::<S>(
                    x * frequency_x,
                    y * frequency_y,
                    z * frequency_z,
                    dimension.seed,
                    cell_scale,
                    settings.carrier_frequency,
                    settings.bandwidth,
                    orientation,
                    settings.radius,
                    settings.gain,
                );
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(result_ptr.add(i));
                i += vector_width;
                x += S::Vf32::set1(vector_width as f32);
            }
            if remainder != 0 {
                let f = gabor_3d::<S>(
                    x * frequency_x,
                    y * frequency_y,
                    z * frequency_z,
                    dimension.seed,
                    cell_scale,
                    settings.carrier_frequency,
                    settings.bandwidth,
                    orientation,
                    settings.radius,
                    settings.gain,
                );
                for j in 0..remainder {
                    let n = f[j];
                    result_ptr.add(i).write(n);
                    if n < min {
                        min = n;
                    }
                    if n > max {
                        max = n;
                    }
                    i += 1;
                }
            }
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

/// 2D Gabor block, with the observed `(min, max)`.
pub fn gabor_2d_block<S: Simd>(settings: &GaborSettings) -> (Vec<f32>, f32, f32) {
    let dimension = settings.dimensions();
    let mut result = vec![0.0f32; dimension.extent_x * dimension.extent_y];
    let (min, max) = unsafe { fill_gabor_2d::<S>(settings, &mut result) };
    (result, min, max)
}

/// 3D Gabor block, with the observed `(min, max)`.
pub fn gabor_3d_block<S: Simd>(settings: &GaborSettings) -> (Vec<f32>, f32, f32) {
    let dimension = settings.dimensions();
    let mut result = vec![0.0f32; dimension.extent_x * dimension.extent_y * dimension.extent_z];
    let (min, max) = unsafe { fill_gabor_3d::<S>(settings, &mut result) };
    (result, min, max)
}

/// 2D Gabor noise and its analytic gradient with respect to the input
/// coordinates.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn gabor_2d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 2],
    radius: i32,
    gain: f32,
) -> (S::Vf32, [S::Vf32; 2]) {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];

    let mut sum = S::Vf32::zeroes();
    let mut gx = S::Vf32::zeroes();
    let mut gy = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let cx = ix + S::Vi32::set1(dx);
            let cy = iy + S::Vi32::set1(dy);
            let zero = S::Vi32::zeroes();
            let h0 = hash3d_i32::<S>(seed, cx, cy, zero);
            let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, zero);

            let rx0: S::Vf32 = rand01::<S>(h0, 0);
            let ry0: S::Vf32 = rand01::<S>(h0, 10);
            let qx: S::Vf32 = cx.cast_f32() + rx0;
            let qy: S::Vf32 = cy.cast_f32() + ry0;
            let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

            let rx = x - qx;
            let ry = y - qy;
            let r2 = rx * rx + ry * ry;
            let exponent: S::Vf32 = r2 * S::Vf32::set1(inv_sigma2_pi);
            let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
            let projection: S::Vf32 = rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy);
            let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
            let carrier: S::Vf32 = angle.cos_u35();
            let sine: S::Vf32 = angle.sin_u35();

            sum += envelope * carrier;

            let env_grad: S::Vf32 = envelope * S::Vf32::set1(-2.0 * inv_sigma2_pi);
            let common: S::Vf32 = envelope * sine * S::Vf32::set1(-omega);
            let gx_contrib: S::Vf32 = env_grad * rx * carrier + common * S::Vf32::set1(ox);
            let gy_contrib: S::Vf32 = env_grad * ry * carrier + common * S::Vf32::set1(oy);
            gx += gx_contrib;
            gy += gy_contrib;

            count += 1.0;
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (sum * scale, [gx * scale, gy * scale])
}

/// 3D Gabor noise and its analytic gradient with respect to the input
/// coordinates.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn gabor_3d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 3],
    radius: i32,
    gain: f32,
) -> (S::Vf32, [S::Vf32; 3]) {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let z = z * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();
    let iz = z.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];
    let oz = orientation[2];

    let mut sum = S::Vf32::zeroes();
    let mut gx = S::Vf32::zeroes();
    let mut gy = S::Vf32::zeroes();
    let mut gz = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let mut dz = -radius;
            while dz <= radius {
                let cx = ix + S::Vi32::set1(dx);
                let cy = iy + S::Vi32::set1(dy);
                let cz = iz + S::Vi32::set1(dz);
                let h0 = hash3d_i32::<S>(seed, cx, cy, cz);
                let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, cz);

                let rx0: S::Vf32 = rand01::<S>(h0, 0);
                let ry0: S::Vf32 = rand01::<S>(h0, 10);
                let rz0: S::Vf32 = rand01::<S>(h0, 20);
                let qx: S::Vf32 = cx.cast_f32() + rx0;
                let qy: S::Vf32 = cy.cast_f32() + ry0;
                let qz: S::Vf32 = cz.cast_f32() + rz0;
                let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

                let rx = x - qx;
                let ry = y - qy;
                let rz = z - qz;
                let r2 = rx * rx + ry * ry + rz * rz;
                let exponent: S::Vf32 = r2 * S::Vf32::set1(inv_sigma2_pi);
                let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
                let projection: S::Vf32 =
                    rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy) + rz * S::Vf32::set1(oz);
                let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
                let carrier: S::Vf32 = angle.cos_u35();
                let sine: S::Vf32 = angle.sin_u35();

                sum += envelope * carrier;

                let env_grad: S::Vf32 = envelope * S::Vf32::set1(-2.0 * inv_sigma2_pi);
                let common: S::Vf32 = envelope * sine * S::Vf32::set1(-omega);
                let gx_contrib: S::Vf32 = env_grad * rx * carrier + common * S::Vf32::set1(ox);
                let gy_contrib: S::Vf32 = env_grad * ry * carrier + common * S::Vf32::set1(oy);
                let gz_contrib: S::Vf32 = env_grad * rz * carrier + common * S::Vf32::set1(oz);
                gx += gx_contrib;
                gy += gy_contrib;
                gz += gz_contrib;

                count += 1.0;
                dz += 1;
            }
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (sum * scale, [gx * scale, gy * scale, gz * scale])
}

/// 2D complex Gabor (phasor) field without derivatives.
///
/// The real part is the ordinary Gabor value; the imaginary part is the
/// quadrature carrier. Return order is `(real, imag)`.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn phasor_2d_value<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 2],
    radius: i32,
    gain: f32,
) -> (S::Vf32, S::Vf32) {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];

    let mut real = S::Vf32::zeroes();
    let mut imag = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let cx = ix + S::Vi32::set1(dx);
            let cy = iy + S::Vi32::set1(dy);
            let zero = S::Vi32::zeroes();
            let h0 = hash3d_i32::<S>(seed, cx, cy, zero);
            let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, zero);

            let rx0: S::Vf32 = rand01::<S>(h0, 0);
            let ry0: S::Vf32 = rand01::<S>(h0, 10);
            let qx: S::Vf32 = cx.cast_f32() + rx0;
            let qy: S::Vf32 = cy.cast_f32() + ry0;
            let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

            let rx = x - qx;
            let ry = y - qy;
            let r2 = rx * rx + ry * ry;
            let exponent: S::Vf32 = r2 * S::Vf32::set1(inv_sigma2_pi);
            let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
            let projection: S::Vf32 = rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy);
            let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;

            real += envelope * angle.cos_u35();
            imag += envelope * angle.sin_u35();
            count += 1.0;
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (real * scale, imag * scale)
}

/// 2D complex Gabor (phasor) field and its analytic derivatives.
///
/// The real part is the ordinary Gabor value; the imaginary part is the
/// quadrature carrier. Return order is `(real, imag, real_grad, imag_grad)`.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn phasor_2d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 2],
    radius: i32,
    gain: f32,
) -> Phasor2<S> {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];

    let mut real = S::Vf32::zeroes();
    let mut imag = S::Vf32::zeroes();
    let mut grx = S::Vf32::zeroes();
    let mut gry = S::Vf32::zeroes();
    let mut gix = S::Vf32::zeroes();
    let mut giy = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let cx = ix + S::Vi32::set1(dx);
            let cy = iy + S::Vi32::set1(dy);
            let zero = S::Vi32::zeroes();
            let h0 = hash3d_i32::<S>(seed, cx, cy, zero);
            let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, zero);

            let rx0: S::Vf32 = rand01::<S>(h0, 0);
            let ry0: S::Vf32 = rand01::<S>(h0, 10);
            let qx: S::Vf32 = cx.cast_f32() + rx0;
            let qy: S::Vf32 = cy.cast_f32() + ry0;
            let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

            let rx = x - qx;
            let ry = y - qy;
            let r2 = rx * rx + ry * ry;
            let exponent: S::Vf32 = r2 * S::Vf32::set1(inv_sigma2_pi);
            let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
            let projection: S::Vf32 = rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy);
            let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
            let carrier: S::Vf32 = angle.cos_u35();
            let sine: S::Vf32 = angle.sin_u35();

            real += envelope * carrier;
            imag += envelope * sine;

            let env_grad: S::Vf32 = envelope * S::Vf32::set1(-2.0 * inv_sigma2_pi);
            let real_common: S::Vf32 = envelope * sine * S::Vf32::set1(-omega);
            let imag_common: S::Vf32 = envelope * carrier * S::Vf32::set1(omega);
            grx += env_grad * rx * carrier + real_common * S::Vf32::set1(ox);
            gry += env_grad * ry * carrier + real_common * S::Vf32::set1(oy);
            gix += env_grad * rx * sine + imag_common * S::Vf32::set1(ox);
            giy += env_grad * ry * sine + imag_common * S::Vf32::set1(oy);

            count += 1.0;
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (
        real * scale,
        imag * scale,
        [grx * scale, gry * scale],
        [gix * scale, giy * scale],
    )
}

/// 3D complex Gabor (phasor) field without derivatives.
///
/// Return order is `(real, imag)`.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn phasor_3d_value<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 3],
    radius: i32,
    gain: f32,
) -> (S::Vf32, S::Vf32) {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let z = z * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();
    let iz = z.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];
    let oz = orientation[2];

    let mut real = S::Vf32::zeroes();
    let mut imag = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let mut dz = -radius;
            while dz <= radius {
                let cx = ix + S::Vi32::set1(dx);
                let cy = iy + S::Vi32::set1(dy);
                let cz = iz + S::Vi32::set1(dz);
                let h0 = hash3d_i32::<S>(seed, cx, cy, cz);
                let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, cz);

                let rx0: S::Vf32 = rand01::<S>(h0, 0);
                let ry0: S::Vf32 = rand01::<S>(h0, 10);
                let rz0: S::Vf32 = rand01::<S>(h0, 20);
                let qx: S::Vf32 = cx.cast_f32() + rx0;
                let qy: S::Vf32 = cy.cast_f32() + ry0;
                let qz: S::Vf32 = cz.cast_f32() + rz0;
                let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

                let rx = x - qx;
                let ry = y - qy;
                let rz = z - qz;
                let r2 = rx * rx + ry * ry + rz * rz;
                let exponent: S::Vf32 = r2 * S::Vf32::set1(inv_sigma2_pi);
                let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
                let projection: S::Vf32 =
                    rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy) + rz * S::Vf32::set1(oz);
                let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;

                real += envelope * angle.cos_u35();
                imag += envelope * angle.sin_u35();
                count += 1.0;
                dz += 1;
            }
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (real * scale, imag * scale)
}

/// 3D complex Gabor (phasor) field and its analytic derivatives.
///
/// Return order is `(real, imag, real_grad, imag_grad)`.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn phasor_3d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 3],
    radius: i32,
    gain: f32,
) -> Phasor3<S> {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let z = z * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();
    let iz = z.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];
    let oz = orientation[2];

    let mut real = S::Vf32::zeroes();
    let mut imag = S::Vf32::zeroes();
    let mut grx = S::Vf32::zeroes();
    let mut gry = S::Vf32::zeroes();
    let mut grz = S::Vf32::zeroes();
    let mut gix = S::Vf32::zeroes();
    let mut giy = S::Vf32::zeroes();
    let mut giz = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let mut dz = -radius;
            while dz <= radius {
                let cx = ix + S::Vi32::set1(dx);
                let cy = iy + S::Vi32::set1(dy);
                let cz = iz + S::Vi32::set1(dz);
                let h0 = hash3d_i32::<S>(seed, cx, cy, cz);
                let h1 = hash3d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, cz);

                let rx0: S::Vf32 = rand01::<S>(h0, 0);
                let ry0: S::Vf32 = rand01::<S>(h0, 10);
                let rz0: S::Vf32 = rand01::<S>(h0, 20);
                let qx: S::Vf32 = cx.cast_f32() + rx0;
                let qy: S::Vf32 = cy.cast_f32() + ry0;
                let qz: S::Vf32 = cz.cast_f32() + rz0;
                let phase: S::Vf32 = rand01::<S>(h1, 0) * S::Vf32::set1(TAU);

                let rx = x - qx;
                let ry = y - qy;
                let rz = z - qz;
                let r2 = rx * rx + ry * ry + rz * rz;
                let exponent: S::Vf32 = r2 * S::Vf32::set1(inv_sigma2_pi);
                let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
                let projection: S::Vf32 =
                    rx * S::Vf32::set1(ox) + ry * S::Vf32::set1(oy) + rz * S::Vf32::set1(oz);
                let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
                let carrier: S::Vf32 = angle.cos_u35();
                let sine: S::Vf32 = angle.sin_u35();

                real += envelope * carrier;
                imag += envelope * sine;

                let env_grad: S::Vf32 = envelope * S::Vf32::set1(-2.0 * inv_sigma2_pi);
                let real_common: S::Vf32 = envelope * sine * S::Vf32::set1(-omega);
                let imag_common: S::Vf32 = envelope * carrier * S::Vf32::set1(omega);
                grx += env_grad * rx * carrier + real_common * S::Vf32::set1(ox);
                gry += env_grad * ry * carrier + real_common * S::Vf32::set1(oy);
                grz += env_grad * rz * carrier + real_common * S::Vf32::set1(oz);
                gix += env_grad * rx * sine + imag_common * S::Vf32::set1(ox);
                giy += env_grad * ry * sine + imag_common * S::Vf32::set1(oy);
                giz += env_grad * rz * sine + imag_common * S::Vf32::set1(oz);

                count += 1.0;
                dz += 1;
            }
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (
        real * scale,
        imag * scale,
        [grx * scale, gry * scale, grz * scale],
        [gix * scale, giy * scale, giz * scale],
    )
}

/// Fill a 2D block with Gabor values and gradients. `gradients` must be
/// `values.len() * 2` long.
#[inline(always)]
pub unsafe fn fill_gabor_2d_gradient<S: Simd>(
    settings: &GaborSettings,
    values: &mut [f32],
    gradients: &mut [f32],
) -> (f32, f32) {
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    assert_eq!(
        values.len(),
        width * height,
        "noise output slice length must equal width * height"
    );
    assert_eq!(
        gradients.len(),
        values.len() * 2,
        "gradient slice length must equal values.len() * 2"
    );

    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let cell_scale = settings.impulse_density.powf(0.5);
    let orientation_len = (settings.orientation_x * settings.orientation_x
        + settings.orientation_y * settings.orientation_y)
        .sqrt();
    let orientation = [
        settings.orientation_x / orientation_len,
        settings.orientation_y / orientation_len,
    ];

    let mut min_s = S::Vf32::set1(f32::INFINITY);
    let mut max_s = S::Vf32::set1(f32::NEG_INFINITY);
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let values_ptr = values.as_mut_ptr();
    let gradients_ptr = gradients.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(dimension.offset_x + i as f32);
    }
    x_arr.set_len(vector_width);

    for row in 0..height {
        let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
        let y = S::Vf32::set1(dimension.offset_y + row as f32);
        for _ in 0..width / vector_width {
            let (f, [gx, gy]) = gabor_2d_deriv::<S>(
                x * frequency_x,
                y * frequency_y,
                dimension.seed,
                cell_scale,
                settings.carrier_frequency,
                settings.bandwidth,
                orientation,
                settings.radius,
                settings.gain,
            );
            let gx = gx * frequency_x;
            let gy = gy * frequency_y;
            max_s = max_s.max(f);
            min_s = min_s.min(f);
            f.copy_to_ptr_unaligned(values_ptr.add(i));
            for lane in 0..vector_width {
                let base = (i + lane) * 2;
                *gradients_ptr.add(base) = gx[lane];
                *gradients_ptr.add(base + 1) = gy[lane];
            }
            i += vector_width;
            x += S::Vf32::set1(vector_width as f32);
        }
        if remainder != 0 {
            let (f, [gx, gy]) = gabor_2d_deriv::<S>(
                x * frequency_x,
                y * frequency_y,
                dimension.seed,
                cell_scale,
                settings.carrier_frequency,
                settings.bandwidth,
                orientation,
                settings.radius,
                settings.gain,
            );
            let gx = gx * frequency_x;
            let gy = gy * frequency_y;
            for lane in 0..remainder {
                let n = f[lane];
                values_ptr.add(i).write(n);
                let base = i * 2;
                *gradients_ptr.add(base) = gx[lane];
                *gradients_ptr.add(base + 1) = gy[lane];
                if n < min {
                    min = n;
                }
                if n > max {
                    max = n;
                }
                i += 1;
            }
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

/// Fill a 3D block with Gabor values and gradients. `gradients` must be
/// `values.len() * 3` long.
#[inline(always)]
pub unsafe fn fill_gabor_3d_gradient<S: Simd>(
    settings: &GaborSettings,
    values: &mut [f32],
    gradients: &mut [f32],
) -> (f32, f32) {
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let depth = dimension.extent_z;
    assert_eq!(
        values.len(),
        width * height * depth,
        "noise output slice length must equal width * height * depth"
    );
    assert_eq!(
        gradients.len(),
        values.len() * 3,
        "gradient slice length must equal values.len() * 3"
    );

    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let frequency_z = S::Vf32::set1(settings.frequency_z);
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

    let mut min_s = S::Vf32::set1(f32::INFINITY);
    let mut max_s = S::Vf32::set1(f32::NEG_INFINITY);
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let values_ptr = values.as_mut_ptr();
    let gradients_ptr = gradients.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(dimension.offset_x + i as f32);
    }
    x_arr.set_len(vector_width);

    for layer in 0..depth {
        let z = S::Vf32::set1(dimension.offset_z + layer as f32);
        for row in 0..height {
            let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
            let y = S::Vf32::set1(dimension.offset_y + row as f32);
            for _ in 0..width / vector_width {
                let (f, [gx, gy, gz]) = gabor_3d_deriv::<S>(
                    x * frequency_x,
                    y * frequency_y,
                    z * frequency_z,
                    dimension.seed,
                    cell_scale,
                    settings.carrier_frequency,
                    settings.bandwidth,
                    orientation,
                    settings.radius,
                    settings.gain,
                );
                let gx = gx * frequency_x;
                let gy = gy * frequency_y;
                let gz = gz * frequency_z;
                max_s = max_s.max(f);
                min_s = min_s.min(f);
                f.copy_to_ptr_unaligned(values_ptr.add(i));
                for lane in 0..vector_width {
                    let base = (i + lane) * 3;
                    *gradients_ptr.add(base) = gx[lane];
                    *gradients_ptr.add(base + 1) = gy[lane];
                    *gradients_ptr.add(base + 2) = gz[lane];
                }
                i += vector_width;
                x += S::Vf32::set1(vector_width as f32);
            }
            if remainder != 0 {
                let (f, [gx, gy, gz]) = gabor_3d_deriv::<S>(
                    x * frequency_x,
                    y * frequency_y,
                    z * frequency_z,
                    dimension.seed,
                    cell_scale,
                    settings.carrier_frequency,
                    settings.bandwidth,
                    orientation,
                    settings.radius,
                    settings.gain,
                );
                let gx = gx * frequency_x;
                let gy = gy * frequency_y;
                let gz = gz * frequency_z;
                for lane in 0..remainder {
                    let n = f[lane];
                    values_ptr.add(i).write(n);
                    let base = i * 3;
                    *gradients_ptr.add(base) = gx[lane];
                    *gradients_ptr.add(base + 1) = gy[lane];
                    *gradients_ptr.add(base + 2) = gz[lane];
                    if n < min {
                        min = n;
                    }
                    if n > max {
                        max = n;
                    }
                    i += 1;
                }
            }
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

/// 2D Gabor values and gradients.
pub fn gabor_2d_gradient<S: Simd>(settings: &GaborSettings) -> NoiseAndGradient {
    let dimension = settings.dimensions();
    let len = dimension.extent_x * dimension.extent_y;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 2];
    let (min, max) = unsafe { fill_gabor_2d_gradient::<S>(settings, &mut values, &mut gradients) };
    NoiseAndGradient {
        values,
        gradients,
        stride: 2,
        min,
        max,
    }
}

/// 3D Gabor values and gradients.
pub fn gabor_3d_gradient<S: Simd>(settings: &GaborSettings) -> NoiseAndGradient {
    let dimension = settings.dimensions();
    let len = dimension.extent_x * dimension.extent_y * dimension.extent_z;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 3];
    let (min, max) = unsafe { fill_gabor_3d_gradient::<S>(settings, &mut values, &mut gradients) };
    NoiseAndGradient {
        values,
        gradients,
        stride: 3,
        min,
        max,
    }
}

#[inline(always)]
fn hash4d_i32<S: Simd>(seed: i32, i: S::Vi32, j: S::Vi32, k: S::Vi32, l: S::Vi32) -> S::Vi32 {
    let mut hash = i ^ S::Vi32::set1(seed);
    hash = j ^ hash;
    hash = k ^ hash;
    hash = l ^ hash;
    hash = ((hash * hash) * S::Vi32::set1(60493)) * hash;
    (hash >> 13) ^ hash
}

/// 4D Gabor noise at `(x, y, z, w)`, in lattice-cell space.
///
/// The fourth axis is typically time. The kernel is the same sparse
/// convolution as 2D/3D, extended to a 4D impulse lattice and a 4D
/// orientation.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn gabor_4d<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 4],
    radius: i32,
    gain: f32,
) -> S::Vf32 {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let z = z * cell_scale;
    let w = w * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();
    let iz = z.floor().cast_i32();
    let iw = w.floor().cast_i32();

    let inv_sigma2_pi = S::Vf32::set1(PI / (bandwidth * bandwidth));
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];
    let oz = orientation[2];
    let ow = orientation[3];

    let mut sum = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let mut dz = -radius;
            while dz <= radius {
                let mut dw = -radius;
                while dw <= radius {
                    let cx = ix + S::Vi32::set1(dx);
                    let cy = iy + S::Vi32::set1(dy);
                    let cz = iz + S::Vi32::set1(dz);
                    let cw = iw + S::Vi32::set1(dw);
                    let h0 = hash4d_i32::<S>(seed, cx, cy, cz, cw);
                    let h1 = hash4d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, cz, cw);

                    let rx0: S::Vf32 = rand01::<S>(h0, 0);
                    let ry0: S::Vf32 = rand01::<S>(h0, 10);
                    let rz0: S::Vf32 = rand01::<S>(h0, 20);
                    let rw0: S::Vf32 = rand01::<S>(h1, 0);
                    let qx: S::Vf32 = cx.cast_f32() + rx0;
                    let qy: S::Vf32 = cy.cast_f32() + ry0;
                    let qz: S::Vf32 = cz.cast_f32() + rz0;
                    let qw: S::Vf32 = cw.cast_f32() + rw0;
                    let phase: S::Vf32 = rand01::<S>(h1, 10) * S::Vf32::set1(TAU);

                    let rx = x - qx;
                    let ry = y - qy;
                    let rz = z - qz;
                    let rw = w - qw;
                    let r2 = rx * rx + ry * ry + rz * rz + rw * rw;
                    let exponent: S::Vf32 = r2 * inv_sigma2_pi;
                    let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
                    let projection: S::Vf32 = rx * S::Vf32::set1(ox)
                        + ry * S::Vf32::set1(oy)
                        + rz * S::Vf32::set1(oz)
                        + rw * S::Vf32::set1(ow);
                    let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
                    let carrier: S::Vf32 = angle.cos_u35();
                    sum += envelope * carrier;

                    count += 1.0;
                    dw += 1;
                }
                dz += 1;
            }
            dy += 1;
        }
        dx += 1;
    }

    sum * S::Vf32::set1(gain / count)
}

/// 4D Gabor noise and its analytic gradient with respect to the input
/// coordinates.
#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn gabor_4d_deriv<S: Simd>(
    x: S::Vf32,
    y: S::Vf32,
    z: S::Vf32,
    w: S::Vf32,
    seed: i32,
    cell_scale: f32,
    carrier_frequency: f32,
    bandwidth: f32,
    orientation: [f32; 4],
    radius: i32,
    gain: f32,
) -> (S::Vf32, [S::Vf32; 4]) {
    let cell_scale = S::Vf32::set1(cell_scale);
    let x = x * cell_scale;
    let y = y * cell_scale;
    let z = z * cell_scale;
    let w = w * cell_scale;
    let ix = x.floor().cast_i32();
    let iy = y.floor().cast_i32();
    let iz = z.floor().cast_i32();
    let iw = w.floor().cast_i32();

    let inv_sigma2_pi = PI / (bandwidth * bandwidth);
    let omega = carrier_frequency * TAU;
    let ox = orientation[0];
    let oy = orientation[1];
    let oz = orientation[2];
    let ow = orientation[3];

    let mut sum = S::Vf32::zeroes();
    let mut gx = S::Vf32::zeroes();
    let mut gy = S::Vf32::zeroes();
    let mut gz = S::Vf32::zeroes();
    let mut gw = S::Vf32::zeroes();
    let mut count = 0.0f32;

    let mut dx = -radius;
    while dx <= radius {
        let mut dy = -radius;
        while dy <= radius {
            let mut dz = -radius;
            while dz <= radius {
                let mut dw = -radius;
                while dw <= radius {
                    let cx = ix + S::Vi32::set1(dx);
                    let cy = iy + S::Vi32::set1(dy);
                    let cz = iz + S::Vi32::set1(dz);
                    let cw = iw + S::Vi32::set1(dw);
                    let h0 = hash4d_i32::<S>(seed, cx, cy, cz, cw);
                    let h1 = hash4d_i32::<S>(seed.wrapping_add(0x5F35_6495), cx, cy, cz, cw);

                    let rx0: S::Vf32 = rand01::<S>(h0, 0);
                    let ry0: S::Vf32 = rand01::<S>(h0, 10);
                    let rz0: S::Vf32 = rand01::<S>(h0, 20);
                    let rw0: S::Vf32 = rand01::<S>(h1, 0);
                    let qx: S::Vf32 = cx.cast_f32() + rx0;
                    let qy: S::Vf32 = cy.cast_f32() + ry0;
                    let qz: S::Vf32 = cz.cast_f32() + rz0;
                    let qw: S::Vf32 = cw.cast_f32() + rw0;
                    let phase: S::Vf32 = rand01::<S>(h1, 10) * S::Vf32::set1(TAU);

                    let rx = x - qx;
                    let ry = y - qy;
                    let rz = z - qz;
                    let rw = w - qw;
                    let r2 = rx * rx + ry * ry + rz * rz + rw * rw;
                    let exponent: S::Vf32 = r2 * inv_sigma2_pi;
                    let envelope: S::Vf32 = (S::Vf32::zeroes() - exponent).exp_u35();
                    let projection: S::Vf32 = rx * S::Vf32::set1(ox)
                        + ry * S::Vf32::set1(oy)
                        + rz * S::Vf32::set1(oz)
                        + rw * S::Vf32::set1(ow);
                    let angle: S::Vf32 = projection * S::Vf32::set1(omega) + phase;
                    let carrier: S::Vf32 = angle.cos_u35();
                    let sine: S::Vf32 = angle.sin_u35();

                    sum += envelope * carrier;

                    let env_grad: S::Vf32 = envelope * S::Vf32::set1(-2.0 * inv_sigma2_pi);
                    let common: S::Vf32 = envelope * sine * S::Vf32::set1(-omega);
                    let gx_contrib: S::Vf32 = env_grad * rx * carrier + common * S::Vf32::set1(ox);
                    let gy_contrib: S::Vf32 = env_grad * ry * carrier + common * S::Vf32::set1(oy);
                    let gz_contrib: S::Vf32 = env_grad * rz * carrier + common * S::Vf32::set1(oz);
                    let gw_contrib: S::Vf32 = env_grad * rw * carrier + common * S::Vf32::set1(ow);
                    gx += gx_contrib;
                    gy += gy_contrib;
                    gz += gz_contrib;
                    gw += gw_contrib;

                    count += 1.0;
                    dw += 1;
                }
                dz += 1;
            }
            dy += 1;
        }
        dx += 1;
    }

    let scale = S::Vf32::set1(gain / count);
    (
        sum * scale,
        [gx * scale, gy * scale, gz * scale, gw * scale],
    )
}

/// Fill a 4D block with Gabor noise. `result` must be
/// `width * height * depth * time` long.
#[inline(always)]
pub unsafe fn fill_gabor_4d<S: Simd>(settings: &GaborSettings, result: &mut [f32]) -> (f32, f32) {
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let depth = dimension.extent_z;
    let time = dimension.extent_w;
    assert_eq!(
        result.len(),
        width * height * depth * time,
        "noise output slice length must equal width * height * depth * time"
    );

    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let frequency_z = S::Vf32::set1(settings.frequency_z);
    let frequency_w = S::Vf32::set1(settings.frequency_w);
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

    let mut min_s = S::Vf32::set1(f32::INFINITY);
    let mut max_s = S::Vf32::set1(f32::NEG_INFINITY);
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let result_ptr = result.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(dimension.offset_x + i as f32);
    }
    x_arr.set_len(vector_width);

    for frame in 0..time {
        let w = S::Vf32::set1(dimension.offset_w + frame as f32);
        for layer in 0..depth {
            let z = S::Vf32::set1(dimension.offset_z + layer as f32);
            for row in 0..height {
                let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
                let y = S::Vf32::set1(dimension.offset_y + row as f32);
                for _ in 0..width / vector_width {
                    let f = gabor_4d::<S>(
                        x * frequency_x,
                        y * frequency_y,
                        z * frequency_z,
                        w * frequency_w,
                        dimension.seed,
                        cell_scale,
                        settings.carrier_frequency,
                        settings.bandwidth,
                        orientation,
                        settings.radius,
                        settings.gain,
                    );
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    f.copy_to_ptr_unaligned(result_ptr.add(i));
                    i += vector_width;
                    x += S::Vf32::set1(vector_width as f32);
                }
                if remainder != 0 {
                    let f = gabor_4d::<S>(
                        x * frequency_x,
                        y * frequency_y,
                        z * frequency_z,
                        w * frequency_w,
                        dimension.seed,
                        cell_scale,
                        settings.carrier_frequency,
                        settings.bandwidth,
                        orientation,
                        settings.radius,
                        settings.gain,
                    );
                    for j in 0..remainder {
                        let n = f[j];
                        result_ptr.add(i).write(n);
                        if n < min {
                            min = n;
                        }
                        if n > max {
                            max = n;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

/// Fill a 4D block with Gabor values and gradients. `gradients` must be
/// `values.len() * 4` long.
#[inline(always)]
pub unsafe fn fill_gabor_4d_gradient<S: Simd>(
    settings: &GaborSettings,
    values: &mut [f32],
    gradients: &mut [f32],
) -> (f32, f32) {
    let dimension = settings.dimensions();
    let width = dimension.extent_x;
    let height = dimension.extent_y;
    let depth = dimension.extent_z;
    let time = dimension.extent_w;
    assert_eq!(
        values.len(),
        width * height * depth * time,
        "noise output slice length must equal width * height * depth * time"
    );
    assert_eq!(
        gradients.len(),
        values.len() * 4,
        "gradient slice length must equal values.len() * 4"
    );

    let frequency_x = S::Vf32::set1(settings.frequency_x);
    let frequency_y = S::Vf32::set1(settings.frequency_y);
    let frequency_z = S::Vf32::set1(settings.frequency_z);
    let frequency_w = S::Vf32::set1(settings.frequency_w);
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

    let mut min_s = S::Vf32::set1(f32::INFINITY);
    let mut max_s = S::Vf32::set1(f32::NEG_INFINITY);
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let values_ptr = values.as_mut_ptr();
    let gradients_ptr = gradients.as_mut_ptr();
    let mut i = 0;
    let vector_width = S::Vf32::WIDTH;
    let remainder = width % vector_width;
    let mut x_arr = Vec::<f32>::with_capacity(vector_width);
    let x_ptr = x_arr.as_mut_ptr();
    for i in (0..vector_width).rev() {
        x_ptr.add(i).write(dimension.offset_x + i as f32);
    }
    x_arr.set_len(vector_width);

    for frame in 0..time {
        let w = S::Vf32::set1(dimension.offset_w + frame as f32);
        for layer in 0..depth {
            let z = S::Vf32::set1(dimension.offset_z + layer as f32);
            for row in 0..height {
                let mut x = S::Vf32::load_from_ptr_unaligned(x_ptr);
                let y = S::Vf32::set1(dimension.offset_y + row as f32);
                for _ in 0..width / vector_width {
                    let (f, [gx, gy, gz, gw]) = gabor_4d_deriv::<S>(
                        x * frequency_x,
                        y * frequency_y,
                        z * frequency_z,
                        w * frequency_w,
                        dimension.seed,
                        cell_scale,
                        settings.carrier_frequency,
                        settings.bandwidth,
                        orientation,
                        settings.radius,
                        settings.gain,
                    );
                    let gx = gx * frequency_x;
                    let gy = gy * frequency_y;
                    let gz = gz * frequency_z;
                    let gw = gw * frequency_w;
                    max_s = max_s.max(f);
                    min_s = min_s.min(f);
                    f.copy_to_ptr_unaligned(values_ptr.add(i));
                    for lane in 0..vector_width {
                        let base = (i + lane) * 4;
                        *gradients_ptr.add(base) = gx[lane];
                        *gradients_ptr.add(base + 1) = gy[lane];
                        *gradients_ptr.add(base + 2) = gz[lane];
                        *gradients_ptr.add(base + 3) = gw[lane];
                    }
                    i += vector_width;
                    x += S::Vf32::set1(vector_width as f32);
                }
                if remainder != 0 {
                    let (f, [gx, gy, gz, gw]) = gabor_4d_deriv::<S>(
                        x * frequency_x,
                        y * frequency_y,
                        z * frequency_z,
                        w * frequency_w,
                        dimension.seed,
                        cell_scale,
                        settings.carrier_frequency,
                        settings.bandwidth,
                        orientation,
                        settings.radius,
                        settings.gain,
                    );
                    let gx = gx * frequency_x;
                    let gy = gy * frequency_y;
                    let gz = gz * frequency_z;
                    let gw = gw * frequency_w;
                    for lane in 0..remainder {
                        let n = f[lane];
                        values_ptr.add(i).write(n);
                        let base = i * 4;
                        *gradients_ptr.add(base) = gx[lane];
                        *gradients_ptr.add(base + 1) = gy[lane];
                        *gradients_ptr.add(base + 2) = gz[lane];
                        *gradients_ptr.add(base + 3) = gw[lane];
                        if n < min {
                            min = n;
                        }
                        if n > max {
                            max = n;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
    for i in 0..vector_width {
        if min_s[i] < min {
            min = min_s[i];
        }
        if max_s[i] > max {
            max = max_s[i];
        }
    }
    (min, max)
}

/// 4D Gabor block, with the observed `(min, max)`.
pub fn gabor_4d_block<S: Simd>(settings: &GaborSettings) -> (Vec<f32>, f32, f32) {
    let dimension = settings.dimensions();
    let mut result =
        vec![
            0.0f32;
            dimension.extent_x * dimension.extent_y * dimension.extent_z * dimension.extent_w
        ];
    let (min, max) = unsafe { fill_gabor_4d::<S>(settings, &mut result) };
    (result, min, max)
}

/// 4D Gabor values and gradients.
pub fn gabor_4d_gradient<S: Simd>(settings: &GaborSettings) -> NoiseAndGradient {
    let dimension = settings.dimensions();
    let len = dimension.extent_x * dimension.extent_y * dimension.extent_z * dimension.extent_w;
    let mut values = vec![0.0f32; len];
    let mut gradients = vec![0.0f32; len * 4];
    let (min, max) = unsafe { fill_gabor_4d_gradient::<S>(settings, &mut values, &mut gradients) };
    NoiseAndGradient {
        values,
        gradients,
        stride: 4,
        min,
        max,
    }
}
