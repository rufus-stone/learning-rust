use structopt::StructOpt;

use crate::mndl::{dimensions::Dimensions, pixel::*};

mod mndl;

fn main() {
    let opt = mndl::cli::Opt::from_args();
    println!("Args: {:?}", opt);

    let filename = opt.file;
    let dimensions = opt.dimensions;
    let upper_left = opt.upper_left;
    let lower_right = opt.lower_right;

    // We'll use this buffer to store the pixel values for our image
    let mut buffer = vec![0; dimensions.pixel_count()];

    // Are we running the calculations in parallel?
    match opt.parallel {
        // Run on multiple threads
        true => {
            let threads = num_cpus::get();
            let rows_per_band = (dimensions.height() as usize / threads) + 1; // +1 to ensure we don't end up chopping anything off should the height not be a multiple of the thread count

            {
                let bands: Vec<&mut [u8]> = buffer
                    .chunks_mut(rows_per_band * (dimensions.width() as usize))
                    .collect();

                // Let's time how long it takes to calculate
                let start = std::time::Instant::now();

                // Chop the image up into bands and process each one on a separate thread
                crossbeam::scope(|scope| {
                    for (idx, band) in bands.into_iter().enumerate() {
                        let top = rows_per_band * idx;
                        let height = band.len() / (dimensions.width() as usize);
                        let band_dimensions = Dimensions::new(dimensions.width(), height);
                        let band_upper_left = pixel_to_complex_point(
                            Pixel::new(0, top),
                            &dimensions,
                            upper_left,
                            lower_right,
                        );
                        let band_lower_right = pixel_to_complex_point(
                            Pixel::new(dimensions.width(), top + height),
                            &dimensions,
                            upper_left,
                            lower_right,
                        );

                        scope.spawn(move |_| {
                            mndl::img::render(
                                band,
                                &band_dimensions,
                                band_upper_left,
                                band_lower_right,
                            );
                        });
                    }
                })
                .expect("Something has gone horribly wrong!");

                let elapsed = std::time::Instant::now().duration_since(start);
                println!("Elapsed: {} seconds", elapsed.as_secs_f32());
            }
        }
        // Run single-threaded
        false => {
            // Let's time how long it takes to calculate
            let start = std::time::Instant::now();

            mndl::img::render(&mut buffer, &dimensions, upper_left, lower_right);

            let elapsed = std::time::Instant::now().duration_since(start);
            println!("Elapsed: {} seconds", elapsed.as_secs_f32());
        }
    }

    // Finally, save the image
    mndl::img::write_image(&filename, &buffer, &dimensions).expect("Error writing image!");
}
