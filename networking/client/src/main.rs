use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();
    log::info!("Client starting...");
}
