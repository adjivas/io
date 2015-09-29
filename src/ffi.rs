// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate libc;

/// The `libc` type is the list of C types required
/// and bits/termios.h.

pub type off_t = libc::types::os::arch::posix88::off_t;
pub type ssize_t = libc::types::os::arch::posix88::ssize_t;
pub type size_t = libc::types::os::arch::c95::size_t;
pub type c_int = libc::types::os::arch::c95::c_int;
pub type c_char = libc::types::os::arch::c95::c_char;
pub type c_ulong = libc::types::os::arch::c95::c_ulong;
pub type cc_t = libc::types::os::arch::c95::c_uchar; // bits/Termios'type.
pub type speed_t = libc::types::os::arch::c95::c_uint; // bits/Termios'type.
pub type tcflag_t = libc::types::os::arch::c95::c_uint; // bits/Termios'type.

/// The `NCCS` const are default values
/// of the struct `termios`.

pub const NCCS:usize = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Termios {
  pub c_iflag: tcflag_t, // input mode flags.
  pub c_oflag: tcflag_t, // output mode flags.
  pub c_cflag: tcflag_t, // control mode flags.
  pub c_lflag: tcflag_t, // local mode flags.
  pub c_line: cc_t,      // line discipline.
  pub c_cc: [cc_t; NCCS],  // control characters.
  pub c_ispeed: speed_t, // input speed.
  pub c_ospeed: speed_t, // output speed.
}

/// All the `const *` are default values
/// of c_cc.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ControlCharacter {
  VINTR    =  0,
  VQUIT    =  1,
  VERASE   =  2,
  VKILL    =  3,
  VEOF     =  4,
  VTIME    =  5,
  VMIN     =  6,
  VSWTC    =  7,
  VSTART   =  8,
  VSTOP    =  9,
  VSUSP    = 10,
  VEOL     = 11,
  VREPRINT = 12,
  VDISCARD = 13,
  VWERASE  = 14,
  VLNEXT   = 15,
  VEOL2    = 16,
}

/// All the `const *` are default values
/// of c_iflag.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum InputModes {
  IGNBRK  = 0o000001,
  BRKINT  = 0o000002,
  IGNPAR  = 0o000004,
  PARMRK  = 0o000010,
  INPCK   = 0o000020,
  ISTRIP  = 0o000040,
  INLCR   = 0o000100,
  IGNCR   = 0o000200,
  ICRNL   = 0o000400,
  IUCLC   = 0o001000,
  IXON    = 0o002000,
  IXANY   = 0o004000,
  IXOFF   = 0o010000,
  IMAXBEL = 0o020000,
  IUTF8   = 0o040000,
}

/// All the `const *` are default values
/// of c_oflag.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum OutputModes {
  OPOST  = 0o000001,
  OLCUC  = 0o000002,
  ONLCR  = 0o000004,
  OCRNL  = 0o000010,
  ONOCR  = 0o000020,
  ONLRET = 0o000040,
  OFILL  = 0o000100,
  OFDEL  = 0o000200,
  VT0    = 0o000000,
  VT1    = 0o040000,
}

/// All the `const *` are default values
/// of c_cflag.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Speed {
  B0       = 0o000000,     /* hang up */
  B50      = 0o000001,
  B75      = 0o000002,
  B110     = 0o000003,
  B134     = 0o000004,
  B150     = 0o000005,
  B200     = 0o000006,
  B300     = 0o000007,
  B600     = 0o000010,
  B1200    = 0o000011,
  B1800    = 0o000012,
  B2400    = 0o000013,
  B4800    = 0o000014,
  B9600    = 0o000015,
  B19200   = 0o000016,
  B38400   = 0o000017,
  B57600   = 0o010001,
  B115200  = 0o010002,
  B230400  = 0o010003,
  B460800  = 0o010004,
  B500000  = 0o010005,
  B576000  = 0o010006,
  B921600  = 0o010007,
  B1000000 = 0o010010,
  B1152000 = 0o010011,
  B1500000 = 0o010012,
  B2000000 = 0o010013,
  B2500000 = 0o010014,
  B3000000 = 0o010015,
  B3500000 = 0o010016,
  B4000000 = 0o010017,
}

/// All the `const *` are default values
/// of c_lflag.

#[repr(C)]
#[derive(Copy, Clone)]
pub enum LocalModes {
  ECHO   = 0o000010,
  ECHOE  = 0o000020,
  ECHOK  = 0o000040,
  ECHONL = 0o000100,
  NOFLSH = 0o000200,
  TOSTOP = 0o000400,
  ISIG   = 0o000001,
  ICANON = 0o000002,
}

/// The `BUFF` and `STDIN_FILENO` const
/// are default values for macros.

pub const BUFF: usize = 1024;
pub const STDIN_FILENO: c_int = libc::consts::os::posix88::STDIN_FILENO;

/// The `C` extern is list of system call from libc required
/// by the library.

pub const TCGETS:c_ulong = 0x5401;
pub const TCSETS:c_ulong = 0x5402;
pub const TCSETSW:c_ulong = 0x5403;
pub const TCSETSF:c_ulong = 0x5404;
pub const TCGETA:c_ulong = 0x5405;
pub const TCSETA:c_ulong = 0x5406;
pub const TCSETAW:c_ulong = 0x5407;
pub const TCSETAF:c_ulong = 0x5408;
pub const TCSBRK:c_ulong = 0x5409;
pub const TCXONC:c_ulong = 0x540a;
pub const TCFLSH:c_ulong = 0x540b;

/// The `C` extern is list of system call from termcaps required
/// by the password input.

#[cfg(any(unix))]
extern "C" {
  pub fn ioctl(fd: c_int, req: c_ulong, term: *mut Termios) -> c_int;
}

/// The `C` extern is list of system call from libc required
/// by the standard input.

#[cfg(any(unix))]
extern "C" {
  pub fn lseek(fd: c_int, offset: off_t , whence: c_int) -> off_t;
  pub fn read(fd: c_int, buf: *mut c_char, len: size_t) -> ssize_t;
}
