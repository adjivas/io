// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate readline;

fn main () {
  let test = readline::ffi::BUFF;

  let line = *b"a\n";
  unsafe {
    readline::ffi::read(
      1,
      line.as_ptr() as *mut readline::ffi::c_char,
      2 as readline::ffi::size_t,
    )
  };
}
