#![allow(dead_code)]

use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Shutdown, SocketAddr};
use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::time::Duration;

pub struct Socket(());

impl Socket {
    pub fn new(_family: i32, _ty: i32) -> io::Result<Socket> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn new_raw(_family: i32, _ty: i32) -> io::Result<Socket> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn new_pair(_family: i32, _ty: i32) -> io::Result<(Socket, Socket)> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn connect_timeout(&self, _addr: &SocketAddr, _timeout: Duration) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn accept(&self, _storage: *mut u8, _len: *mut u32) -> io::Result<Socket> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn duplicate(&self) -> io::Result<Socket> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn shutdown(&self, _how: Shutdown) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn set_linger(&self, _linger: Option<Duration>) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        Ok(None)
    }

    pub fn recv_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn send_to(&self, _buf: &[u8], _addr: &SocketAddr) -> io::Result<usize> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn set_ttl(&self, _ttl: u32) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn ttl(&self) -> io::Result<u32> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn set_only_v6(&self, _only_v6: bool) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn connect(&self, _addr: &SocketAddr) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn listen(&self, _backlog: i32) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }

    pub fn bind(&self, _addr: &SocketAddr) -> io::Result<()> {
        Err(io::const_error!(
            io::ErrorKind::Unsupported,
            "socket operations not supported on doors"
        ))
    }
}

impl AsInner<()> for Socket {
    #[inline]
    fn as_inner(&self) -> &() {
        &self.0
    }
}

impl IntoInner<()> for Socket {
    fn into_inner(self) -> () {
        self.0
    }
}

impl FromInner<()> for Socket {
    fn from_inner(_inner: ()) -> Socket {
        Socket(())
    }
}

impl fmt::Debug for Socket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Socket").finish()
    }
}
