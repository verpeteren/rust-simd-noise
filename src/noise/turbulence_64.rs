use crate::noise::simplex_64::{
    simplex_1d, simplex_1d_deriv, simplex_2d, simplex_2d_deriv, simplex_3d, simplex_3d_deriv,
    simplex_4d, simplex_4d_deriv, simplex_5d, simplex_5d_deriv, simplex_6d, simplex_6d_deriv,
};

use simdeez::prelude::*;

// Turbulence value + analytic gradient (f64 twins of
// `turbulence_32::turbulence_*_deriv`, issue #24). Turbulence stacks
// `|gain^k * simplex|`, so each octave's gradient is
// `sign(gain^k*simplex) * gain^k * lacunarity^k * simplexGrad`.

#[inline(always)]
pub fn turbulence_1d_deriv<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> (S::Vf64, S::Vf64) {
    let zero = S::Vf64::zeroes();
    let (s0, d0) = simplex_1d_deriv::<S>(x, seed);
    let mut result = s0.abs();
    let mut deriv = s0.cmp_lt(zero).blendv(d0, zero - d0);
    let mut amp = S::Vf64::set1(1.0);
    let mut lac_pow = S::Vf64::set1(1.0);
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        lac_pow *= lacunarity;
        let (s, d) = simplex_1d_deriv::<S>(x, seed);
        let q = s * amp;
        result += q.abs();
        let base = d * amp * lac_pow;
        deriv += q.cmp_lt(zero).blendv(base, zero - base);
    }
    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        lac_pow *= lacunarity;
        let fracv = S::Vf64::set1(frac.into());
        let (s, d) = simplex_1d_deriv::<S>(x, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let base = d * amp * lac_pow;
        deriv += q.cmp_lt(zero).blendv(base, zero - base) * fracv;
    }
    (result, deriv)
}

#[inline(always)]
pub fn turbulence_2d_deriv<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 2]) {
    let zero = S::Vf64::zeroes();
    let (s0, d0) = simplex_2d_deriv::<S>(x, y, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
    let mut amp = S::Vf64::set1(1.0);
    let mut lac_pow = S::Vf64::set1(1.0);
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (s, d) = simplex_2d_deriv::<S>(x, y, seed);
        let q = s * amp;
        result += q.abs();
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
    }
    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf64::set1(frac.into());
        let (s, d) = simplex_2d_deriv::<S>(x, y, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
    }
    (result, [dx, dy])
}

#[inline(always)]
pub fn turbulence_3d_deriv<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 3]) {
    let zero = S::Vf64::zeroes();
    let (s0, d0) = simplex_3d_deriv::<S>(x, y, z, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
    let mut dz = neg0.blendv(d0[2], zero - d0[2]);
    let mut amp = S::Vf64::set1(1.0);
    let mut lac_pow = S::Vf64::set1(1.0);
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (s, d) = simplex_3d_deriv::<S>(x, y, z, seed);
        let q = s * amp;
        result += q.abs();
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
        dz += neg.blendv(bz, zero - bz);
    }
    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf64::set1(frac.into());
        let (s, d) = simplex_3d_deriv::<S>(x, y, z, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
        dz += neg.blendv(bz, zero - bz) * fracv;
    }
    (result, [dx, dy, dz])
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_4d_deriv<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 4]) {
    let zero = S::Vf64::zeroes();
    let (s0, d0) = simplex_4d_deriv::<S>(x, y, z, w, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
    let mut dz = neg0.blendv(d0[2], zero - d0[2]);
    let mut dw = neg0.blendv(d0[3], zero - d0[3]);
    let mut amp = S::Vf64::set1(1.0);
    let mut lac_pow = S::Vf64::set1(1.0);
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (s, d) = simplex_4d_deriv::<S>(x, y, z, w, seed);
        let q = s * amp;
        result += q.abs();
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let bw = d[3] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
        dz += neg.blendv(bz, zero - bz);
        dw += neg.blendv(bw, zero - bw);
    }
    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf64::set1(frac.into());
        let (s, d) = simplex_4d_deriv::<S>(x, y, z, w, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let bw = d[3] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
        dz += neg.blendv(bz, zero - bz) * fracv;
        dw += neg.blendv(bw, zero - bw) * fracv;
    }
    (result, [dx, dy, dz, dw])
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_5d_deriv<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    mut u: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 5]) {
    let zero = S::Vf64::zeroes();
    let (s0, d0) = simplex_5d_deriv::<S>(x, y, z, w, u, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
    let mut dz = neg0.blendv(d0[2], zero - d0[2]);
    let mut dw = neg0.blendv(d0[3], zero - d0[3]);
    let mut du = neg0.blendv(d0[4], zero - d0[4]);
    let mut amp = S::Vf64::set1(1.0);
    let mut lac_pow = S::Vf64::set1(1.0);
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (s, d) = simplex_5d_deriv::<S>(x, y, z, w, u, seed);
        let q = s * amp;
        result += q.abs();
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let bw = d[3] * wt;
        let bu = d[4] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
        dz += neg.blendv(bz, zero - bz);
        dw += neg.blendv(bw, zero - bw);
        du += neg.blendv(bu, zero - bu);
    }
    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf64::set1(frac.into());
        let (s, d) = simplex_5d_deriv::<S>(x, y, z, w, u, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let bw = d[3] * wt;
        let bu = d[4] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
        dz += neg.blendv(bz, zero - bz) * fracv;
        dw += neg.blendv(bw, zero - bw) * fracv;
        du += neg.blendv(bu, zero - bu) * fracv;
    }
    (result, [dx, dy, dz, dw, du])
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_6d_deriv<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    mut u: S::Vf64,
    mut t: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> (S::Vf64, [S::Vf64; 6]) {
    let zero = S::Vf64::zeroes();
    let (s0, d0) = simplex_6d_deriv::<S>(x, y, z, w, u, t, seed);
    let mut result = s0.abs();
    let neg0 = s0.cmp_lt(zero);
    let mut dx = neg0.blendv(d0[0], zero - d0[0]);
    let mut dy = neg0.blendv(d0[1], zero - d0[1]);
    let mut dz = neg0.blendv(d0[2], zero - d0[2]);
    let mut dw = neg0.blendv(d0[3], zero - d0[3]);
    let mut du = neg0.blendv(d0[4], zero - d0[4]);
    let mut dt = neg0.blendv(d0[5], zero - d0[5]);
    let mut amp = S::Vf64::set1(1.0);
    let mut lac_pow = S::Vf64::set1(1.0);
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        lac_pow *= lac;
        let (s, d) = simplex_6d_deriv::<S>(x, y, z, w, u, t, seed);
        let q = s * amp;
        result += q.abs();
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let bw = d[3] * wt;
        let bu = d[4] * wt;
        let bt = d[5] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx);
        dy += neg.blendv(by, zero - by);
        dz += neg.blendv(bz, zero - bz);
        dw += neg.blendv(bw, zero - bw);
        du += neg.blendv(bu, zero - bu);
        dt += neg.blendv(bt, zero - bt);
    }
    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        lac_pow *= lac;
        let fracv = S::Vf64::set1(frac.into());
        let (s, d) = simplex_6d_deriv::<S>(x, y, z, w, u, t, seed);
        let q = s * amp;
        result += q.abs() * fracv;
        let wt = amp * lac_pow;
        let bx = d[0] * wt;
        let by = d[1] * wt;
        let bz = d[2] * wt;
        let bw = d[3] * wt;
        let bu = d[4] * wt;
        let bt = d[5] * wt;
        let neg = q.cmp_lt(zero);
        dx += neg.blendv(bx, zero - bx) * fracv;
        dy += neg.blendv(by, zero - by) * fracv;
        dz += neg.blendv(bz, zero - bz) * fracv;
        dw += neg.blendv(bw, zero - bw) * fracv;
        du += neg.blendv(bu, zero - bu) * fracv;
        dt += neg.blendv(bt, zero - bt) * fracv;
    }
    (result, [dx, dy, dz, dw, du, dt])
}

#[inline(always)]
pub fn turbulence_1d<S: Simd>(
    mut x: S::Vf64,
    lacunarity: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> S::Vf64 {
    let mut amp = S::Vf64::set1(1.0);
    let mut result = simplex_1d::<S>(x, seed).abs();

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lacunarity;
        amp *= gain;
        result += (simplex_1d::<S>(x, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lacunarity;
        amp *= gain;
        result += (simplex_1d::<S>(x, seed) * amp).abs() * S::Vf64::set1(frac.into());
    }

    result
}

#[inline(always)]
pub fn turbulence_2d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_2d::<S>(x, y, seed).abs();

    let mut amp = S::Vf64::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        amp *= gain;
        result += (simplex_2d::<S>(x, y, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        amp *= gain;
        result += (simplex_2d::<S>(x, y, seed) * amp).abs() * S::Vf64::set1(frac.into());
    }

    result
}

#[inline(always)]
pub fn turbulence_3d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_3d::<S>(x, y, z, seed).abs();
    let mut amp = S::Vf64::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result += (simplex_3d::<S>(x, y, z, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        amp *= gain;
        result += (simplex_3d::<S>(x, y, z, seed) * amp).abs() * S::Vf64::set1(frac.into());
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_4d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_4d::<S>(x, y, z, w, seed).abs();
    let mut amp = S::Vf64::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result += (simplex_4d::<S>(x, y, z, w, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        amp *= gain;
        result += (simplex_4d::<S>(x, y, z, w, seed) * amp).abs() * S::Vf64::set1(frac.into());
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_5d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    mut u: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_5d::<S>(x, y, z, w, u, seed).abs();
    let mut amp = S::Vf64::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        result += (simplex_5d::<S>(x, y, z, w, u, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        amp *= gain;
        result += (simplex_5d::<S>(x, y, z, w, u, seed) * amp).abs() * S::Vf64::set1(frac.into());
    }

    result
}

#[inline(always)]
#[allow(clippy::too_many_arguments)]
pub fn turbulence_6d<S: Simd>(
    mut x: S::Vf64,
    mut y: S::Vf64,
    mut z: S::Vf64,
    mut w: S::Vf64,
    mut u: S::Vf64,
    mut t: S::Vf64,
    lac: S::Vf64,
    gain: S::Vf64,
    octaves: f32,
    seed: i64,
) -> S::Vf64 {
    let mut result = simplex_6d::<S>(x, y, z, w, u, t, seed).abs();
    let mut amp = S::Vf64::set1(1.0);

    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    for _ in 1..(n as usize) {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        result += (simplex_6d::<S>(x, y, z, w, u, t, seed) * amp).abs();
    }

    if frac > 0.0 {
        x *= lac;
        y *= lac;
        z *= lac;
        w *= lac;
        u *= lac;
        t *= lac;
        amp *= gain;
        result +=
            (simplex_6d::<S>(x, y, z, w, u, t, seed) * amp).abs() * S::Vf64::set1(frac.into());
    }

    result
}
