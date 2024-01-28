use num::Complex;

#[derive(Debug)]
pub struct Mandelbrot {
    bounds: (usize, usize),
}

impl Mandelbrot {
    pub fn new(bounds: (usize, usize)) -> Mandelbrot {
        Mandelbrot { bounds }
    }

    pub fn render(&self, viewport: Viewport) -> Vec<u8> {
        let threads = 8;
        let mut pixels = vec![0; self.bounds.0 * self.bounds.1];
        let rows_per_band = self.bounds.1 / threads + 1;

        {
            let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * self.bounds.0).collect();
            crossbeam::scope(|spawner| {
                for (i, band) in bands.into_iter().enumerate() {
                    let top = rows_per_band * i;
                    let height = band.len() / self.bounds.0;
                    let band_bounds = (self.bounds.0, height);
                    let band_upper_left =
                        self.pixel_to_point(self.bounds, (0, top), viewport.ul, viewport.lr);
                    let band_lower_right = self.pixel_to_point(
                        self.bounds,
                        (self.bounds.0, top + height),
                        viewport.ul,
                        viewport.lr,
                    );
                    spawner.spawn(move |_| {
                        self.partial_render(band, band_bounds, band_upper_left, band_lower_right);
                    });
                }
            })
            .unwrap();
        }
        return pixels;
    }

    /// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
    /// iterations to decide.
    ///
    /// If `c` is not a member, return `Some(i)`, where `i` is the number of
    /// iterations it took for `c` to leave the circle of radius 2 centered on the
    /// origin. If `c` seems to be a member (more precisely, if we reached the
    /// iteration limit without being able to prove that `c` is not a member),
    /// return `None`.
    fn escape_time(&self, c: Complex<f64>, limit: usize) -> Option<usize> {
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..limit {
            if z.norm_sqr() > 4.0 {
                return Some(i);
            }
            z = z * z + c;
        }
        None
    }

    /// Given the row and column of a pixel in the output image, return the
    /// corresponding point on the complex plane.
    ///
    /// `bounds` is a pair giving the width and height of the image in pixels.
    /// `pixel` is a (column, row) pair indicating a particular pixel in that image.
    /// The `upper_left` and `lower_right` parameters are points on the complex
    /// plane designating the area our image covers.
    fn pixel_to_point(
        &self,
        bounds: (usize, usize),
        pixel: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) -> Complex<f64> {
        let (width, height) = (
            lower_right.re - upper_left.re,
            upper_left.im - lower_right.im,
        );

        Complex {
            re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
            im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
        }
    }

    /// Render a rectangle of the Mandelbrot set into a buffer of pixels.
    ///
    /// The `bounds` argument gives the width and height of the buffer `pixels`,
    /// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
    /// arguments specify points on the complex plane corresponding to the upper-
    /// left and lower-right corners of the pixel buffer.
    fn partial_render(
        &self,
        pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) {
        assert!(pixels.len() == bounds.0 * bounds.1);

        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let point = self.pixel_to_point(bounds, (column, row), upper_left, lower_right);
                pixels[row * bounds.0 + column] = match self.escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    pub ul: Complex<f64>,
    pub lr: Complex<f64>,
}

impl Viewport {
    pub fn new(ul: Complex<f64>, lr: Complex<f64>) -> Viewport {
        Viewport { ul, lr }
    }

    pub fn zoom(&mut self, factor: f64) {
        let center_x = (self.ul.im + self.lr.im) / 2.0;
        let center_y = (self.ul.re + self.lr.re) / 2.0;

        let width = self.lr.im - self.ul.im;
        let height = self.lr.re - self.ul.re;

        let new_width = width * factor;
        let new_height = height * factor;

        self.ul.im = center_x - new_width / 2.0;
        self.ul.re = center_y - new_height / 2.0;

        self.lr.im = center_x + new_width / 2.0;
        self.lr.re = center_y + new_height / 2.0;
    }

    pub fn translate_x(&mut self, factor: f64) {
        let width = self.lr.im - self.ul.im;
        self.ul.re += width * factor;
        self.lr.re += width * factor;
    }

    pub fn translate_y(&mut self, factor: f64) {
        let height = self.lr.re - self.ul.re;
        self.ul.im += height * factor;
        self.lr.im += height * factor;
    }
}
