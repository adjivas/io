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
    read!(ffi::BUFF)
  });
  ($len: expr) => ({
    read!($len, 0)
  });
  ($len: expr, $ins: expr) => ({
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


/// The `read` macro reads the input and
/// returns None or a tuple (len, text).

#[macro_export]
macro_rules! ioctl {
  () => ({
    let mut term = Box::new(ffi::Termios {
      c_iflag: 0,
      c_oflag: 0,
      c_cflag: 0,
      c_lflag: 0,
      c_line: 0,
      c_cc: [0; ffi::NCCS],
      c_ispeed: 0,
      c_ospeed: 0,
    });

    ioctl!(ffi::TCGETA, &mut *term);
    term
  });
  ($req: expr, $term: expr) => ({
    unsafe {
      ffi::ioctl(ffi::STDIN_FILENO, $req, $term)
    }
  });
}
