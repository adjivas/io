// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate io;

fn main () {
  let mut term = ioctl!();

  term.c_lflag = 0x8a73;
  ioctl!(io::ffi::TCSETS, &mut *term);

  let (len, text):(i64, [i8; io::ffi::BUFF]) = read!().unwrap();

  term.c_lflag = 0x8a3b;
  ioctl!(io::ffi::TCSETAW, &mut *term);
}
