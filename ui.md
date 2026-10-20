i# Plan — full dimensional + engine parity (then reassess before the UI)

## Context (why)

We want an interactive **noise viewer/inspector** (à la Auburn/FastNoiseSIMD) for
`verpeteren/rust-simd-noise`. Before designing the UI we first bring the library itself to a
complete, uniform matrix of "variants and knobs" so the UI can expose them consistently. This
document is the single reference: capability inventory, notes/decisions, and a **task table** to
close the gaps. Execution runs **Phases 0–3 on one branch, then stops before Phase 4 (UI)** so we
can reassess.

Recent groundwork already merged (`#72`): the f64 SIMD path (blendv masks + 64-bit `hash3d`) is
fixed, so **f64 works** for everything currently implemented. Remaining gaps: cellular missing
1D/4D, AVX-512 not a first-class compile-time engine, nothing above 4D.

Decisions with the user:
- **Dimensions:** cellular/cellular2 → 1D+4D, *and* extend every algorithm up to **6D**.
- **AVX-512:** full compile-time parity (new `intrinsics::avx512` engine + fixtures).
- **Tests:** full golden-fixture coverage (per-engine `.bin`), matching today's pattern.
- **Process:** no GitHub issues; **one branch** for all of Phases 0–3; **do not** commit this plan
  into the repo; fix the pre-existing clippy/fmt warnings as prework; **stop at Phase 4**.

---

## 1. Capability inventory

Legend: `v` = implemented & working · `v*` = implemented, **experimental** (5D/6D: empirical
normalization, no golden fixtures yet) · blank = not needed. (All cells below are now implemented;
`x→v` transitions from the original plan are done and merged.)

| Algorithm | knobs | 1D | 2D | 3D | 4D | 5D | 6D |
|---|---|:--:|:--:|:--:|:--:|:--:|:--:|
| Gradient (raw Simplex) | seed, freq(x..) | v | v | v | v | v* | v* |
| FBM | + lacunarity, gain, octaves | v | v | v | v | v* | v* |
| Ridge | + lacunarity, gain, octaves | v | v | v | v | v* | v* |
| Turbulence | + lacunarity, gain, octaves | v | v | v | v | v* | v* |
| Cellular (Voronoi F1) | seed, freq(x,y,z..), distance_fn, return_type, jitter | v | v | v | v | v* | v* |
| Cellular2 (Voronoi F2) | + return_type2, index0, index1 | v | v | v | v | v* | v* |

- **Value type:** `f32` and `f64` both work on every `v` cell. The crate is **not** generic over the
  float — parallel `generate()`/`generate_f64()` twins (also `_scaled_64`, `_into_64`). The UI wraps
  both behind its own generic `T`.
- **Engines:** Scalar `v` · SSE2 `v` · SSE4.1 `v` · AVX2 `v` · NEON `v` (aarch64) · WASM simd128 `v`
  (wasm32, compile-time) · **AVX-512** runtime `v` / compile-time `x→v`.
- **Selection:** Runtime (`simd_runtime_generate!`) auto-picks best (x86: AVX-512→AVX2→SSE4.1→SSE2→
  scalar; aarch64: NEON). Compile-time via `scalar` feature or the `intrinsics::{engine}` modules.
- **Output modes** (Settings trait): `generate` → `(Vec,min,max)` · `generate_scaled` ·
  `fill` · `fill_scaled` · `fill_range::<T>` (u8/u16/u32/i16/i32), each
  with an f64 twin except `range_into` (f32-only). Per-axis offsets via `*_offset` builders.
- **Defaults** (seed UI controls): seed=1 · freq=0.02 · lacunarity=0.5, gain=2.0, octaves=3 ·
  cellular Euclidean/Distance/jitter=0.25 · cellular2 +Distance2/index0=0/index1=1
  (`validate`: index0≤2, index1≤3, index0<index1).

---

## 2. Notes / key findings

- **f64 is fixed** (#72) — not a gap anywhere currently implemented.
- **AVX-512 already runs at runtime**: simdeez 3.0.1 ships `engines::avx512::Avx512` (16-wide f32
  `__m512`, 8-wide f64 `__m512d`); `simd_runtime_generate!` selects it ahead of AVX2. This dev box
  has `avx512f/bw/dq/vl`. `lib.rs:49`'s "AVX512 TODO" is stale for the runtime path. simdeez
  already exposes `try_transmute_{from_,}avx512` (`base/transmute.rs`), which the `intrinsics`
  macros require for the compile-time engine.
- **NEON f64 cellular stays excluded** (approx `rsqrt` → NaN; `tests/helpers.rs:11`,
  `intrinsics_cellular_64.rs` header). New cellular dims also use `rsqrt`, so NEON f64 cellular is
  excluded across **all** dims. NEON f64 simplex-family is fine.
- **f32/i32 cellular hash-bit budget:** 2D/3D pack 10-bit jitter fields at bits 0–9/10–19/20–29; a
  4th axis has only 2 bits left in a 32-bit hash. So **4D+ f32 cellular needs a second avalanche
  hash round** to draw fresh bits for axes ≥4 (keeps ~10-bit resolution). f64/i64 is fine to ~6
  axes.
- **5D/6D simplex is the main cost/risk:** `simplex_4d` (`noise/simplex_32.rs:446`) hand-codes the
  simplical subdivision — rank-order via `C(n,2)` compares (4D=6 → 5D=10, 6D=15), `n+1` corners,
  `gradN`, `hashN`, skew/unskew `F_n/G_n`, and a scalar **normalization constant** for `[-1,1]`.
  No canonical reference output exists for 5D/6D → validate by scalar↔SIMD cross-check and an
  **empirically calibrated** normalization constant.
- **1D cellular semantics:** feature point degenerates to `±jitter`; 1D cellular2 with `index1=3`
  has only 3 candidate cells (4th slot stays sentinel) → tighten `validate` for `dim==1`.
- **`NoiseDimensions`** (`noise_dimensions.rs`) caps dim at `1..=4` with `x,y,z,w` +
  `width,height,depth,time`; 5D/6D needs 2 more offsets + 2 more extents (prefer refactor to
  `offsets:[f32;6]`/`extents:[usize;6]`).

---

## 3. Prework & branch (Phase 0)

- **Branch:** one long-lived branch off fresh `master` for all of Phases 0–3 (suggest
  `feat/dimensional-parity`). No GitHub issues. Do not commit this plan file into the repo.
- Fix the pre-existing warnings so CI (`-D warnings`) is clean before feature work:

| ID | Task | Cx | Files to touch | What to implement | What to test | Error handling |
|---|---|:--:|---|---|---|---|
| 0.1 | Branch setup | S | — | `git checkout -B feat/dimensional-parity origin/master` | builds on clean master | — |
| 0.2 | Fix `fmt` | S | `src/lib.rs`, `src/noise/simplex_32.rs` | run `cargo fmt` | `cargo fmt --check` clean | — |
| 0.3 | Fix clippy `too_many_arguments` (24) | S | `src/intrinsics/mod.rs` (`fbm!`/`ridge!`/`turbulence!` macros) | add `#[allow(clippy::too_many_arguments)]` to the generated `pub unsafe fn`s (consistent with existing allows in `noise_builder.rs`) | `cargo clippy --all-targets -- -D warnings` clean | ensure allow doesn't mask unrelated lints |

---

## 4. Phase 1 — Cellular & Cellular2 → 1D + 4D (f32 + f64)

Kernel work (4.1–4.4) is the prerequisite for the settings bodies (4.5–4.6).

| ID | Task | Cx | Files to touch | What to implement | What to test | Error handling |
|---|---|:--:|---|---|---|---|
| 1.1 | Distance strategy 1D/4D | S | `noise/cell_strategy.rs` | add `new_distance_1d`/`new_distance_4d` to `CellDistance` trait + `Euclidean`/`Manhattan`/`Natural` impls (`CellReturn`/`Cell2Return` unchanged) | unit: 1D dist == |x|; 4D == metric of 4 comps | — |
| 1.2 | Hashes + bit budget | M | `noise/cellular_32.rs`, `noise/cellular_64.rs` | add `hash_1d`/`hash_4d`, `W_PRIME_{32,64}`; **f32 4D second-hash** for the 4th jitter field | 4D f32 jitter well-distributed (hist); f64 uses bits 30–39 | wrapping mul; no overflow panic |
| 1.3 | Cellular kernels 1D/4D | M | `noise/cell_32.rs`, `noise/cell_64.rs` | `cellular_1d` (one `for 0..3`), `cellular_4d` (4 nested loops; `inv_mag=jitter*(x²+y²+z²+w²).rsqrt()`) mirroring `cellular_2d`/`_3d` | engine==scalar (x86); finite | zero-distance `rsqrt`→inf guard as in 2D/3D |
| 1.4 | Cellular2 kernels 1D/4D | M | `noise/cell2_32.rs`, `noise/cell2_64.rs` | `cellular2_1d`/`cellular2_4d` (same insertion-sort body as 2D/3D) | engine==scalar; finite | 1D: only 3 candidates vs 4 slots |
| 1.5 | CellularSettings wiring | M | `settings/cellular_settings.rs` | impl `sample_1d`/`sample_4d` (f32+f64 via `dispatch_cell!`), `with_freq_4d`, `freq_w`; add `1`/`4` arms to all 9 `generate*` | `generate`/`_64`/`_into` for 1D & 4D all dims | keep `panic!` for 5/6 until Phase 3 |
| 1.6 | Cellular2Settings wiring | M | `settings/cellular2_settings.rs` | same as 1.5 for cellular2 | same + index combos | tighten `validate` for `dim==1` (`index1≤2`) |
| 1.7 | Builders | S | `noise_builder.rs` | `cellular_1d/_4d(+_offset)`, `cellular2_1d/_4d(+_offset)` (8 fns) mirroring `fbm_1d`/`fbm_4d` | builder round-trips to correct dims | `too_many_arguments` allow on `_4d_offset` |
| 1.8 | Dispatch arms | S | `noise_helpers_32.rs`, `noise_helpers_64.rs` | replace `Cellular(_)/Cellular2(_)=>panic!` in the 1D & 4D `get_*` dispatchers with the `..._helper_*::<S, _>` calls (mirror 2D/3D); f64 has no `range_into` | all `get_{1,4}d_*` for cellular run | keep `panic!` arms for dims 5/6 |
| 1.9 | Single-value intrinsics (opt.) | S | `intrinsics/mod.rs`, `scalar.rs`/`sse2.rs`/`sse41.rs`/`avx2.rs`(+`avx512.rs`/`neon.rs`) | add `"1d"`/`"4d"` arms to `cellular!` macro + instantiate | per-engine single-value fns compile/run | parity only; cell2 has no macro |
| 1.10 | Tests + golden fixtures | L | `tests/intrinsics_x86.rs`, `intrinsics_neon.rs`, `intrinsics_cellular_64.rs`, `noisebuilder.rs`, `tests/assets/*.bin` | add 1D/4D cases (f32: scalar/sse2/sse41/avx2 + neon; f64: x86 engines only); regen via `SIMDNOISE_REGEN=1` / uncomment `save_to_file_*`; add scalar↔SIMD cross-check | ~430 new `.bin`; 0 ignored/0 panic; cross-check finite + `<tol` | NEON f64 cellular stays excluded; f32 neon regen on user's Mac |
| 1.11 | Docs | S | `src/lib.rs:12` | "Cellular … 2D and 3D" → "1D–4D" | doc-tests | — |

---

## 5. Phase 2 — AVX-512 first-class compile-time engine

| ID | Task | Cx | Files to touch | What to implement | What to test | Error handling |
|---|---|:--:|---|---|---|---|
| 2.1 | `intrinsics::avx512` module | M | `src/intrinsics/avx512.rs` (new) | mirror `avx2.rs` with `__m512`/`__m512d`, `try_transmute_{from_,}avx512`; all macro invocations + `generate_noise!`/`generate_noise_scaled!` | `intrinsics::avx512::get_*noise::<Avx512>` runs here | transmute panics if `S≠Avx512` (by design) |
| 2.2 | Register + cfg gates | S | `src/intrinsics/mod.rs` | `#[cfg(x86/x86_64)] pub mod avx512;`; extend `cellular!/simplex!/fbm!/ridge!/turbulence!` cfg to include `target_feature="avx512f"` | compiles on this box with `-C target-feature=+avx512f` | gate so non-AVX512 builds unaffected |
| 2.3 | AVX-512 golden fixtures | L | `tests/intrinsics_*`, `tests/assets/*avx512*.bin` | generate `avx512` variants for all noise types/dims/combos (on this AVX-512 box); add `avx512` arm to engine-iterating tests | avx512 == scalar within tol | generate only where `avx512f` present |
| 2.4 | Verify + docs | S | `src/lib.rs` (docs) | confirm runtime selects AVX-512 here; remove stale `:49` TODO; update feature list | runtime path uses Avx512 | — |

---

## 6. Phase 3 — 5D / 6D extension (largest; research-grade)

> Status: **Phases 0–3 are COMPLETE and MERGED to `master`.** 5D/6D for all six algorithms (f32+f64,
> every engine), the public-API plumbing, and the behavioral tests landed via PR #75 on branch
> `feat/higher-dimensional-noise`. 5D/6D is shipped as **experimental** (empirically calibrated
> normalization, no reference impl): it is covered by cross-engine / f32↔f64 / range-finiteness tests
> but has **no stored golden fixtures yet** (deferred until the output stabilizes; NEON regen also
> deferred). This is documented in the README and crate docs. 1D–4D keep full golden coverage.
> The notes below (6.0–6.3) are retained as the implementation record.

### 6.0 Research & derived facts (from issue #2 + the crate's own simplex)

Origin: `verpeteren/rust-simd-noise` **issue #2** ("Higher Dimensional noise"). Useful references
it surfaced (all currently **blocked by the session network policy** — fetch via browser or widen
Network access if needed, but they are **not required**, since the crate already encodes the same
convention): Gustavson's simplex PDF; the hilvi/pastel `simplex_noise_notes`; `amaranth`'s
any-dimension gradient gist (`gist.github.com/amaranth/a9ed8fff9a3c029aa743`); and `felix91gr`'s
prototype `github.com/felix91gr/rust-noiseToys` (`src/gradient.rs` read — confirms the gradient set
below).

**Everything needed is determined** (verified against `simplex_32.rs`'s existing 2D/3D/4D constants):

- **Skew / unskew:** `F_n = (√(n+1) − 1)/n`, `G_n = (1 − 1/√(n+1))/n` (matches F2=0.366025,
  F4=0.309017, G4=0.138197 in the crate). New constants to add:
  - 5D: `F5 = 0.289_897_95`, `G5 = 0.118_350_34`
  - 6D: `F6 = 0.274_291_89`, `G6 = 0.103_672_59`
  (f32 const + f64 const each, mirroring the `*_32`/`*_64` pairs at `simplex_32.rs:16-42`.)
- **Gradient set** (`felix`'s `create_gradient` == the crate's existing `grad4` family): a vector
  with **one component 0 and the rest ±1, scaled by `1/√(n−1)`**. 5D → 5·2⁴ = **80** gradients
  ×`0.5`; 6D → 6·2⁵ = **192** gradients ×`1/√5`. Implement by generalizing `grad4`'s bit-trick
  (`gradient_32.rs`): hash → a selector for which axis is zeroed + `(n−1)` sign bits; computed at
  runtime, **no 2ⁿ table**.
- **Kernel:** per corner `t = r² − |d|²`; if `t > 0`, contribution `t⁴ · dot(grad, d)`. Use
  `r² = 0.6` (the crate's 3D/4D value, `simplex_32.rs:335-338`) as the starting radius.
- **Corner traversal:** generalize `simplex_4d`'s rank-ordering (`rank_x/y/z/w`, the `C(n,2)`
  pairwise compares at `simplex_32.rs:489-537`) to 5 coords (10 compares) / 6 coords (15 compares);
  the ranks give the `n+1` corner offsets.
- **Normalization scale:** empirical per-dimension constant (like 2D's `45.264_507`,
  `simplex_32.rs:237`). **No closed form at any dimension** — must be calibrated by sampling (see
  3.2). This and the kernel radius are the only non-mechanical pieces.

### 6.1 Open decision for the user (settle before 3.3/3.4 simplex kernels)

`amaranth` (issue #2): *classic simplex shows visible **surflet artifacts past 4D**.* So:
- **(A) Classic simplex 5D/6D** — consistent with the crate's existing 1D–4D family, minimal new
  concepts, accepts the higher-D visual artifacts. **Recommended** (matches the codebase; the
  viewer can reveal whether the artifacts matter in practice).
- **(B) OpenSimplex-style kernel for 5D/6D** — better high-D quality, but a different, more complex,
  slower algorithm that would not share the existing simplex code. Larger, separate effort.
This plan assumes **(A)** unless the user chooses (B).

### 6.2 Task breakdown

| ID | Task | Cx | Files to touch | What to implement | What to test | Error handling |
|---|---|:--:|---|---|---|---|
| 3.1 | `NoiseDimensions` → 6D | M | `noise_dimensions.rs` (+ every field user: `settings/*`, `noise_builder.rs`, `noise_helpers_{32,64}`, `shared.rs`) | allow `dim 1..=6`; add 2 axis offsets + 2 extents. **Prefer** refactoring `x,y,z,w`→`offsets:[f32;6]` and `width,height,depth,time`→`extents:[usize;6]` (keep thin accessors so existing call sites compile), else add named `u,v`/`extent5,extent6`. | all existing 1–4D tests unchanged; `default(5)`/`default(6)` construct; dim product correct | widen the `1..=4` bounds `panic!` + message; `fill` length assert uses the new product |
| 3.2 | Skew/unskew consts + calibration harness | M | `noise/simplex_32.rs`, `simplex_64.rs` | add `F5/G5/F6/G6` (+ `G*5`/`G*6` multiples as 4D has); add an **env-gated calibration harness** (`tests/`, like the NEON regen) that samples a large 5D/6D block from the *unscaled* kernel, reports `1/max|value|` → the normalization constant to bake in | consts match the `(√(n+1)−1)/n` / `(1−1/√(n+1))/n` formulas | — |
| 3.3 | `grad5`/`grad6` gradients | M | `noise/gradient_32.rs`, `gradient_64.rs`, `noise/hash3d_*`-style helper | generalize `grad4`'s bit-trick to the "one-zero, rest-±1, ×`1/√(n−1)`" set (80 / 192 gradients); add `hash5`/`hash6` seed-mixers | unit: generated gradients have exactly one zero component, others ±`1/√(n−1)`, |grad|≈1; distribution ~ `felix`'s scalar `create_gradient` | wrapping hash mul |
| 3.4 | `simplex_5d` kernel + normalization | XL | `noise/simplex_32.rs`, `simplex_64.rs` | the full 5D kernel: skew by `F5`, floor→cell, fractional coords, **rank-order (10 compares)** → 6 corner offsets, unskew by `G5`, per-corner `t⁴·dot(grad5,d)` with `r²=0.6`, sum ×`scale5`; bake `scale5` from 3.2. Land this **end-to-end and verified before 6D**. | scalar==sse2==sse41==avx2==avx512 (golden, exact per engine); f64≈f32; finite; **calibrated output within [−1,1]**; mean≈0 over a large block | no oracle → cross-engine + statistical range are the gate |
| 3.5 | `simplex_6d` kernel + normalization | XL | `noise/simplex_32.rs`, `simplex_64.rs` | same as 3.4 for 6D: `F6`/`G6`, **15 compares** → 7 corners, `grad6`, `scale6` | same gates as 3.4 at 6D | same |
| 3.6 | FBM/Ridge/Turbulence 5D/6D | M | `noise/{fbm,ridge,turbulence}_{32,64}.rs` | add `*_5d`/`*_6d` stacking the new simplex (mechanical once 3.4/3.5 land); extend `theoretical_min_max` reasoning | engine==scalar; scaled range consistent with `amplitude_sum` | `octaves.max(1)` etc. unchanged |
| 3.7 | Cellular/Cellular2 5D/6D | L | `noise/cell_strategy.rs`, `cellular_{32,64}.rs`, `cell*_{32,64}.rs` | `new_distance_5d/6d`; `hash_5d/6d` + `V_PRIME`/`U_PRIME`; `3⁵=243` / `3⁶=729` neighbour loops; extend the Phase-1 2nd-hash scheme (f32 needs a 2nd round by 5D, a 3rd by 6D; f64 ok to ~6 via bits 0–59) | engine==scalar; finite | rsqrt guards; 729-cell loop is a perf (not correctness) cost; NEON f64 cellular still excluded |
| 3.8 | Plumbing 5D/6D | M | `noise_builder.rs`, `settings/*`, `noise_helpers_{32,64}.rs`, `intrinsics/mod.rs` + all engine modules, `src/lib.rs` | `NoiseBuilder::*_5d/_6d(+_offset)`; settings `5`/`6` `generate*` arms (or generalise the `match d`); `noise_helpers` `get_5d/6d_*` + `Sample*::sample_5d/6d`; `simplex!`/`fbm!`/… `"5d"/"6d"` macro arms + instantiate in scalar/sse2/sse41/avx2/avx512/neon; public `generate_5d_noise*`/`generate_6d_noise*` in `lib.rs` (both `#[cfg(scalar)]` and runtime variants) | end-to-end `NoiseBuilder…generate()`/`generate_f64()` for every algorithm at 5D & 6D | replace the dim `panic!` guards, don't just extend |
| 3.9 | Tests + golden fixtures 5D/6D | L | new compact `tests/*_5d6d*.rs` (loop-based, like `cellular_1d4d_*`), `tests/assets/*.bin` | golden per engine (scalar/sse2/sse41/avx2/avx512 x86; neon f32 on Mac) + scalar cross-check + finiteness/range sanity; **small extents** (e.g. 5D `8×8×4×2×2`, 6D `8×4×4×2×2×2`) to bound fixture size | 0 ignored; per-engine golden exact; simplex-family ≈ scalar within tol; all finite & in range | f32 neon regen on Mac; f64 cellular NEON excluded |

### 6.3 Verification (Phase 3)
- Per-engine golden exact-match (regression) + **cross-engine agreement** scalar≈sse2≈sse41≈avx2≈
  avx512 within fp tolerance for simplex-family (the correctness gate, since there is no external
  oracle); cellular via golden + finiteness.
- **Statistical sanity** on a large scalar block per dim: all finite, `min ≥ −1−ε`, `max ≤ 1+ε`
  after calibration, mean ≈ 0 (gradient/simplex).
- `cargo test --workspace`, `--features scalar`, doc-tests, `fmt --check`, `clippy -D warnings` all
  clean; sizes kept down via the small 5D/6D extents in 3.9.
- Optional visual check via the Phase 4 viewer once it exists (reveals the surflet artifacts, if any).

**→ Phase 3 complete & merged. Reassessment done — Phase 4 (the Slint viewer) is now in design; see §7.**

---

## 7. Phase 4 — Slint noise viewer (Phase 3 merged; Phase 4 in design)

### 7.0 Infrastructure (unchanged decisions)
- **Framework:** Slint. Separate crate (e.g. `viewer/`) depending on local `simdnoise`.
- **Responsiveness:** generate on a **worker thread** (channel + `Weak` callback to UI); never block
  the UI thread; debounce/coalesce rapid slider changes.
- **Preallocated memory:** reuse one output buffer across regenerations via the emplace API
  (`fill` / `fill_scaled` / `fill_range::<T>` into a persistent buffer).
  NB: a 6D cellular block needs a **large worker-thread stack** (issue #38); spawn the worker with
  an enlarged stack.
- **Value type:** `f32`/`f64` toggle behind the UI's own generic `T`, dispatching to `generate` vs
  `generate_f64`.

### 7.1 Layout
Central **canvas** (rendered image + a `min` / `max` / dimensions-product stats readout; post-MVP a
zoom/pan/rotate toolbar) with a scrollable **side panel** of collapsible sections in the A→F order
below. Sections **C (Fractal)** and **D (Cellular)** swap in/out by algorithm. Every per-axis row in
**B** and every cross-section slider in **F** **enables/disables** off the active-dimension radio.
Mirrors the Auburn/FastNoiseSIMD inspector (controls beside a live preview).

### 7.2 Parameter inventory (grouped; defaults from §1)

**A. Noise (always shown)** — Algorithm radio {Gradient, FBM, Ridge, Turbulence, Cellular F1,
Cellular2 F2}; Value-type toggle f32/f64; Seed (slider+entry, 1).

**B. Domain / grid (always shown; per-axis rows gate on active dim, greyed beyond it)** — one row per
axis 1→6:
- Size/extent sliders: width,height,depth,time,extent5,extent6.
- Offset sliders: x,y,z,w,offset5,offset6 (default 0).
- Frequency sliders: freq_x…freq_v (default 0.02). **Decision: per-axis sliders shown at all times**
  (not a uniform-with-expander); maps to `with_freq_Nd`.

**C. Fractal (FBM/Ridge/Turbulence only)** — Octaves slider (3); Lacunarity slider (0.5); Gain
slider (2.0).

**D. Cellular (Cellular/Cellular2 only)** — Jitter slider (0.25); Distance-function dropdown
{Euclidean, Manhattan, Natural}; Return-type dropdown {Distance, CellValue}; *Cellular2 adds* Return-
type2 dropdown {Distance2, …Add, …Sub, …Mul, …Div}, index0 stepper (0, ≤2), index1 stepper (1, ≤3,
`index0<index1`). Enforce the `validate` constraints in the UI (and the dim==1 tightening).

**E. Output (always shown; all MVP)** — Scaling toggle {Raw→report min/max | Scaled→`generate_scaled`}
with a Scale min/max slider pair when Scaled; Quantize dropdown {none(f32), u8, u16, u32, i16, i32}
→ `fill_range::<T>`. **Decision: Quantize is in the MVP.**

**F. View (always shown; all MVP)** — Cross-section sliders (**the user's sliders**), one per
non-drawn higher axis, each ranging over its extent and greyed when inactive:
- 1D → strip drawn as 2D; 2D → image; **3D → single 2D image with the 3rd axis colour-encoded
  (decision: colour, not a z-slice)**, 0 cross-section sliders;
- 4D → 1 slider (fix w); 5D → 2 sliders (fix w,u); 6D → 3 sliders (fix w,u,v).
- Colormap dropdown {grayscale, heatmap, …} — **MVP**. Engine dropdown {auto, scalar, sse2, sse41,
  avx2, avx512, neon} defaulting to runtime-auto — **MVP (debug aid)**.

**G. Canvas interaction — POST-MVP** — Zoom (scale about cursor); Pan (drag; **enabled only when
zoomed**); Rotate (3D view orientation / slice-plane rotation, meaningful once volume/colour 3D
rendering matures).

### 7.3 MVP vs post-MVP
- **MVP:** everything in A–F, including per-axis frequency sliders, Quantize, Colormap, and the
  Engine debug dropdown.
- **Post-MVP:** only group **G** (zoom / pan / rotate) and any richer 3D/volume rendering.

---

## 8. Verification (per phase, on the single branch)

- `cargo test --workspace` + each `tests/intrinsics_*`: **0 ignored, 0 panics**.
- Cross-engine: scalar == sse2 == sse41 == avx2 == **avx512** (x86) within tol for every new dim;
  NEON == scalar on the user's Mac; f64 ≈ f32 within precision.
- `cargo build`, `--features scalar`, doc-tests, `tests/issue_38.rs` green.
- `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` clean (kept clean from Phase 0).
- AVX-512: confirm runtime + `intrinsics::avx512` run here (`avx512f/bw/dq/vl` present).
- 5D/6D: generated blocks finite; calibrated normalization keeps scaled output in range.
- NEON f32 fixtures regenerated by the user on aarch64; f64 cellular stays excluded on NEON.

## 9. Sequencing & risk
1. **Phase 0** prework — quick, unblocks clean CI.
2. **Phase 1** cellular 1D/4D — mostly mechanical; one bit-budget decision (second hash).
3. **Phase 2** AVX-512 — mechanical mirror + large fixture generation (fine on this box).
4. **Phase 3** 5D/6D — large, research-grade (normalization has no golden reference); land
   simplex-5D fully (incl. fbm/ridge/turbulence) before 6D and before cellular 5D/6D.
5. **Stop. Reassess** before building the Slint viewer (Phase 4).
