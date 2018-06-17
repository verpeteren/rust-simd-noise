<img align="left" width="120" src="https://raw.githubusercontent.com/jackmott/rust-simd-noise/master/logo.jpg"/>

# SIMDNoise
Super fast SIMD noise library for Rust. PRs welcome!  
Available on [crates.io](https://crates.io/crates/simdnoise).  
[Documentation](https://docs.rs/simdnoise/2.0.0/simdnoise/).  
Requires nightly until 1.27 drops  

## Features

* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
* AVX2 version also leverages FMA3
* Runtime detection picks the best available instruction set
* Simplex noise, fractal brownian motion, and turbulence
* 2d and 3d

## Benchmarks
*Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz*
*Single Threaded*

### 2D 1000x1000 FBM Noise, 3 Octaves

* scalar_2d ... bench:  74,207,703 ns/iter (+/- 2,184,952)
* sse2_2d   ... bench:  23,863,725 ns/iter (+/- 746,331)
* sse41_2d  ... bench:  22,440,765 ns/iter (+/- 995,336)
* avx2_2d   ... bench:  12,022,253 ns/iter (+/- 508,793)

### 3D 100x100x100 FBM Noise, 3 Octaves

* scalar_3d ... bench: 102,543,499 ns/iter (+/- 3,310,472)
* sse2_3d   ... bench:  39,991,825 ns/iter (+/- 1,043,332)
* sse41_3d  ... bench:  38,852,436 ns/iter (+/- 1,350,831)
* avx2_3d   ... bench:  23,231,237 ns/iter (+/- 777,420)

## Todo

* AVX512 support
* ARM NEON support
* Voroni, Cell, and other noise types

## Get a block of noise with runtime SIMD detection

The library will, at runtime, pick the fastest available options between SSE2, SSE41, and AVX2

```rust
use simdnoise::*;

// Set up noise type and parameters
let noise_type = simdnoise::NoiseType::FBM {
      freq: 0.04,
      lacunarity: 0.5,
      gain: 2.0,
      octaves: 3,
}; 

// Get a block of 2d 800x600 noise, with no scaling of resulting values
// min and max values are returned so you can apply your own scaling
let (an_f32_vec,min,max) = simdnoise::get_2d_noise(0.0, 800, 0.0, 600, noise_type);

// Get a block of 200x200x200 3d noise
let (an_f32_vec,min,max) = simdnoise::get_3d_noise(0.0, 200, 0.0, 200,0.0, 200, noise_type);

// Get a block of noise scaled between -1 and 1
let an_f32_vec = simdnoise::get_2d_scaled_noise(0.0, 800, 0.0, 600, noise_type,-1.0,1.0);
```

## Call noise functions directly
Sometimes you need something other than a block, like the points on the surface of a sphere.
Sometimes you may want to use SSE41 even with AVX2 is available

```rust

// get a block of 100x100 sse41 noise, skip runtime detection
let (noise,min,max) = simdnoise::sse41::get_2d_noise(0.0,100,0.0,100,noise_type);

// send your own SIMD x,y values to the noise functions directly
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
```






