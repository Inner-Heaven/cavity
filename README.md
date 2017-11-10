[![Crates.io](https://img.shields.io/crates/v/cavity.svg)](https://crates.io/crates/cavity)

## Cavity

Very small library to create files. It's an alternative to macOS's
`mkfile(8)` that works on everything that has rust's std.
I have no idea how fast or perfomant this is, so good luck.


## Installation

`cavity` is available on crates.io and can be included in your Cargo enabled project like this:

```
[dependencies]
cavity = "1.0.0"
```

## Example
Read the ["docs"](https://docs.rs/libnv).

```rust
use cavity::{fill, Bytes, WriteMode};
use std::fs::File;
let mut f = File::create("wat.test").unwrap();
fill(Bytes::MegaBytes(5), None, WriteMode::FlushEvery, &mut f).unwrap();
```
