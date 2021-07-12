use num::Complex;

use super::dimensions::Dimensions;

pub struct Pixel {
    pub x: usize,
    pub y: usize,
}

impl Pixel {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

/// For the pixel at the specified x and y position in the image, return the corresponding point on the complex plane
pub fn pixel_to_complex_point(
    pixel: Pixel,
    dimensions: &Dimensions,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    Complex {
        re: upper_left.re + pixel.x as f64 * width / dimensions.width() as f64,
        im: upper_left.im - pixel.y as f64 * height / dimensions.height() as f64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pxl_2_cmplx_point() {
        let pixel = Pixel::new(25, 175);
        let dimensions = Dimensions::new(100, 200);
        let upper_left = Complex { re: -1.0, im: 1.0 };
        let lower_right = Complex { re: 1.0, im: -1.0 };

        let point = pixel_to_complex_point(pixel, &dimensions, upper_left, lower_right);

        assert_eq!(
            point,
            Complex {
                re: -0.5,
                im: -0.75
            }
        );
    }
}
