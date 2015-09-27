// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/read-line
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
mod macros;
mod ffi;

fn main () {
  let (_, text):(i64, [ffi::c_char; ffi::BUFF]) = read!().unwrap();

  println!("{:?}", text);
}
