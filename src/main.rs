use ::image as im;
use mandelbrot::{Mandelbrot, Viewport};
use num::Complex;
use piston_window::*;

mod mandelbrot;

const SCREEN_SIZE: (usize, usize) = (1024, 768);

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow =
        WindowSettings::new("Mandelbrot", (SCREEN_SIZE.0 as u32, SCREEN_SIZE.1 as u32))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

    let mut viewport = Viewport::new(
        Complex {
            re: -1.20,
            im: 0.35,
        },
        Complex { re: -1.0, im: 0.20 },
    );
    let mandelbrot = Mandelbrot::new(SCREEN_SIZE);

    let mut canvas = im::ImageBuffer::new(SCREEN_SIZE.0 as u32, SCREEN_SIZE.1 as u32);
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture =
        Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            let pixels = mandelbrot.render(viewport);

            for x in 0..SCREEN_SIZE.0 {
                for y in 0..SCREEN_SIZE.1 {
                    let pixel_color = pixels[y * SCREEN_SIZE.0 + x];
                    canvas.put_pixel(
                        x as u32,
                        y as u32,
                        im::Rgba([pixel_color, pixel_color, pixel_color, 255]),
                    );
                }
            }

            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                texture_context.encoder.flush(device);

                clear([0.0, 0.0, 0.0, 255.0], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::Right) {
                viewport.translate_x(0.01);
            } else if button == Button::Keyboard(Key::Left) {
                viewport.translate_x(-0.01);
            } else if button == Button::Keyboard(Key::Up) {
                viewport.translate_y(0.01);
            } else if button == Button::Keyboard(Key::Down) {
                viewport.translate_y(-0.01);
            } else if button == Button::Keyboard(Key::Z) {
                viewport.zoom(0.9);
            } else if button == Button::Keyboard(Key::X) {
                viewport.zoom(1.1);
            }
        };
    }
}
