use std::mem;
use std::os::unix::io::RawFd;

use nix::libc as libc;

// These are wrapped in a module because they're `pub` by default
mod raw {
    use nix::libc as libc;
    ioctl!(bad none tiocexcl with libc::TIOCEXCL);
    ioctl!(bad none tiocnxcl with libc::TIOCNXCL);
    ioctl!(bad read tiocmget with libc::TIOCMGET; libc::c_int);
    ioctl!(bad write_ptr tiocmbic with libc::TIOCMBIC; libc::c_int);
    ioctl!(bad write_ptr tiocmbis with libc::TIOCMBIS; libc::c_int);
    ioctl!(
        #[cfg(any(target_os = "android", target_os = "linux"))]
        read tcgets2 with b'T', 0x2A; libc::termios2);
    ioctl!(
        #[cfg(any(target_os = "android", target_os = "linux"))]
        write_ptr tcsets2 with b'T', 0x2B; libc::termios2);
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    const IOSSIOSPEED: libc::c_ulong = 0x80045402;
    ioctl!(
        #[cfg(any(target_os = "ios", target_os = "macos"))]
        bad write_ptr iossiospeed with IOSSIOSPEED; libc::speed_t);
}

bitflags!{
    /// Flags to indicate which wires in a serial connection to use
    pub struct SerialLines: libc::c_int {
        const DATA_SET_READY = libc::TIOCM_DSR;
        const DATA_TERMINAL_READY = libc::TIOCM_DTR;
        const REQUEST_TO_SEND = libc::TIOCM_RTS;
        const SECONDARY_TRANSMIT = libc::TIOCM_ST;
        const SECONDARY_RECEIVE = libc::TIOCM_SR;
        const CLEAR_TO_SEND = libc::TIOCM_CTS;
        const DATA_CARRIER_DETECT = libc::TIOCM_CAR;
        const RING = libc::TIOCM_RNG;
    }
}

pub fn tiocexcl(fd: RawFd) -> ::Result<()> {
    unsafe { raw::tiocexcl(fd) }.map(|_| ()).map_err(|e| e.into())
}

pub fn tiocnxcl(fd: RawFd) -> ::Result<()> {
    unsafe { raw::tiocnxcl(fd) }.map(|_| ()).map_err(|e| e.into())
}

pub fn tiocmget(fd: RawFd) -> ::Result<SerialLines> {
    let mut status = unsafe { mem::uninitialized() };
    let x = unsafe { raw::tiocmget(fd, &mut status) };
    x.map(SerialLines::from_bits_truncate).map_err(|e| e.into())
}

pub fn tiocmbic(fd: RawFd, status: SerialLines) -> ::Result<()> {
    let bits = status.bits() as libc::c_int;
    unsafe { raw::tiocmbic(fd, &bits) }.map(|_| ()).map_err(|e| e.into())
}

pub fn tiocmbis(fd: RawFd, status: SerialLines) -> ::Result<()> {
    let bits = status.bits() as libc::c_int;
    unsafe { raw::tiocmbis(fd, &bits) }.map(|_| ()).map_err(|e| e.into())
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn tcgets2(fd: RawFd) -> ::Result<libc::termios2> {
    let mut options = unsafe { mem::uninitialized() };
    match unsafe { raw::tcgets2(fd, &mut options) } {
        Ok(_) => Ok(options),
        Err(e) => Err(e.into()),
    }
}

#[cfg(any(target_os = "android", target_os = "linux"))]
pub fn tcsets2(fd: RawFd, options: &libc::termios2) -> ::Result<()> {
    unsafe { raw::tcsets2(fd, options) }.map(|_| ()).map_err(|e| e.into())
}

#[cfg(any(target_os = "ios", target_os = "macos"))]
pub fn iossiospeed(fd: RawFd, baud_rate: &libc::speed_t) -> ::Result<()> {
    unsafe { raw::iossiospeed(fd, baud_rate) }.map(|_| ()).map_err(|e| e.into())
}
