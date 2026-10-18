//! Background generation worker: one long-lived thread with a large stack (6D
//! cellular needs it, issue #38). The UI posts `Job`s; the worker coalesces to the
//! newest, generates + renders off the UI thread, and posts the image back via
//! `invoke_from_event_loop`, dropping results superseded by newer input.

use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use slint::{Image, SharedPixelBuffer};

use crate::params::Params;
use crate::render::Probe;
use crate::{dispatch, render, AppWindow};

pub struct Job {
    pub id: u64,
    pub params: Params,
}

/// Shared "newest job id" so both the worker loop and the event-loop callback can
/// discard stale work.
pub type Latest = Arc<AtomicU64>;

/// The per-pixel probe for the frame currently on screen, read by hover/click.
pub type ProbeCell = Arc<Mutex<Option<Probe>>>;

/// The RGB bytes of the frame currently on screen (width, height, rgb8), for Save.
pub type FrameCell = Arc<Mutex<Option<(u32, u32, Vec<u8>)>>>;

pub fn spawn(
    ui: slint::Weak<AppWindow>,
    latest: Latest,
    probe: ProbeCell,
    frame: FrameCell,
) -> Sender<Job> {
    let (tx, rx) = channel::<Job>();
    std::thread::Builder::new()
        .name("noise-worker".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || worker_loop(rx, ui, latest, probe, frame))
        .expect("failed to spawn noise worker thread");
    tx
}

fn worker_loop(
    rx: Receiver<Job>,
    ui: slint::Weak<AppWindow>,
    latest: Latest,
    probe: ProbeCell,
    frame: FrameCell,
) {
    // Result cache: parameter combinations that were slow to compute are kept so
    // flipping back to them (e.g. undoing a change) is instant. Bounded in both
    // entry count and per-entry size so it can't grow without limit.
    const CACHE_CAP: usize = 6;
    const CACHE_MAX_PIXELS: usize = 1_500_000;
    const SLOW_MS: u128 = 80;
    let mut cache: HashMap<Vec<u64>, render::Rendered> = HashMap::new();
    let mut order: VecDeque<Vec<u64>> = VecDeque::new();

    let mut buf = dispatch::Buffers::default();
    while let Ok(mut job) = rx.recv() {
        // Coalesce: skip to the most recently queued job so a burst of UI changes
        // collapses to only the latest (the superseded ones are never computed).
        while let Ok(newer) = rx.try_recv() {
            job = newer;
        }
        if job.id < latest.load(Ordering::SeqCst) {
            continue;
        }

        let t_job = Instant::now();
        let key = job.params.cache_key();
        let rendered = if let Some(hit) = cache.get(&key) {
            hit.clone()
        } else {
            let t0 = Instant::now();
            dispatch::generate(&job.params, &mut buf);
            let r = render::to_image(&buf, &job.params);
            // Cache only jobs that were expensive and aren't enormous.
            if t0.elapsed().as_millis() >= SLOW_MS && r.pixels.len() <= CACHE_MAX_PIXELS {
                cache.insert(key.clone(), r.clone());
                order.push_back(key);
                while order.len() > CACHE_CAP {
                    if let Some(old) = order.pop_front() {
                        cache.remove(&old);
                    }
                }
            }
            r
        };
        // Wall time to produce this frame (near-zero on a cache hit), for the
        // "can't keep up" affordance while animating.
        let job_ms = t_job.elapsed().as_millis().min(i32::MAX as u128) as i32;

        let id = job.id;
        let ui = ui.clone();
        let latest = latest.clone();
        let probe = probe.clone();
        let frame = frame.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if id < latest.load(Ordering::SeqCst) {
                return;
            }
            if let Some(ui) = ui.upgrade() {
                // slint::Image is not Send, so build it here on the UI thread.
                let mut pb = SharedPixelBuffer::new(rendered.width, rendered.height);
                pb.make_mut_slice().copy_from_slice(&rendered.pixels);
                // Keep the RGB bytes for the Save button.
                let mut rgb = Vec::with_capacity(rendered.pixels.len() * 3);
                for px in &rendered.pixels {
                    rgb.push(px.r);
                    rgb.push(px.g);
                    rgb.push(px.b);
                }
                // Update the probe/frame first so the `changed canvas` re-probe
                // (triggered by set_canvas below) reads this frame's data.
                if let Ok(mut guard) = probe.lock() {
                    *guard = Some(rendered.probe);
                }
                if let Ok(mut guard) = frame.lock() {
                    *guard = Some((rendered.width, rendered.height, rgb));
                }
                ui.set_img_w(rendered.width as i32);
                ui.set_img_h(rendered.height as i32);
                ui.set_last_render_ms(job_ms);
                ui.set_busy(false);
                ui.set_canvas(Image::from_rgb8(pb));
            }
        });
    }
}
