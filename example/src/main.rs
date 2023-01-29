use clap::{Parser, Subcommand};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser, default_value_t = WIDTH, help="The width of the generated image", global=true)]
    pub width: usize,

    #[clap(long, value_parser, default_value_t = HEIGHT, help="The height of the generated image", global=true)]
    pub height: usize,

    #[command(subcommand)]
    command: Commands,
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
    let noise = match args.command {
        Commands::Ridge { frequency, octaves } => {
            simdnoise::NoiseBuilder::ridge_3d_offset(1200.0, width, 200.0, height, 1.0, 1)
                .with_freq(frequency)
                .with_octaves(octaves)
                .generate_scaled(0.0, 255.0)
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
