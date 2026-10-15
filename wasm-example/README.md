# simdnoise — headless WebAssembly example

A minimal, headless [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
crate that calls `simdnoise` from WebAssembly and returns the generated noise as
bytes. There is no HTML or canvas — the point is only to show that the ordinary
`simdnoise` API works unchanged on `wasm32`.

On `wasm32`, `simdnoise` (through `simdeez`) automatically dispatches to the
wasm32 **SIMD128** engine, so none of the calls in [`src/lib.rs`](src/lib.rs) are
wasm-specific.

## Why a build flag is needed

WebAssembly has **no runtime SIMD feature detection**, so `simdeez` always emits
its SIMD128 engine on `wasm32`. Build with SIMD128 enabled:

```sh
# this directory ships a .cargo/config.toml that sets
#   target = "wasm32-unknown-unknown"
# and rustflags = ["-C", "target-feature=+simd128"]
cargo build --release
```

The resulting module requires a **SIMD128-capable runtime** — every current
browser, Node ≥ 16, Deno, and wasmtime qualify.

### No-SIMD fallback

For a runtime *without* SIMD128, build against simdnoise's `scalar` backend and
drop the SIMD128 flag:

```sh
RUSTFLAGS="" cargo build --release --features scalar
```

(`RUSTFLAGS=""` overrides the `+simd128` rustflags from `.cargo/config.toml` for
this build.) The `backend()` export reports which path the module was built
with.

## Generating the JS bindings (raw `wasm-bindgen-cli`)

Install the CLI once, matching the `wasm-bindgen` crate version in
[`Cargo.toml`](Cargo.toml):

```sh
cargo install wasm-bindgen-cli
```

Then build and run `wasm-bindgen` over the artifact:

```sh
cargo build --release
wasm-bindgen \
  target/wasm32-unknown-unknown/release/simdnoise_wasm_example.wasm \
  --out-dir pkg \
  --target web        # or: --target nodejs / bundler / deno
```

This writes `pkg/simdnoise_wasm_example.js` (loader) and
`pkg/simdnoise_wasm_example_bg.wasm` (the processed module).

## Running it

### Node (`--target nodejs`)

```sh
wasm-bindgen target/wasm32-unknown-unknown/release/simdnoise_wasm_example.wasm \
  --out-dir pkg --target nodejs
```

```js
// run with: node demo.js
const noise = require("./pkg/simdnoise_wasm_example.js");

console.log("backend:", noise.backend());        // "wasm32-simd128"
const bytes = noise.fbm_2d(64, 64, 0.05, 1337);  // Uint8Array, length 64*64
console.log("samples:", bytes.length, "first row:", bytes.slice(0, 8));
```

### Browser (`--target web`)

```sh
wasm-bindgen target/wasm32-unknown-unknown/release/simdnoise_wasm_example.wasm \
  --out-dir pkg --target web
```

```html
<script type="module">
  import init, { fbm_2d, backend } from "./pkg/simdnoise_wasm_example.js";
  await init();
  console.log(backend());
  const w = 256, h = 256;
  const bytes = fbm_2d(w, h, 0.03, 42); // Uint8Array, grayscale, row-major
  // e.g. expand to RGBA and blit into a <canvas> via ImageData.
</script>
```

Serve the directory over HTTP (ES modules + `fetch` of the `.wasm` do not work
from `file://`), e.g. `python3 -m http.server`.

## Exported functions

| Export | Returns | Notes |
|--------|---------|-------|
| `fbm_2d(width, height, freq, seed)`      | `Uint8Array` (`width*height`) | fractal brownian motion, scaled to `[0,255]` |
| `gradient_2d(width, height, freq, seed)` | `Uint8Array` (`width*height`) | gradient (simplex) noise, scaled to `[0,255]` |
| `backend()`                              | `string` | `"wasm32-simd128"` or `"scalar"` |

## Note

This crate is intentionally **excluded** from the repository's Cargo workspace
(it targets `wasm32` and pulls in `wasm-bindgen`), so it is built on its own as
shown above and does not affect `cargo build`/`cargo test` at the repo root.
