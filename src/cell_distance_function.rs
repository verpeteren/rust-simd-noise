#[derive(Copy, Clone)]
/// The function to use to compute distance between cells
pub enum CellDistanceFunction {
    /// The actual straight line distance
    Euclidean,
    /// Sum of the X and Y distances
    Manhattan,
    /// Combines Manhattan and Euclidean
    Natural,
}
