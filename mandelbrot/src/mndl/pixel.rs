use num::Complex;

use super::resolution::Resolution;

pub struct Pixel {
    pub x: u32,
    pub y: u32,
}

impl Pixel {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

/// For the pixel at the specified x and y position in the image, return the corresponding point on the complex plane
pub fn pixel_to_complex_point(
    pixel: Pixel,
    resolution: &Resolution,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.x as f64 * width / resolution.width() as f64,
        im: upper_left.im - pixel.y as f64 * height / resolution.height() as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pxl_2_cmplx_point() {
        let pixel = Pixel::new(25, 175);
        let resolution = Resolution::new(100, 200);
        let upper_left = Complex { re: -1.0, im: 1.0 };
        let lower_right = Complex { re: 1.0, im: -1.0 };

        let point = pixel_to_complex_point(pixel, &resolution, upper_left, lower_right);

        assert_eq!(
            point,
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}
