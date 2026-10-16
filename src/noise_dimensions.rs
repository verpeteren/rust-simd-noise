#[derive(Copy, Clone)]
pub struct NoiseDimensions {
    pub dim: usize,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
    /// Coordinate offsets for the 5th and 6th axes. Axes 1-4 use `x`/`y`/`z`/`w`;
    /// these extend the same idea to 5D/6D noise.
    pub offset5: f32,
    pub offset6: f32,
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub time: usize,
    /// Block extents for the 5th and 6th axes. Axes 1-4 use
    /// `width`/`height`/`depth`/`time`; these extend them to 5D/6D.
    pub extent5: usize,
    pub extent6: usize,
    pub min: f32,
    pub max: f32,
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
