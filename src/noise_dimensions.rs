/// The size and placement of the block of noise to generate, shared by every
/// noise variant.
///
/// Noise is defined over a continuous, effectively infinite space; generating a
/// "block" means **sampling it on a regular grid**. This struct says how many
/// dimensions that grid has (`dim`), how many samples to take along each axis
/// (the *extents*: `extent_x`, `extent_y`, …), and where the grid starts in
/// noise space (the per-axis *offsets*: `offset_x`, `offset_y`, …). Shifting an
/// offset slides the sampling window across the field — i.e. it pans the
/// pattern; the step between adjacent samples is set by the frequency on each
/// `*Settings` type.
///
/// Axes are named by the coordinate letters `x, y, z, w, u, v` (axes 1–6), so
/// `offset_x`/`extent_x` describe the 1st axis, `offset_v`/`extent_v` the 6th.
/// Only the axes below `dim` are used (e.g. `dim = 2` uses the `x`/`y` pair).
///
/// You rarely build this by hand — `NoiseBuilder` fills it in for you.
#[derive(Copy, Clone)]
pub struct NoiseDimensions {
    /// Number of dimensions of the noise, 1–6. Determines which axes below are
    /// used (e.g. `dim = 2` uses `offset_x`/`offset_y` and `extent_x`/`extent_y`).
    pub dim: usize,
    /// Coordinate offset (start position in noise space) of the 1st axis (X).
    /// Adds to every sample's X coordinate, so changing it pans the pattern
    /// horizontally. Measured in grid steps: an offset of `1.0` shifts the
    /// window by one sample.
    pub offset_x: f32,
    /// Coordinate offset of the 2nd axis (Y); see [`offset_x`](Self::offset_x).
    pub offset_y: f32,
    /// Coordinate offset of the 3rd axis (Z); see [`offset_x`](Self::offset_x).
    pub offset_z: f32,
    /// Coordinate offset of the 4th axis (W); see [`offset_x`](Self::offset_x).
    pub offset_w: f32,
    /// Coordinate offset of the 5th axis (U); see [`offset_x`](Self::offset_x).
    pub offset_u: f32,
    /// Coordinate offset of the 6th axis (V); see [`offset_x`](Self::offset_x).
    pub offset_v: f32,
    /// Number of samples along the 1st axis (X), i.e. the grid width. The total
    /// number of values produced is the product of the extents of the active
    /// axes.
    pub extent_x: usize,
    /// Number of samples along the 2nd axis (Y), i.e. the grid height; see
    /// [`extent_x`](Self::extent_x).
    pub extent_y: usize,
    /// Number of samples along the 3rd axis (Z), i.e. the grid depth; see
    /// [`extent_x`](Self::extent_x).
    pub extent_z: usize,
    /// Number of samples along the 4th axis (W), often used as time for
    /// animation; see [`extent_x`](Self::extent_x).
    pub extent_w: usize,
    /// Number of samples along the 5th axis (U); see [`extent_x`](Self::extent_x).
    pub extent_u: usize,
    /// Number of samples along the 6th axis (V); see [`extent_x`](Self::extent_x).
    pub extent_v: usize,
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
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            offset_w: 0.0,
            offset_u: 0.0,
            offset_v: 0.0,
            extent_x: 1,
            extent_y: 1,
            extent_z: 1,
            extent_w: 1,
            extent_u: 1,
            extent_v: 1,
            min: 0.0,
            max: 1.0,
            seed: 1,
        }
    }
}
