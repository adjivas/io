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
    ($text: expr, $len: expr) => ({
        write!($text, $len, 1)
    });
    ($text: expr, $len: expr, $out: expr) => ({
        extern crate io_synesthesist;

        match unsafe {
            io_synesthesist::ffi::write (
                $out as i32,
                $text as *const u8,
                $len as i32,
            )
        } {
          -1 => false,
          _ => true,
        }
    });
}
