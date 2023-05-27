use std::collections::HashMap;

use hex::ToHex;
use rand::Rng;
use sha1::{Digest, Sha1};

use simdnoise::NoiseBuilder;
use simdnoise::Settings;

#[test]
fn test_issue_42() {
    let width = 32;
    let height = 32;
    let mut ground = vec![0; width * height];
    let mut subjcts = HashMap::new();
    let mut rng = rand::thread_rng();
    for _i in 0..1000 {
        let seed: u32 = rng.gen();
        let noise = NoiseBuilder::fbm_2d(width, height)
            .with_seed(seed as i32)
            .generate_scaled(0., 255.);

        for i in 0..width * height {
            ground[i] = noise[i] as u8;
        }

        let mut hasher = Sha1::new();
        hasher.update(&ground);
        let result = hasher.finalize();
        let hash_s = result.encode_hex::<String>();

        *subjcts.entry(hash_s).or_insert(1) += 1;
    }
    assert_ne!(subjcts.len(), 8);
}
