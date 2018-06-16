extern crate rand;
extern crate simdnoise;

use rand::prelude::*;
use std::arch::x86_64::*;

pub union M128Array {
    pub simd: __m128,
    pub array: [f32; 4],
}

pub fn main() {
    unsafe {
        for i in 0..100000 {
            let mut rng = rand::thread_rng();
            let x = rng.gen_range(-1000.0, 1000.0);
            let y = rng.gen_range(-1000.0, 1000.0);
            let z = rng.gen_range(-1000.0, 1000.0);

            let scalar = simdnoise::scalar::simplex_3d(x, y, z);
            let simd = M128Array {
                simd: simdnoise::sse2::simplex_3d(_mm_set1_ps(x), _mm_set1_ps(y), _mm_set1_ps(z)),
            };
            if (scalar - simd.array[0]).abs() > 0.001 {
                println!("Disagree at iteration: {}", i);
                println!("{},{},{}", x, y, z);
                println!("simd:{} scalar:{}", simd.array[0], scalar);
                break;
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
