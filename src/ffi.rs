// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]

extern crate libc;

/// The `libc` type is the list of C types.

pub type off_t = libc::types::os::arch::posix88::off_t;
pub type ssize_t = libc::types::os::arch::posix88::ssize_t;
pub type size_t = libc::types::os::arch::c95::size_t;
pub type c_int = libc::types::os::arch::c95::c_int;
pub type c_char = libc::types::os::arch::c95::c_char;

/// The `BUFF` const are default values
/// for macros.

pub const BUFF: usize = 3;

/// The `C` extern is list of libc functions required
/// by the library.

#[allow(dead_code)]
#[cfg(any(unix))]
extern "C" {
  pub fn lseek(fd: c_int, offset: off_t , whence: c_int) -> off_t;
  pub fn read(fd: c_int, buf: *mut c_char, len: size_t) -> ssize_t;
}
