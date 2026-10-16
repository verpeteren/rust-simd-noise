use simdeez::prelude::*;

pub struct Hash3d<S: Simd> {
    // Masks guiding dimension selection
    pub l8: S::Vf64,
    pub l4: S::Vf64,
    pub h12_or_14: S::Vf64,

    // Signs for the selected dimensions
    pub h1: S::Vf64,
    pub h2: S::Vf64,
}

impl<S> Hash3d<S>
where
    S: Simd,
{
    #[allow(dead_code)]
    pub fn new(l8: S::Vf64, l4: S::Vf64, h12_or_14: S::Vf64, h1: S::Vf64, h2: S::Vf64) -> Self {
        Self {
            l8,
            l4,
            h12_or_14,
            h1,
            h2,
        }
    }
}
/// Compute hash values used by `grad3d` and `grad3d_dot`
///
/// 64-bit port of the 32-bit `hash3d` bit-trick. Two things differ from a naive
/// copy of the 32-bit code (which is why this was left unimplemented):
/// - the dimension-selection fields are **bitmasks**, so the comparison results
///   are reinterpreted with `bitcast_f64` (NOT `cast_f64`, which would numerically
///   turn the all-ones mask into `-1.0` and violate simdeez's `blendv`/`&` contract);
/// - the sign fields move a hash bit into the float sign bit, which for f64 is
///   bit 63 (shift 63), and for the `& 2` case bit 1 → bit 63 (shift 62) — the
///   32-bit code used 31/30 for f32's bit-31 sign.
///
/// Correctness is cross-checked at runtime by the `debug_assert_eq!`s in
/// `grad3d`/`grad3d_dot` (gradient has exactly two ±1 axes; dot matches).
#[inline(always)]
pub fn hash3d<S: Simd>(seed: i64, i: S::Vi64, j: S::Vi64, k: S::Vi64) -> Hash3d<S> {
    let mut hash = i ^ S::Vi64::set1(seed);
    hash = j ^ hash;
    hash = k ^ hash;
    hash = ((hash * hash) * S::Vi64::set1(60493)) * hash;
    hash = (hash >> 13) ^ hash;
    let hasha13 = hash & S::Vi64::set1(13);
    Hash3d::new(
        hasha13.cmp_lt(S::Vi64::set1(8)).bitcast_f64(),
        hasha13.cmp_lt(S::Vi64::set1(2)).bitcast_f64(),
        hasha13.cmp_eq(S::Vi64::set1(12)).bitcast_f64(),
        (hash << 63).bitcast_f64(),
        ((hash & S::Vi64::set1(2)) << 62).bitcast_f64(),
    )
}
