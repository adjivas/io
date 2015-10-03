// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.


#![allow(non_camel_case_types)]

extern crate libc;
extern crate readline;

pub type c_int = libc::types::os::arch::c95::c_int;
pub type c_char = libc::types::os::arch::c95::c_char;
pub type size_t = libc::types::os::arch::c95::size_t;
pub type ssize_t = libc::types::os::arch::posix88::ssize_t;

extern "C" {
  pub fn write(fd: c_int, buf: *const c_char, len: size_t) -> ssize_t;
}

macro_rules! write {
  ($text: expr, $len: expr) => ({
    write!($text, $len, 1)
  });
  ($text: expr, $len: expr, $out: expr) => ({
    match unsafe {
      write($out, $text as *const c_char, $len as size_t)
    } {
      -1 => None,
      xsi => Some(xsi as i32),
    }
  });
}

fn main () {
  let (len, line) = readline::line().unwrap();

  write!(&line, len);
}
