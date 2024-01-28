use image::png::PNGEncoder;
use image::ColorType;
use mandelbrot::{Mandelbrot, Viewport};
use num::Complex;
use std::fs::File;

mod mandelbrot;

const SCREEN_SIZE: (usize, usize) = (1024, 768);

fn main() {
    let viewport = Viewport {
        ul: Complex {
            re: -1.20,
            im: 0.35,
        },
        lr: Complex { re: -1.0, im: 0.20 },
    };

    let mandelbrot = Mandelbrot::new(SCREEN_SIZE);
    let pixels = mandelbrot.render(viewport);

    write_image("mandel.png", &pixels, SCREEN_SIZE).expect("error writting PNG file");
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let enconder = PNGEncoder::new(output);
    enconder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
