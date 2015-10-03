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

/// The `line` function reads and returns the tuple (len, line).

pub fn line (
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

/// The `character` function reads and returns only one character.

pub fn one_char (
) -> Option<[ffi::c_char; 1]> {
  let chr: [ffi::c_char; 1] = [0; 1];

  match unsafe {
    ffi::read (
      ffi::STDIN_FILENO,
      chr.as_ptr() as *mut ffi::c_char,
      1,
    )
  } {
    -1 => None,
    _ => Some(chr),
  }
}

/// The `number_at` function reads character by character and
/// returns a integer between 0 and u64::MAX + the first acc.

fn number_at (
  acc: u64
) -> Option<(u64)> {
  match self::one_char() {
    Some(number) => match {
      unsafe {
        *number.as_ptr().as_ref().unwrap() as u8 as char
      }
    } {
      n @ '0' ... '9' => number_at(acc * 10 + n as u64 - 48),
      _ => Some(acc),
    },
    None => None,
  }
}

/// The `natural_number` function reads character by character and
/// returns a integer between 0 and u64::MAX.

pub fn natural_number (
) -> Option<(u64)> {
  number_at(0)
}
