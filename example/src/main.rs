use std::fmt::{Display, Formatter, Result};

use clap::{Parser, Subcommand, ValueEnum};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

const OFFSET_X: f32 = 1200.0;
const OFFSET_Y: f32 = 200.0;
const OFFSET_Z: f32 = 1.0;
const SCALE_MIN: f32 = 0.0;
const SCALE_MAX: f32 = 255.0;
const DEPTH: usize = 1;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = WIDTH, help="The width of the generated image", global=true)]
    pub width: usize,

    #[clap(long, value_parser, default_value_t = HEIGHT, help="The height of the generated image", global=true)]
    pub height: usize,

    #[clap(long, value_parser, default_value_t = Dimension::Three, help="The number of dimensions of the generated noice", global=true)]
    pub dimension: Dimension,

    #[clap(
        long,
        value_parser,
        default_value_t = false,
        help = "Use an offset",
        global = true
    )]
    pub offset: bool,

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

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Ridge {
        #[arg(short, long, value_parser, default_value_t = 1.2)]
        frequency: f32,
        #[arg(short, long, value_parser, default_value_t = 8)]
        octaves: u8,
        //todo scaled
    },
}

fn main() {
    let args = Args::parse();
    let width = args.width;
    let height = args.height;
    let dimension = args.dimension;
    let offset = args.offset;
    let noise = match (args.command, dimension, offset) {
        (Commands::Ridge { frequency, octaves }, Dimension::Three, true) => {
            simdnoise::NoiseBuilder::ridge_3d_offset(
                OFFSET_X, width, OFFSET_Y, height, OFFSET_Z, DEPTH,
            )
            .with_freq(frequency)
            .with_octaves(octaves)
            .generate_scaled(SCALE_MIN, SCALE_MAX)
        }
        _ => {
            unimplemented!();
        }
    };
    let buffer: Vec<u32> = noise.iter().map(|x| *x as u32).collect();
    let mut window = Window::new(
        "Test - ESC to exit",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, width, height).unwrap();
    }
}
