// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(ptr_as_ref)]

#[macro_use]
pub mod macros;
pub mod ffi;

/// The `text` function reads and returns None
/// or the tuple (len, line).

pub fn text (
) -> Option<(i64, [ffi::c_char; ffi::BUFF])> {
  let line: [ffi::c_char; ffi::BUFF] = [0; ffi::BUFF];

  match unsafe {
    ffi::read (
      ffi::STDIN_FILENO,
      line.as_ptr() as *mut ffi::c_char,
      ffi::BUFF as u64,
    )
  } {
    -1 => None,
    ret => Some((ret, line)),
  }
}

/// The `number` function calls the text function and
/// parses a integer.

pub fn number (
) -> Option<(i64)> {
  match self::text() {
    Some((_, line)) => {
      fn f (target: *const i8, acc: i64) -> i64 {
        unsafe {
          match {
            *target.as_ref().unwrap() as i64 - 48
          } {
            n @ 0 ... 9 => f(target.offset(1), acc * 10 + n),
            _ => acc,
          }
        }
      }
      Some(f(line.as_ptr(), 0))
    },
    None => None ,
  }
}
