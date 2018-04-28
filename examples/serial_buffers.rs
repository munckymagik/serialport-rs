extern crate serialport;

use std::thread;
use std::time::Duration;

fn main() {
    if let Err(e) = run() {
        println!("ERROR {}", e);
    }
}

fn run() -> serialport::Result<()> {
    #[cfg(windows)]
    let mut serialport = serialport::open("COM6")?;
    #[cfg(not(windows))]
    let mut serialport = serialport::open("/dev/cu.usbmodem1421")?;

    serialport.set_baud_rate(115200)?;
    serialport.set_timeout(Duration::from_millis(500))?;
    stats(&serialport)?;

    serialport.discard_in_buffer()?;
    serialport.discard_out_buffer()?;

    write(&mut serialport, b"\r\n")?;
    println!("Read: {}", read(&mut serialport)?);

    write(&mut serialport, b"hello world\r\n")?;
    println!("Read: {}", read(&mut serialport)?);

    Ok(())
}

fn stats<T: AsRef<serialport::SerialPort>>(serialport: T) -> serialport::Result<()> {
    println!("Bytes to read: {}", serialport.as_ref().bytes_to_read()?);
    println!("Bytes to write: {}", serialport.as_ref().bytes_to_write()?);
    Ok(())
}

fn pause(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn read(serialport: &mut Box<serialport::SerialPort>) -> serialport::Result<String> {
    let mut buf = vec![0; 20];
    println!("Bytes to read: {}", serialport.bytes_to_read()?);
    let bytes_read = serialport.read(&mut buf)?;
    Ok(String::from_utf8_lossy(&buf[..bytes_read]).to_string())
}

fn write(serialport: &mut Box<serialport::SerialPort>, buf: &[u8]) -> serialport::Result<()> {
    serialport.write_all(buf)?;
    println!("Bytes to write: {}", serialport.bytes_to_write()?);
    pause(100);
    Ok(())
}
