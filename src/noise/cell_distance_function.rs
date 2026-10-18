/// How the distance between a sample and a cell's feature point is measured,
/// which sets the **shape of the cells** in
/// [`CellularSettings`](crate::CellularSettings) and
/// [`Cellular2Settings`](crate::Cellular2Settings).
#[derive(Copy, Clone)]
pub enum CellDistanceFunction {
    /// Straight-line (Euclidean / L2) distance. Gives round, bubble-like cells.
    /// The usual default.
    Euclidean,
    /// Sum of the per-axis distances (Manhattan / "taxicab" / L1). Gives
    /// diamond-shaped, axis-aligned cells with straight edges.
    Manhattan,
    /// A blend of the Euclidean and Manhattan metrics, giving cells with a more
    /// irregular, organic outline than either on its own.
    Natural,
}
