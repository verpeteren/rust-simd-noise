/// How [`Cellular2Settings`](crate::Cellular2Settings) combines the distances to
/// its two chosen feature points into the final value.
///
/// Write **F1** for the distance to the `index0`-th nearest feature point and
/// **F2** for the `index1`-th nearest (by default the 1st- and 2nd-nearest, so
/// F1 ≤ F2). [`Distance2Sub`](Self::Distance2Sub) is the classic Worley
/// cell-edge / crack pattern.
#[derive(Copy, Clone)]
pub enum Cell2ReturnType {
    /// Just **F2** — the distance to the farther of the two chosen points. Rounded
    /// cells, a bit larger and smoother than single-point `Distance`.
    Distance2,
    /// **F1 + F2**. Soft, rounded mounds.
    Distance2Add,
    /// **F1 − F2** (≤ 0, mirror of the textbook F2 − F1): near zero right along the
    /// cell borders and more negative toward cell centres, which draws the crisp
    /// cell edges / crack network Worley noise is known for.
    Distance2Sub,
    /// **F1 × F2**.
    Distance2Mul,
    /// **F1 ÷ F2** (≤ 1).
    Distance2Div,
}
