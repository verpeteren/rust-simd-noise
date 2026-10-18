/// What a [`CellularSettings`](crate::CellularSettings) sample returns once the
/// nearest feature point has been found.
#[derive(Copy, Clone)]
pub enum CellReturnType {
    /// One random value per cell, identical across the whole cell — a flat mosaic
    /// of tiles (a Voronoi diagram).
    CellValue,
    /// The distance from the sample to the nearest feature point: smallest at the
    /// point itself and rising toward the cell borders, so each cell looks like a
    /// rounded bump with dark seams between cells.
    Distance,
}
