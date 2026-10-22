//! Compile-time strategies for cellular noise configuration (issue #48).
//!
//! The cellular kernels in `cell_32`/`cell_64`
//! and `cell2_32`/`cell2_64` used to take
//! the distance function and the return type as runtime `enum` values and
//! `match` on them on every sample (for 3D cellular and both cellular2
//! functions, on every one of the 9/27 inner-loop iterations). That branch is
//! fixed for an entire generated block, so it is pure overhead and blocks
//! optimization.
//!
//! These zero-sized marker types + traits turn that configuration into
//! compile-time type parameters. A single `match` at the call boundary (see the
//! `Sample32`/`Sample64` impls and the `cellular!` intrinsics macro) selects the
//! monomorphized instantiation once per SIMD-vector batch; the inner loops then
//! contain no configuration branch at all.
//!
//! The traits are written once, generic over simdeez's shared [`SimdFloat`]
//! supertrait, so the same markers serve both the f32 (`S::Vf32`) and f64
//! (`S::Vf64`) kernels. They are `pub` only because the `noise::cell_*` kernels
//! that take them are themselves public (like every other `noise::*` kernel);
//! the configuration-facing public surface stays the `CellDistanceFunction` /
//! `CellReturnType` / `Cell2ReturnType` enums and the builder.

use simdeez::prelude::*;

// ---------------------------------------------------------------------------
// Distance function (shared by cellular and cellular2)
// ---------------------------------------------------------------------------

/// Computes the per-cell distance metric from the coordinate deltas.
pub trait CellDistance {
    fn new_distance_1d<V: SimdFloat>(xd: V) -> V;
    fn new_distance_2d<V: SimdFloat>(xd: V, yd: V) -> V;
    fn new_distance_3d<V: SimdFloat>(xd: V, yd: V, zd: V) -> V;
    fn new_distance_4d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V) -> V;
    fn new_distance_5d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V) -> V;
    fn new_distance_6d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V, td: V) -> V;
}

pub struct Euclidean;
pub struct Manhattan;
pub struct Natural;

impl CellDistance for Euclidean {
    #[inline(always)]
    fn new_distance_1d<V: SimdFloat>(xd: V) -> V {
        xd * xd
    }
    #[inline(always)]
    fn new_distance_2d<V: SimdFloat>(xd: V, yd: V) -> V {
        (xd * xd) + (yd * yd)
    }
    #[inline(always)]
    fn new_distance_3d<V: SimdFloat>(xd: V, yd: V, zd: V) -> V {
        (xd * xd) + ((yd * yd) + (zd * zd))
    }
    #[inline(always)]
    fn new_distance_4d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V) -> V {
        (xd * xd) + ((yd * yd) + ((zd * zd) + (wd * wd)))
    }
    #[inline(always)]
    fn new_distance_5d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V) -> V {
        (xd * xd) + ((yd * yd) + ((zd * zd) + ((wd * wd) + (ud * ud))))
    }
    #[inline(always)]
    fn new_distance_6d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V, td: V) -> V {
        (xd * xd) + ((yd * yd) + ((zd * zd) + ((wd * wd) + ((ud * ud) + (td * td)))))
    }
}

impl CellDistance for Manhattan {
    #[inline(always)]
    fn new_distance_1d<V: SimdFloat>(xd: V) -> V {
        xd.abs()
    }
    #[inline(always)]
    fn new_distance_2d<V: SimdFloat>(xd: V, yd: V) -> V {
        xd.abs() + yd.abs()
    }
    #[inline(always)]
    fn new_distance_3d<V: SimdFloat>(xd: V, yd: V, zd: V) -> V {
        (xd.abs() + yd.abs()) + zd.abs()
    }
    #[inline(always)]
    fn new_distance_4d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V) -> V {
        (xd.abs() + yd.abs()) + (zd.abs() + wd.abs())
    }
    #[inline(always)]
    fn new_distance_5d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V) -> V {
        ((xd.abs() + yd.abs()) + (zd.abs() + wd.abs())) + ud.abs()
    }
    #[inline(always)]
    fn new_distance_6d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V, td: V) -> V {
        ((xd.abs() + yd.abs()) + (zd.abs() + wd.abs())) + (ud.abs() + td.abs())
    }
}

impl CellDistance for Natural {
    #[inline(always)]
    fn new_distance_1d<V: SimdFloat>(xd: V) -> V {
        (xd * xd) + xd.abs()
    }
    #[inline(always)]
    fn new_distance_2d<V: SimdFloat>(xd: V, yd: V) -> V {
        let euc = (xd * xd) + (yd * yd);
        let man = xd.abs() + yd.abs();
        euc + man
    }
    #[inline(always)]
    fn new_distance_3d<V: SimdFloat>(xd: V, yd: V, zd: V) -> V {
        let euc = (xd * xd) + ((yd * yd) + (zd * zd));
        let man = (xd.abs() + yd.abs()) + zd.abs();
        euc + man
    }
    #[inline(always)]
    fn new_distance_4d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V) -> V {
        let euc = (xd * xd) + ((yd * yd) + ((zd * zd) + (wd * wd)));
        let man = (xd.abs() + yd.abs()) + (zd.abs() + wd.abs());
        euc + man
    }
    #[inline(always)]
    fn new_distance_5d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V) -> V {
        let euc = (xd * xd) + ((yd * yd) + ((zd * zd) + ((wd * wd) + (ud * ud))));
        let man = ((xd.abs() + yd.abs()) + (zd.abs() + wd.abs())) + ud.abs();
        euc + man
    }
    #[inline(always)]
    fn new_distance_6d<V: SimdFloat>(xd: V, yd: V, zd: V, wd: V, ud: V, td: V) -> V {
        let euc = (xd * xd) + ((yd * yd) + ((zd * zd) + ((wd * wd) + ((ud * ud) + (td * td)))));
        let man = ((xd.abs() + yd.abs()) + (zd.abs() + wd.abs())) + (ud.abs() + td.abs());
        euc + man
    }
}

// ---------------------------------------------------------------------------
// Cellular return type (CellReturnType)
//
// An accumulator strategy so each monomorphization does exactly the work it did
// before: the `Distance` path keeps only the running minimum and never touches
// cell values, while `CellValue` also tracks the nearest cell's value. The
// candidate cell value is passed as a lazy closure so the `Distance`
// monomorphization never evaluates (and the compiler drops) the hash multiply.
// ---------------------------------------------------------------------------

/// The sentinel the running minimum distance starts from (matches the previous
/// `set1(999999.0)`; representable exactly in f32 and f64).
#[inline(always)]
fn distance_sentinel<V: SimdFloat>() -> V {
    V::from_f64(999_999.0)
}

pub trait CellReturn<V: SimdFloat> {
    type Acc: Copy;
    fn init() -> Self::Acc;
    fn accumulate(acc: &mut Self::Acc, new_distance: V, new_cell_value: impl FnOnce() -> V);
    fn finish(acc: Self::Acc) -> V;
}

pub struct Distance;
pub struct CellValue;

impl<V: SimdFloat> CellReturn<V> for Distance {
    type Acc = V;
    #[inline(always)]
    fn init() -> V {
        distance_sentinel::<V>()
    }
    #[inline(always)]
    fn accumulate(acc: &mut V, new_distance: V, _new_cell_value: impl FnOnce() -> V) {
        *acc = new_distance.min(*acc);
    }
    #[inline(always)]
    fn finish(acc: V) -> V {
        acc
    }
}

impl<V: SimdFloat> CellReturn<V> for CellValue {
    type Acc = (V, V); // (running min distance, nearest cell value)
    #[inline(always)]
    fn init() -> (V, V) {
        (distance_sentinel::<V>(), V::zeroes())
    }
    #[inline(always)]
    fn accumulate(acc: &mut (V, V), new_distance: V, new_cell_value: impl FnOnce() -> V) {
        let new_cell_value = new_cell_value();
        let closer = new_distance.cmp_lt(acc.0);
        acc.0 = new_distance.min(acc.0);
        acc.1 = closer.blendv(acc.1, new_cell_value);
    }
    #[inline(always)]
    fn finish(acc: (V, V)) -> V {
        acc.1
    }
}

// ---------------------------------------------------------------------------
// Cellular2 return type (Cell2ReturnType)
//
// Applied once at the end as an arithmetic combination of the two tracked
// nearest distances. `index0`/`index1` stay runtime `usize` (array indices and
// the inner `while` bound), so only the combination is specialized.
// ---------------------------------------------------------------------------

pub trait Cell2Return {
    fn combine<V: SimdFloat>(distance: &[V; 4], index0: usize, index1: usize) -> V;
}

pub struct Distance2;
pub struct Distance2Add;
pub struct Distance2Sub;
pub struct Distance2Mul;
pub struct Distance2Div;

impl Cell2Return for Distance2 {
    #[inline(always)]
    fn combine<V: SimdFloat>(distance: &[V; 4], _index0: usize, index1: usize) -> V {
        distance[index1]
    }
}
impl Cell2Return for Distance2Add {
    #[inline(always)]
    fn combine<V: SimdFloat>(distance: &[V; 4], index0: usize, index1: usize) -> V {
        distance[index0] + distance[index1]
    }
}
impl Cell2Return for Distance2Sub {
    #[inline(always)]
    fn combine<V: SimdFloat>(distance: &[V; 4], index0: usize, index1: usize) -> V {
        distance[index0] - distance[index1]
    }
}
impl Cell2Return for Distance2Mul {
    #[inline(always)]
    fn combine<V: SimdFloat>(distance: &[V; 4], index0: usize, index1: usize) -> V {
        distance[index0] * distance[index1]
    }
}
impl Cell2Return for Distance2Div {
    #[inline(always)]
    fn combine<V: SimdFloat>(distance: &[V; 4], index0: usize, index1: usize) -> V {
        distance[index0] / distance[index1]
    }
}
