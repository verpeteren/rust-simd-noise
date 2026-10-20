//! Guards the zero-allocation property of the typed emplace path: filling a
//! caller-provided integer slice must not allocate an output-sized f32 scratch.
//!
//! Uses a counting global allocator (which is why this lives in its own test
//! binary) and measures bytes allocated across a single `fill_range`
//! call, after the output buffer is already allocated.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use simdnoise::{NoiseBuilder, Settings};

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATED.fetch_add(layout.size(), Ordering::Relaxed);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static GLOBAL: Counting = Counting;

#[test]
fn typed_emplace_does_not_allocate_output_sized_scratch() {
    let (w, h) = (256usize, 256usize);
    // Allocate the output up front, outside the measured window.
    let mut out = vec![0u16; w * h];

    let before = ALLOCATED.load(Ordering::Relaxed);
    NoiseBuilder::fbm_2d(w, h)
        .with_seed(1)
        .with_freq_2d(0.02, 0.02)
        .fill_range(-1.0, 1.0, &mut out);
    let delta = ALLOCATED.load(Ordering::Relaxed) - before;

    // A fused writer only touches the tiny per-call coordinate scratch, never an
    // output-sized f32 buffer. The scratch-based implementation would allocate at
    // least w*h*4 bytes here.
    let output_f32_bytes = w * h * std::mem::size_of::<f32>();
    assert!(
        delta < output_f32_bytes,
        "typed emplace allocated {} bytes; expected far less than {} \
         (an output-sized f32 scratch would mean the fused path regressed)",
        delta,
        output_f32_bytes
    );
}
