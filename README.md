<img align="left" width="120" src="https://raw.githubusercontent.com/jackmott/rust-simd-noise/master/logo.jpg"/>

# SIMDNoise
Super fast SIMD noise library for Rust. PRs welcome!  
Available on [crates.io](https://crates.io/crates/simdnoise).  
See above crates.io link for Docs.

## Features

* Gradient Noise (Simplex aka Perlin) 1D,2D,3D,4D
* Fractal Brownian Motion, Ridge, and Turbulence 
* Cellular Noise (aka Voroni) 2D, 3D 
* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
* AVX2 version also leverages FMA3
* Runtime detection picks the best available instruction set

## Benchmarks
*Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz*
*Single Threaded*
*using Criterion.rs*

### 1D 100,000 points of FBM Noise, 3 Octaves

* scalar 21.289 us
* sse2   8.203  us
* sse41  6.598  us
* avx2   3.174  us

### 2D 1000x1000 FBM Noise, 3 Octaves

* scalar  5.351 ms
* sse2    1.766 ms
* sse41   1.562 ms
* avx2    0.880 ms

### 3D 64x64x64 FBM Noise, 3 Octaves

* scalar  3.173 ms
* sse2    1.631 ms
* sse41   1.440 ms
* avx2    0.882 ms 

### 4D 24x24x24x24 FBM Noise, 3 Octaves

* scalar 534 us
* sse2   345 us
* sse41  305 us
* avx2   144 us

### 2D Cell Noise 

* scalar 37.220 ms 
* sse2   30.780 ms
* sse41  20.492 ms
* avx2   8.697  ms

### 3D Cell Noise

* scalar 3.964 ms
* sse2   3.660 ms
* sse41  2.251 ms 
* avx2   1.182 ms


## Todo

* AVX512 support
* ARM NEON support
* Other noise types

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






