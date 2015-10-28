// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

/// The `write` macro writes to output the text
/// and returns the Some 0i32 or None according to success.

#[macro_export]
macro_rules! write {
    ($text: expr) => ({
        let mut result: bool = false;

        for len in 0i32..std::i32::MAX {
            if unsafe {
                *$text.offset(len as isize)
            } == 0u8 {
                result = write!($text, len);
                break ;
            }
        }
        result
    });
    ($text: expr, $len: expr) => ({
        write!($text, $len, 1)
    });
    ($text: expr, $len: expr, $out: expr) => ({
        extern crate io;
        match unsafe {
            io::ffi::write (
                $out as i32,
                $text as *const i8,
                $len as i32,
            )
        } {
          -1 => false,
          _ => true,
        }
    });
}

/// The `writeln` macro writes to output the text with a breakline
/// and returns the Some 0i32 or None according to success.

#[macro_export]
macro_rules! writeln {
    ($text: expr) => ({
        write!($text) && write!("\n".as_ptr(), 1)
    });
    ($text: expr, $len: expr) => ({
        write!($text, $len) && write!("\n".as_ptr(), 1)
    });
    ($text: expr, $len: expr, $out: expr) => ({
        write!($text, $len, $out) && write!("\n".as_ptr(), $out)
    });
}

/// The `write_number` macro writes to output the number
/// and returns the Some 0i32 or None according to success.

#[macro_export]
macro_rules! write_number {
    ($number: expr) => ({
        write_number!($number, 1)
    });
    ($number: expr, $out: expr) => ({
        let mut decimal = $number;
        let mut buf: [u8; 64] = [0; 64];
        let mut result: bool = false;

        for target in (0..64).rev() {
            buf[target] = {decimal % 10 + 48} as u8;
            decimal /= 10;
            if decimal == 0 {
                result = write! (
                    buf.as_ptr().offset (
                        target as isize
                    ),
                    64 - target,
                    $out
                );
                break ;
            }
        }
        result
    });
}

/// The `writeln_number` macro writes to output the number with a breakline
/// and returns the Some 0i32 or None according to success.

#[macro_export]
macro_rules! writeln_number {
    ($number: expr) => ({
        writeln_number!($number, 1)
    });
    ($number: expr, $out: expr) => ({
        write_number!($number, $out) && write!("\n".as_ptr(), $out)
    });
}

/// The `write_error` macro writes to output the error
/// and returns the Some 0i32 or None according to success.

#[macro_export]
macro_rules! write_err {
    ($text: expr) => ({
        let mut result: bool = false;

        for len in 0i32..std::i32::MAX {
            if unsafe {
                *$text.offset(len as isize)
            } == 0u8 {
                result = write_err!($text, len);
                break ;
            }
        }
        result
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
        extern crate io;
        let line: [i8; io::ffi::BUFF] = [0; io::ffi::BUFF];

        match unsafe {
          io::ffi::read (
            $ins as i32,
            line.as_ptr() as *mut i8,
            $len as i32,
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


/// The `read_command` macro reads and
/// returns the addition of all letter.

#[macro_export]
macro_rules! read_command {
    () => ({
        match {
            read_character!()
        } {
            Some(47) => Some(read_command!(0u64)),
            _ => None ,
        }
    });
    ($start: expr) => ({
        fn result (acc: u64) -> u64 {
            match {
                read_character!()
            } {
                Some(d @ 97...122) => result(acc + {d - 97i8} as u64),
                _ => acc,
            }
        }
        result($start as u64)
    });
}

/// The `ioctl` macro reads the input and
/// returns None or a tuple (len, text).

#[macro_export]
macro_rules! ioctl {
    () => ({
        extern crate io;
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
        extern crate io;
        unsafe {
            io::ffi::ioctl (
                io::ffi::STDIN_FILENO,
                $req as u64,
                $term,
           )
        }
    });
}
