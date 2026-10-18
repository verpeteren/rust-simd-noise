//! Plain-Rust snapshot of the UI state. Built on the UI thread, then sent to the
//! worker thread, so everything here is `Send` and owns its data (no Slint types).

use simdnoise::{Cell2ReturnType, CellDistanceFunction, CellReturnType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Algo {
    Gradient,
    Fbm,
    Ridge,
    Turbulence,
    Cellular,
    Cellular2,
}

impl Algo {
    pub fn from_index(i: i32) -> Self {
        match i {
            1 => Algo::Fbm,
            2 => Algo::Ridge,
            3 => Algo::Turbulence,
            4 => Algo::Cellular,
            5 => Algo::Cellular2,
            _ => Algo::Gradient,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Float {
    F32,
    F64,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IntType {
    U8,
    U16,
    U32,
    I16,
    I32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OutMode {
    Raw,
    Scaled {
        min: f32,
        max: f32,
    },
    Quant {
        ty: IntType,
        src_min: f32,
        src_max: f32,
    },
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Colormap {
    Grayscale,
    Heatmap,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    Auto,
    Scalar,
    Sse2,
    Sse41,
    Avx2,
    Avx512,
    Neon,
}

impl Engine {
    pub fn all() -> [Engine; 7] {
        [
            Engine::Auto,
            Engine::Scalar,
            Engine::Sse2,
            Engine::Sse41,
            Engine::Avx2,
            Engine::Avx512,
            Engine::Neon,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Engine::Auto => "auto (runtime)",
            Engine::Scalar => "scalar",
            Engine::Sse2 => "sse2",
            Engine::Sse41 => "sse4.1",
            Engine::Avx2 => "avx2",
            Engine::Avx512 => "avx-512",
            Engine::Neon => "neon",
        }
    }

    /// Whether this engine can run on the current build/CPU. Non-auto engines are
    /// Raw-output-only (see `dispatch`). `auto` and `scalar` are always available.
    pub fn available(self) -> bool {
        match self {
            Engine::Auto | Engine::Scalar => true,
            Engine::Neon => cfg!(all(target_arch = "aarch64", target_feature = "neon")),
            Engine::Sse2 => x86_has("sse2"),
            Engine::Sse41 => x86_has("sse4.1"),
            Engine::Avx2 => x86_has("avx2"),
            Engine::Avx512 => x86_has("avx512f"),
        }
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn x86_has(feature: &str) -> bool {
    match feature {
        "sse2" => std::is_x86_feature_detected!("sse2"),
        "sse4.1" => std::is_x86_feature_detected!("sse4.1"),
        "avx2" => std::is_x86_feature_detected!("avx2"),
        "avx512f" => std::is_x86_feature_detected!("avx512f"),
        _ => false,
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn x86_has(_feature: &str) -> bool {
    false
}

#[derive(Clone, Copy)]
pub struct Fractal {
    pub octaves: f32,
    pub lacunarity: f32,
    pub gain: f32,
}

#[derive(Clone, Copy)]
pub struct Cellular {
    pub jitter: f32,
    pub dist_fn: CellDistanceFunction,
    pub return_type: CellReturnType,
    pub ret2: Cell2ReturnType,
    pub index0: usize,
    pub index1: usize,
}

#[derive(Clone)]
pub struct Params {
    pub algo: Algo,
    pub dim: usize,
    pub float: Float,
    pub seed: i32,
    pub offset: [f32; 6],
    pub extent: [usize; 6],
    pub freq: [f32; 6],
    pub fractal: Fractal,
    pub cellular: Cellular,
    pub out: OutMode,
    pub engine: Engine,
    /// Cross-section indices for the fixed higher axes: [w (axis4), u (axis5), v (axis6)].
    pub cross: [usize; 3],
    pub colormap: Colormap,
}

impl Params {
    /// Number of samples = product of the active extents.
    pub fn count(&self) -> usize {
        self.extent[..self.dim].iter().product()
    }

    /// Exact, hashable key over every value that affects the rendered image, for
    /// the worker's result cache. Floats are keyed by their bit pattern and enums
    /// by an explicit tag, so equal parameters always map to the same key.
    pub fn cache_key(&self) -> Vec<u64> {
        let fb = |x: f32| x.to_bits() as u64;
        let mut k: Vec<u64> = Vec::with_capacity(40);

        k.push(match self.algo {
            Algo::Gradient => 0,
            Algo::Fbm => 1,
            Algo::Ridge => 2,
            Algo::Turbulence => 3,
            Algo::Cellular => 4,
            Algo::Cellular2 => 5,
        });
        k.push(self.dim as u64);
        k.push(match self.float {
            Float::F32 => 0,
            Float::F64 => 1,
        });
        k.push(self.seed as u32 as u64);
        for v in self.offset {
            k.push(fb(v));
        }
        for v in self.extent {
            k.push(v as u64);
        }
        for v in self.freq {
            k.push(fb(v));
        }
        k.push(fb(self.fractal.octaves));
        k.push(fb(self.fractal.lacunarity));
        k.push(fb(self.fractal.gain));
        k.push(fb(self.cellular.jitter));
        k.push(match self.cellular.dist_fn {
            CellDistanceFunction::Euclidean => 0,
            CellDistanceFunction::Manhattan => 1,
            CellDistanceFunction::Natural => 2,
        });
        k.push(match self.cellular.return_type {
            CellReturnType::CellValue => 0,
            CellReturnType::Distance => 1,
        });
        k.push(match self.cellular.ret2 {
            Cell2ReturnType::Distance2 => 0,
            Cell2ReturnType::Distance2Add => 1,
            Cell2ReturnType::Distance2Sub => 2,
            Cell2ReturnType::Distance2Mul => 3,
            Cell2ReturnType::Distance2Div => 4,
        });
        k.push(self.cellular.index0 as u64);
        k.push(self.cellular.index1 as u64);
        match self.out {
            OutMode::Raw => k.push(0),
            OutMode::Scaled { min, max } => {
                k.push(1);
                k.push(fb(min));
                k.push(fb(max));
            }
            OutMode::Quant {
                ty,
                src_min,
                src_max,
            } => {
                k.push(2);
                k.push(match ty {
                    IntType::U8 => 0,
                    IntType::U16 => 1,
                    IntType::U32 => 2,
                    IntType::I16 => 3,
                    IntType::I32 => 4,
                });
                k.push(fb(src_min));
                k.push(fb(src_max));
            }
        }
        k.push(match self.engine {
            Engine::Auto => 0,
            Engine::Scalar => 1,
            Engine::Sse2 => 2,
            Engine::Sse41 => 3,
            Engine::Avx2 => 4,
            Engine::Avx512 => 5,
            Engine::Neon => 6,
        });
        for v in self.cross {
            k.push(v as u64);
        }
        k.push(match self.colormap {
            Colormap::Grayscale => 0,
            Colormap::Heatmap => 1,
        });
        k
    }

    /// A small valid preset, handy for tests and as a sane starting point.
    #[cfg(test)]
    pub fn preset() -> Params {
        use simdnoise::{Cell2ReturnType, CellDistanceFunction, CellReturnType};
        Params {
            algo: Algo::Gradient,
            dim: 2,
            float: Float::F32,
            seed: 1,
            offset: [0.0; 6],
            extent: [16, 16, 4, 2, 2, 2],
            freq: [0.05; 6],
            fractal: Fractal {
                octaves: 3.0,
                lacunarity: 0.5,
                gain: 2.0,
            },
            cellular: Cellular {
                jitter: 0.25,
                dist_fn: CellDistanceFunction::Euclidean,
                return_type: CellReturnType::Distance,
                ret2: Cell2ReturnType::Distance2,
                index0: 0,
                index1: 1,
            },
            out: OutMode::Raw,
            engine: Engine::Auto,
            cross: [0, 0, 0],
            colormap: Colormap::Grayscale,
        }
    }
}

pub fn dist_from_index(i: i32) -> CellDistanceFunction {
    match i {
        1 => CellDistanceFunction::Manhattan,
        2 => CellDistanceFunction::Natural,
        _ => CellDistanceFunction::Euclidean,
    }
}

pub fn ret_from_index(i: i32) -> CellReturnType {
    match i {
        0 => CellReturnType::CellValue,
        _ => CellReturnType::Distance,
    }
}

pub fn ret2_from_index(i: i32) -> Cell2ReturnType {
    match i {
        1 => Cell2ReturnType::Distance2Add,
        2 => Cell2ReturnType::Distance2Sub,
        3 => Cell2ReturnType::Distance2Mul,
        4 => Cell2ReturnType::Distance2Div,
        _ => Cell2ReturnType::Distance2,
    }
}

pub fn int_from_index(i: i32) -> Option<IntType> {
    match i {
        1 => Some(IntType::U8),
        2 => Some(IntType::U16),
        3 => Some(IntType::U32),
        4 => Some(IntType::I16),
        5 => Some(IntType::I32),
        _ => None,
    }
}
