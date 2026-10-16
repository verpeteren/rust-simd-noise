// Only compiled with `--features scalar`: proves that with the feature enabled
// the public API routes through the scalar engine, so no CPU SIMD intrinsic is
// executed (which is what makes the crate usable under e.g. the Cranelift
// backend). We assert the public function returns bit-for-bit the same values
// as calling the scalar intrinsics module directly.
#![cfg(feature = "scalar")]

use simdnoise::{NoiseBuilder, Settings};

#[test]
fn public_api_uses_scalar_engine() {
    let nt = NoiseBuilder::gradient_2d(32, 16).with_seed(1).wrap();
    let via_public = simdnoise::get_2d_noise(&nt).0;
    let via_scalar =
        unsafe { simdnoise::intrinsics::scalar::get_2d_noise::<simdeez::scalar::Scalar>(&nt).0 };
    assert_eq!(via_public, via_scalar);
}
