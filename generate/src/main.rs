//! `simdnoise-generate` — a CLI generator/viewer for the `simdnoise` crate.
//!
//! The option surface mirrors the Slint `viewer` crate so the viewer's "Submit"
//! button can emit a faithful, runnable command. The arg → noise-block pipeline
//! lives in the library (`lib.rs`, so it can be unit-tested and fuzzed); this
//! binary adds only the minifb display loop. The generated block is shown in
//! grayscale; higher dimensions cycle through 2D frames.

use clap::Parser;
use minifb::{Key, Window, WindowOptions};
use simdnoise_generate::{generate, resolve, to_frames, Args};

const FPS: usize = 30;

fn main() {
    let args = Args::parse();
    let o = resolve(args);

    let (values, lo, hi) = generate(&o);
    let buffers = to_frames(&values, lo, hi, &o);

    let width = o.extent[0];
    let height = o.extent[1];
    let mut window = Window::new(
        "simdnoise-generate - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{}", e));
    window.set_target_fps(FPS);

    let mut frame = 0usize;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let buffer = &buffers[frame % buffers.len()];
        window.update_with_buffer(buffer, width, height).unwrap();
        if buffers.len() > 1 {
            frame += 1;
        }
    }
}
