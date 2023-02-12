#[derive(Copy, Clone)]
/// Determines what final value is returned for the cell2 noise
pub enum Cell2ReturnType {
    Distance2,
    Distance2Add,
    Distance2Sub,
    Distance2Mul,
    Distance2Div,
}
