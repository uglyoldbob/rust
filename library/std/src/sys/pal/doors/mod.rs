#![deny(unsafe_op_in_unsafe_fn)]
#![allow(dead_code)]
#![allow(unused_imports)]

use crate::io;

pub mod os;
pub mod pipe;
pub mod time;

mod common;
pub use common::*;

pub fn cvt<T>(_t: T) -> io::Result<T> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "system calls not supported on doors"))
}

pub fn cvt_r<T>(_t: T) -> io::Result<T> {
    Err(io::const_error!(io::ErrorKind::Unsupported, "system calls not supported on doors"))
}

pub mod net;
pub use net::Socket;
