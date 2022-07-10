#![stable(feature = "rust1", since = "1.0.0")]

use crate::fs;
use crate::net::{TcpListener, TcpStream, UdpSocket};
use crate::sys::net;
use crate::sys_common::{IntoInner, AsInner, AsInnerMut, FromInner};
use hermit_abi as abi;

#[stable(feature = "rust1", since = "1.0.0")]
pub type RawSocket = abi::Socket;

/// convert a std type into a hermit-abi socket without passing ownership
#[stable(feature = "rust1", since = "1.0.0")]
pub trait AsRawSocket {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn as_raw_socket(&self) -> RawSocket;
}

/// convert a hermit-abi socket into a std type
#[stable(feature = "rust1", since = "1.0.0")]
pub trait FromRawSocket {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn from_raw_socket(socket: abi::Socket) -> Self;
}

/// convert a std type into a hermit-abi socket while passing ownership
#[stable(feature = "rust1", since = "1.0.0")]
pub trait IntoRawSocket {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn into_raw_socket(self) -> RawSocket;
}

#[stable(feature = "rust1", since = "1.0.0")]
impl FromRawSocket for TcpStream {
    fn from_raw_socket(socket: RawSocket) -> Self {
        TcpStream::from_inner(net::TcpStream::from_raw_socket(socket))
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl AsRawSocket for TcpStream {
    fn as_raw_socket(&self) -> RawSocket {
        let inner = self.as_inner().clone();
        inner.into_socket()
    }
}

#[stable(feature = "rust1", since = "1.0.0")]
impl IntoRawSocket for TcpStream {
    fn into_raw_socket(self) -> RawSocket {
        self.into_inner().into_socket()
    }
}