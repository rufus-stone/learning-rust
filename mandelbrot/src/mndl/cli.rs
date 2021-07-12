use std::str::FromStr;

use num::Complex;
use structopt::StructOpt;

use super::dimensions::*;

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// Filename to save the image as, e.g. 'image.png'
    #[structopt(short, long, default_value = "mandel.png")]
    pub file: String,

    /// Image resolution as "WIDTHxHEIGHT", e.g. '1000x750'
    #[structopt(short, long, default_value = "800x600", parse(try_from_str = Dimensions::from_str))]
    pub dimensions: Dimensions,

    /// Upper left bounds of Mandelbrot as complex number, e.g. '-1.20,0.35'
    #[structopt(short, long, allow_hyphen_values = true, default_value = "-1.20 + 0.35i", parse(try_from_str = Complex::<f64>::from_str))]
    pub upper_left: Complex<f64>,

    /// Lower right bounds of Mandelbrot as complex number, e.g. '-1.00,0.20'
    #[structopt(short, long, allow_hyphen_values = true, default_value = "-1.00 + 0.20i", parse(try_from_str = Complex::<f64>::from_str))]
    pub lower_right: Complex<f64>,

    /// Lower bounds of Mandelbrot as complex number, e.g. '-1.00,0.20'
    #[structopt(short, long)]
    pub parallel: bool,
}
