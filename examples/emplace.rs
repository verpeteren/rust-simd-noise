//! Generating noise into caller-owned memory with the `*_into` / `generate_into`
//! API (issue #5), instead of receiving a freshly allocated `Vec` every call.
//!
//! Run with: `cargo run --example emplace`

use simdnoise::{NoiseBuilder, Settings};

fn main() {
    // 1. Fill a buffer you already own. `generate_into` writes into the slice and
    //    returns the generated (min, max) so you can scale it yourself later.
    let (width, height) = (256, 256);
    let mut buffer = vec![0.0f32; width * height];
    let (min, max) = NoiseBuilder::fbm_2d(width, height)
        .with_seed(1337)
        .with_freq_2d(0.02, 0.02)
        .generate_into(&mut buffer);
    println!(
        "1) filled {} values in place; range [{min:.3}, {max:.3}]",
        buffer.len()
    );

    // 2. The motivating use case: generate one big block in parallel. Split a
    //    single preallocated buffer into row-bands and fill each band on its own
    //    thread, giving each the matching y offset. No per-chunk allocation, no
    //    copy back. (std::thread::scope keeps it dependency-free; a rayon
    //    `par_chunks_mut` would look the same.)
    let (w, h) = (512, 512);
    let bands = 4;
    let band_h = h / bands; // evenly divisible for the demo
    let mut big = vec![0.0f32; w * h];
    std::thread::scope(|scope| {
        for (band, chunk) in big.chunks_mut(w * band_h).enumerate() {
            scope.spawn(move || {
                let y_offset = (band * band_h) as f32;
                NoiseBuilder::fbm_2d_offset(0.0, w, y_offset, band_h)
                    .with_seed(1337)
                    .with_freq_2d(0.02, 0.02)
                    .generate_into(chunk);
            });
        }
    });
    println!("2) filled {w}x{h} noise across {bands} threads into one buffer");

    // 3. Scaled output maps the result onto a target range in place...
    let mut scaled = vec![0.0f32; width * height];
    NoiseBuilder::gradient_2d(width, height)
        .with_seed(42)
        .generate_scaled_into(0.0, 255.0, &mut scaled);
    println!("3) scaled-in-place sample: {:.3}", scaled[0]);

    // 4. ...and f64 variants exist for every entry point (`*_into_64`).
    let mut doubles = vec![0.0f64; width];
    NoiseBuilder::fbm_1d(width)
        .with_seed(42)
        .generate_into_64(&mut doubles);
    println!("4) f64 sample: {:.6}", doubles[0]);
}
