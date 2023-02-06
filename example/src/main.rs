use std::fmt::{Display, Formatter, Result};

use clap::{Parser, Subcommand, ValueEnum};
use minifb::{Key, Window, WindowOptions};
use simdnoise::CellDistanceFunction;

const FPS: u64 = 60;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const DEPTH: usize = 1;
const TIME: usize = 5;

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

    #[clap(long, value_parser, default_value_t = Dimension::Three, help="The number of dimensions of the generated noice", global=true)]
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
        #[arg(short, long, value_parser, default_value_t = 1.2)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = 1.2)]
        jitter: f32,
        //@TODO: index0/1
    },
    #[command(arg_required_else_help = true)]
    Ridge {
        #[arg(short, long, value_parser, default_value_t = 1.2)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = 8)]
        octaves: u8,
    },
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

fn main() {
    let args = Args::parse();
    let position = Coordinate::new(args.width, args.height, args.depth, args.time);
    let offset = Coordinate::new_checked(
        args.dimension,
        args.offset_x,
        args.offset_y,
        args.offset_z,
        args.offset_w,
    );
    macro_rules! common_build_settings {
        ($builder: expr, $seed : expr, $scale_min: expr, $scale_max: expr) => {
            $builder
                .with_seed($seed)
                .generate_scaled($scale_min, $scale_max)
        };
    }
    macro_rules! noise_build_settings {
        ($builder: expr, $frequency: expr, $octaves: expr/* TODO: and more */) => {
            $builder.with_freq($frequency).with_octaves($octaves)
        };
    }
    let buffer: Vec<u32> = match args.command {
        Commands::Cellular {
            frequency,
            jitter,
            distance,
        } => {
            macro_rules! cellular_build_settings {
                ($builder: expr, $freq: expr, $jitter: expr, $distance: expr) => {
                    $builder
                        .with_freq($freq)
                        .with_jitter($jitter)
                        .with_distance_function($distance)
                };
            }
            let noise = match args.dimension {
                Dimension::Two => {
                    let mut builder = simdnoise::NoiseBuilder::cellular_2d_offset(
                        offset.x, position.x, offset.y, position.y,
                    );
                    let builder =
                        cellular_build_settings!(builder, frequency, jitter, distance.into());
                    common_build_settings!(builder, args.seed, SCALE_MIN, SCALE_MAX)
                }
                Dimension::Three => {
                    let mut builder = simdnoise::NoiseBuilder::cellular_3d_offset(
                        offset.x, position.x, offset.y, position.y, offset.z, position.z,
                    );
                    let builder =
                        cellular_build_settings!(builder, frequency, jitter, distance.into());
                    common_build_settings!(builder, args.seed, SCALE_MIN, SCALE_MAX)
                }
                _ => {
                    unimplemented!()
                }
            };
            noise.iter().map(|x| *x as u32).collect()
        }
        Commands::Ridge { frequency, octaves } => {
            let noise = match args.dimension {
                Dimension::One => {
                    let mut builder =
                        simdnoise::NoiseBuilder::ridge_1d_offset(offset.x, position.x);
                    let builder = noise_build_settings!(builder, frequency, octaves);
                    let noise = common_build_settings!(builder, args.seed, SCALE_MIN, SCALE_MAX);
                    let x: Vec<u32> = noise.iter().map(|x| *x as u32).collect();
                    let mut xy = Vec::with_capacity(x.len() * position.y);
                    for _i in 0..(position.y) {
                        xy.extend_from_slice(x.as_slice());
                    }
                    xy
                }
                Dimension::Two => {
                    let mut builder = simdnoise::NoiseBuilder::ridge_2d_offset(
                        offset.x, position.x, offset.y, position.y,
                    );
                    let builder = noise_build_settings!(builder, frequency, octaves);
                    let noise = common_build_settings!(builder, args.seed, SCALE_MIN, SCALE_MAX);
                    noise.iter().map(|x| *x as u32).collect()
                }
                Dimension::Three => {
                    let mut builder = simdnoise::NoiseBuilder::ridge_3d_offset(
                        offset.x, position.x, offset.y, position.y, offset.z, position.z,
                    );
                    let builder = noise_build_settings!(builder, frequency, octaves);
                    let noise = common_build_settings!(builder, args.seed, SCALE_MIN, SCALE_MAX);
                    noise.iter().map(|x| *x as u32).collect()
                }
                Dimension::Four => {
                    let mut builder = simdnoise::NoiseBuilder::ridge_4d_offset(
                        offset.x, position.x, offset.y, position.y, offset.z, position.z, offset.w,
                        position.z,
                    );
                    let builder = noise_build_settings!(builder, frequency, octaves);
                    let noise = common_build_settings!(builder, args.seed, SCALE_MIN, SCALE_MAX);
                    noise.iter().map(|x| *x as u32).collect() // TODO: probably animate
                }
            };
            noise
        }
    };
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
    let refresh_interval = 1_000_000 / FPS;
    window.limit_update_rate(Some(std::time::Duration::from_micros(refresh_interval)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
