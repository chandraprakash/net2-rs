// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::io;
use std::net::SocketAddr;
use std::mem;
use libc::{self, c_int, socklen_t, sockaddr};

use sys;

pub struct Socket {
    inner: sys::Socket,
}

impl Socket {
    pub fn new(family: c_int, ty: c_int) -> io::Result<Socket> {
        Ok(Socket { inner: try!(sys::Socket::new(family, ty)) })
    }

    pub fn bind(&self, addr: &SocketAddr) -> io::Result<()> {
        let (addr, len) = addr2raw(addr);
        unsafe {
            ::cvt(libc::bind(self.inner.raw(), addr, len)).map(|_| ())
        }
    }

    pub fn listen(&self, backlog: i32) -> io::Result<()> {
        unsafe {
            ::cvt(libc::listen(self.inner.raw(), backlog)).map(|_| ())
        }
    }

    pub fn connect(&self, addr: &SocketAddr) -> io::Result<()> {
        let (addr, len) = addr2raw(addr);
        unsafe {
            ::cvt(libc::connect(self.inner.raw(), addr, len)).map(|_| ())
        }
    }
}

impl ::AsInner for Socket {
    type Inner = sys::Socket;
    fn as_inner(&self) -> &sys::Socket { &self.inner }
}

impl ::FromInner for Socket {
    type Inner = sys::Socket;
    fn from_inner(sock: sys::Socket) -> Socket {
        Socket { inner: sock }
    }
}

impl ::IntoInner for Socket {
    type Inner = sys::Socket;
    fn into_inner(self) -> sys::Socket { self.inner }
}

fn addr2raw(addr: &SocketAddr) -> (*const sockaddr, socklen_t) {
    match *addr {
        SocketAddr::V4(ref a) => {
            (a as *const _ as *const _, mem::size_of_val(a) as socklen_t)
        }
        SocketAddr::V6(ref a) => {
            (a as *const _ as *const _, mem::size_of_val(a) as socklen_t)
        }
    }
}
