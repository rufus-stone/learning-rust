use std::error::Error;
use std::fs::File;

use image::png::PngEncoder;
use image::ColorType;
use num::Complex;

use crate::mndl::core::escape_time;

use super::pixel::*;
use super::resolution::Resolution;

/// Render the Mandelbrot image
pub fn render(
    buffer: &mut [u8],
    resolution: &Resolution,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(buffer.len() == resolution.pixel_count());

    for x in 0..resolution.width() {
        for y in 0..resolution.height() {
            let complex_point =
                pixel_to_complex_point(Pixel::new(x, y), &resolution, upper_left, lower_right);

            buffer[(x + (resolution.width() * y)) as usize] = match escape_time(complex_point, 255)
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
    resolution: &Resolution,
) -> Result<(), Box<dyn Error>> {
    let output_file = File::create(filename)?;
    let encoder = PngEncoder::new(output_file);

    encoder.encode(
        &buffer,
        resolution.width(),
        resolution.height(),
        ColorType::L8,
    )?;

    Ok(())
}
