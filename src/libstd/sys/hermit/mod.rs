//! System bindings for HermitCore
//!
//! This module contains the facade (aka platform-specific) implementations of
//! OS level functionality for HermitCore.
//!
//! This is all super highly experimental and not actually intended for
//! wide/production use yet, it's still all in the experimental category. This
//! will likely change over time.
//!
//! Currently all functions here are basically stubs that immediately return
//! errors. The hope is that with a portability lint we can turn actually just
//! remove all this and just omit parts of the standard library if we're
//! compiling for wasm. That way it's a compile time error for something that's
//! guaranteed to be a runtime error!

use crate::os::raw::c_char;

pub mod alloc;
pub mod args;
#[cfg(feature = "backtrace")]
pub mod backtrace;
pub mod condvar;
pub mod stdio;
pub mod memchr;
pub mod io;
pub mod mutex;
pub mod rwlock;
pub mod os;
pub mod os_str;
pub mod cmath;
pub mod thread;
pub mod env;
pub mod fs;
pub mod net;
pub mod path;
pub mod pipe;
pub mod process;
pub mod stack_overflow;
pub mod time;
pub mod thread_local;
pub mod fast_thread_local;

use core::{mem,slice};

pub fn unsupported<T>() -> crate::io::Result<T> {
    Err(unsupported_err())
}

pub fn unsupported_err() -> crate::io::Error {
    crate::io::Error::new(crate::io::ErrorKind::Other,
           "operation not supported on hermit yet")
}

pub fn decode_error_kind(_code: i32) -> crate::io::ErrorKind {
    crate::io::ErrorKind::Other
}

// This enum is used as the storage for a bunch of types which can't actually
// exist.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum Void {}

pub unsafe fn strlen(mut s: *const c_char) -> usize {
    let mut n = 0;
    while *s != 0 {
        n += 1;
        s = s.offset(1);
    }
    return n
}

pub unsafe fn abort_internal() -> ! {
    extern "C" {
        fn sys_abort() ->!;
    }

    sys_abort();
}

// TODO: just a workaround to test the system
pub fn hashmap_random_keys() -> (u64, u64) {
    (1, 2)
}

#[cfg(not(test))]
pub fn init() {
}

unsafe fn run_init_array(
    init_array_start: &extern "C" fn(),
    init_array_end: &extern "C" fn(),
) {
    let n = (init_array_end as *const _ as usize -
        init_array_start as *const _ as usize) /
        mem::size_of::<extern "C" fn()>();

    for f in slice::from_raw_parts(init_array_start, n) {
        f();
    }
}

#[no_mangle]
pub fn libc_start(_argc: i32, _argv: *mut *mut u8, _env: *mut *mut u8) -> ! {
    extern "C" {
        fn main();
        fn sys_exit(arg: i32) ->!;
        //fn _init();

        #[linkage = "extern_weak"]
        static __preinit_array_start: *const u8;
        #[linkage = "extern_weak"]
        static __preinit_array_end: *const u8;
        #[linkage = "extern_weak"]
        static __init_array_start: *const u8;
        #[linkage = "extern_weak"]
        static __init_array_end: *const u8;
    }

    unsafe {
        // run preinit array
        if __preinit_array_end as usize - __preinit_array_start as usize > 0 {
            run_init_array(mem::transmute::<&*const u8, &extern "C" fn()>(&__preinit_array_start), mem::transmute::<&*const u8, &extern "C" fn()>(&__preinit_array_end));
        }

        //_init();

        // run init array
        if __init_array_end as usize - __init_array_start as usize > 0 {
            run_init_array(mem::transmute::<&*const u8, &extern "C" fn()>(&__init_array_start), mem::transmute::<&*const u8, &extern "C" fn()>(&__init_array_end));
        }

        main();

        sys_exit(0);
    }
}
