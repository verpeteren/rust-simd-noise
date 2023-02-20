use std::fs::metadata;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};

pub fn read_from_file_f32(file_name: &str) -> std::io::Result<Vec<f32>> {
    let file = File::open(file_name)?;
    let len = metadata(file_name)?.len();
    let mut noise = Vec::with_capacity((len / 4) as usize);
    let mut reader = BufReader::new(file);
    let mut block: [u8; 4] = [0; 4];
    loop {
        match reader.read_exact(&mut block) {
            Ok(_) => {
                let f = LittleEndian::read_f32(&mut block);
                noise.push(f);
            }
            Err(_) => {
                break;
            }
        };
    }
    Ok(noise)
}

pub fn read_from_file_f64(file_name: &str) -> std::io::Result<Vec<f64>> {
    let file = File::open(file_name)?;
    let len = metadata(file_name)?.len();
    let mut noise = Vec::with_capacity((len / 4) as usize);
    let mut reader = BufReader::new(file);
    let mut block: [u8; 8] = [0; 8];
    loop {
        match reader.read_exact(&mut block) {
            Ok(_) => {
                let f = LittleEndian::read_f64(&mut block);
                noise.push(f);
            }
            Err(_) => {
                break;
            }
        };
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
