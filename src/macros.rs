// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `read` macro reads the input and
/// returns None or a tuple (len, text).

#[macro_export]
macro_rules! read {
  () => ({
    read!(0)
  });
  ($ins: expr) => ({
    read!($ins, ffi::BUFF)
  });
  ($ins: expr, $len: expr) => ({
    let line: [ffi::c_char; ffi::BUFF] = [0; ffi::BUFF];

    match unsafe {
      ffi::read (
        $ins as ffi::c_int,
        line.as_ptr() as *mut ffi::c_char,
        $len as ffi::size_t,
      )
    } {
      -1 => None,
      ret => Some((ret, line)),
    }
  });
}
