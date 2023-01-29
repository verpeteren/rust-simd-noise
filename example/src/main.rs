use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() {
    let noise = simdnoise::NoiseBuilder::ridge_3d_offset(1200.0, WIDTH, 200.0, HEIGHT, 1.0, 1)
        .with_freq(1.2)
        .with_octaves(8)
        .generate_scaled(0.0, 255.0);
    let buffer: Vec<u32> = noise.iter().map(|x| *x as u32).collect();
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
