use std::error::Error;
use std::fs::File;

use image::png::PngEncoder;
use image::ColorType;
use num::Complex;

use crate::mndl::core::escape_time;

use super::dimensions::Dimensions;
use super::pixel::*;

/// Render the Mandelbrot image
pub fn render(
    buffer: &mut [u8],
    dimensions: &Dimensions,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(buffer.len() == dimensions.pixel_count());

    for x in 0..dimensions.width() {
        for y in 0..dimensions.height() {
            let complex_point =
                pixel_to_complex_point(Pixel::new(x, y), &dimensions, upper_left, lower_right);

            buffer[(x + (dimensions.width() * y)) as usize] = match escape_time(complex_point, 255)
            {
                Some(count) => 255 - (count as u8),
                None => 0,
            };
        }
    }
}

/// Save the Mandelbrot image to file
pub fn write_image(
    filename: &str,
    buffer: &[u8],
    dimensions: &Dimensions,
) -> Result<(), Box<dyn Error>> {
    let output_file = File::create(filename)?;
    let encoder = PngEncoder::new(output_file);

    encoder.encode(
        &buffer,
        dimensions.width() as u32,
        dimensions.height() as u32,
        ColorType::L8,
    )?;

    Ok(())
}
