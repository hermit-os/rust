#![allow(dead_code)]

use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Shutdown, SocketAddr};
use crate::sys::fd::{RawFd, FileDesc, AsRawFd};
use crate::sys_common::FromInner;
use crate::sys_common::IntoInner;
use crate::sys_common::AsInner;
use crate::time::Duration;

#[allow(unused_extern_crates)]
pub extern crate hermit_abi as netc;

pub use crate::sys::{cvt, cvt_r};

pub type wrlen_t = usize;

pub fn cvt_gai(err: i32) -> io::Result<()> {
    if err == 0 {
        return Ok(());
    }

    let detail = "";

    Err(io::Error::new(
        io::ErrorKind::Uncategorized,
        &format!("failed to lookup address information: {detail}")[..],
    ))
}

/// Checks whether the HermitCore's socket interface has been started already, and
/// if not, starts it.
pub fn init() {
    if unsafe { netc::network_init() } < 0 {
        panic!("Unable to initialize network interface");
    }
}

#[derive(Debug, Clone)]
pub struct Socket(FileDesc);

impl Socket {
    pub fn new(addr: &SocketAddr, ty: i32) -> io::Result<Socket> {
        let fam = match *addr {
            SocketAddr::V4(..) => netc::AF_INET,
            SocketAddr::V6(..) => netc::AF_INET6,
        };
        Socket::new_raw(fam, ty)
    }

    pub fn new_raw(fam: i32, ty: i32) -> io::Result<Socket> {
        let fd = cvt(unsafe { netc::socket(fam, ty, 0) })?;
        Ok(Socket(FileDesc::new(fd)))
    }

    pub fn new_pair(_fam: i32, _ty: i32) -> io::Result<(Socket, Socket)> {
        unimplemented!()
    }

    pub fn connect_timeout(&self, _addr: &SocketAddr, _timeout: Duration) -> io::Result<()> {
        unimplemented!()
    }

    pub fn accept(&self, storage: *mut netc::sockaddr, len: *mut netc::socklen_t) -> io::Result<Socket> {
        let fd = cvt(unsafe { netc::accept(self.0.as_raw_fd(), storage, len) })?;
        Ok(Socket(FileDesc::new(fd)))
    }

    pub fn duplicate(&self) -> io::Result<Socket> {
        unimplemented!()
    }

    fn recv_with_flags(&self, _buf: &mut [u8], _flags: i32) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn read(&self, _buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn peek(&self, _buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unimplemented!()
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        unimplemented!()
    }

    pub fn recv_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unimplemented!()
    }

    pub fn peek_from(&self, _buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unimplemented!()
    }

    pub fn write(&self, _buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unimplemented!()
    }

    pub fn is_write_vectored(&self) -> bool {
        unimplemented!()
    }

    pub fn set_timeout(&self, _dur: Option<Duration>, _kind: i32) -> io::Result<()> {
        unimplemented!()
    }

    pub fn timeout(&self, _kind: i32) -> io::Result<Option<Duration>> {
        unimplemented!()
    }

    pub fn shutdown(&self, _how: Shutdown) -> io::Result<()> {
        unimplemented!()
    }

    pub fn set_linger(&self, _linger: Option<Duration>) -> io::Result<()> {
        unimplemented!()
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        unimplemented!()
    }

    pub fn set_nodelay(&self, _nodelay: bool) -> io::Result<()> {
        unimplemented!()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        unimplemented!()
    }

    pub fn set_nonblocking(&self, _nonblocking: bool) -> io::Result<()> {
        unimplemented!()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unimplemented!()
    }

    // This is used by sys_common code to abstract over Windows and Unix.
    pub fn as_raw(&self) -> RawFd {
        self.as_raw_fd()
    }
}

impl AsInner<FileDesc> for Socket {
    fn as_inner(&self) -> &FileDesc {
        &self.0
    }
}

impl IntoInner<FileDesc> for Socket {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl FromInner<FileDesc> for Socket {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}

/*impl AsFd for Socket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}*/

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

/*impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for Socket {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(FromRawFd::from_raw_fd(raw_fd))
    }
}*/