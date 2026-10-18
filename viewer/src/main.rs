//! simdnoise viewer — an interactive Slint inspector for the `simdnoise` crate.
//! See `ui.md` §7 for the design.

// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dispatch;
mod params;
mod render;
mod worker;

use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use clap::Parser;
use slint::{ComponentHandle, ModelRc, SharedString, TimerMode, VecModel};

use params::*;
use simdnoise::{Cell2ReturnType, CellDistanceFunction, CellReturnType};
use worker::Job;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Engine dropdown: only engines runnable on this build/CPU. Keep a parallel
    // Vec so the selected index maps back to an `Engine`.
    let engines: Vec<Engine> = Engine::all()
        .into_iter()
        .filter(|e| e.available())
        .collect();
    let engine_labels: Vec<SharedString> = engines
        .iter()
        .map(|e| SharedString::from(e.label()))
        .collect();
    ui.set_engine_model(ModelRc::new(VecModel::from(engine_labels)));
    let engines = Rc::new(engines);

    let latest: worker::Latest = Arc::new(AtomicU64::new(0));
    let probe: worker::ProbeCell = Arc::new(Mutex::new(None));
    let frame: worker::FrameCell = Arc::new(Mutex::new(None));
    let tx = worker::spawn(ui.as_weak(), latest.clone(), probe.clone(), frame.clone());

    // One trigger closure, shared by the debounce timer and the initial render.
    let do_trigger: Rc<dyn Fn()> = {
        let ui_weak = ui.as_weak();
        let tx = tx.clone();
        let latest = latest.clone();
        let engines = engines.clone();
        Rc::new(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let p = build_params(&ui, &engines);
                let id = latest.fetch_add(1, Ordering::SeqCst) + 1;
                let _ = tx.send(Job { id, params: p });
            }
        })
    };

    // Debounce: coalesce a burst of slider changes into one job after a short idle.
    let timer = Rc::new(slint::Timer::default());
    let last_dim = Rc::new(std::cell::Cell::new(0i32));
    {
        let timer = timer.clone();
        let do_trigger = do_trigger.clone();
        let ui_weak = ui.as_weak();
        let last_dim = last_dim.clone();
        ui.on_regenerate(move || {
            let mut playing = false;
            if let Some(ui) = ui_weak.upgrade() {
                // Rebuild the animation axis dropdown only when the dimension count
                // changes, not on every slider tick.
                if last_dim.get() != ui.get_dim() {
                    refresh_anim_axis_model(&ui);
                    last_dim.set(ui.get_dim());
                }
                // `busy` always tracks an in-flight render (the overlay itself is
                // hidden during playback in the .slint). The animation loop reads it
                // to self-throttle, so it must be accurate even while playing.
                ui.set_busy(true);
                playing = ui.get_anim_playing();
            }
            if playing {
                // While animating there's no burst to coalesce (the loop advances
                // one frame at a time and only when idle), so skip the debounce and
                // start the latest frame immediately.
                do_trigger();
            } else {
                // Debounce: coalesce a burst of slider changes into one job.
                let do_trigger = do_trigger.clone();
                timer.start(
                    TimerMode::SingleShot,
                    Duration::from_millis(30),
                    move || {
                        do_trigger();
                    },
                );
            }
        });
    }

    // Animation: a repeating timer advances the scrubber value `anim-t` while
    // playing. It is *self-throttling* — it skips a tick while a render is still
    // in flight (`busy`), so frames never pile up and the one on screen is always
    // the latest. Advancement is scaled by real elapsed time, so the pan/zoom
    // speed stays constant even when the renderer runs slower than the tick rate.
    let anim_timer = Rc::new(slint::Timer::default());
    let last_tick = Rc::new(std::cell::Cell::new(Instant::now()));
    {
        let ui_weak = ui.as_weak();
        let last_tick = last_tick.clone();
        anim_timer.start(TimerMode::Repeated, Duration::from_millis(33), move || {
            if let Some(ui) = ui_weak.upgrade() {
                if !ui.get_anim_playing() {
                    last_tick.set(Instant::now());
                    return;
                }
                // Wait for the in-flight frame instead of queuing another.
                if ui.get_busy() {
                    return;
                }
                let now = Instant::now();
                let dt = (now - last_tick.get()).as_secs_f32().min(0.2);
                last_tick.set(now);
                let speed = ui.get_anim_speed().clamp(0.0, 1.0);
                let mut next = ui.get_anim_t();
                if ui.get_anim_drive_idx() == 1 {
                    // Freq (zoom): sweep within [0.001, 0.5], wrapping.
                    const FREQ_RATE: f32 = 0.4; // band-widths per second at speed 1
                    next += speed * FREQ_RATE * dt;
                    if next > 0.5 {
                        next = 0.001;
                    }
                } else {
                    // Offset (pan): sweep ±extent (a full image width), wrapping.
                    let dim = ui.get_dim().clamp(1, 6) as usize;
                    let axis = (ui.get_anim_axis_idx().max(0) as usize).min(dim - 1);
                    let ext = extent_axis(&ui, axis).max(1.0);
                    let rate = 2.0 * ext / 8.0; // cross the full span in ~8s at speed 1
                    next += speed * rate * dt;
                    if next > ext {
                        next -= 2.0 * ext;
                    }
                }
                ui.set_anim_t(next);
            }
        });
    }

    // Scrub: write the scrubber value onto the chosen axis' offset or freq.
    {
        let ui_weak = ui.as_weak();
        ui.on_scrub(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let dim = ui.get_dim().clamp(1, 6) as usize;
                let axis = (ui.get_anim_axis_idx().max(0) as usize).min(dim - 1);
                let t = ui.get_anim_t();
                if ui.get_anim_drive_idx() == 1 {
                    set_freq_axis(&ui, axis, t.clamp(0.001, 0.5));
                } else {
                    let ext = extent_axis(&ui, axis);
                    set_offset_axis(&ui, axis, t.clamp(-ext, ext));
                }
            }
        });
    }

    // Sync-t: refresh the scrubber from the chosen axis' current value (on an
    // axis or drive change) so it starts from the right position.
    {
        let ui_weak = ui.as_weak();
        ui.on_sync_t(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let dim = ui.get_dim().clamp(1, 6) as usize;
                let axis = (ui.get_anim_axis_idx().max(0) as usize).min(dim - 1);
                let v = if ui.get_anim_drive_idx() == 1 {
                    freq_axis(&ui, axis)
                } else {
                    offset_axis(&ui, axis)
                };
                ui.set_anim_t(v);
            }
        });
    }

    // Reset every control to its default and re-render.
    {
        let ui_weak = ui.as_weak();
        ui.on_reset(move || {
            if let Some(ui) = ui_weak.upgrade() {
                apply_defaults(&ui);
            }
        });
    }

    // Submit: print the equivalent `generate` CLI command to stdout and copy it to
    // the clipboard.
    {
        let ui_weak = ui.as_weak();
        let engines = engines.clone();
        ui.on_submit(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let cmd = generate_command(&build_params(&ui, &engines));
                println!("{cmd}");
                if let Ok(mut cb) = arboard::Clipboard::new() {
                    let _ = cb.set_text(cmd);
                }
            }
        });
    }

    // Save: write the frame currently on screen to a PNG chosen via a file dialog.
    {
        let frame = frame.clone();
        ui.on_save(move || {
            let snapshot = frame.lock().ok().and_then(|g| g.clone());
            if let Some((w, h, rgb)) = snapshot {
                if let Some(mut path) = rfd::FileDialog::new()
                    .set_file_name("noise.png")
                    .add_filter("PNG image", &["png"])
                    .save_file()
                {
                    // We only write PNG, and the encoder picks the format from the
                    // path's extension — so force `.png` if the typed name lacks it
                    // (e.g. "noise") or used a different one.
                    let is_png = path
                        .extension()
                        .and_then(|e| e.to_str())
                        .is_some_and(|e| e.eq_ignore_ascii_case("png"));
                    if !is_png {
                        path.set_extension("png");
                    }
                    if let Err(e) =
                        image::save_buffer(&path, &rgb, w, h, image::ExtendedColorType::Rgb8)
                    {
                        eprintln!("failed to save image: {e}");
                    }
                }
            }
        });
    }

    // Last cursor position over the canvas (logical px), so a new frame can
    // refresh the readout for a stationary cursor (e.g. while animating).
    let last_hover = Rc::new(std::cell::Cell::new((f32::NAN, f32::NAN)));

    // Hover: show the point under the cursor in the status bar.
    {
        let ui_weak = ui.as_weak();
        let probe = probe.clone();
        let last_hover = last_hover.clone();
        ui.on_point_hover(move |mx, my| {
            last_hover.set((mx, my));
            if let Some(ui) = ui_weak.upgrade() {
                let (px, py) = pixel_at(&ui, mx, my);
                let text = probe
                    .lock()
                    .ok()
                    .and_then(|g| g.as_ref().and_then(|p| p.at(px, py)).map(|(t, _)| t));
                ui.set_probe_text(text.unwrap_or_default().into());
            }
        });
    }

    // Re-probe: after a new frame lands, refresh the readout at the last cursor
    // position so a stationary cursor still tracks the changing value (animation).
    {
        let ui_weak = ui.as_weak();
        let probe = probe.clone();
        let last_hover = last_hover.clone();
        ui.on_reprobe(move || {
            let (mx, my) = last_hover.get();
            if !mx.is_finite() || !my.is_finite() {
                return;
            }
            if let Some(ui) = ui_weak.upgrade() {
                let (px, py) = pixel_at(&ui, mx, my);
                if let Some((text, _)) = probe
                    .lock()
                    .ok()
                    .and_then(|g| g.as_ref().and_then(|p| p.at(px, py)))
                {
                    ui.set_probe_text(text.into());
                }
            }
        });
    }

    // Click: populate the JSON and report whether a point was hit (the .slint then
    // shows the popup).
    {
        let ui_weak = ui.as_weak();
        let probe = probe.clone();
        ui.on_point_click(move |mx, my| -> bool {
            if let Some(ui) = ui_weak.upgrade() {
                let (px, py) = pixel_at(&ui, mx, my);
                if let Some((text, json)) = probe
                    .lock()
                    .ok()
                    .and_then(|g| g.as_ref().and_then(|p| p.at(px, py)))
                {
                    ui.set_probe_text(text.into());
                    ui.set_point_json(json.into());
                    return true;
                }
            }
            false
        });
    }

    ui.on_copy(move |text| {
        if let Ok(mut cb) = arboard::Clipboard::new() {
            let _ = cb.set_text(text.to_string());
        }
    });

    // Flag debug builds so the UI can warn that rendering is slow.
    ui.set_debug_build(cfg!(debug_assertions));

    // If launched with CLI arguments, open pre-populated with the same options the
    // `generate` tool accepts — so a command emitted by the viewer's own "Submit"
    // button round-trips straight back into the viewer. With no args, the .slint
    // defaults (Turbulence 2D) stand.
    if std::env::args().len() > 1 {
        apply_opts(
            &ui,
            &simdnoise_generate::resolve(simdnoise_generate::Args::parse()),
        );
    }

    // First frame.
    refresh_anim_axis_model(&ui);
    last_dim.set(ui.get_dim());
    do_trigger();

    ui.run()
}

/// Axis labels for the animation dropdown (the active axes for the current `dim`).
fn refresh_anim_axis_model(ui: &AppWindow) {
    const NAMES: [&str; 6] = ["1 · x", "2 · y", "3 · z", "4 · w", "5 · u", "6 · v"];
    let dim = (ui.get_dim().clamp(1, 6)) as usize;
    let labels: Vec<SharedString> = NAMES[..dim]
        .iter()
        .map(|s| SharedString::from(*s))
        .collect();
    ui.set_anim_axis_model(ModelRc::new(VecModel::from(labels)));
    let idx = ui.get_anim_axis_idx().clamp(0, dim as i32 - 1);
    ui.set_anim_axis_idx(idx);
}

fn offset_axis(ui: &AppWindow, i: usize) -> f32 {
    match i {
        0 => ui.get_off0(),
        1 => ui.get_off1(),
        2 => ui.get_off2(),
        3 => ui.get_off3(),
        4 => ui.get_off4(),
        _ => ui.get_off5(),
    }
}

fn set_offset_axis(ui: &AppWindow, i: usize, v: f32) {
    match i {
        0 => ui.set_off0(v),
        1 => ui.set_off1(v),
        2 => ui.set_off2(v),
        3 => ui.set_off3(v),
        4 => ui.set_off4(v),
        _ => ui.set_off5(v),
    }
}

fn extent_axis(ui: &AppWindow, i: usize) -> f32 {
    let e = match i {
        0 => ui.get_ext0(),
        1 => ui.get_ext1(),
        2 => ui.get_ext2(),
        3 => ui.get_ext3(),
        4 => ui.get_ext4(),
        _ => ui.get_ext5(),
    };
    e.max(1) as f32
}

fn freq_axis(ui: &AppWindow, i: usize) -> f32 {
    match i {
        0 => ui.get_frq0(),
        1 => ui.get_frq1(),
        2 => ui.get_frq2(),
        3 => ui.get_frq3(),
        4 => ui.get_frq4(),
        _ => ui.get_frq5(),
    }
}

fn set_freq_axis(ui: &AppWindow, i: usize, v: f32) {
    match i {
        0 => ui.set_frq0(v),
        1 => ui.set_frq1(v),
        2 => ui.set_frq2(v),
        3 => ui.set_frq3(v),
        4 => ui.set_frq4(v),
        _ => ui.set_frq5(v),
    }
}

/// Map a cursor position inside the canvas TouchArea (logical px) to a pixel in the
/// displayed image, accounting for `image-fit: contain` letterboxing. Returns
/// (-1, -1) when outside the image.
fn pixel_at(ui: &AppWindow, mx: f32, my: f32) -> (i32, i32) {
    let aw = ui.get_area_w();
    let ah = ui.get_area_h();
    let iw = ui.get_img_w() as f32;
    let ih = ui.get_img_h() as f32;
    if iw <= 0.0 || ih <= 0.0 || aw <= 0.0 || ah <= 0.0 {
        return (-1, -1);
    }
    let scale = (aw / iw).min(ah / ih);
    let disp_w = iw * scale;
    let disp_h = ih * scale;
    let off_x = (aw - disp_w) / 2.0;
    let off_y = (ah - disp_h) / 2.0;
    let px = ((mx - off_x) / scale).floor() as i32;
    let py = ((my - off_y) / scale).floor() as i32;
    (px, py)
}

/// Reset the controls to their defaults, keeping the current Algorithm and
/// Dimensions (they define the "mode"; Reset restores everything within it).
fn apply_defaults(ui: &AppWindow) {
    ui.set_float_idx(0);
    ui.set_seed(1);
    ui.set_ext0(256);
    ui.set_ext1(256);
    ui.set_ext2(8);
    ui.set_ext3(4);
    ui.set_ext4(2);
    ui.set_ext5(2);
    ui.set_off0(0.0);
    ui.set_off1(0.0);
    ui.set_off2(0.0);
    ui.set_off3(0.0);
    ui.set_off4(0.0);
    ui.set_off5(0.0);
    ui.set_frq0(0.02);
    ui.set_frq1(0.02);
    ui.set_frq2(0.02);
    ui.set_frq3(0.02);
    ui.set_frq4(0.02);
    ui.set_frq5(0.02);
    ui.set_octaves(3.0);
    ui.set_lacunarity(0.5);
    ui.set_gain(2.0);
    ui.set_jitter(0.25);
    ui.set_dist_idx(0);
    ui.set_ret_idx(1);
    ui.set_ret2_idx(0);
    ui.set_index0(0);
    ui.set_index1(1);
    ui.set_scaling_idx(0);
    ui.set_scale_min(-1.0);
    ui.set_scale_max(1.0);
    ui.set_quant_idx(0);
    ui.set_colormap_idx(0);
    ui.set_engine_idx(0);
    ui.set_cross0(0.0);
    ui.set_cross1(0.0);
    ui.set_cross2(0.0);
    ui.set_anim_playing(false);
    ui.set_anim_axis_idx(0);
    ui.set_anim_drive_idx(0);
    ui.set_anim_speed(0.3);
    ui.set_anim_t(0.0); // offset default on axis x
}

/// Populate the UI from CLI options parsed with the `generate` tool's own parser,
/// so the viewer can be launched with the exact command the "Submit" button emits.
/// Engine and colormap have no CLI flag (they don't change the generated values),
/// so they keep their UI defaults.
fn apply_opts(ui: &AppWindow, o: &simdnoise_generate::Opts) {
    use simdnoise_generate::{
        Commands, DistanceArg, FloatKind, QuantArg, Return2Arg, ReturnArg, ScalingArg,
    };

    ui.set_dim(o.dim as i32);
    let e = o.extent;
    ui.set_ext0(e[0] as i32);
    ui.set_ext1(e[1] as i32);
    ui.set_ext2(e[2] as i32);
    ui.set_ext3(e[3] as i32);
    ui.set_ext4(e[4] as i32);
    ui.set_ext5(e[5] as i32);
    let o_ = o.offset;
    ui.set_off0(o_[0]);
    ui.set_off1(o_[1]);
    ui.set_off2(o_[2]);
    ui.set_off3(o_[3]);
    ui.set_off4(o_[4]);
    ui.set_off5(o_[5]);
    let f = o.freq;
    ui.set_frq0(f[0]);
    ui.set_frq1(f[1]);
    ui.set_frq2(f[2]);
    ui.set_frq3(f[3]);
    ui.set_frq4(f[4]);
    ui.set_frq5(f[5]);
    ui.set_seed(o.seed);
    ui.set_float_idx(match o.float {
        FloatKind::F32 => 0,
        FloatKind::F64 => 1,
    });
    ui.set_scaling_idx(match o.scaling {
        ScalingArg::Raw => 0,
        ScalingArg::Scaled => 1,
    });
    ui.set_scale_min(o.scale_min);
    ui.set_scale_max(o.scale_max);
    ui.set_quant_idx(match o.quant {
        QuantArg::None => 0,
        QuantArg::U8 => 1,
        QuantArg::U16 => 2,
        QuantArg::U32 => 3,
        QuantArg::I16 => 4,
        QuantArg::I32 => 5,
    });

    let dist_idx = |d: DistanceArg| match d {
        DistanceArg::Euclidean => 0,
        DistanceArg::Manhattan => 1,
        DistanceArg::Natural => 2,
    };
    match o.command {
        Commands::Gradient => ui.set_algo_idx(0),
        Commands::Fbm {
            lacunarity,
            gain,
            octaves,
        } => {
            ui.set_algo_idx(1);
            ui.set_lacunarity(lacunarity);
            ui.set_gain(gain);
            ui.set_octaves(octaves);
        }
        Commands::Ridge {
            lacunarity,
            gain,
            octaves,
        } => {
            ui.set_algo_idx(2);
            ui.set_lacunarity(lacunarity);
            ui.set_gain(gain);
            ui.set_octaves(octaves);
        }
        Commands::Turbulence {
            lacunarity,
            gain,
            octaves,
        } => {
            ui.set_algo_idx(3);
            ui.set_lacunarity(lacunarity);
            ui.set_gain(gain);
            ui.set_octaves(octaves);
        }
        Commands::Cellular {
            jitter,
            distance,
            return_type,
        } => {
            ui.set_algo_idx(4);
            ui.set_jitter(jitter);
            ui.set_dist_idx(dist_idx(distance));
            ui.set_ret_idx(match return_type {
                ReturnArg::CellValue => 0,
                ReturnArg::Distance => 1,
            });
        }
        Commands::Cellular2 {
            jitter,
            distance,
            return2,
            index0,
            index1,
        } => {
            ui.set_algo_idx(5);
            ui.set_jitter(jitter);
            ui.set_dist_idx(dist_idx(distance));
            ui.set_ret2_idx(match return2 {
                Return2Arg::Distance2 => 0,
                Return2Arg::Distance2Add => 1,
                Return2Arg::Distance2Sub => 2,
                Return2Arg::Distance2Mul => 3,
                Return2Arg::Distance2Div => 4,
            });
            ui.set_index0(index0 as i32);
            ui.set_index1(index1 as i32);
        }
    }
}

fn build_params(ui: &AppWindow, engines: &[Engine]) -> Params {
    let algo = Algo::from_index(ui.get_algo_idx());
    let dim = (ui.get_dim().clamp(1, 6)) as usize;
    let float = if ui.get_float_idx() == 0 {
        Float::F32
    } else {
        Float::F64
    };

    let extent = [
        ui.get_ext0().max(1) as usize,
        ui.get_ext1().max(1) as usize,
        ui.get_ext2().max(1) as usize,
        ui.get_ext3().max(1) as usize,
        ui.get_ext4().max(1) as usize,
        ui.get_ext5().max(1) as usize,
    ];
    let offset = [
        ui.get_off0(),
        ui.get_off1(),
        ui.get_off2(),
        ui.get_off3(),
        ui.get_off4(),
        ui.get_off5(),
    ];
    let freq = [
        ui.get_frq0(),
        ui.get_frq1(),
        ui.get_frq2(),
        ui.get_frq3(),
        ui.get_frq4(),
        ui.get_frq5(),
    ];

    let fractal = Fractal {
        octaves: ui.get_octaves(),
        lacunarity: ui.get_lacunarity(),
        gain: ui.get_gain(),
    };

    // Clamp cellular2 indices to the library's `validate` rules so a bad combo can
    // never panic the worker, and reflect the clamp back into the UI.
    let mut index0 = ui.get_index0().max(0) as usize;
    let mut index1 = ui.get_index1().max(0) as usize;
    index0 = index0.min(2);
    let index1_cap = if dim == 1 { 2 } else { 3 };
    index1 = index1.min(index1_cap);
    if index1 <= index0 {
        index1 = (index0 + 1).min(index1_cap);
        index0 = index1 - 1;
    }
    ui.set_index0(index0 as i32);
    ui.set_index1(index1 as i32);

    let cellular = Cellular {
        jitter: ui.get_jitter(),
        dist_fn: dist_from_index(ui.get_dist_idx()),
        return_type: ret_from_index(ui.get_ret_idx()),
        ret2: ret2_from_index(ui.get_ret2_idx()),
        index0,
        index1,
    };

    let engine = engines
        .get(ui.get_engine_idx().max(0) as usize)
        .copied()
        .unwrap_or(Engine::Auto);

    // Output mode. Non-auto engines are Raw-only; quantize has no f64 twin.
    let scale_min = ui.get_scale_min();
    let scale_max = ui.get_scale_max();
    let quant = int_from_index(ui.get_quant_idx());
    let out = if engine != Engine::Auto {
        OutMode::Raw
    } else if float == Float::F32 {
        match quant {
            Some(ty) => OutMode::Quant {
                ty,
                src_min: scale_min,
                src_max: scale_max,
            },
            None if ui.get_scaling_idx() == 1 => OutMode::Scaled {
                min: scale_min,
                max: scale_max,
            },
            None => OutMode::Raw,
        }
    } else if ui.get_scaling_idx() == 1 {
        OutMode::Scaled {
            min: scale_min,
            max: scale_max,
        }
    } else {
        OutMode::Raw
    };

    let cross = [
        ui.get_cross0().max(0.0).round() as usize,
        ui.get_cross1().max(0.0).round() as usize,
        ui.get_cross2().max(0.0).round() as usize,
    ];

    let colormap = if ui.get_colormap_idx() == 0 {
        Colormap::Grayscale
    } else {
        Colormap::Heatmap
    };

    Params {
        algo,
        dim,
        float,
        seed: ui.get_seed(),
        offset,
        extent,
        freq,
        fractal,
        cellular,
        out,
        engine,
        cross,
        colormap,
    }
}

/// Equivalent `generate` CLI command for the current settings. The `generate`
/// binary mirrors the viewer's full option surface, so the command reproduces the
/// same noise. Engine/colormap don't change the generated values (all engines
/// agree, the colormap is display-only), so they're omitted; a non-auto engine
/// adds one note.
fn generate_command(p: &Params) -> String {
    let mut parts: Vec<String> = vec!["cargo run --bin generate --".into()];

    parts.push(format!("--dim {}", p.dim));
    let ext_flags = [
        "--width", "--height", "--depth", "--time", "--ext5", "--ext6",
    ];
    for (i, flag) in ext_flags.iter().enumerate().take(p.dim) {
        parts.push(format!("{} {}", flag, p.extent[i]));
    }

    let off_flags = [
        "--offset-x",
        "--offset-y",
        "--offset-z",
        "--offset-w",
        "--offset-u",
        "--offset-v",
    ];
    for (i, flag) in off_flags.iter().enumerate().take(p.dim) {
        if p.offset[i] != 0.0 {
            parts.push(format!("{} {}", flag, p.offset[i]));
        }
    }

    parts.push(format!("--seed {}", p.seed));
    if p.float == Float::F64 {
        parts.push("--float f64".into());
    }

    // Frequency: uniform -> one --freq, otherwise per-axis flags.
    let f0 = p.freq[0];
    if p.freq[..p.dim].iter().all(|&f| (f - f0).abs() <= 1e-9) {
        parts.push(format!("--freq {f0}"));
    } else {
        let freq_flags = [
            "--freq-x", "--freq-y", "--freq-z", "--freq-w", "--freq-u", "--freq-v",
        ];
        for (i, flag) in freq_flags.iter().enumerate().take(p.dim) {
            parts.push(format!("{} {}", flag, p.freq[i]));
        }
    }

    // Output mode.
    match &p.out {
        OutMode::Raw => {}
        OutMode::Scaled { min, max } => {
            parts.push(format!(
                "--scaling scaled --scale-min {min} --scale-max {max}"
            ));
        }
        OutMode::Quant {
            ty,
            src_min,
            src_max,
        } => {
            let tok = match ty {
                IntType::U8 => "u8",
                IntType::U16 => "u16",
                IntType::U32 => "u32",
                IntType::I16 => "i16",
                IntType::I32 => "i32",
            };
            parts.push(format!(
                "--quantize {tok} --scale-min {src_min} --scale-max {src_max}"
            ));
        }
    }

    // Subcommand + its options.
    match p.algo {
        Algo::Gradient => parts.push("gradient".into()),
        Algo::Fbm | Algo::Ridge | Algo::Turbulence => {
            parts.push(
                match p.algo {
                    Algo::Fbm => "fbm",
                    Algo::Ridge => "ridge",
                    _ => "turbulence",
                }
                .into(),
            );
            parts.push(format!("--lacunarity {}", p.fractal.lacunarity));
            parts.push(format!("--gain {}", p.fractal.gain));
            parts.push(format!("--octaves {}", p.fractal.octaves));
        }
        Algo::Cellular => {
            parts.push("cellular".into());
            parts.push(format!("--jitter {}", p.cellular.jitter));
            parts.push(format!("--distance {}", dist_token(p.cellular.dist_fn)));
            parts.push(format!("--return {}", ret_token(p.cellular.return_type)));
        }
        Algo::Cellular2 => {
            parts.push("cellular2".into());
            parts.push(format!("--jitter {}", p.cellular.jitter));
            parts.push(format!("--distance {}", dist_token(p.cellular.dist_fn)));
            parts.push(format!("--return2 {}", ret2_token(p.cellular.ret2)));
            parts.push(format!("--index0 {}", p.cellular.index0));
            parts.push(format!("--index1 {}", p.cellular.index1));
        }
    }

    let mut out = parts.join(" ");
    if p.engine != Engine::Auto {
        out.push_str(&format!(
            "   # viewer used the {} engine (same values); example uses runtime auto",
            p.engine.label()
        ));
    }
    out
}

fn dist_token(d: CellDistanceFunction) -> &'static str {
    match d {
        CellDistanceFunction::Euclidean => "euclidean",
        CellDistanceFunction::Manhattan => "manhattan",
        CellDistanceFunction::Natural => "natural",
    }
}

fn ret_token(r: CellReturnType) -> &'static str {
    match r {
        CellReturnType::CellValue => "cellvalue",
        CellReturnType::Distance => "distance",
    }
}

fn ret2_token(r: Cell2ReturnType) -> &'static str {
    match r {
        Cell2ReturnType::Distance2 => "distance2",
        Cell2ReturnType::Distance2Add => "distance2add",
        Cell2ReturnType::Distance2Sub => "distance2sub",
        Cell2ReturnType::Distance2Mul => "distance2mul",
        Cell2ReturnType::Distance2Div => "distance2div",
    }
}

#[cfg(test)]
mod tests {
    use super::params::*;
    use super::{dispatch, render};

    fn each_algo() -> [Algo; 6] {
        [
            Algo::Gradient,
            Algo::Fbm,
            Algo::Ridge,
            Algo::Turbulence,
            Algo::Cellular,
            Algo::Cellular2,
        ]
    }

    /// 6D generation needs a large stack (issue #38), which the app's worker thread
    /// provides. Mirror that for the tests, and propagate any panic to fail the test.
    fn big_stack<F: FnOnce() + Send + 'static>(f: F) {
        std::thread::Builder::new()
            .stack_size(256 * 1024 * 1024)
            .spawn(f)
            .unwrap()
            .join()
            .unwrap();
    }

    /// Generate + render every algorithm at every dimension (f32 and f64) and
    /// confirm the frame is the right size with valid pixels and finite stats.
    #[test]
    fn full_matrix_generates_and_renders() {
        big_stack(|| {
            let mut buf = dispatch::Buffers::default();
            for algo in each_algo() {
                for dim in 1..=6usize {
                    for float in [Float::F32, Float::F64] {
                        let mut p = Params::preset();
                        p.algo = algo;
                        p.dim = dim;
                        p.float = float;
                        dispatch::generate(&p, &mut buf);
                        let r = render::to_image(&buf, &p);
                        assert_eq!(
                            r.pixels.len(),
                            (r.width * r.height) as usize,
                            "pixel count mismatch at dim {dim}"
                        );
                        // NEON f64 cellular is deliberately excluded (approx rsqrt -> NaN),
                        // and Auto selects NEON on aarch64, so skip the finite check there.
                        let known_nan = matches!(algo, Algo::Cellular | Algo::Cellular2)
                            && float == Float::F64
                            && cfg!(target_arch = "aarch64");
                        if !known_nan {
                            assert!(
                                r.probe.values.iter().all(|v| v.is_finite()),
                                "non-finite values at dim {dim}"
                            );
                        }
                    }
                }
            }
        });
    }

    /// Raw, Scaled and every quantize integer type run without panic (f32).
    #[test]
    fn output_modes() {
        big_stack(|| {
            let mut buf = dispatch::Buffers::default();
            let modes = [
                OutMode::Raw,
                OutMode::Scaled {
                    min: 0.0,
                    max: 255.0,
                },
                OutMode::Quant {
                    ty: IntType::U8,
                    src_min: -1.0,
                    src_max: 1.0,
                },
                OutMode::Quant {
                    ty: IntType::U16,
                    src_min: -1.0,
                    src_max: 1.0,
                },
                OutMode::Quant {
                    ty: IntType::U32,
                    src_min: -1.0,
                    src_max: 1.0,
                },
                OutMode::Quant {
                    ty: IntType::I16,
                    src_min: -1.0,
                    src_max: 1.0,
                },
                OutMode::Quant {
                    ty: IntType::I32,
                    src_min: -1.0,
                    src_max: 1.0,
                },
            ];
            for out in modes {
                let mut p = Params::preset();
                p.algo = Algo::Fbm;
                p.out = out;
                dispatch::generate(&p, &mut buf);
                let r = render::to_image(&buf, &p);
                assert_eq!(r.pixels.len(), (r.width * r.height) as usize);
            }
        });
    }

    /// The always-available scalar reference engine (Raw block path) works at
    /// every dimension for both float widths.
    #[test]
    fn scalar_engine() {
        big_stack(|| {
            let mut buf = dispatch::Buffers::default();
            for dim in 1..=6usize {
                for float in [Float::F32, Float::F64] {
                    let mut p = Params::preset();
                    p.algo = Algo::Cellular;
                    p.engine = Engine::Scalar;
                    p.dim = dim;
                    p.float = float;
                    dispatch::generate(&p, &mut buf);
                    let r = render::to_image(&buf, &p);
                    assert_eq!(r.pixels.len(), (r.width * r.height) as usize);
                }
            }
        });
    }

    /// Cross-section indices on a 6D block stay in bounds (no OOB panic).
    #[test]
    fn cross_sections_6d() {
        big_stack(|| {
            let mut buf = dispatch::Buffers::default();
            let mut p = Params::preset();
            p.algo = Algo::Gradient;
            p.dim = 6;
            p.cross = [1, 1, 1];
            dispatch::generate(&p, &mut buf);
            let _ = render::to_image(&buf, &p);
        });
    }

    #[test]
    fn generate_command_full_parity() {
        // A plain 2D FBM maps cleanly with no notes.
        let mut p = Params::preset();
        p.algo = Algo::Fbm;
        let cmd = super::generate_command(&p);
        assert!(cmd.starts_with("cargo run --bin generate --"));
        assert!(cmd.contains("--dim 2"));
        assert!(cmd.contains("--width 16"));
        assert!(cmd.contains("fbm"));
        assert!(cmd.contains("--octaves"));
        assert!(!cmd.contains('#'), "unexpected note: {cmd}");

        // Everything the viewer can do is representable, including 6D cellular2.
        let mut p = Params::preset();
        p.algo = Algo::Cellular2;
        p.dim = 6;
        p.cellular.index0 = 1;
        p.cellular.index1 = 3;
        let cmd = super::generate_command(&p);
        assert!(cmd.contains("--dim 6"));
        assert!(cmd.contains("--ext6"));
        assert!(cmd.contains("cellular2"));
        assert!(cmd.contains("--return2 distance2"));
        assert!(cmd.contains("--index0 1"));
        assert!(cmd.contains("--index1 3"));
        assert!(!cmd.contains('#'), "unexpected note: {cmd}");

        // Per-axis freq + quantize + f64 all appear.
        let mut p = Params::preset();
        p.dim = 3;
        p.freq = [0.01, 0.02, 0.03, 0.0, 0.0, 0.0];
        let cmd = super::generate_command(&p);
        assert!(cmd.contains("--freq-x 0.01"));
        assert!(cmd.contains("--freq-z 0.03"));

        let mut p = Params::preset();
        p.float = Float::F64;
        let cmd = super::generate_command(&p);
        assert!(cmd.contains("--float f64"));

        let mut p = Params::preset();
        p.out = OutMode::Quant {
            ty: IntType::U16,
            src_min: -1.0,
            src_max: 1.0,
        };
        let cmd = super::generate_command(&p);
        assert!(cmd.contains("--quantize u16"));
        assert!(cmd.contains("--scale-min -1"));
    }

    #[test]
    fn probe_point_lookup() {
        big_stack(|| {
            let mut buf = dispatch::Buffers::default();
            let mut p = Params::preset();
            p.algo = Algo::Gradient;
            p.dim = 2;
            dispatch::generate(&p, &mut buf);
            let r = render::to_image(&buf, &p);

            let hit = r.probe.at((r.width / 2) as i32, (r.height / 2) as i32);
            assert!(hit.is_some());
            let (text, json) = hit.unwrap();
            assert!(text.contains("value="));
            assert!(json.trim_start().starts_with('{'));
            assert!(json.contains("\"x\""));
            assert!(json.contains("\"value\""));
            // Position JSON must not carry image-wide metadata.
            assert!(!json.contains("algo"));

            // Out of bounds returns nothing.
            assert!(r.probe.at(-1, 0).is_none());
            assert!(r.probe.at(r.width as i32, 0).is_none());
        });
    }
}
