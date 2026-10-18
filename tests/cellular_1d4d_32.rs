//! Behavioral sanity for 1D/4D cellular **f32** noise.
//!
//! The golden-fixture suite that lived here (exact per-engine `.bin` comparisons)
//! was removed. This keeps a value-level invariant check on the scalar reference
//! engine: cellular `Distance` output must be finite and non-negative. That
//! catches gross kernel bugs (NaN/inf, a wrong metric) without pinning exact
//! bytes the way a golden fixture does.

use simdnoise::intrinsics::scalar;
use simdnoise::{
    CellDistanceFunction, CellReturnType, CellularSettings, NoiseDimensions, Settings,
};

fn dims(dim: usize, w: usize, h: usize, d: usize, t: usize) -> NoiseDimensions {
    let mut nd = NoiseDimensions::default(dim);
    nd.width = w;
    nd.height = h;
    nd.depth = d;
    nd.time = t;
    nd
}

#[test]
fn cellular_1d4d_32_distance_sane() {
    for df in [
        CellDistanceFunction::Euclidean,
        CellDistanceFunction::Manhattan,
        CellDistanceFunction::Natural,
    ] {
        for (dim, w, h, d, t) in [(1usize, 64, 1, 1, 1), (4, 16, 8, 4, 4)] {
            let nt = CellularSettings::default(dims(dim, w, h, d, t))
                .with_seed(1337)
                .with_distance_function(df)
                .with_return_type(CellReturnType::Distance)
                .wrap();
            // Forced scalar engine: available on every target and free of the
            // NEON f64 rsqrt NaN caveat, so this runs everywhere.
            let noise = unsafe {
                if dim == 1 {
                    scalar::get_1d_noise::<simdeez::scalar::Scalar>(&nt).0
                } else {
                    scalar::get_4d_noise::<simdeez::scalar::Scalar>(&nt).0
                }
            };
            assert!(noise.iter().all(|v| v.is_finite()), "finite dim={}", dim);
            assert!(noise.iter().all(|v| *v >= 0.0), "distance>=0 dim={}", dim);
        }
    }
}
