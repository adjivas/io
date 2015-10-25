# In/Out-Put

[![Build Status](https://travis-ci.org/adjivas/io.svg)](https://travis-ci.org/adjivas/io)
[![GPLv3 License](http://img.shields.io/badge/license-GPLv3-blue.svg)](https://www.gnu.org/copyleft/gpl.html)

#### How to build:
```shell
git clone https://github.com/adjivas/io.git io && cd io
cargo build
```

#### How to use:
* cargo run --example char < <(echo 'a')
* cargo run --example line < <(echo "hello")
* cargo run --example number < <(echo 42)
* cargo run --example number < <(echo $SECRET)

#### Directory-Tree:
```shell
.
|__ Cargo.toml
|__ LICENSE
|__ README.md
|__ examples
|   |__ char.rs
|   |__ line.rs
|   |__ pass.rs
|   |__ number.rs
|   \__ command.rs
\__ src
    |__ ffi.rs
    |__ macros.rs
    \__ lib.rs
```

#### License:
*io*'s code in this repo uses the [GNU GPL v3](http://www.gnu.org/licenses/gpl-3.0.html) [license](https://github.com/adjivas/io/blob/master/LICENSE).
