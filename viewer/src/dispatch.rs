//! Params -> `*Settings` -> output buffer.
//!
//! Two levels keep the 6-algorithm × 6-dimension × float × output-mode matrix down
//! to ~18 hand-written arms: `by_dim!` resolves the builder arity, one generic
//! `emplace::<S: Settings>` covers float × output mode, and `run` picks the engine.

use simdnoise::{NoiseBuilder as NB, NoiseType, Settings, SimplexSettings};

use crate::params::*;

/// Persistent output buffers, reused across regenerations (`ui.md` §7.0). Only the
/// buffer the current (engine, float, output-mode) will write is resized.
#[derive(Default)]
pub struct Buffers {
    pub v_f32: Vec<f32>,
    pub v_f64: Vec<f64>,
    pub v_u8: Vec<u8>,
    pub v_u16: Vec<u16>,
    pub v_u32: Vec<u32>,
    pub v_i16: Vec<i16>,
    pub v_i32: Vec<i32>,
    /// (min, max) of the generated block, used to normalize Raw output for display.
    pub stats: (f32, f32),
}

impl Buffers {
    fn prepare(&mut self, n: usize, p: &Params) {
        // Forced (non-auto) engines produce a fresh `Vec`; nothing to pre-size.
        if p.engine != Engine::Auto {
            return;
        }
        match (p.float, &p.out) {
            (Float::F32, OutMode::Quant { ty, .. }) => match ty {
                IntType::U8 => self.v_u8.resize(n, 0),
                IntType::U16 => self.v_u16.resize(n, 0),
                IntType::U32 => self.v_u32.resize(n, 0),
                IntType::I16 => self.v_i16.resize(n, 0),
                IntType::I32 => self.v_i32.resize(n, 0),
            },
            (Float::F32, _) => self.v_f32.resize(n, 0.0),
            (Float::F64, _) => self.v_f64.resize(n, 0.0),
        }
    }
}

/// Builds a `*Settings` by dimension. The six builder paths differ only in arity
/// (offset/extent interleaved per axis), so one macro keyed on them avoids 36 arms.
macro_rules! by_dim {
    ($dim:expr, $o:expr, $e:expr,
     $f1:path, $f2:path, $f3:path, $f4:path, $f5:path, $f6:path) => {
        match $dim {
            1 => $f1($o[0], $e[0]),
            2 => $f2($o[0], $e[0], $o[1], $e[1]),
            3 => $f3($o[0], $e[0], $o[1], $e[1], $o[2], $e[2]),
            4 => $f4($o[0], $e[0], $o[1], $e[1], $o[2], $e[2], $o[3], $e[3]),
            5 => $f5(
                $o[0], $e[0], $o[1], $e[1], $o[2], $e[2], $o[3], $e[3], $o[4], $e[4],
            ),
            _ => $f6(
                $o[0], $e[0], $o[1], $e[1], $o[2], $e[2], $o[3], $e[3], $o[4], $e[4], $o[5], $e[5],
            ),
        }
    };
}

pub fn generate(p: &Params, buf: &mut Buffers) {
    let n = p.count();
    buf.prepare(n, p);
    let o = &p.offset;
    let e = &p.extent;

    match p.algo {
        Algo::Gradient => {
            let mut s = by_dim!(
                p.dim,
                o,
                e,
                NB::gradient_1d_offset,
                NB::gradient_2d_offset,
                NB::gradient_3d_offset,
                NB::gradient_4d_offset,
                NB::gradient_5d_offset,
                NB::gradient_6d_offset
            );
            s.with_seed(p.seed);
            apply_freq(&mut s, p);
            run(s, p, buf);
        }
        Algo::Fbm => {
            let mut s = by_dim!(
                p.dim,
                o,
                e,
                NB::fbm_1d_offset,
                NB::fbm_2d_offset,
                NB::fbm_3d_offset,
                NB::fbm_4d_offset,
                NB::fbm_5d_offset,
                NB::fbm_6d_offset
            );
            s.with_seed(p.seed)
                .with_octaves(p.fractal.octaves)
                .with_lacunarity(p.fractal.lacunarity)
                .with_gain(p.fractal.gain);
            apply_freq(&mut s, p);
            run(s, p, buf);
        }
        Algo::Ridge => {
            let mut s = by_dim!(
                p.dim,
                o,
                e,
                NB::ridge_1d_offset,
                NB::ridge_2d_offset,
                NB::ridge_3d_offset,
                NB::ridge_4d_offset,
                NB::ridge_5d_offset,
                NB::ridge_6d_offset
            );
            s.with_seed(p.seed)
                .with_octaves(p.fractal.octaves)
                .with_lacunarity(p.fractal.lacunarity)
                .with_gain(p.fractal.gain);
            apply_freq(&mut s, p);
            run(s, p, buf);
        }
        Algo::Turbulence => {
            let mut s = by_dim!(
                p.dim,
                o,
                e,
                NB::turbulence_1d_offset,
                NB::turbulence_2d_offset,
                NB::turbulence_3d_offset,
                NB::turbulence_4d_offset,
                NB::turbulence_5d_offset,
                NB::turbulence_6d_offset
            );
            s.with_seed(p.seed)
                .with_octaves(p.fractal.octaves)
                .with_lacunarity(p.fractal.lacunarity)
                .with_gain(p.fractal.gain);
            apply_freq(&mut s, p);
            run(s, p, buf);
        }
        Algo::Cellular => {
            let mut s = by_dim!(
                p.dim,
                o,
                e,
                NB::cellular_1d_offset,
                NB::cellular_2d_offset,
                NB::cellular_3d_offset,
                NB::cellular_4d_offset,
                NB::cellular_5d_offset,
                NB::cellular_6d_offset
            );
            s.with_seed(p.seed)
                .with_jitter(p.cellular.jitter)
                .with_distance_function(p.cellular.dist_fn)
                .with_return_type(p.cellular.return_type);
            apply_freq(&mut s, p);
            run(s, p, buf);
        }
        Algo::Cellular2 => {
            let mut s = by_dim!(
                p.dim,
                o,
                e,
                NB::cellular2_1d_offset,
                NB::cellular2_2d_offset,
                NB::cellular2_3d_offset,
                NB::cellular2_4d_offset,
                NB::cellular2_5d_offset,
                NB::cellular2_6d_offset
            );
            s.with_seed(p.seed)
                .with_jitter(p.cellular.jitter)
                .with_distance_function(p.cellular.dist_fn)
                .with_return_type(p.cellular.ret2)
                .with_index0(p.cellular.index0)
                .with_index1(p.cellular.index1);
            apply_freq(&mut s, p);
            run(s, p, buf);
        }
    }
}

/// All six `with_freq_*` setters exist; dispatch by dim so we never assume a
/// higher-arity setter is valid at a lower dimension.
fn apply_freq<S: Settings>(s: &mut S, p: &Params) {
    let f = &p.freq;
    match p.dim {
        1 => {
            s.with_freq(f[0]);
        }
        2 => {
            s.with_freq_2d(f[0], f[1]);
        }
        3 => {
            s.with_freq_3d(f[0], f[1], f[2]);
        }
        4 => {
            s.with_freq_4d(f[0], f[1], f[2], f[3]);
        }
        5 => {
            s.with_freq_5d(f[0], f[1], f[2], f[3], f[4]);
        }
        _ => {
            s.with_freq_6d(f[0], f[1], f[2], f[3], f[4], f[5]);
        }
    }
}

fn run<S: Settings>(s: S, p: &Params, buf: &mut Buffers) {
    match p.engine {
        Engine::Auto => emplace(s, p, buf),
        _ => forced(s.wrap(), p, buf),
    }
}

/// Auto engine: the full emplace/scaled/quantize API (buffer reuse).
fn emplace<S: Settings>(s: S, p: &Params, buf: &mut Buffers) {
    match (p.float, &p.out) {
        (Float::F32, OutMode::Raw) => {
            buf.stats = s.generate_into(&mut buf.v_f32);
        }
        (Float::F32, OutMode::Scaled { min, max }) => {
            s.generate_scaled_into(*min, *max, &mut buf.v_f32);
            buf.stats = (*min, *max);
        }
        (Float::F64, OutMode::Raw) => {
            let (lo, hi) = s.generate_into_64(&mut buf.v_f64);
            buf.stats = (lo as f32, hi as f32);
        }
        (Float::F64, OutMode::Scaled { min, max }) => {
            s.generate_scaled_into_64(*min as f64, *max as f64, &mut buf.v_f64);
            buf.stats = (*min, *max);
        }
        (
            Float::F32,
            OutMode::Quant {
                ty,
                src_min,
                src_max,
            },
        ) => {
            match ty {
                IntType::U8 => s.generate_range_into::<u8>(*src_min, *src_max, &mut buf.v_u8),
                IntType::U16 => s.generate_range_into::<u16>(*src_min, *src_max, &mut buf.v_u16),
                IntType::U32 => s.generate_range_into::<u32>(*src_min, *src_max, &mut buf.v_u32),
                IntType::I16 => s.generate_range_into::<i16>(*src_min, *src_max, &mut buf.v_i16),
                IntType::I32 => s.generate_range_into::<i32>(*src_min, *src_max, &mut buf.v_i32),
            }
            buf.stats = (*src_min, *src_max);
        }
        // quantize has no f64 twin; the UI prevents this combination, but stay safe.
        (Float::F64, OutMode::Quant { .. }) => {
            let (lo, hi) = s.generate_into_64(&mut buf.v_f64);
            buf.stats = (lo as f32, hi as f32);
        }
    }
}

/// Forced engine: block `Vec` output only (no scaled/quantize), copied into the
/// buffer. Raw-only — the UI forces Output to Raw when a non-auto engine is picked.
fn forced(nt: NoiseType, p: &Params, buf: &mut Buffers) {
    match p.float {
        Float::F32 => {
            let (v, lo, hi) = f32_block(p.engine, p.dim, &nt);
            buf.v_f32 = v;
            buf.stats = (lo, hi);
        }
        Float::F64 => {
            let (v, lo, hi) = f64_block(p.engine, p.dim, &nt);
            buf.v_f64 = v;
            buf.stats = (lo as f32, hi as f32);
        }
    }
}

fn f32_block(engine: Engine, dim: usize, nt: &NoiseType) -> (Vec<f32>, f32, f32) {
    match engine {
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        Engine::Neon => neon_f32(dim, nt),
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        Engine::Sse2 | Engine::Sse41 | Engine::Avx2 | Engine::Avx512 => x86_f32(engine, dim, nt),
        _ => scalar_f32(dim, nt),
    }
}

fn f64_block(engine: Engine, dim: usize, nt: &NoiseType) -> (Vec<f64>, f64, f64) {
    match engine {
        #[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
        Engine::Neon => neon_f64(dim, nt),
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        Engine::Sse2 | Engine::Sse41 | Engine::Avx2 | Engine::Avx512 => x86_f64(engine, dim, nt),
        _ => scalar_f64(dim, nt),
    }
}

fn scalar_f32(dim: usize, nt: &NoiseType) -> (Vec<f32>, f32, f32) {
    use simdeez::scalar::Scalar;
    use simdnoise::intrinsics::scalar as s;
    unsafe {
        match dim {
            1 => s::get_1d_noise::<Scalar>(nt),
            2 => s::get_2d_noise::<Scalar>(nt),
            3 => s::get_3d_noise::<Scalar>(nt),
            4 => s::get_4d_noise::<Scalar>(nt),
            5 => s::get_5d_noise::<Scalar>(nt),
            _ => s::get_6d_noise::<Scalar>(nt),
        }
    }
}

fn scalar_f64(dim: usize, nt: &NoiseType) -> (Vec<f64>, f64, f64) {
    use simdeez::scalar::Scalar;
    use simdnoise::intrinsics::scalar as s;
    unsafe {
        match dim {
            1 => s::get_1d_noise_64::<Scalar>(nt),
            2 => s::get_2d_noise_64::<Scalar>(nt),
            3 => s::get_3d_noise_64::<Scalar>(nt),
            4 => s::get_4d_noise_64::<Scalar>(nt),
            5 => s::get_5d_noise_64::<Scalar>(nt),
            _ => s::get_6d_noise_64::<Scalar>(nt),
        }
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn neon_f32(dim: usize, nt: &NoiseType) -> (Vec<f32>, f32, f32) {
    use simdnoise::intrinsics::neon as n;
    match dim {
        1 => n::get_1d_noise(nt),
        2 => n::get_2d_noise(nt),
        3 => n::get_3d_noise(nt),
        4 => n::get_4d_noise(nt),
        5 => n::get_5d_noise(nt),
        _ => n::get_6d_noise(nt),
    }
}

#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
fn neon_f64(dim: usize, nt: &NoiseType) -> (Vec<f64>, f64, f64) {
    use simdnoise::intrinsics::neon as n;
    match dim {
        1 => n::get_1d_noise_64(nt),
        2 => n::get_2d_noise_64(nt),
        3 => n::get_3d_noise_64(nt),
        4 => n::get_4d_noise_64(nt),
        5 => n::get_5d_noise_64(nt),
        _ => n::get_6d_noise_64(nt),
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn x86_f32(engine: Engine, dim: usize, nt: &NoiseType) -> (Vec<f32>, f32, f32) {
    use simdnoise::intrinsics::{avx2, avx512, sse2, sse41};
    unsafe {
        match engine {
            Engine::Sse2 => match dim {
                1 => sse2::get_1d_noise::<simdeez::engines::sse2::Sse2>(nt),
                2 => sse2::get_2d_noise::<simdeez::engines::sse2::Sse2>(nt),
                3 => sse2::get_3d_noise::<simdeez::engines::sse2::Sse2>(nt),
                4 => sse2::get_4d_noise::<simdeez::engines::sse2::Sse2>(nt),
                5 => sse2::get_5d_noise::<simdeez::engines::sse2::Sse2>(nt),
                _ => sse2::get_6d_noise::<simdeez::engines::sse2::Sse2>(nt),
            },
            Engine::Sse41 => match dim {
                1 => sse41::get_1d_noise::<simdeez::engines::sse41::Sse41>(nt),
                2 => sse41::get_2d_noise::<simdeez::engines::sse41::Sse41>(nt),
                3 => sse41::get_3d_noise::<simdeez::engines::sse41::Sse41>(nt),
                4 => sse41::get_4d_noise::<simdeez::engines::sse41::Sse41>(nt),
                5 => sse41::get_5d_noise::<simdeez::engines::sse41::Sse41>(nt),
                _ => sse41::get_6d_noise::<simdeez::engines::sse41::Sse41>(nt),
            },
            Engine::Avx2 => match dim {
                1 => avx2::get_1d_noise::<simdeez::engines::avx2::Avx2>(nt),
                2 => avx2::get_2d_noise::<simdeez::engines::avx2::Avx2>(nt),
                3 => avx2::get_3d_noise::<simdeez::engines::avx2::Avx2>(nt),
                4 => avx2::get_4d_noise::<simdeez::engines::avx2::Avx2>(nt),
                5 => avx2::get_5d_noise::<simdeez::engines::avx2::Avx2>(nt),
                _ => avx2::get_6d_noise::<simdeez::engines::avx2::Avx2>(nt),
            },
            _ => match dim {
                1 => avx512::get_1d_noise::<simdeez::engines::avx512::Avx512>(nt),
                2 => avx512::get_2d_noise::<simdeez::engines::avx512::Avx512>(nt),
                3 => avx512::get_3d_noise::<simdeez::engines::avx512::Avx512>(nt),
                4 => avx512::get_4d_noise::<simdeez::engines::avx512::Avx512>(nt),
                5 => avx512::get_5d_noise::<simdeez::engines::avx512::Avx512>(nt),
                _ => avx512::get_6d_noise::<simdeez::engines::avx512::Avx512>(nt),
            },
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn x86_f64(engine: Engine, dim: usize, nt: &NoiseType) -> (Vec<f64>, f64, f64) {
    use simdnoise::intrinsics::{avx2, avx512, sse2, sse41};
    unsafe {
        match engine {
            Engine::Sse2 => match dim {
                1 => sse2::get_1d_noise_64::<simdeez::engines::sse2::Sse2>(nt),
                2 => sse2::get_2d_noise_64::<simdeez::engines::sse2::Sse2>(nt),
                3 => sse2::get_3d_noise_64::<simdeez::engines::sse2::Sse2>(nt),
                4 => sse2::get_4d_noise_64::<simdeez::engines::sse2::Sse2>(nt),
                5 => sse2::get_5d_noise_64::<simdeez::engines::sse2::Sse2>(nt),
                _ => sse2::get_6d_noise_64::<simdeez::engines::sse2::Sse2>(nt),
            },
            Engine::Sse41 => match dim {
                1 => sse41::get_1d_noise_64::<simdeez::engines::sse41::Sse41>(nt),
                2 => sse41::get_2d_noise_64::<simdeez::engines::sse41::Sse41>(nt),
                3 => sse41::get_3d_noise_64::<simdeez::engines::sse41::Sse41>(nt),
                4 => sse41::get_4d_noise_64::<simdeez::engines::sse41::Sse41>(nt),
                5 => sse41::get_5d_noise_64::<simdeez::engines::sse41::Sse41>(nt),
                _ => sse41::get_6d_noise_64::<simdeez::engines::sse41::Sse41>(nt),
            },
            Engine::Avx2 => match dim {
                1 => avx2::get_1d_noise_64::<simdeez::engines::avx2::Avx2>(nt),
                2 => avx2::get_2d_noise_64::<simdeez::engines::avx2::Avx2>(nt),
                3 => avx2::get_3d_noise_64::<simdeez::engines::avx2::Avx2>(nt),
                4 => avx2::get_4d_noise_64::<simdeez::engines::avx2::Avx2>(nt),
                5 => avx2::get_5d_noise_64::<simdeez::engines::avx2::Avx2>(nt),
                _ => avx2::get_6d_noise_64::<simdeez::engines::avx2::Avx2>(nt),
            },
            _ => match dim {
                1 => avx512::get_1d_noise_64::<simdeez::engines::avx512::Avx512>(nt),
                2 => avx512::get_2d_noise_64::<simdeez::engines::avx512::Avx512>(nt),
                3 => avx512::get_3d_noise_64::<simdeez::engines::avx512::Avx512>(nt),
                4 => avx512::get_4d_noise_64::<simdeez::engines::avx512::Avx512>(nt),
                5 => avx512::get_5d_noise_64::<simdeez::engines::avx512::Avx512>(nt),
                _ => avx512::get_6d_noise_64::<simdeez::engines::avx512::Avx512>(nt),
            },
        }
    }
}
