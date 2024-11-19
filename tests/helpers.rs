// Helpers are shared across test binaries; not every binary uses all of them.
#![allow(dead_code)]

use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

// Fixtures are stored per architecture because the SIMD backends differ bit-for-bit
// (notably the approximate `rsqrt` used by cellular noise). Tests pick the matching
// set via this suffix. Only x86 and aarch64/NEON fixtures are generated.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const ARCH: &str = "x86";
#[cfg(all(target_arch = "aarch64", target_feature = "neon"))]
pub const ARCH: &str = "neon";
#[cfg(not(any(
    target_arch = "x86",
    target_arch = "x86_64",
    all(target_arch = "aarch64", target_feature = "neon")
)))]
compile_error!("test fixtures are only generated for x86 and aarch64/NEON targets");

pub const BIN_PATH: &str = "tests/assets";

pub fn read_from_file_f32(file_name: &str) -> std::io::Result<Vec<f32>> {
    let file = File::open(file_name)?;
    let len = metadata(file_name)?.len();
    let mut noise = Vec::with_capacity((len / 4) as usize);
    let mut reader = BufReader::new(file);
    let mut block: [u8; 4] = [0; 4];
    while reader.read_exact(&mut block).is_ok() {
        let f = LittleEndian::read_f32(&block);
        noise.push(f);
    }
    Ok(noise)
}

pub fn read_from_file_f64(file_name: &str) -> std::io::Result<Vec<f64>> {
    let file = File::open(file_name)?;
    let len = metadata(file_name)?.len();
    let mut noise = Vec::with_capacity((len / 4) as usize);
    let mut reader = BufReader::new(file);
    let mut block: [u8; 8] = [0; 8];
    while reader.read_exact(&mut block).is_ok() {
        let f = LittleEndian::read_f64(&block);
        noise.push(f);
    }
    Ok(noise)
}

pub fn save_to_file_f32(file_name: &str, noise: &[f32]) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    for f in noise {
        file.write_f32::<LittleEndian>(*f)?;
    }
    Ok(())
}

pub fn save_to_file_f64(file_name: &str, noise: &[f64]) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    for f in noise {
        file.write_f64::<LittleEndian>(*f)?;
    }
    Ok(())
}
