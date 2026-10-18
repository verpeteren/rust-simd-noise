//! Core of `simdnoise-generate`: CLI option types and the arg → noise-block
//! pipeline (`resolve` → `generate` → `to_frames`), kept free of any windowing so
//! it can be unit-tested and fuzzed. The `generate` binary adds only the minifb
//! display loop on top.
//!
//! The option surface mirrors the Slint `viewer` crate so the viewer's "Submit"
//! button can emit a faithful, runnable command: all six algorithms, dimensions
//! 1-6, per-axis frequency/offset, f32/f64, and raw/scaled/quantized output.

use clap::{Parser, Subcommand, ValueEnum};
use simdnoise::{
    Cell2ReturnType, CellDistanceFunction, CellReturnType, NoiseBuilder as NB, Settings,
    SimplexSettings,
};

// ---------- CLI value enums (explicit names so the viewer can emit them) ----------

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum FloatArg {
    #[value(name = "f32")]
    F32,
    #[value(name = "f64")]
    F64,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ScalingArg {
    #[value(name = "raw")]
    Raw,
    #[value(name = "scaled")]
    Scaled,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum QuantArg {
    #[value(name = "none")]
    None,
    #[value(name = "u8")]
    U8,
    #[value(name = "u16")]
    U16,
    #[value(name = "u32")]
    U32,
    #[value(name = "i16")]
    I16,
    #[value(name = "i32")]
    I32,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum DistanceArg {
    #[value(name = "euclidean")]
    Euclidean,
    #[value(name = "manhattan")]
    Manhattan,
    #[value(name = "natural")]
    Natural,
}

impl From<DistanceArg> for CellDistanceFunction {
    fn from(d: DistanceArg) -> Self {
        match d {
            DistanceArg::Euclidean => CellDistanceFunction::Euclidean,
            DistanceArg::Manhattan => CellDistanceFunction::Manhattan,
            DistanceArg::Natural => CellDistanceFunction::Natural,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum ReturnArg {
    #[value(name = "cellvalue")]
    CellValue,
    #[value(name = "distance")]
    Distance,
}

impl From<ReturnArg> for CellReturnType {
    fn from(r: ReturnArg) -> Self {
        match r {
            ReturnArg::CellValue => CellReturnType::CellValue,
            ReturnArg::Distance => CellReturnType::Distance,
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum Return2Arg {
    #[value(name = "distance2")]
    Distance2,
    #[value(name = "distance2add")]
    Distance2Add,
    #[value(name = "distance2sub")]
    Distance2Sub,
    #[value(name = "distance2mul")]
    Distance2Mul,
    #[value(name = "distance2div")]
    Distance2Div,
}

impl From<Return2Arg> for Cell2ReturnType {
    fn from(r: Return2Arg) -> Self {
        match r {
            Return2Arg::Distance2 => Cell2ReturnType::Distance2,
            Return2Arg::Distance2Add => Cell2ReturnType::Distance2Add,
            Return2Arg::Distance2Sub => Cell2ReturnType::Distance2Sub,
            Return2Arg::Distance2Mul => Cell2ReturnType::Distance2Mul,
            Return2Arg::Distance2Div => Cell2ReturnType::Distance2Div,
        }
    }
}

// ---------- CLI ----------

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, allow_negative_numbers = true)]
pub struct Args {
    #[arg(long, default_value_t = 2, help = "Number of dimensions (1-6)")]
    pub dim: u8,

    #[arg(long, default_value_t = 512)]
    pub width: usize,
    #[arg(long, default_value_t = 512)]
    pub height: usize,
    #[arg(long, default_value_t = 8)]
    pub depth: usize,
    #[arg(long, default_value_t = 4)]
    pub time: usize,
    #[arg(long, default_value_t = 2)]
    pub ext5: usize,
    #[arg(long, default_value_t = 2)]
    pub ext6: usize,

    #[arg(long = "offset-x", default_value_t = 0.0)]
    pub offset_x: f32,
    #[arg(long = "offset-y", default_value_t = 0.0)]
    pub offset_y: f32,
    #[arg(long = "offset-z", default_value_t = 0.0)]
    pub offset_z: f32,
    #[arg(long = "offset-w", default_value_t = 0.0)]
    pub offset_w: f32,
    #[arg(long = "offset-u", default_value_t = 0.0)]
    pub offset_u: f32,
    #[arg(long = "offset-v", default_value_t = 0.0)]
    pub offset_v: f32,

    #[arg(long, default_value_t = 1)]
    pub seed: i32,

    #[arg(
        long,
        default_value_t = 0.02,
        help = "Uniform frequency (per-axis overrides below)"
    )]
    pub freq: f32,
    #[arg(long = "freq-x")]
    pub freq_x: Option<f32>,
    #[arg(long = "freq-y")]
    pub freq_y: Option<f32>,
    #[arg(long = "freq-z")]
    pub freq_z: Option<f32>,
    #[arg(long = "freq-w")]
    pub freq_w: Option<f32>,
    #[arg(long = "freq-u")]
    pub freq_u: Option<f32>,
    #[arg(long = "freq-v")]
    pub freq_v: Option<f32>,

    #[arg(long, value_enum, default_value_t = FloatArg::F32)]
    pub float: FloatArg,
    #[arg(long, value_enum, default_value_t = ScalingArg::Raw)]
    pub scaling: ScalingArg,
    #[arg(long = "scale-min", default_value_t = -1.0)]
    pub scale_min: f32,
    #[arg(long = "scale-max", default_value_t = 1.0)]
    pub scale_max: f32,
    #[arg(long, value_enum, default_value_t = QuantArg::None)]
    pub quantize: QuantArg,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone, Copy)]
pub enum Commands {
    Gradient,
    Fbm {
        #[arg(long, default_value_t = 0.5)]
        lacunarity: f32,
        #[arg(long, default_value_t = 2.0)]
        gain: f32,
        #[arg(long, default_value_t = 3.0)]
        octaves: f32,
    },
    Ridge {
        #[arg(long, default_value_t = 0.5)]
        lacunarity: f32,
        #[arg(long, default_value_t = 2.0)]
        gain: f32,
        #[arg(long, default_value_t = 3.0)]
        octaves: f32,
    },
    Turbulence {
        #[arg(long, default_value_t = 0.5)]
        lacunarity: f32,
        #[arg(long, default_value_t = 2.0)]
        gain: f32,
        #[arg(long, default_value_t = 3.0)]
        octaves: f32,
    },
    Cellular {
        #[arg(long, default_value_t = 0.25)]
        jitter: f32,
        #[arg(long, value_enum, default_value_t = DistanceArg::Euclidean)]
        distance: DistanceArg,
        #[arg(long = "return", value_enum, default_value_t = ReturnArg::Distance)]
        return_type: ReturnArg,
    },
    Cellular2 {
        #[arg(long, default_value_t = 0.25)]
        jitter: f32,
        #[arg(long, value_enum, default_value_t = DistanceArg::Euclidean)]
        distance: DistanceArg,
        #[arg(long = "return2", value_enum, default_value_t = Return2Arg::Distance2)]
        return2: Return2Arg,
        #[arg(long, default_value_t = 0)]
        index0: usize,
        #[arg(long, default_value_t = 1)]
        index1: usize,
    },
}

// ---------- resolved options ----------

#[derive(Copy, Clone)]
pub enum FloatKind {
    F32,
    F64,
}

pub struct Opts {
    pub dim: usize,
    pub extent: [usize; 6],
    pub offset: [f32; 6],
    pub freq: [f32; 6],
    pub seed: i32,
    pub float: FloatKind,
    pub scaling: ScalingArg,
    pub scale_min: f32,
    pub scale_max: f32,
    pub quant: QuantArg,
    pub command: Commands,
}

impl Opts {
    pub fn count(&self) -> usize {
        self.extent[..self.dim].iter().product()
    }
}

/// Normalize raw CLI args into validated options: clamp `dim` to 1-6 and every
/// extent to at least 1, and spread the uniform `--freq` into per-axis defaults.
pub fn resolve(args: Args) -> Opts {
    let dim = (args.dim.clamp(1, 6)) as usize;
    Opts {
        dim,
        extent: [
            args.width.max(1),
            args.height.max(1),
            args.depth.max(1),
            args.time.max(1),
            args.ext5.max(1),
            args.ext6.max(1),
        ],
        offset: [
            args.offset_x,
            args.offset_y,
            args.offset_z,
            args.offset_w,
            args.offset_u,
            args.offset_v,
        ],
        freq: [
            args.freq_x.unwrap_or(args.freq),
            args.freq_y.unwrap_or(args.freq),
            args.freq_z.unwrap_or(args.freq),
            args.freq_w.unwrap_or(args.freq),
            args.freq_u.unwrap_or(args.freq),
            args.freq_v.unwrap_or(args.freq),
        ],
        seed: args.seed,
        float: match args.float {
            FloatArg::F32 => FloatKind::F32,
            FloatArg::F64 => FloatKind::F64,
        },
        scaling: args.scaling,
        scale_min: args.scale_min,
        scale_max: args.scale_max,
        quant: args.quantize,
        command: args.command,
    }
}

/// Clamp cellular2's `index0`/`index1` to the library's `validate` rules so a bad
/// pair can never panic generation (mirrors the viewer's `build_params`).
fn clamp_indices(dim: usize, mut index0: usize, mut index1: usize) -> (usize, usize) {
    index0 = index0.min(2);
    let index1_cap = if dim == 1 { 2 } else { 3 };
    index1 = index1.min(index1_cap);
    if index1 <= index0 {
        index1 = (index0 + 1).min(index1_cap);
        index0 = index1 - 1;
    }
    (index0, index1)
}

// ---------- generation (mirrors the viewer's dispatch) ----------

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

fn apply_freq<S: Settings>(s: &mut S, o: &Opts) {
    let f = &o.freq;
    match o.dim {
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

/// Generate the block and return (display values as f32, lo, hi) for normalization.
pub fn generate(o: &Opts) -> (Vec<f32>, f32, f32) {
    let (oo, e) = (&o.offset, &o.extent);
    match o.command {
        Commands::Gradient => {
            let mut s = by_dim!(
                o.dim,
                oo,
                e,
                NB::gradient_1d_offset,
                NB::gradient_2d_offset,
                NB::gradient_3d_offset,
                NB::gradient_4d_offset,
                NB::gradient_5d_offset,
                NB::gradient_6d_offset
            );
            s.with_seed(o.seed);
            apply_freq(&mut s, o);
            emit(s, o)
        }
        Commands::Fbm {
            lacunarity,
            gain,
            octaves,
        } => {
            let mut s = by_dim!(
                o.dim,
                oo,
                e,
                NB::fbm_1d_offset,
                NB::fbm_2d_offset,
                NB::fbm_3d_offset,
                NB::fbm_4d_offset,
                NB::fbm_5d_offset,
                NB::fbm_6d_offset
            );
            s.with_seed(o.seed)
                .with_lacunarity(lacunarity)
                .with_gain(gain)
                .with_octaves(octaves);
            apply_freq(&mut s, o);
            emit(s, o)
        }
        Commands::Ridge {
            lacunarity,
            gain,
            octaves,
        } => {
            let mut s = by_dim!(
                o.dim,
                oo,
                e,
                NB::ridge_1d_offset,
                NB::ridge_2d_offset,
                NB::ridge_3d_offset,
                NB::ridge_4d_offset,
                NB::ridge_5d_offset,
                NB::ridge_6d_offset
            );
            s.with_seed(o.seed)
                .with_lacunarity(lacunarity)
                .with_gain(gain)
                .with_octaves(octaves);
            apply_freq(&mut s, o);
            emit(s, o)
        }
        Commands::Turbulence {
            lacunarity,
            gain,
            octaves,
        } => {
            let mut s = by_dim!(
                o.dim,
                oo,
                e,
                NB::turbulence_1d_offset,
                NB::turbulence_2d_offset,
                NB::turbulence_3d_offset,
                NB::turbulence_4d_offset,
                NB::turbulence_5d_offset,
                NB::turbulence_6d_offset
            );
            s.with_seed(o.seed)
                .with_lacunarity(lacunarity)
                .with_gain(gain)
                .with_octaves(octaves);
            apply_freq(&mut s, o);
            emit(s, o)
        }
        Commands::Cellular {
            jitter,
            distance,
            return_type,
        } => {
            let mut s = by_dim!(
                o.dim,
                oo,
                e,
                NB::cellular_1d_offset,
                NB::cellular_2d_offset,
                NB::cellular_3d_offset,
                NB::cellular_4d_offset,
                NB::cellular_5d_offset,
                NB::cellular_6d_offset
            );
            s.with_seed(o.seed)
                .with_jitter(jitter)
                .with_distance_function(distance.into())
                .with_return_type(return_type.into());
            apply_freq(&mut s, o);
            emit(s, o)
        }
        Commands::Cellular2 {
            jitter,
            distance,
            return2,
            index0,
            index1,
        } => {
            let (index0, index1) = clamp_indices(o.dim, index0, index1);
            let mut s = by_dim!(
                o.dim,
                oo,
                e,
                NB::cellular2_1d_offset,
                NB::cellular2_2d_offset,
                NB::cellular2_3d_offset,
                NB::cellular2_4d_offset,
                NB::cellular2_5d_offset,
                NB::cellular2_6d_offset
            );
            s.with_seed(o.seed)
                .with_jitter(jitter)
                .with_distance_function(distance.into())
                .with_return_type(return2.into())
                .with_index0(index0)
                .with_index1(index1);
            apply_freq(&mut s, o);
            emit(s, o)
        }
    }
}

fn emit<S: Settings>(s: S, o: &Opts) -> (Vec<f32>, f32, f32) {
    match o.float {
        FloatKind::F32 => match o.quant {
            QuantArg::None => match o.scaling {
                ScalingArg::Raw => s.generate(),
                ScalingArg::Scaled => {
                    let v = s.generate_scaled(o.scale_min, o.scale_max);
                    (v, o.scale_min, o.scale_max)
                }
            },
            q => quant(s, o, q),
        },
        FloatKind::F64 => match o.scaling {
            ScalingArg::Raw => {
                let (v, mn, mx) = s.generate_64();
                (
                    v.into_iter().map(|x| x as f32).collect(),
                    mn as f32,
                    mx as f32,
                )
            }
            ScalingArg::Scaled => {
                let v = s.generate_scaled_64(o.scale_min as f64, o.scale_max as f64);
                (
                    v.into_iter().map(|x| x as f32).collect(),
                    o.scale_min,
                    o.scale_max,
                )
            }
        },
    }
}

fn quant<S: Settings>(s: S, o: &Opts, q: QuantArg) -> (Vec<f32>, f32, f32) {
    let n = o.count();
    macro_rules! go {
        ($t:ty, $lo:expr, $hi:expr) => {{
            let mut b = vec![<$t>::default(); n];
            s.generate_range_into::<$t>(o.scale_min, o.scale_max, &mut b);
            (b.into_iter().map(|x| x as f32).collect(), $lo, $hi)
        }};
    }
    match q {
        QuantArg::U8 => go!(u8, 0.0, u8::MAX as f32),
        QuantArg::U16 => go!(u16, 0.0, u16::MAX as f32),
        QuantArg::U32 => go!(u32, 0.0, u32::MAX as f32),
        QuantArg::I16 => go!(i16, i16::MIN as f32, i16::MAX as f32),
        QuantArg::I32 => go!(i32, i32::MIN as f32, i32::MAX as f32),
        QuantArg::None => s.generate(),
    }
}

// ---------- display helpers (pure; the window loop lives in the binary) ----------

pub fn gray(t: f32) -> u32 {
    let c = (t.clamp(0.0, 1.0) * 255.0) as u32;
    (c << 16) | (c << 8) | c
}

/// Split the block into W x H grayscale frames (cycled in the window). 1D is drawn
/// as a single row replicated to fill the height.
pub fn to_frames(values: &[f32], lo: f32, hi: f32, o: &Opts) -> Vec<Vec<u32>> {
    let w = o.extent[0];
    let h = o.extent[1];
    let norm = |v: f32| if hi > lo { (v - lo) / (hi - lo) } else { 0.5 };

    if o.dim == 1 {
        let row: Vec<u32> = (0..w)
            .map(|x| gray(norm(values[x.min(values.len().saturating_sub(1))])))
            .collect();
        let mut frame = Vec::with_capacity(w * h);
        for _ in 0..h {
            frame.extend_from_slice(&row);
        }
        return vec![frame];
    }

    let plane = (w * h).max(1);
    let frames = (values.len() / plane).max(1);
    (0..frames)
        .map(|f| {
            values[f * plane..((f + 1) * plane).min(values.len())]
                .iter()
                .map(|v| gray(norm(*v)))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build default options for `dim`/`command`, overriding only the extents to
    /// keep tests fast. Runs generation on a big stack (6D needs it, issue #38).
    fn run(dim: u8, command: Commands) -> (Vec<f32>, f32, f32) {
        let args = Args {
            dim,
            width: 8,
            height: 8,
            depth: 3,
            time: 2,
            ext5: 2,
            ext6: 2,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            offset_w: 0.0,
            offset_u: 0.0,
            offset_v: 0.0,
            seed: 1,
            freq: 0.05,
            freq_x: None,
            freq_y: None,
            freq_z: None,
            freq_w: None,
            freq_u: None,
            freq_v: None,
            float: FloatArg::F32,
            scaling: ScalingArg::Raw,
            scale_min: -1.0,
            scale_max: 1.0,
            quantize: QuantArg::None,
            command,
        };
        let o = resolve(args);
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(move || {
                let (v, lo, hi) = generate(&o);
                // to_frames must agree on the pixel count for the W x H plane.
                let frames = to_frames(&v, lo, hi, &o);
                assert!(frames.iter().all(|f| f.len() == o.extent[0] * o.extent[1]));
                (v, lo, hi)
            })
            .unwrap()
            .join()
            .unwrap()
    }

    #[test]
    fn resolve_clamps_dim_and_extents() {
        let mut args = Args::parse_from(["generate", "--dim", "9", "--width", "0", "gradient"]);
        args.height = 0;
        let o = resolve(args);
        assert_eq!(o.dim, 6);
        assert_eq!(o.extent[0], 1);
        assert_eq!(o.extent[1], 1);
    }

    #[test]
    fn every_algo_every_dim_generates() {
        let cmds = [
            Commands::Gradient,
            Commands::Fbm {
                lacunarity: 0.5,
                gain: 2.0,
                octaves: 3.0,
            },
            Commands::Ridge {
                lacunarity: 0.5,
                gain: 2.0,
                octaves: 3.0,
            },
            Commands::Turbulence {
                lacunarity: 0.5,
                gain: 2.0,
                octaves: 3.0,
            },
            Commands::Cellular {
                jitter: 0.25,
                distance: DistanceArg::Euclidean,
                return_type: ReturnArg::Distance,
            },
            Commands::Cellular2 {
                jitter: 0.25,
                distance: DistanceArg::Euclidean,
                return2: Return2Arg::Distance2,
                index0: 0,
                index1: 1,
            },
        ];
        for cmd in cmds {
            for dim in 1..=6u8 {
                let (v, _, _) = run(dim, cmd);
                assert!(!v.is_empty(), "empty block at dim {}", dim);
            }
        }
    }

    #[test]
    fn cellular2_bad_indices_do_not_panic() {
        // index1 <= index0, and both out of range: `generate` must clamp, not panic.
        for dim in 1..=6u8 {
            run(
                dim,
                Commands::Cellular2 {
                    jitter: 0.25,
                    distance: DistanceArg::Natural,
                    return2: Return2Arg::Distance2Div,
                    index0: 9,
                    index1: 2,
                },
            );
        }
    }
}
