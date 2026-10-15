// Reproduction of issue #38: stack overflow with 3D noise on AVX2.
//
// Root cause: `#[inline(always)]` on the large 3D/4D noise compute functions
// collapses the whole computation into one giant stack frame. Under AVX2
// (32-byte vectors), in an unoptimized build, that single frame exceeds a
// memory page / a small stack, which trips Windows' __chkstk guard (and
// overflows small stacks in general).
//
// On Linux `main` has an 8 MiB stack, so we reproduce the Windows (1 MiB
// main-thread) / Rust test-thread (~2 MiB) condition by running each call in a
// thread with a 1 MiB stack.
//
//   cargo run --example repro_issue38            # debug
//   cargo run --release --example repro_issue38  # release
use simdnoise::{NoiseBuilder, Settings};

// A stack overflow aborts the whole process (it is not catchable via `join`),
// so when running every case in one process the first overflow stops the rest.
// Pass a case name as an argument to run exactly one case in its own process.
fn run(name: &str, f: fn()) {
    let handle = std::thread::Builder::new()
        .stack_size(1024 * 1024) // 1 MiB, like the Windows main thread
        .spawn(f)
        .unwrap();
    handle.join().unwrap();
    println!("  ok     {name}");
}

fn cases() -> Vec<(&'static str, fn())> {
    vec![
        ("gradient_3d", || {
            NoiseBuilder::gradient_3d(1, 1, 1).generate();
        }),
        ("fbm_3d", || {
            NoiseBuilder::fbm_3d(1, 1, 1).generate();
        }),
        ("ridge_3d", || {
            NoiseBuilder::ridge_3d(1, 1, 1).generate();
        }),
        ("turbulence_3d", || {
            NoiseBuilder::turbulence_3d(1, 1, 1).generate();
        }),
        ("cellular_3d", || {
            NoiseBuilder::cellular_3d(1, 1, 1).generate();
        }),
        ("cellular2_3d", || {
            NoiseBuilder::cellular2_3d(1, 1, 1).generate();
        }),
        // 4D simplex is the largest frame of all.
        ("gradient_4d", || {
            NoiseBuilder::gradient_4d(1, 1, 1, 1).generate();
        }),
    ]
}

fn main() {
    let all = cases();
    match std::env::args().nth(1) {
        Some(sel) => {
            let (name, f) = all
                .into_iter()
                .find(|(n, _)| *n == sel)
                .unwrap_or_else(|| panic!("unknown case '{}'", sel));
            run(name, f);
        }
        None => {
            println!("3D/4D noise on a 1 MiB stack:");
            for (name, f) in all {
                run(name, f);
            }
            println!("done");
        }
    }
}
