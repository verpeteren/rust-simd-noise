extern crate simdnoise;
extern crate rand;

use simdnoise::scalar::*;
use simdnoise::sse2::*;
use std::arch::x86_64::*;
use rand::prelude::*;

pub union M128Array {
    pub simd: __m128,
    pub array: [f32; 4],
}

pub fn main() {
    unsafe {
        for _ in 0 .. 100000 {
        let mut rng = rand::thread_rng();
        let hash = rng.gen_range(-100,100);
        let x = rng.gen_range(-10.0,10.0); 
        let y = rng.gen_range(-10.0,10.0);
        let z = rng.gen_range(-10.0,10.0);

        let scalar = simdnoise::scalar::grad3(hash, x, y, z);
        let simd = M128Array {
            simd: simdnoise::sse2::grad3d_simd(
                _mm_set1_epi32(hash),
                _mm_set1_ps(x),
                _mm_set1_ps(y),
                _mm_set1_ps(z),
            ),
        };
        if scalar != simd.array[0] {
            println!("error!");
        }
        /*
        println!("scalar    {}", scalar);
        println!(
            "simd      {} , {} , {} , {}",
            simd.array[0], simd.array[1], simd.array[2], simd.array[3]
        );*/
        }
    }
}
