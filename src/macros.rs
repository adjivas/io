// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `read` macro writes the output and returns the Some 0i32
/// or None according to success.

#[macro_export]
macro_rules! write {
  ($text: expr, $len: expr) => ({
    write!($text, $len, 1)
  });
  ($text: expr, $len: expr, $out: expr) => ({
    match unsafe {
        io::ffi::write (
            $out, $text as *const io::ffi::c_char, $len as io::ffi::size_t
        )
    } {
      -1 => false,
      _ => true,
    }
  });
}

/// The `read` macro reads the input and returns None
/// or the Some of thetuple (len, text).

#[macro_export]
macro_rules! read {
  () => ({
    read!(io::ffi::BUFF)
  });
  ($len: expr) => ({
    read!($len, 0)
  });
  ($len: expr, $ins: expr) => ({
    let line: [io::ffi::c_char; io::ffi::BUFF] = [0; io::ffi::BUFF];

    match unsafe {
      io::ffi::read (
        $ins as io::ffi::c_int,
        line.as_ptr() as *mut io::ffi::c_char,
        $len as io::ffi::size_t,
      )
    } {
      -1 => None,
      ret => Some((ret, line)),
    }
  });
}


/// The `read_character` macro reads and
/// returns one character.

#[macro_export]
macro_rules! read_character {
    () => ({
        match read!(1) {
            Some((_, c)) => Some(c[0]),
            None => None,
        }
    });
}


/// The `read_number` macro reads and
/// returns the number.

#[macro_export]
macro_rules! read_number {
    () => ({
        match {
            read_character!()
        } {
            Some(45) => Some(-read_number!(0i8)),
            Some(d @ 48...57) => Some(read_number!(d - 48i8)),
            _ => None ,
        }
    });
    ($start: expr) => ({
        fn result (acc: i64) -> i64 {
            match {
                read_character!()
            } {
                Some(d @ 48...57) => result({acc * 10i64} + {d - 48i8} as i64),
                _ => acc,
            }
        }
        result($start as i64)
    });
}

/// The `ioctl` macro reads the input and
/// returns None or a tuple (len, text).

#[macro_export]
macro_rules! ioctl {
    () => ({
        let mut term = Box::new(io::ffi::Termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; io::ffi::NCCS],
            c_ispeed: 0,
            c_ospeed: 0,
        });

        ioctl!(io::ffi::TermiosControl::GETA, &mut *term);
        term

    });  
    ($control: expr) => ({
        let mut term = ioctl!();

        term.c_lflag = $control;
        term
    });
    ($req: expr, $term: expr) => ({
        unsafe {
            io::ffi::ioctl (
                io::ffi::STDIN_FILENO,
                $req as u64,
                $term,
           )
        }
    });
}
