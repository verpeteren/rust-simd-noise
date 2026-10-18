/// The size and placement of the block of noise to generate, shared by every
/// noise variant.
///
/// Noise is defined over a continuous, effectively infinite space; generating a
/// "block" means **sampling it on a regular grid**. This struct says how many
/// dimensions that grid has (`dim`), how many samples to take along each axis
/// (the *extents*: `width`, `height`, …), and where the grid starts in noise
/// space (the per-axis *offsets*: `x`, `y`, …). Shifting an offset slides the
/// sampling window across the field — i.e. it pans the pattern; the step between
/// adjacent samples is set by the frequency on each `*Settings` type.
///
/// You rarely build this by hand — `NoiseBuilder` fills it in for you.
#[derive(Copy, Clone)]
pub struct NoiseDimensions {
    /// Number of dimensions of the noise, 1–6. Determines which axes below are
    /// used (e.g. `dim = 2` uses `x`/`y` and `width`/`height`).
    pub dim: usize,
    /// Coordinate offset (start position in noise space) of the 1st axis. Adds to
    /// every sample's X coordinate, so changing it pans the pattern horizontally.
    /// Measured in grid steps: an offset of `1.0` shifts the window by one sample.
    pub x: f32,
    /// Coordinate offset of the 2nd axis (Y); see [`x`](Self::x).
    pub y: f32,
    /// Coordinate offset of the 3rd axis (Z); see [`x`](Self::x).
    pub z: f32,
    /// Coordinate offset of the 4th axis (W); see [`x`](Self::x).
    pub w: f32,
    /// Coordinate offsets for the 5th and 6th axes. Axes 1-4 use `x`/`y`/`z`/`w`;
    /// these extend the same idea to 5D/6D noise.
    pub offset5: f32,
    pub offset6: f32,
    /// Number of samples along the 1st axis (the grid width). The total number of
    /// values produced is the product of the extents of the active axes.
    pub width: usize,
    /// Number of samples along the 2nd axis (grid height); see [`width`](Self::width).
    pub height: usize,
    /// Number of samples along the 3rd axis (grid depth); see [`width`](Self::width).
    pub depth: usize,
    /// Number of samples along the 4th axis (often used as time, for animation);
    /// see [`width`](Self::width).
    pub time: usize,
    /// Block extents for the 5th and 6th axes. Axes 1-4 use
    /// `width`/`height`/`depth`/`time`; these extend them to 5D/6D.
    pub extent5: usize,
    pub extent6: usize,
    /// Lower bound of the output range, used only by the `generate_scaled*` paths,
    /// which remap the raw noise so its minimum becomes `min`.
    pub min: f32,
    /// Upper bound of the output range for `generate_scaled*`; see [`min`](Self::min).
    pub max: f32,
    /// Seed for the pseudo-random gradients/cells. Changing it gives a completely
    /// different-looking field with the same statistical character; the same seed
    /// always reproduces the same noise.
    pub seed: i32,
}

impl NoiseDimensions {
    pub fn default(d: usize) -> NoiseDimensions {
        if !(1..=6).contains(&d) {
            panic!("dimension invalid: must be between 1 and 6");
        }
        NoiseDimensions {
            dim: d,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
            offset5: 0.0,
            offset6: 0.0,
            width: 1,
            height: 1,
            depth: 1,
            time: 1,
            extent5: 1,
            extent6: 1,
            min: 0.0,
            max: 1.0,
            seed: 1,
        }
    }
}
