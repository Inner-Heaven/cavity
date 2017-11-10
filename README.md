[![Crates.io](https://img.shields.io/crates/v/cavity.svg)](https://crates.io/crates/cavity)

## Cavity

Very small library to create files. It's an alternative to macOS's
`mkfile(8)` that works on everything that has rust's std. [Unlike](http://blog.metaobject.com/2017/02/mkfile8-is-severely-syscall-limited-on.html) `mkfile`
this uses configurable chunksize and has default of 512Kb. 


## Installation

`cavity` is available on crates.io and can be included in your Cargo enabled project like this:

```
[dependencies]
cavity = "1.1.0"
```

## Example
Read the ["docs"](https://docs.rs/libnv).

```rust
use cavity::{fill, Bytes, WriteMode};
use std::fs::File;
let mut f = File::create("wat.test").unwrap();
fill(Bytes::MegaBytes(5), None, WriteMode::FlushEvery, &mut f).unwrap();
```
