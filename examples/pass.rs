// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate io;

fn main () {
    let mut term = ioctl!(0x8a73);

    ioctl!(io::ffi::TermiosControl::SETS, &mut *term);
    if let Some((len, text)) = read!() {
        term.c_lflag = 0x8a3b;

        ioctl!(io::ffi::TermiosControl::SETAW, &mut *term);
        write!(&text, len);
    }
    else {
        term.c_lflag = 0x8a3b;
        ioctl!(io::ffi::TermiosControl::SETAW, &mut *term);
    }
}
