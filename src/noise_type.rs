use super::{
    Cell2ReturnType, CellReturnType, Cellular2Settings, CellularSettings, DimensionalBeing,
    FbmSettings, GradientSettings, NoiseDimensions, RidgeSettings, TurbulenceSettings,
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
    fn dimensions(&self) -> NoiseDimensions {
        match self {
            NoiseType::Fbm(s) => s.dimensions(),
            NoiseType::Ridge(s) => s.dimensions(),
            NoiseType::Turbulence(s) => s.dimensions(),
            NoiseType::Gradient(s) => s.dimensions(),
            NoiseType::Cellular(s) => s.dimensions(),
            NoiseType::Cellular2(s) => s.dimensions(),
        }
    }
}

/// Sum of the per-octave amplitudes: `1 + gain + ... + gain^(n-1) + frac*gain^n`,
/// where `octaves = n + frac`. The fractional top octave contributes `frac` of
/// its amplitude `gain^n`, matching the fractional-octave kernels; an integer
/// `octaves` (frac == 0) reduces to `1 + gain + ... + gain^(octaves-1)`.
///
/// At least one octave always contributes (the base octave runs before the
/// octave loop), so an octave count below 1 is treated as 1.
fn amplitude_sum(octaves: f32, gain: f32) -> f32 {
    let oct = octaves.max(1.0);
    let n = oct.floor();
    let frac = oct - n;
    let mut amp = 1.0;
    let mut sum = 1.0;
    let mut k = 1.0;
    while k < n {
        amp *= gain;
        sum += amp;
        k += 1.0;
    }
    if frac > 0.0 {
        amp *= gain;
        sum += frac * amp;
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
    /// For cellular noise most returns have no simple closed form (they depend on
    /// the distance function, return type and jitter) and yield `None`, so callers
    /// fall back to the observed range; the exceptions are `CellValue` (`[-1, 1]`)
    /// and `Distance2Div` (`[0, 1]`), which are bounded regardless of jitter.
    pub fn theoretical_min_max(&self) -> Option<(f32, f32)> {
        match self {
            // A single simplex octave is normalized to [-1, 1].
            NoiseType::Gradient(_) => Some((-1.0, 1.0)),
            // Sum of signed octaves: `+/- sum(amp)`.
            NoiseType::Fbm(s) => {
                let a = amplitude_sum(s.octaves, s.gain);
                Some((-a, a))
            }
            // Sum of `|simplex| * amp` (>= 0) over every octave, 1D included (all
            // dimensions apply `amp` per octave).
            NoiseType::Turbulence(s) => Some((0.0, amplitude_sum(s.octaves, s.gain))),
            // Sum of `1 - |simplex| * amp`: the max is `octaves` (reached when every
            // `|simplex|` is 0), the min when each octave hits its amplitude.
            NoiseType::Ridge(s) => {
                let oct = s.octaves.max(1.0);
                Some((oct - amplitude_sum(s.octaves, s.gain), oct))
            }
            // Cellular has a closed-form range only for a couple of return types;
            // the rest are jitter/metric-dependent, so callers fall back to the
            // observed per-block range for those.
            NoiseType::Cellular(s) => match s.return_type {
                // CellValue = hash * 2^-31, independent of jitter and metric.
                CellReturnType::CellValue => Some((-1.0, 1.0)),
                CellReturnType::Distance => None,
            },
            NoiseType::Cellular2(s) => match s.return_type {
                // F1/F2 with ascending-sorted distances lands in [0, 1].
                Cell2ReturnType::Distance2Div => Some((0.0, 1.0)),
                _ => None,
            },
        }
    }
}
