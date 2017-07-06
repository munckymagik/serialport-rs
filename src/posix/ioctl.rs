use std::{io, mem};
use std::os::unix::io::RawFd;

use libc;

pub fn tiocexcl(fd: RawFd) -> ::Result<()> {
    match unsafe { libc::ioctl(fd, libc::TIOCEXCL) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error().into()),
    }
}

pub fn tiocnxcl(fd: RawFd) -> ::Result<()> {
    match unsafe { libc::ioctl(fd, libc::TIOCNXCL) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error().into()),
    }
}

pub fn tiocmget(fd: RawFd) -> ::Result<libc::c_int> {
    let mut modem_status = unsafe { mem::uninitialized() };
    match unsafe { libc::ioctl(fd, libc::TIOCMGET, &mut modem_status) } {
        0 => Ok(modem_status),
        _ => Err(io::Error::last_os_error().into()),
    }
}

pub fn tiocmset(fd: RawFd, modem_status: libc::c_int) -> ::Result<()> {
    match unsafe { libc::ioctl(fd, libc::TIOCMSET, &modem_status) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error().into()),
    }
}

pub fn tiocmbic(fd: RawFd, modem_status: libc::c_int) -> ::Result<()> {
    match unsafe { libc::ioctl(fd, libc::TIOCMBIC, &modem_status) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error().into()),
    }
}

pub fn tiocmbis(fd: RawFd, modem_status: libc::c_int) -> ::Result<()> {
    match unsafe { libc::ioctl(fd, libc::TIOCMBIS, &modem_status) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error().into()),
    }
}
