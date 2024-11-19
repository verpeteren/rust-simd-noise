macro_rules! cellular {
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d cellular/voroni noise
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            distance_function: CellDistanceFunction,
            return_type: CellReturnType,
            jitter: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::cellular_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                distance_function,
                return_type,
                $transmute_from(jitter),
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d cellular/voroni noise
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            distance_function: CellDistanceFunction,
            return_type: CellReturnType,
            jitter: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::cellular_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                distance_function,
                return_type,
                $transmute_from(jitter),
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! simplex {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 1d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S: simdeez::Simd>(x: $f_type, seed: $seed_type) -> $f_type {
            $mod::simplex_1d::<S>($transmute_from(x), seed).$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::simplex_2d::<S>($transmute_from(x), $transmute_from(y), seed).$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::simplex_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 4d simplex noise, results are not scaled.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            seed: $seed_type,
        ) -> $f_type {
            $mod::simplex_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! fbm {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 1d fractal brownian motion.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::fbm_1d::<S>(
                $transmute_from(x),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d fractal brownian motion.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::fbm_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d fractal brownian motion.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::fbm_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 4d fractal brownian motion.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::fbm_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
}
macro_rules! ridge {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 1d ridge noise.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::ridge_1d::<S>(
                $transmute_from(x),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d ridge noise.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::ridge_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d ridge noise.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::ridge_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 4d ridge noise.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::ridge_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! turbulence {
    ("1d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 1d turbulence.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::turbulence_1d::<S>(
                $transmute_from(x),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("2d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 2d turbulence.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::turbulence_2d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("3d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 3d turbulence.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::turbulence_3d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
    ("4d", $fn_name: ident, $f_type: ty, $transmute_from: path, $seed_type: ty, $mod: ident, $transmute_to: ident) => {
        #[cfg(any(
            target_feature = "sse2",
            target_feature = "sse4.1",
            target_feature = "avx2"
        ))]
        /// Get a single value of 4d turbulence.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            x: $f_type,
            y: $f_type,
            z: $f_type,
            w: $f_type,
            lacunarity: $f_type,
            gain: $f_type,
            octaves: u8,
            seed: $seed_type,
        ) -> $f_type {
            $mod::turbulence_4d::<S>(
                $transmute_from(x),
                $transmute_from(y),
                $transmute_from(z),
                $transmute_from(w),
                $transmute_from(lacunarity),
                $transmute_from(gain),
                octaves,
                seed,
            )
            .$transmute_to()
        }
    };
}

macro_rules! get_noise {
    ($call: ident, $fn_name: ident, $f_type: ty, $mod: ident) => {
        /// Gets a width sized block of noise, unscaled.
        /// `start_x` can be used to provide an offset in the
        /// coordinates. Results are unscaled, 'min' and 'max' noise values
        /// are returned so you can scale and transform the noise as you see fit
        /// in a single pass.
        pub unsafe fn $fn_name<S: simdeez::Simd>(
            noise_type: &NoiseType,
        ) -> (Vec<$f_type>, $f_type, $f_type) {
            $mod::$call::<S>(noise_type)
        }
    };
}
macro_rules! get_noise_scaled {
    ($call: ident, $fn_name: ident, $f_type: ty) => {
        /// Gets a width sized block of scaled noise
        /// `start_x` can be used to provide an offset in the coordinates.
        /// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.

        pub unsafe fn $fn_name<S: simdeez::Simd>(noise_type: &NoiseType) -> Vec<$f_type> {
            let (mut noise, min, max) = $call::<S>(noise_type);
            let dim = noise_type.get_dimensions();
            scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
            noise
        }
    };
}

pub mod avx2;
pub mod neon;
pub mod scalar;
pub mod sse2;
pub mod sse41;
