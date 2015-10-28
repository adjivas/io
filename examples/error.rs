// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate io;

fn main() {
    writeln_err!("The error work\0".as_ptr());
}
