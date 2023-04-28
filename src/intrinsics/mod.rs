pub mod scalar;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse2;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod sse41;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod avx2;

macro_rules! define_all_simd_functions {
    ($transmute_from:ident, $transmute_to:ident, $f32_vec:ty, $f64_vec:ty) => {
        simd_unsafe_generate_all!(
            /// Get a single value of 2d cellular/voroni noise
            pub fn cellular_2d(
                x: $f32_vec,
                y: $f32_vec,
                distance_function: CellDistanceFunction,
                return_type: CellReturnType,
                jitter: $f32_vec,
                seed: i32,
            ) -> $f32_vec {
                cell_32::cellular_2d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    distance_function,
                    return_type,
                    SimdTransmuteF32::$transmute_from(jitter),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d cellular/voroni noise
            pub fn cellular_3d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                distance_function: CellDistanceFunction,
                return_type: CellReturnType,
                jitter: $f32_vec,
                seed: i32,
            ) -> $f32_vec {
                cell_32::cellular_3d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    distance_function,
                    return_type,
                    SimdTransmuteF32::$transmute_from(jitter),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d cellular/voroni noise
            pub fn cellular_2d_f64(
                x: $f64_vec,
                y: $f64_vec,
                distance_function: CellDistanceFunction,
                return_type: CellReturnType,
                jitter: $f64_vec,
                seed: i64,
            ) -> $f64_vec {
                cell_64::cellular_2d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    distance_function,
                    return_type,
                    SimdTransmuteF64::$transmute_from(jitter),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d cellular/voroni noise
            pub fn cellular_3d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                distance_function: CellDistanceFunction,
                return_type: CellReturnType,
                jitter: $f64_vec,
                seed: i64,
            ) -> $f64_vec {
                cell_64::cellular_3d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    distance_function,
                    return_type,
                    SimdTransmuteF64::$transmute_from(jitter),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 1d simplex noise, results
            /// are not scaled.
            pub fn simplex_1d(x: $f32_vec, seed: i32) -> $f32_vec {
                simplex_32::simplex_1d::<S>(SimdTransmuteF32::$transmute_from(x), seed)
                    .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 1d fractal brownian motion.
            pub fn fbm_1d(
                x: $f32_vec,
                lacunarity: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                fbm_32::fbm_1d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(lacunarity),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d ridge noise.
            pub fn ridge_1d(
                x: $f32_vec,
                lacunarity: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                ridge_32::ridge_1d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(lacunarity),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d turbulence.
            pub fn turbulence_1d(
                x: $f32_vec,
                lacunarity: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                turbulence_32::turbulence_1d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(lacunarity),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 1d simplex noise, results
            /// are not scaled.
            pub fn simplex_1d_f64(x: $f64_vec, seed: i64) -> $f64_vec {
                simplex_64::simplex_1d::<S>(SimdTransmuteF64::$transmute_from(x), seed)
                    .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 1d fractal brownian motion.
            pub fn fbm_1d_f64(
                x: $f64_vec,
                lacunarity: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                fbm_64::fbm_1d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(lacunarity),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d ridge noise.
            pub fn ridge_1d_f64(
                x: $f64_vec,
                lacunarity: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                ridge_64::ridge_1d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(lacunarity),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d turbulence.
            pub fn turbulence_1d_f64(
                x: $f64_vec,
                lacunarity: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                turbulence_64::turbulence_1d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(lacunarity),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width sized block of 1d noise, unscaled.
            /// `start_x` can be used to provide an offset in the
            /// coordinates. Results are unscaled, 'min' and 'max' noise values
            /// are returned so you can scale and transform the noise as you see fit
            /// in a single pass.
            pub fn get_1d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
                crate::noise_helpers_32::get_1d_noise::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width sized block of 1d noise, unscaled.
            /// `start_x` can be used to provide an offset in the
            /// coordinates. Results are unscaled, 'min' and 'max' noise values
            /// are returned so you can scale and transform the noise as you see fit
            /// in a single pass.
            pub fn get_1d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
                crate::noise_helpers_64::get_1d_noise_f64::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width sized block of scaled 2d noise
            /// `start_x` can be used to provide an offset in the
            /// coordinates.
            /// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
            pub fn get_1d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
                let (mut noise, min, max) = get_1d_noise_generic::<S>(noise_type);
                let dim = noise_type.get_dimensions();
                scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
                noise
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d simplex noise, results
            /// are not scaled.
            pub fn simplex_2d(x: $f32_vec, y: $f32_vec, seed: i32) -> $f32_vec {
                simplex_32::simplex_2d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d fractal brownian motion.
            pub fn fbm_2d(
                x: $f32_vec,
                y: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                fbm_32::fbm_2d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d ridge noise.
            pub fn ridge_2d(
                x: $f32_vec,
                y: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                ridge_32::ridge_2d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d turbulence.
            pub fn turbulence_2d(
                x: $f32_vec,
                y: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                turbulence_32::turbulence_2d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d simplex noise, results
            /// are not scaled.
            pub fn simplex_2d_f64(x: $f64_vec, y: $f64_vec, seed: i64) -> $f64_vec {
                simplex_64::simplex_2d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d fractal brownian motion.
            pub fn fbm_2d_f64(
                x: $f64_vec,
                y: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                fbm_64::fbm_2d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d ridge noise.
            pub fn ridge_2d_f64(
                x: $f64_vec,
                y: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                ridge_64::ridge_2d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 2d turbulence.
            pub fn turbulence_2d_f64(
                x: $f64_vec,
                y: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                turbulence_64::turbulence_2d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width X height sized block of 2d noise, unscaled.
            /// `start_x` and `start_y` can be used to provide an offset in the
            /// coordinates. Results are unscaled, 'min' and 'max' noise values
            /// are returned so you can scale and transform the noise as you see fit
            /// in a single pass.
            pub fn get_2d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
                crate::noise_helpers_32::get_2d_noise::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            pub fn get_2d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
                crate::noise_helpers_64::get_2d_noise_f64::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width X height sized block of scaled 2d noise
            /// `start_x` and `start_y` can be used to provide an offset in the
            /// coordinates.
            /// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
            pub fn get_2d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
                let (mut noise, min, max) = get_2d_noise_generic::<S>(noise_type);
                let dim = noise_type.get_dimensions();
                scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
                noise
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d simplex noise, results
            /// are not scaled.
            pub fn simplex_3d(x: $f32_vec, y: $f32_vec, z: $f32_vec, seed: i32) -> $f32_vec {
                simplex_32::simplex_3d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d fractal brownian motion.
            pub fn fbm_3d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                fbm_32::fbm_3d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d ridge noise.
            pub fn ridge_3d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                ridge_32::ridge_3d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d turbulence.
            pub fn turbulence_3d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                turbulence_32::turbulence_3d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d simplex noise, results
            /// are not scaled.
            pub fn simplex_3d_f64(x: $f64_vec, y: $f64_vec, z: $f64_vec, seed: i64) -> $f64_vec {
                simplex_64::simplex_3d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d fractal brownian motion.
            pub fn fbm_3d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                fbm_64::fbm_3d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d ridge noise.
            pub fn ridge_3d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                ridge_64::ridge_3d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 3d turbulence.
            pub fn turbulence_3d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                turbulence_64::turbulence_3d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width X height X depth sized block of 3d noise, unscaled,
            /// `start_x`,`start_y` and `start_z` can be used to provide an offset in the
            /// coordinates. Results are unscaled, 'min' and 'max' noise values
            /// are returned so you can scale and transform the noise as you see fit
            /// in a single pass.
            pub fn get_3d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
                crate::noise_helpers_32::get_3d_noise::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            pub fn get_3d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
                crate::noise_helpers_64::get_3d_noise_f64::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width X height X depth sized block of scaled 3d noise
            /// `start_x`, `start_y` and `start_z` can be used to provide an offset in the
            /// coordinates.
            /// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
            pub fn get_3d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
                let (mut noise, min, max) = get_3d_noise_generic::<S>(noise_type);
                let dim = noise_type.get_dimensions();
                scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
                noise
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d simplex noise, results
            /// are not scaled.
            pub fn simplex_4d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                w: $f32_vec,
                seed: i32,
            ) -> $f32_vec {
                simplex_32::simplex_4d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(w),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d fractal brownian motion.
            pub fn fbm_4d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                w: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                fbm_32::fbm_4d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(w),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d ridge noise.
            pub fn ridge_4d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                w: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                ridge_32::ridge_4d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(w),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d turbulence.
            pub fn turbulence_4d(
                x: $f32_vec,
                y: $f32_vec,
                z: $f32_vec,
                w: $f32_vec,
                lac: $f32_vec,
                gain: $f32_vec,
                octaves: u8,
                seed: i32,
            ) -> $f32_vec {
                turbulence_32::turbulence_4d::<S>(
                    SimdTransmuteF32::$transmute_from(x),
                    SimdTransmuteF32::$transmute_from(y),
                    SimdTransmuteF32::$transmute_from(z),
                    SimdTransmuteF32::$transmute_from(w),
                    SimdTransmuteF32::$transmute_from(lac),
                    SimdTransmuteF32::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d simplex noise, results
            /// are not scaled.
            pub fn simplex_4d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                w: $f64_vec,
                seed: i64,
            ) -> $f64_vec {
                simplex_64::simplex_4d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(w),
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d fractal brownian motion.
            pub fn fbm_4d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                w: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                fbm_64::fbm_4d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(w),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d ridge noise.
            pub fn ridge_4d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                w: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                ridge_64::ridge_4d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(w),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Get a single value of 4d turbulence.
            pub fn turbulence_4d_f64(
                x: $f64_vec,
                y: $f64_vec,
                z: $f64_vec,
                w: $f64_vec,
                lac: $f64_vec,
                gain: $f64_vec,
                octaves: u8,
                seed: i64,
            ) -> $f64_vec {
                turbulence_64::turbulence_4d::<S>(
                    SimdTransmuteF64::$transmute_from(x),
                    SimdTransmuteF64::$transmute_from(y),
                    SimdTransmuteF64::$transmute_from(z),
                    SimdTransmuteF64::$transmute_from(w),
                    SimdTransmuteF64::$transmute_from(lac),
                    SimdTransmuteF64::$transmute_from(gain),
                    octaves,
                    seed,
                )
                .$transmute_to()
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width X height X depth x time sized block of 4d noise, unscaled,
            /// `start_*` can be used to provide an offset in the
            /// coordinates. Results are unscaled, 'min' and 'max' noise values
            /// are returned so you can scale and transform the noise as you see fit
            /// in a single pass.
            pub fn get_4d_noise(noise_type: &NoiseType) -> (Vec<f32>, f32, f32) {
                crate::noise_helpers_32::get_4d_noise::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            pub fn get_4d_noise_64(noise_type: &NoiseType) -> (Vec<f64>, f64, f64) {
                crate::noise_helpers_64::get_4d_noise_f64::<S>(noise_type)
            }
        );

        simd_unsafe_generate_all!(
            /// Gets a width X height X depth X time sized block of scaled 4d noise
            /// `start_*` can be used to provide an offset in the
            /// coordinates.
            /// `scaled_min` and `scaled_max` specify the range you want the noise scaled to.
            pub fn get_4d_scaled_noise(noise_type: &NoiseType) -> Vec<f32> {
                let (mut noise, min, max) = get_4d_noise_generic::<S>(noise_type);
                let dim = noise_type.get_dimensions();
                scale_noise::<S>(dim.min, dim.max, min, max, &mut noise);
                noise
            }
        );
    };
}
use define_all_simd_functions;
