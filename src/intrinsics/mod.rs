macro_rules! cellular {
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d cellular/voroni noise
        pub unsafe fn $fn_name<S>(
            x: $f_type,
            y: $f_type,
            distance_function: CellDistanceFunction,
            return_type: CellReturnType,
            jitter: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::cellular_2d::<$intrinsic>(
                $transmute_from(x),
                $transmute_from(y),
                distance_function,
                return_type,
                $transmute_from(jitter),
                seed,
            )
            .0
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d cellular/voroni noise
        pub unsafe fn $fn_name<S>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            distance_function: CellDistanceFunction,
            return_type: CellReturnType,
            jitter: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::cellular_3d::<$intrinsic>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                distance_function,
                return_type,
                $transmute_from(jitter),
                seed,
            )
            .0
        }
    };
}

macro_rules! simplex {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 1d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S>(x: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_1d::<$intrinsic>($transmute_from(x), seed).0
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S>(x: $f_type, y: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_2d::<$intrinsic>($transmute_from(x), $transmute_from(y), seed).0
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S>(x: $f_type, y: $f_type, z: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_3d::<$intrinsic>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                seed,
            )
            .0
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 4d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::simplex_4d::<$intrinsic>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                seed,
            )
            .0
        }
    };
}

pub mod avx2;
pub mod scalar;
pub mod sse2;
pub mod sse41;
