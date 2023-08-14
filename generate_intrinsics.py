#!/usr/bin/env python3

from typing import List
import os

noise_types = {
    'cellular': ['2', '3'],
    'cellular2': ['2', '3'],
    'ridge': ['1', '2', '3', '4'],
    'fbm': ['1', '2', '3', '4'],
    'turbulence': ['1', '2', '3', '4'],
    'gradient': ['1', '2', '3', '4']
    }
float_types = [
    '32',
    '64'
    ]
intrinsics = [
    'avx2',
    'scalar',
    'sse2',
    'sse41'
    ]

def generate_intrinsic_tests()-> List[str]:
    codes = [
"""
use simdeez::prelude::*;
use simdeez::scalar::*;
use simdeez::sse2::*;
use simdeez::sse41::*
use simdeez::avx2::*;

use core::arch::x86_64::__m256;
use simdnoise::intrinsics::{avx2, scalar, sse2, sse41};
use simdnoise::{NoiseType, TurbulenceSettings, RidgeSettings, FbmSettings, CellularSettings, Cellular2Settings, GradientSettings, SimplexSettings, Settings, NoiseDimensions, CellDistanceFunction, CellReturnType, Cell2ReturnType};

mod helpers;
use helpers::{BIN_PATH, read_from_file_f32, save_to_file_f32, read_from_file_f64, save_to_file_f64};
"""
    ]
    dim_lookup = {
        '1': 'width: 64,',
        '2': 'width: 64, height: 32,',
        '3': 'width: 64, height: 32, depth: 16,',
        '4': 'width: 64, height: 32, depth: 16, time: 8,',
    }
    cell_options = {
        'cellular': ( "", ["CellValue", "Distance"]),
        'cellular2': ("2", ["Distance2", "Distance2Add", "Distance2Sub", "Distance2Mul", "Distance2Div"]),
    }

    for noise_type, dimensions in noise_types.items():
        options = {"normal": ""}
        if noise_type in ['fbm', 'turbulence', 'ridge']:
            options = {"normal": f"""
        .with_lacunarity(0.5)
        .with_gain(2.0)
        .with_octaves(5)
        """}
        elif noise_type in cell_options:
            (count, dist_ret) = cell_options[noise_type]
            for dist in ["Euclidean", "Manhattan", "Natural"]:
                for ret in dist_ret:
                    title = f"{dist.lower()}_{ret.lower()}"
                    option = f"""
                        .with_distance_function(CellDistanceFunction::{dist})
                        .with_return_type(Cell{count}ReturnType::{ret})
                        """
                    options[title] = option
        for (postfix, option) in options.items():
            for dimension in dimensions:
                dims = dim_lookup[dimension]
                for intrinsic in intrinsics:
                    engine = f"simdeez::{intrinsic}::{intrinsic.capitalize()}"
                    for float_type in float_types:
                        if float_type == '64' and noise_type in ['cellular', 'cellular2']:
                            # we skip these due to overflow errors
                            continue
                        variant = ["", f"_{float_type}"][float_type!="32"]
                        fn_name = f"intrinsic_{noise_type}_{dimension}_{intrinsic}_{float_type}_{postfix}"
                        enabled = ""
                        if intrinsic == "sse41":
                            enabled = "#[target_feature(enable = \"sse4.1\")]"
                        elif intrinsic != "scalar":
                            enabled = f"#[target_feature(enable = \"{intrinsic}\")]"
                        block = f"""
{enabled}
unsafe fn do_{fn_name}() -> Vec<f{float_type}>{{
    let dims = NoiseDimensions {{
        {dims}
        ..NoiseDimensions::default({dimension})
    }};

    let noise_type = {noise_type.capitalize()}Settings::default(dims)
        .with_seed(1337)
        {option}
        .wrap();
    let (noise, _min, _max) = {intrinsic}::get_{dimension}d_noise{variant}::<{engine}>(&noise_type);
    noise
}}

#[test]
fn test_{fn_name} () {{
    let file_name = format!(
        "{{}}/{{}}_{{}}_{{}}_{{}}_{{}}_{{}}.bin",
        BIN_PATH, "intrinsics", "{noise_type}", "{float_type}", "{dimension}d", "{intrinsic}", "{postfix}"
    );
    unsafe {{
        let noise = do_{fn_name}();
        //save_to_file_f{float_type}(&file_name, noise.as_slice()).unwrap();
        let expected = read_from_file_f{float_type}(&file_name).unwrap();
        assert_eq!(expected, noise);
    }}
}}
"""
                        codes.append(block)
    return codes

def main() :
    codes = generate_intrinsic_tests()
    file_name = "tests/intrinsics.rs"
    with open(file_name, "w") as file_h:
        source = "\n".join(codes)
        file_h.write(source)
    os.system(f"rustfmt {file_name}");

if __name__ == '__main__':
    main()
