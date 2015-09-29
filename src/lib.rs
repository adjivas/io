// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
pub mod macros;
pub mod ffi;

/// The `sentence` function reads returns None
/// or the tuple (len, sentence).

pub fn sentence (
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
