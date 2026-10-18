#![no_main]
//! Fuzz the `generate` tool's input parameters: map arbitrary bytes to a full set
//! of CLI options and drive the real `resolve` → `generate` → `to_frames`
//! pipeline, asserting it never panics on any parameter combination.
//!
//! Bytes are decoded into a structured `FuzzInput` (bounded where unbounded values
//! would only waste the fuzzer's time: extents are capped so a single run can't
//! OOM, and `octaves` is kept finite so a fractal can't spin for billions of
//! iterations). Everything else — offsets, frequencies, seed, scale range, the
//! algorithm and its knobs, f32/f64, scaling and quantize — is taken raw,
//! including non-finite floats, so odd inputs are exercised.
//!
//! Generation runs on a 256 MiB-stack thread, matching the viewer's worker, so
//! the deep 6D path (issue #38) can't trip a stack-overflow false positive.

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use simdnoise_generate::{
    generate, resolve, to_frames, Args, Commands, DistanceArg, FloatArg, QuantArg, Return2Arg,
    ReturnArg, ScalingArg,
};

#[derive(Arbitrary, Debug)]
struct FuzzInput {
    dim: u8,
    ext: [u8; 6],
    offset: [f32; 6],
    seed: i32,
    freq: f32,
    freq_axis: [Option<f32>; 6],
    float_f64: bool,
    scaled: bool,
    scale_min: f32,
    scale_max: f32,
    quant: u8,
    algo: u8,
    lacunarity: f32,
    gain: f32,
    octaves: f32,
    jitter: f32,
    distance: u8,
    ret: u8,
    ret2: u8,
    index0: u8,
    index1: u8,
}

fn distance(n: u8) -> DistanceArg {
    match n % 3 {
        0 => DistanceArg::Euclidean,
        1 => DistanceArg::Manhattan,
        _ => DistanceArg::Natural,
    }
}

fn quantize(n: u8) -> QuantArg {
    match n % 6 {
        0 => QuantArg::None,
        1 => QuantArg::U8,
        2 => QuantArg::U16,
        3 => QuantArg::U32,
        4 => QuantArg::I16,
        _ => QuantArg::I32,
    }
}

fn command(fi: &FuzzInput) -> Commands {
    // Keep octaves finite and modest so a fractal can't loop pathologically long.
    let octaves = if fi.octaves.is_finite() {
        fi.octaves.clamp(0.0, 8.0)
    } else {
        3.0
    };
    match fi.algo % 6 {
        0 => Commands::Gradient,
        1 => Commands::Fbm {
            lacunarity: fi.lacunarity,
            gain: fi.gain,
            octaves,
        },
        2 => Commands::Ridge {
            lacunarity: fi.lacunarity,
            gain: fi.gain,
            octaves,
        },
        3 => Commands::Turbulence {
            lacunarity: fi.lacunarity,
            gain: fi.gain,
            octaves,
        },
        4 => Commands::Cellular {
            jitter: fi.jitter,
            distance: distance(fi.distance),
            return_type: if fi.ret % 2 == 0 {
                ReturnArg::CellValue
            } else {
                ReturnArg::Distance
            },
        },
        _ => Commands::Cellular2 {
            jitter: fi.jitter,
            distance: distance(fi.distance),
            return2: match fi.ret2 % 5 {
                0 => Return2Arg::Distance2,
                1 => Return2Arg::Distance2Add,
                2 => Return2Arg::Distance2Sub,
                3 => Return2Arg::Distance2Mul,
                _ => Return2Arg::Distance2Div,
            },
            // Raw (unclamped) indices — `generate` must clamp them itself.
            index0: fi.index0 as usize,
            index1: fi.index1 as usize,
        },
    }
}

fuzz_target!(|fi: FuzzInput| {
    // Cap dim to 1..=6 and every extent to 1..=16 so one run stays small.
    let ext = |i: usize| 1 + (fi.ext[i] % 16) as usize;
    let args = Args {
        dim: 1 + (fi.dim % 6),
        width: ext(0),
        height: ext(1),
        depth: ext(2),
        time: ext(3),
        ext5: ext(4),
        ext6: ext(5),
        offset_x: fi.offset[0],
        offset_y: fi.offset[1],
        offset_z: fi.offset[2],
        offset_w: fi.offset[3],
        offset_u: fi.offset[4],
        offset_v: fi.offset[5],
        seed: fi.seed,
        freq: fi.freq,
        freq_x: fi.freq_axis[0],
        freq_y: fi.freq_axis[1],
        freq_z: fi.freq_axis[2],
        freq_w: fi.freq_axis[3],
        freq_u: fi.freq_axis[4],
        freq_v: fi.freq_axis[5],
        float: if fi.float_f64 {
            FloatArg::F64
        } else {
            FloatArg::F32
        },
        scaling: if fi.scaled {
            ScalingArg::Scaled
        } else {
            ScalingArg::Raw
        },
        scale_min: fi.scale_min,
        scale_max: fi.scale_max,
        quantize: quantize(fi.quant),
        command: command(&fi),
    };

    let o = resolve(args);
    // Run on a big stack like the viewer's worker (6D needs it — issue #38).
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            let (values, lo, hi) = generate(&o);
            let _ = to_frames(&values, lo, hi, &o);
        })
        .unwrap()
        .join()
        .unwrap();
});
