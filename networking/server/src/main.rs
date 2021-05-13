use serialport::{SerialPortInfo, SerialPortType};
use simple_logger::SimpleLogger;

use tokio::net::UdpSocket;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    SimpleLogger::new().init().unwrap();

    // First, set up the serial port
    let ports = serialport::available_ports().expect("Error looking for serial ports!");
    if ports.is_empty() {
        log::warn!("No usable serial ports found!");
    }

    for p in &ports {
        log::info!("{:?}", p);
    }

    // Look for connected Arduinos
    let arduino_serials = ports
        .into_iter()
        .filter(|port| {
            if let SerialPortType::UsbPort(usb_port_info) = &port.port_type {
                match &usb_port_info.manufacturer {
                    Some(manufacturer) => manufacturer.contains("Arduino"),
                    None => false,
                }
            } else {
                false
            }
        })
        .collect::<Vec<SerialPortInfo>>();

    // Only allow a single connected Arduino
    if arduino_serials.len() > 1 {
        log::warn!(
            "Found {} different Arduinos! Make sure only one is connected and try again...",
            arduino_serials.len()
        );
        std::process::exit(0);
    }

    // Get the Arduino serial port
    let arduino_serial = arduino_serials.first().unwrap();
    log::info!(
        "Arduino detected on serial port: {:?}",
        &arduino_serial.port_name
    );

    // Open the serial connection to the Arduino
    log::info!("Opening serial connection...");
    let mut port = serialport::new(&arduino_serial.port_name, 115200)
        .timeout(std::time::Duration::from_millis(10))
        .open()
        .expect("Failed to open port!");

    // Send a test message
    log::info!("Sending PING...");
    let buf = "PING\n".as_bytes();
    port.write_all(buf).expect("Write failed!");

    let udp_socket = UdpSocket::bind("0.0.0.0:47563").await?;

    log::info!(
        "Server started listening on {}",
        udp_socket.local_addr().unwrap()
    );

    loop {
        let mut data = [0; 1024];
        let valid_bytes = udp_socket.recv(&mut data).await?;
        let data = &data[..valid_bytes];

        log::info!(
            "Read {} bytes: {}",
            data.len(),
            std::str::from_utf8(&data).unwrap()
        );

        if !data.is_empty() {
            log::info!("Forwarding msg to Arduino...");
            let msg = std::str::from_utf8(&data).unwrap();

            // Send it to the Arduino
            let buf = msg.as_bytes();
            port.write_all(buf).expect("Write failed!");
        }
    }
}
