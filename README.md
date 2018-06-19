<img align="left" width="120" src="https://raw.githubusercontent.com/jackmott/rust-simd-noise/master/logo.jpg"/>

# SIMDNoise
Super fast SIMD noise library for Rust. PRs welcome!  
Available on [crates.io](https://crates.io/crates/simdnoise).  
[Documentation](https://docs.rs/simdnoise/2.2.0/simdnoise/).  
Requires nightly until 1.27 drops  

## Features

* SSE2, SSE41, and AVX2 instruction sets, along with non SIMD fallback
* AVX2 version also leverages FMA3
* Runtime detection picks the best available instruction set
* Simplex noise, fractal brownian motion, turbulence, and ridge
* 1D, 2D, 3D, and 4D

## Benchmarks
*Intel(R) Core(TM) i7-6700 CPU @ 3.40GHz*
*Single Threaded*

### 1D 100,000 points of FBM Noise, 3 Octaves

* scalar ... bench:   2,022,968 ns/iter (+/- 68,981)
* sse2   ... bench:     847,764 ns/iter (+/- 43,875)
* sse41  ... bench:     666,731 ns/iter (+/- 10,219)
* avx2   ... bench:     306,011 ns/iter (+/- 14,347)
 
### 2D 1000x1000 FBM Noise, 3 Octaves

* scalar ... bench:  74,686,044 ns/iter (+/- 3,053,838)
* sse2   ... bench:  23,619,783 ns/iter (+/- 1,008,879)
* sse41  ... bench:  21,847,769 ns/iter (+/- 914,364)
* avx2   ... bench:  11,791,738 ns/iter (+/- 446,718)

### 3D 64x64x64 FBM Noise, 3 Octaves

* scalar ... bench:  22,219,344 ns/iter (+/- 817,769)
* sse2   ... bench:  10,331,856 ns/iter (+/- 450,920)
* sse41  ... bench:   9,766,523 ns/iter (+/- 604,034)
* avx2   ... bench:   5,566,535 ns/iter (+/- 181,791)

### 4D 24x24x24x24 FBM Noise, 3 Octaves

* scalar  ... bench:  48,324,536 ns/iter (+/- 1,813,984)
* sse2    ... bench:  26,955,224 ns/iter (+/- 1,253,751)
* sse41   ... bench:  25,792,680 ns/iter (+/- 749,234)
* avx2    ... bench:  13,080,348 ns/iter (+/- 491,006)

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






