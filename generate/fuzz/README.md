# Fuzzing the `generate` parameters

A [`cargo-fuzz`](https://rust-fuzz.github.io/book/cargo-fuzz.html) (libFuzzer)
harness over the `generate` tool's input parameters. The target
[`generate_params`](fuzz_targets/generate_params.rs) maps arbitrary bytes to a
full set of options and drives the real pipeline exposed by the `generate`
library — `resolve` → `generate` → `to_frames` — asserting it never panics on
any parameter combination.

## What it covers

- All six algorithms and dimensions 1–6.
- Offsets, per-axis / uniform frequencies, seed, scale range, f32/f64, every
  scaling and quantize mode — taken raw, **including non-finite floats**.
- Cellular2 `index0` / `index1` given **unclamped**, so the harness checks that
  `generate` clamps them to the library's `validate` rules itself (a bad pair
  must not panic).

To keep each run cheap, extents are capped to 1–16 and `octaves` is kept finite
and ≤ 8 (an unbounded octave count only makes a fractal spin, it finds no bug).
Generation runs on a 256 MiB-stack thread, matching the viewer's worker, so the
deep 6D path ([issue #38](https://github.com/verpeteren/rust-simd-noise/issues/38))
cannot trip a stack-overflow false positive.

## Running

Needs the nightly toolchain and `cargo-fuzz`:

```bash
rustup toolchain install nightly
cargo install cargo-fuzz
```

Then, from the `generate/` directory:

```bash
# Fuzz for 60 seconds
cargo +nightly fuzz run generate_params -- -max_total_time=60

# Reproduce / debug a saved crash input
cargo +nightly fuzz run generate_params fuzz/artifacts/generate_params/<crash-file>
```

This crate is its own workspace and is excluded from the host workspace, so a
normal `cargo build` / `cargo test` at the repo root never touches it.
