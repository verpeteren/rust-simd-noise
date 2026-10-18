# simdnoise-generate

A single CLI binary that can generate and display (via `minifb`) every
`simdnoise` variant. Its option surface mirrors the Slint `viewer` crate, so the
viewer's **Submit** button emits a command that reproduces exactly the noise you
were looking at.

The window shows the result in grayscale; dimensions above 2 are displayed as
cycling 2D `width × height` frames. Press `ESC` to exit.

## Building / running

From the workspace root:

```bash
cargo run --bin generate -- --help
```

## Global options

```text
--dim <1-6>                 number of dimensions (default 2)
--width / --height / --depth / --time / --ext5 / --ext6   per-axis extents
--offset-x/-y/-z/-w/-u/-v   per-axis coordinate offsets (default 0)
--seed <i32>                seed (default 1)
--freq <f32>                uniform frequency (default 0.02)
--freq-x/-y/-z/-w/-u/-v     per-axis frequency overrides
--float <f32|f64>           value type (default f32)
--scaling <raw|scaled>      output scaling (default raw)
--scale-min / --scale-max   target range when scaled, or source range when quantizing
--quantize <none|u8|u16|u32|i16|i32>   integer quantization (f32 only, default none)
```

Only the extents/offsets/frequencies up to `--dim` are used.

## Subcommands

| Command | Options |
|---|---|
| `gradient` | — |
| `fbm` / `ridge` / `turbulence` | `--lacunarity` `--gain` `--octaves` |
| `cellular` | `--jitter` `--distance <euclidean\|manhattan\|natural>` `--return <cellvalue\|distance>` |
| `cellular2` | `--jitter` `--distance …` `--return2 <distance2\|distance2add\|distance2sub\|distance2mul\|distance2div>` `--index0` `--index1` |

## Examples

```bash
# 2D FBM
cargo run --bin generate -- --dim 2 --width 512 --height 512 --seed 1 --freq 0.02 \
    fbm --lacunarity 0.5 --gain 2 --octaves 3

# 3D gradient, quantized to u16, shown as cycling depth slices
cargo run --bin generate -- --dim 3 --width 256 --height 256 --depth 8 \
    --quantize u16 --scale-min -1 --scale-max 1 --freq 0.05 gradient

# 6D Cellular2 (small extents), f64, Manhattan distance
cargo run --bin generate -- --dim 6 --width 16 --height 16 --depth 4 --time 2 \
    --ext5 2 --ext6 2 --float f64 --freq 0.05 \
    cellular2 --jitter 0.3 --distance manhattan --return2 distance2mul --index0 1 --index1 3
```

## Internals & fuzzing

The arg → noise-block pipeline (`resolve` → `generate` → `to_frames`) lives in
the crate library (`src/lib.rs`), so the binary is just a thin minifb display
loop. That split lets the pipeline be unit-tested (`cargo test -p
simdnoise-generate`) and fuzzed: a [`cargo-fuzz`](fuzz/README.md) target feeds
random parameter combinations through it and asserts it never panics. Invalid
Cellular2 `index0`/`index1` pairs are clamped to the library's rules (matching
the viewer), so a bad `--index0`/`--index1` can't crash the tool.
