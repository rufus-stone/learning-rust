use structopt::StructOpt;

mod mndl;

fn main() {
    let opt = mndl::cli::Opt::from_args();
    println!("Args: {:?}", opt);

    let filename = opt.file;
    let resolution = opt.res;
    let upper = opt.upper;
    let lower = opt.lower;

    let mut buffer = vec![0; resolution.pixel_count()];

    // Let's time how long it takes to calculate
    let start = std::time::Instant::now();

    mndl::img::render(&mut buffer, &resolution, upper, lower);

    let elapsed = std::time::Instant::now().duration_since(start);
    println!("Elapsed: {} seconds", elapsed.as_secs_f32());

    mndl::img::write_image(&filename, &buffer, &resolution).expect("Error writing image!");
}
