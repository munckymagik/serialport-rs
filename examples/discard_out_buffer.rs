// THIS IS A WORK IN PROGRESS - doesn't work yet because out buffer drains too quickly
// Need to find a way to make it buffer and wait.
//
// WIP Arduino sample:
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
//     Serial.print("Bytes in arduino in-buffer: ");
//     Serial.print(Serial.available());
//     Serial.println();
//
//     delay(1000);
// }
// ```

extern crate argparse;
extern crate serialport;

use std::io::{self, Read, BufReader, BufRead};
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
            "Report how many bytes are waiting to be sent and periodically clear the out buffer"
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

    if let Ok(mut port) = serialport::open_with_settings(&port_name, &settings) {
        let (thread_handle, quit_tx) = stream_from_device_service(port.try_clone().unwrap());

        println!("Connected to {} at {} baud", &port_name, &baud_rate);
        println!("Ctrl+D (Unix) or Ctrl+Z (Win) to stop. Any other input will fill the buffer.");

        let mut data = [0, 64];

        loop {
            match io::stdin().read(&mut data) {
                Ok(0) => break,
                Ok(n) => port.write_all(&data[..n]).expect("Error while writing data to the port."),
                Err(e) => panic!(e),
            }

            match port.bytes_to_write() {
                Ok(n) => println!("Bytes queued to send: {}", n),
                Err(e) => panic!(e)
            }
        }


        println!("Stopping main.");
        drop(quit_tx);
        thread_handle.join().unwrap();
    } else {
        println!("Error: Port '{}' not available", &port_name);
    }
}

fn stream_from_device_service<T: 'static + Read + Send>(in_stream: T) -> (thread::JoinHandle<()>, mpsc::Sender<()>) {
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut buffer = String::new();
        let mut reader = BufReader::new(in_stream);
        loop {
            buffer.clear();

            match reader.read_line(&mut buffer) {
                Ok(_) => println!("From device: {}", buffer.trim_right()),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => panic!(e),
            }

            if let Err(mpsc::TryRecvError::Disconnected) = rx.try_recv() {
                println!("Stopping thread.");
                break;
            }
        }
    });

    (handle, tx)
}
