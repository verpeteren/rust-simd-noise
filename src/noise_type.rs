use super::{
    Cellular2Settings, CellularSettings, DimensionalBeing, FbmSettings, GradientSettings,
    NoiseDimensions, RidgeSettings, TurbulenceSettings,
};

/// Specifies what type of noise to generate and contains any relevant settings.
#[derive(Copy, Clone)]
pub enum NoiseType {
    Fbm(FbmSettings),
    Ridge(RidgeSettings),
    Turbulence(TurbulenceSettings),
    Gradient(GradientSettings),
    Cellular(CellularSettings),
    Cellular2(Cellular2Settings),
}

impl DimensionalBeing for NoiseType {
    fn get_dimensions(&self) -> NoiseDimensions {
        match self {
            NoiseType::Fbm(s) => s.get_dimensions(),
            NoiseType::Ridge(s) => s.get_dimensions(),
            NoiseType::Turbulence(s) => s.get_dimensions(),
            NoiseType::Gradient(s) => s.get_dimensions(),
            NoiseType::Cellular(s) => s.get_dimensions(),
            NoiseType::Cellular2(s) => s.get_dimensions(),
        }
    }
}

/// Sum of the per-octave amplitudes: `1 + gain + gain^2 + ... + gain^(octaves-1)`.
///
/// At least one octave always contributes (the base octave runs before the
/// `1..octaves` loop), so a 0 octave count is treated as 1.
fn amplitude_sum(octaves: u8, gain: f32) -> f32 {
    let mut amp = 1.0;
    let mut sum = 1.0;
    for _ in 1..octaves.max(1) {
        amp *= gain;
        sum += amp;
    }
    sum
}

impl NoiseType {
    /// The offset-independent theoretical range `(min, max)` of the **unscaled**
    /// noise, derived in closed form from the octave/gain settings rather than
    /// from the samples actually produced.
    ///
    /// [`Settings::generate_scaled`](crate::Settings::generate_scaled) uses this
    /// so that scaling is identical regardless of the offset/size of the block,
    /// which is what makes adjacent blocks line up seamlessly. The bound is not
    /// tight (the noise rarely reaches the extremes), so scaled output has a
    /// little less contrast than scaling to the per-block observed range would
    /// give — that is the price of a consistent mapping across blocks.
    ///
    /// Returns `None` for cellular noise, whose range has no simple closed form
    /// (it depends on the distance function, return type and jitter); callers
    /// fall back to the observed range for those.
    pub fn theoretical_min_max(&self) -> Option<(f32, f32)> {
        match self {
            // A single simplex octave is normalized to [-1, 1].
            NoiseType::Gradient(_) => Some((-1.0, 1.0)),
            // Sum of signed octaves: `+/- sum(amp)`.
            NoiseType::Fbm(s) => {
                let a = amplitude_sum(s.octaves, s.gain);
                Some((-a, a))
            }
            // Sum of `|simplex| * amp` (>= 0). The 1D path does not apply `amp`,
            // so each of its `octaves` terms is just in [0, 1].
            NoiseType::Turbulence(s) => {
                if self.get_dimensions().dim == 1 {
                    Some((0.0, s.octaves.max(1) as f32))
                } else {
                    Some((0.0, amplitude_sum(s.octaves, s.gain)))
                }
            }
            // Sum of `1 - |simplex| * amp`; max when every `|simplex|` is 0. The
            // 1D path does not apply `amp`, so each term is in [0, 1].
            NoiseType::Ridge(s) => {
                let oct = s.octaves.max(1) as f32;
                if self.get_dimensions().dim == 1 {
                    Some((0.0, oct))
                } else {
                    Some((oct - amplitude_sum(s.octaves, s.gain), oct))
                }
            }
            NoiseType::Cellular(_) | NoiseType::Cellular2(_) => None,
        }
    }
}
