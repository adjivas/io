// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/io
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate io_synesthesist;

fn main() {
    if let Some(c) = read_character!() {
        write_character!(c);
        write_character!('\n');
    }
}
