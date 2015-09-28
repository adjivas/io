// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod macros;
mod ffi;

/// The `write` macro writes the message and
/// returns None if fails or Some(0).

#[macro_export]
macro_rules! write {
  ($text: expr, $len: expr) => ({
    write!($text, $len, 1)
  });
  ($text: expr, $len: expr, $out: expr) => ({
    match unsafe {
      ffi::write($out, $text as *const ffi::c_char, $len as ffi::size_t)
    } {
      -1 => None,
      xsi => Some(xsi as i32),
    }
  });
}

fn main () {
  let mut term = ioctl!();

  term.c_lflag = 0x8a73;
  ioctl!(ffi::TCSETS, &mut *term);

  let (len, text):(i64, [ffi::c_char; ffi::BUFF]) = read!().unwrap();

  term.c_lflag = 0x8a3b;
  ioctl!(ffi::TCSETAW, &mut *term);

  write!(&text, len);
}
