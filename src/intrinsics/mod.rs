macro_rules! cellular {
    ("2d", $fn_name: ident, $f32_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        pub unsafe fn $fn_name<S>(
            x: $f32_type,
            y: $f32_type,
            distance_function: CellDistanceFunction,
            return_type: CellReturnType,
            jitter: $f32_type,
            seed: $seed_type,
        ) -> $f32_type {
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
    ("3d", $fn_name: ident, $f32_type: ty, $transmute_from: ident, $seed_type: ty, $mod: ident, $intrinsic: ty) => {
        pub unsafe fn $fn_name<S>(
            x: $f32_type,
            y: $f32_type,
            z: $f32_type,
            distance_function: CellDistanceFunction,
            return_type: CellReturnType,
            jitter: $f32_type,
            seed: $seed_type,
        ) -> $f32_type {
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

pub mod scalar;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse2;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse41;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;
