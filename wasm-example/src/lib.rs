//! Headless WebAssembly example for `simdnoise`.
//!
//! This crate exposes a few noise generators to JavaScript via `wasm-bindgen`.
//! It is "headless": it has no HTML/canvas harness and renders nothing itself —
//! each function just returns the generated noise as bytes so a host (a browser,
//! Node, Deno, wasmtime, ...) can do whatever it likes with it.
//!
//! The point of the example is to show that `simdnoise`'s ordinary API works
//! unchanged on `wasm32`: on that target simdeez dispatches to its wasm32
//! SIMD128 engine automatically, so there is nothing wasm-specific in the calls
//! below. See the crate README for how to build and run it.

use simdnoise::{NoiseBuilder, Settings};
use wasm_bindgen::prelude::*;

/// Generate a `width * height` block of 2D fractal brownian motion noise,
/// scaled to `[0, 255]` and returned as one byte per sample (row-major).
///
/// From JavaScript this comes back as a `Uint8Array` of length `width * height`
/// — e.g. ready to drop into a grayscale `ImageData` / canvas.
#[wasm_bindgen]
pub fn fbm_2d(width: usize, height: usize, freq: f32, seed: i32) -> Vec<u8> {
    let noise = NoiseBuilder::fbm_2d(width, height)
        .with_freq(freq)
        .with_seed(seed)
        .generate_scaled(0.0, 255.0);
    noise.into_iter().map(|v| v as u8).collect()
}

/// Generate a `width * height` block of 2D gradient (simplex) noise, scaled to
/// `[0, 255]` and returned as one byte per sample (row-major).
#[wasm_bindgen]
pub fn gradient_2d(width: usize, height: usize, freq: f32, seed: i32) -> Vec<u8> {
    let noise = NoiseBuilder::gradient_2d(width, height)
        .with_freq(freq)
        .with_seed(seed)
        .generate_scaled(0.0, 255.0);
    noise.into_iter().map(|v| v as u8).collect()
}

/// Which simdnoise backend this module was built against, so a host can confirm
/// whether it is exercising the wasm32 SIMD128 engine or the scalar fallback.
#[wasm_bindgen]
pub fn backend() -> String {
    if cfg!(feature = "scalar") {
        "scalar".to_string()
    } else {
        "wasm32-simd128".to_string()
    }
}
