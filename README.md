# rust-simd-noise
SIMD noise library for Rust. Currently in alpha. PRs welcome!

## Features

* SSE2, SSE41, and AVX2 instruction sets
* simplex noise, fractal brownian motion, and turbulence
* 2d and 3d

## Todo

* scalar support
* AVX512 support
* ARM NEON support
* Voroni and other noise types

## Get a block of noise with runtime SIMD detection

The library will at runtime pick the fastest available options between SSE2, SSE41, and AVX2

```rust
use simdnoise::*;

// A struct to set up nosie parameters
let fractal_settings = simdnoise::FractalSettings {
      freq: 0.04,
      lacunarity: 0.5,
      gain: 2.0,
      octaves: 3,
      noise_type: simdnoise::NoiseType::FBM,
}; 

// Get a block of 2d 800x600 noise, with no scaling of resulting values
// min and max values are returned so you can apply your own scaling
let (an_f32_vec,min,max) = simdnoise::get_2d_noise(0.0, 800, 0.0, 600, fractal_settings);

// Get a block of 200x200x200 3d noise
let (an_f32_vec,min,max) = simdnoise::get_3d_noise(0.0, 200, 0.0, 200,0.0, 200, fractal_settings);

// Get a block of noise scaled between -1 and 1
let (an_f32_vec,min,max) = simdnoise::get_2d_scaled_noise(0.0, 800, 0.0, 600, fractal_settings,-1.0,1.0);

## Call noise functions directly

Sometimes you need something other than a block, like the points on the surface of a sphere.
Sometimes you may want to use SSE41 even with AVX2 is available

// get a block of 100x100 sse41 noise, skip runtime detection
let (noise,min,max) = simdnoise::sse41::get_2d_noise(0.0,100,0.0,100,fractal_settings);

unsafe {
  // sse2 simplex noise
  let x = _mm_set1_ps(5.0);
  let y = _mm_set1_ps(10.0);
  let f : __m128 = simdnoise::sse2::simplex_2d(x,y);
  
  // avx2 turbulence
  let x = _mm256_set1_ps(5.0);
  let y = _mm256_set1_ps(10.0);
  let freq = _mm_256_set1_ps(1.0);
  let lacunarity = _mm256_set1_ps(0.5);
  let gain = _mm256_set1_ps(2.0);
  let octaves = 3;
  let f_turbulence : __m256 = simdnoise::avx2::turbulence_2d(x,y,freq,lacunarity,gain,octaves);
    
}
  






