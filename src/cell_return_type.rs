#[derive(Copy, Clone)]
/// Determines what final value is returned for the cell noise
pub enum CellReturnType {
    /// Will return solid colors in each cell
    CellValue,
    /// Color will be a gradient as you approach edge of cell
    Distance,
}
