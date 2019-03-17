use crate::io;
use crate::str;
use core::fmt::Write;
use hermit::console::*;

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn new() -> io::Result<Stdin> {
        Ok(Stdin)
    }

    pub fn read(&self, _data: &mut [u8]) -> io::Result<usize> {
        Ok(0)
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> {
        Ok(Stdout)
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        match CONSOLE.lock().write_str(str::from_utf8(data).unwrap()) {
            Err(_err) => Err(io::Error::new(io::ErrorKind::Other, "Stdout is not able to print")),
            _ => Ok(data.len())
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> {
        Ok(Stderr)
    }

    pub fn write(&self, data: &[u8]) -> io::Result<usize> {
        match CONSOLE.lock().write_str(str::from_utf8(data).unwrap()) {
            Err(_err) => Err(io::Error::new(io::ErrorKind::Other, "Stderr is not able to print")),
            _ => Ok(data.len())
        }
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }
}

impl io::Write for Stderr {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        (&*self).write(data)
    }
    fn flush(&mut self) -> io::Result<()> {
        (&*self).flush()
    }
}

pub const STDIN_BUF_SIZE: usize = 0;

pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Stderr::new().ok()
}
