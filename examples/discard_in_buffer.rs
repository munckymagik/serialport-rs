//
// Provides a way to test SerialPort::bytes_to_read() and SerialPort::bytes_to_read().
//
// The following Arduino firmware could be used to generate input:
//
// ```
// #include <Arduino.h>
//
// void setup() {
//     Serial.begin(9600);
//     while (!Serial) {
//         ; // wait for serial port to connect. Needed for native USB
//     }
// }
//
// int iter = 0;
//
// void loop() {
//     Serial.print(iter);
//     if (++iter == 10) {
//         Serial.println();
//         iter = 0;
//     }
//     delay(1000 / 20);
// }
// ```

extern crate argparse;
extern crate serialport;

use std::io::{self, Read};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;

use argparse::{ArgumentParser, Store};
use serialport::prelude::*;

fn main() {
    let mut port_name = "".to_string();
    let mut baud_rate = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(
            "Report how many bytes are ready to read and periodically clear the read buffer"
        );
        ap.refer(&mut port_name)
            .add_argument("port", Store, "Port name")
            .required();
        ap.refer(&mut baud_rate)
            .add_argument("baud", Store, "Baud rate")
            .required();
        ap.parse_args_or_exit();
    }

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    if let Ok(rate) = baud_rate.parse::<u32>() {
        settings.baud_rate = rate.into();
    } else {
        println!("Error: Invalid baud rate '{}' specified", baud_rate);
        return;
    }

    if let Ok(port) = serialport::open_with_settings(&port_name, &settings) {
        let chan_clear_buf = input_service();

        println!("Connected to {} at {} baud", &port_name, &baud_rate);
        println!("Ctrl+D (Unix) or Ctrl+Z (Win) to stop. Any other input will clear the buffer.");
        loop {
            match port.bytes_to_read() {
                Ok(n) => println!("Bytes available: {}", n),
                Err(e) => eprintln!("{:?}", e),
            }

            match chan_clear_buf.try_recv() {
                Ok(_) => {
                    println!("Discarding buffer.");
                    port.discard_in_buffer().expect("Failed to discard in buffer")
                }
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => {
                    println!("Stopping.");
                    break;
                },
            }

            thread::sleep(Duration::from_millis(500));
        }
    } else {
        println!("Error: Port '{}' not available", &port_name);
    }
}

fn input_service() -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let mut buffer = [0; 32];
        loop {
            // Block awaiting any user input
            match io::stdin().read(&mut buffer) {
                Ok(0) => {
                    drop(tx); // EOF, drop the channel and stop the thread
                    break;
                }
                Ok(_) => tx.send(()).unwrap(), // Signal main to clear the buffer
                Err(e) => panic!(e),
            }
        }
    });

    rx
}
