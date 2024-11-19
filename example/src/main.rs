use std::fmt::{Display, Formatter, Result};
use std::slice::Iter;

use clap::{Parser, Subcommand, ValueEnum};
use minifb::{Key, Window, WindowOptions};
use simdnoise::{CellDistanceFunction, Settings, SimplexSettings};

const FPS: usize = 60;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const DEPTH: usize = 1;
const TIME: usize = 5;
const DEFAULT_FREQUENCY: f32 = 1.2;
const DEFAULT_JITTER: f32 = 1.2;
const DEFAULT_LACUNARITY: f32 = 0.5;
const DEFAULT_GAIN: f32 = 2.0;
const DEFAULT_OCTAVES: u8 = 3;

const SCALE_MIN: f32 = 0.0;
const SCALE_MAX: f32 = 255.0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = WIDTH, help="The width of the generated image", global=true)]
    pub width: usize,

    #[clap(long, value_parser, default_value_t = HEIGHT, help="The height of the generated image", global=true)]
    pub height: usize,

    #[clap(long, value_parser, default_value_t = DEPTH, help="The z dimension of the generated image", global=true, required=false)]
    pub depth: usize,

    #[clap(long, value_parser, default_value_t = TIME, help="The w dimension of the generated image", global=true, required=false)]
    pub time: usize,

    #[clap(long, value_parser, default_value_t = Dimension::Three, help="The number of dimensions of the generated noise", global=true)]
    pub dimension: Dimension,

    #[clap(
        long,
        value_parser,
        default_value_t = 8,
        help = "The initial seed value",
        global = true
    )]
    pub seed: i32,

    #[clap(
        long,
        value_parser,
        help = "Use an offset for the first dimension",
        global = true,
        required = false
    )]
    pub offset_x: Option<f32>,

    #[clap(
        long,
        value_parser,
        help = "Use an offset for the second dimension",
        global = true,
        required = false
    )]
    pub offset_y: Option<f32>,

    #[clap(
        long,
        value_parser,
        help = "Use an offset for the third dimension",
        global = true,
        required = false
    )]
    pub offset_z: Option<f32>,

    #[clap(
        long,
        value_parser,
        help = "Use an offset for the fourth dimension",
        global = true,
        required = false
    )]
    pub offset_w: Option<f32>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Dimension {
    One,
    Two,
    Three,
    Four,
}

impl Display for Dimension {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let num = match self {
            Dimension::One => "one",
            Dimension::Two => "two",
            Dimension::Three => "three",
            Dimension::Four => "four",
        };
        write!(f, "{}", num)
    }
}

// should we expose ValueEnum to CellDistanceFunction?
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Distance {
    Euclidean,
    Manhattan,
    Natural,
}

impl From<Distance> for CellDistanceFunction {
    fn from(distance: Distance) -> Self {
        match distance {
            Distance::Euclidean => CellDistanceFunction::Euclidean,
            Distance::Manhattan => CellDistanceFunction::Manhattan,
            Distance::Natural => CellDistanceFunction::Natural,
        }
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let num = match self {
            Distance::Euclidean => "euclidean",
            Distance::Manhattan => "manhattan",
            Distance::Natural => "natural",
        };
        write!(f, "{}", num)
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Cellular {
        #[clap(long, value_parser, default_value_t = Distance::Euclidean, help="The distance function", global=true)]
        distance: Distance,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_FREQUENCY)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_JITTER)]
        jitter: f32,
        //@TODO: index0/1
    },
    #[command(arg_required_else_help = true)]
    FBM {
        #[arg(short, long, value_parser, default_value_t = DEFAULT_FREQUENCY)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_LACUNARITY)]
        lacunarity: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_GAIN)]
        gain: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_OCTAVES)]
        octaves: u8,
    },
    #[command(arg_required_else_help = true)]
    Ridge {
        #[arg(short, long, value_parser, default_value_t = DEFAULT_FREQUENCY)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_LACUNARITY)]
        lacunarity: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_GAIN)]
        gain: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_OCTAVES)]
        octaves: u8,
    },
    #[command(arg_required_else_help = true)]
    Turbulence {
        #[arg(short, long, value_parser, default_value_t = DEFAULT_FREQUENCY)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_LACUNARITY)]
        lacunarity: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_GAIN)]
        gain: f32,
        #[arg(short, long, value_parser, default_value_t = DEFAULT_OCTAVES)]
        octaves: u8,
    },
    #[command(arg_required_else_help = true)]
    Gradient {},
}

struct Coordinate<T> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl<T> Default for Coordinate<T>
where
    T: Default,
{
    fn default() -> Self {
        Coordinate {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }
}

impl<T> Coordinate<T>
where
    T: Default,
{
    fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
    fn new_checked(
        dimension: Dimension,
        x: Option<T>,
        y: Option<T>,
        z: Option<T>,
        w: Option<T>,
    ) -> Self {
        let mut coord = Coordinate::<T>::default();
        match (dimension, x, y, z, w) {
            (Dimension::One, x, None, None, None) => {
                coord.x = match x {
                    Some(xu) => xu,
                    None => T::default(),
                };
            }
            (Dimension::Two, x, y, None, None) => {
                coord.x = match x {
                    Some(xu) => xu,
                    None => T::default(),
                };
                coord.y = match y {
                    Some(yu) => yu,
                    None => T::default(),
                };
            }
            (Dimension::Three, x, y, z, None) => {
                coord.x = match x {
                    Some(xu) => xu,
                    None => T::default(),
                };
                coord.y = match y {
                    Some(yu) => yu,
                    None => T::default(),
                };
                coord.z = match z {
                    Some(zu) => zu,
                    None => T::default(),
                };
            }
            (Dimension::Four, x, y, z, w) => {
                coord.x = match x {
                    Some(xu) => xu,
                    None => T::default(),
                };
                coord.y = match y {
                    Some(yu) => yu,
                    None => T::default(),
                };
                coord.z = match z {
                    Some(zu) => zu,
                    None => T::default(),
                };
                coord.w = match w {
                    Some(wu) => wu,
                    None => T::default(),
                };
            }
            _ => {
                panic!(
                    "Coordinate parameters are not matching for this dimension {:?}",
                    dimension
                );
            }
        }
        coord
    }
}

fn noise_1d_to_frames(noise: Vec<f32>, position: Coordinate<usize>) -> Vec<Vec<u32>> {
    let x: Vec<u32> = noise.iter().map(|x| *x as u32).collect();
    let mut xy = Vec::with_capacity(x.len() * position.y);
    for _i in 0..(position.y) {
        xy.extend_from_slice(x.as_slice());
    }
    vec![xy]
}

fn noise_2d_to_frames(noise: Vec<f32>) -> Vec<Vec<u32>> {
    vec![noise.iter().map(|x| *x as u32).collect()]
}

fn noise_3d_to_frames(noise: Vec<f32>) -> Vec<Vec<u32>> {
    noise_2d_to_frames(noise)
}

fn noise_4d_to_frames(noise: Vec<f32>, position: Coordinate<usize>) -> Vec<Vec<u32>> {
    let mut frames = Vec::with_capacity(position.z * position.z);
    let frame_size = position.x * position.y;
    let mut niter: Iter<f32> = noise.iter();
    for _w in 0..position.w {
        for _z in 0..position.z {
            let mut frame = Vec::with_capacity(frame_size);
            for _xy in 0..frame_size {
                let pix = niter.next();
                if let Some(xy) = pix {
                    frame.push(*xy as u32);
                }
            }
            frames.push(frame);
        }
    }
    frames
}

macro_rules! common_build_settings {
    ($builder: expr, $seed : expr, $scale_min: expr, $scale_max: expr) => {
        $builder
            .with_seed($seed)
            .generate_scaled($scale_min, $scale_max)
    };
}
macro_rules! cellular_build_settings {
    ($builder: expr, $freq: expr, $jitter: expr, $distance: expr) => {
        $builder
            .with_freq($freq)
            .with_jitter($jitter)
            .with_distance_function($distance)
    };
}
macro_rules! noise_build_settings {
    ($builder: expr, $frequency: expr, $lacunarity: expr, $gain: expr, $octaves: expr) => {
        $builder
            .with_freq($frequency)
            .with_lacunarity($lacunarity)
            .with_gain($gain)
            .with_octaves($octaves)
    };
}
macro_rules! process_noise_command {
    ($func_1d: ident, $func_2d: ident, $func_3d: ident, $func_4d: ident, $dimension: expr, $seed: expr, $position: expr, $offset: expr, $frequency: expr, $lacunarity: expr, $gain: expr, $octaves: expr) => {
        match $dimension {
            Dimension::One => {
                let mut builder = simdnoise::NoiseBuilder::$func_1d($offset.x, $position.x);
                let builder =
                    noise_build_settings!(builder, $frequency, $lacunarity, $gain, $octaves);
                let noise = common_build_settings!(builder, $seed, SCALE_MIN, SCALE_MAX);
                noise_1d_to_frames(noise, $position)
            }
            Dimension::Two => {
                let mut builder = simdnoise::NoiseBuilder::$func_2d(
                    $offset.x,
                    $position.x,
                    $offset.y,
                    $position.y,
                );
                let builder =
                    noise_build_settings!(builder, $frequency, $lacunarity, $gain, $octaves);
                let noise = common_build_settings!(builder, $seed, SCALE_MIN, SCALE_MAX);
                noise_2d_to_frames(noise)
            }
            Dimension::Three => {
                let mut builder = simdnoise::NoiseBuilder::$func_3d(
                    $offset.x,
                    $position.x,
                    $offset.y,
                    $position.y,
                    $offset.z,
                    $position.z,
                );
                let builder =
                    noise_build_settings!(builder, $frequency, $lacunarity, $gain, $octaves);
                let noise = common_build_settings!(builder, $seed, SCALE_MIN, SCALE_MAX);
                noise_3d_to_frames(noise)
            }
            Dimension::Four => {
                let mut builder = simdnoise::NoiseBuilder::$func_4d(
                    $offset.x,
                    $position.x,
                    $offset.y,
                    $position.y,
                    $offset.z,
                    $position.z,
                    $offset.w,
                    $position.w,
                );
                let builder =
                    noise_build_settings!(builder, $frequency, $lacunarity, $gain, $octaves);
                let noise = common_build_settings!(builder, $seed, SCALE_MIN, SCALE_MAX);
                noise_4d_to_frames(noise, $position)
            }
        }
    };
}

fn process_command(
    command: Commands,
    dimension: Dimension,
    seed: i32,
    position: Coordinate<usize>,
    offset: Coordinate<f32>,
) -> Vec<Vec<u32>> {
    match command {
        Commands::Cellular {
            frequency,
            jitter,
            distance,
        } => match dimension {
            Dimension::Two => {
                let mut builder = simdnoise::NoiseBuilder::cellular_2d_offset(
                    offset.x, position.x, offset.y, position.y,
                );
                let builder = cellular_build_settings!(builder, frequency, jitter, distance.into());
                let noise = common_build_settings!(builder, seed, SCALE_MIN, SCALE_MAX);
                noise_2d_to_frames(noise)
            }
            Dimension::Three => {
                let mut builder = simdnoise::NoiseBuilder::cellular_3d_offset(
                    offset.x, position.x, offset.y, position.y, offset.z, position.z,
                );
                let builder = cellular_build_settings!(builder, frequency, jitter, distance.into());
                let noise = common_build_settings!(builder, seed, SCALE_MIN, SCALE_MAX);
                noise_3d_to_frames(noise)
            }
            _ => {
                unimplemented!()
            }
        },

        Commands::FBM {
            frequency,
            lacunarity,
            gain,
            octaves,
        } => process_noise_command!(
            fbm_1d_offset,
            fbm_2d_offset,
            fbm_3d_offset,
            fbm_4d_offset,
            dimension,
            seed,
            position,
            offset,
            frequency,
            lacunarity,
            gain,
            octaves
        ),
        Commands::Ridge {
            frequency,
            lacunarity,
            gain,
            octaves,
        } => process_noise_command!(
            ridge_1d_offset,
            ridge_2d_offset,
            ridge_3d_offset,
            ridge_4d_offset,
            dimension,
            seed,
            position,
            offset,
            frequency,
            lacunarity,
            gain,
            octaves
        ),
        Commands::Turbulence {
            frequency,
            lacunarity,
            gain,
            octaves,
        } => process_noise_command!(
            turbulence_1d_offset,
            turbulence_2d_offset,
            turbulence_3d_offset,
            turbulence_4d_offset,
            dimension,
            seed,
            position,
            offset,
            frequency,
            lacunarity,
            gain,
            octaves
        ),

        Commands::Gradient {} => match dimension {
            Dimension::One => {
                let mut builder = simdnoise::NoiseBuilder::gradient_1d_offset(offset.x, position.x);
                let noise = common_build_settings!(builder, seed, SCALE_MIN, SCALE_MAX);
                let x: Vec<u32> = noise.iter().map(|x| *x as u32).collect();
                let mut xy = Vec::with_capacity(x.len() * position.y);
                for _i in 0..(position.y) {
                    xy.extend_from_slice(x.as_slice());
                }
                vec![xy]
            }
            Dimension::Two => {
                let mut builder = simdnoise::NoiseBuilder::gradient_2d_offset(
                    offset.x, position.x, offset.y, position.y,
                );
                let noise = common_build_settings!(builder, seed, SCALE_MIN, SCALE_MAX);
                noise_2d_to_frames(noise)
            }
            Dimension::Three => {
                let mut builder = simdnoise::NoiseBuilder::gradient_3d_offset(
                    offset.x, position.x, offset.y, position.y, offset.z, position.z,
                );
                let noise = common_build_settings!(builder, seed, SCALE_MIN, SCALE_MAX);
                noise_3d_to_frames(noise)
            }
            Dimension::Four => {
                let mut builder = simdnoise::NoiseBuilder::gradient_4d_offset(
                    offset.x, position.x, offset.y, position.y, offset.z, position.z, offset.w,
                    position.w,
                );
                let noise = common_build_settings!(builder, seed, SCALE_MIN, SCALE_MAX);
                noise_4d_to_frames(noise, position)
            }
        },
    }
}

fn main() {
    let args = Args::parse();
    let width = args.width;
    let height = args.height;
    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(FPS);

    let position = Coordinate::new(args.width, args.height, args.depth, args.time);
    let offset = Coordinate::new_checked(
        args.dimension,
        args.offset_x,
        args.offset_y,
        args.offset_z,
        args.offset_w,
    );
    let buffers = process_command(args.command, args.dimension, args.seed, position, offset);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for buffer in &buffers {
            window.update_with_buffer(&buffer, width, height).unwrap();
        }
    }
}
