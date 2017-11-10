[![Crates.io](https://img.shields.io/crates/v/cavity.svg)](https://crates.io/crates/cavity)

## Cavity

Very small library to create files. It's an alternative to macOS's
`mkfile(8)` that works on everything that has rust's std. [Unlike](mkfile_bad) `mkfile(8)`
this uses configurable chunksize and has default of 512Kb. 

## CLI tool

Cli version of this called `cavity-cli` located in this repo. The tool is 2x times faster than `mkfile(8)`. Don't think it's faster because its rust. [Read this instead](mkfile_bad).
Tool has some configuration option, but defaults are somewhat perfect. CLI tool has a lot of `.unwrap()` calls. Very unscientific benchmark:

```
 $  time target/release/cavity-cli  10G test.wat
target/release/cavity-cli 10G test.wat  0.00s user 9.92s system 49% cpu 20.110 total
 @ac-book.local   2:09PM  ~/Dev/Heaven/cavity/cavity-cli   master 
 $  time mkfile 10G test2
mkfile 10G test2.wat  1.83s user 31.84s system 87% cpu 38.339 total
```

[![asciicast](https://asciinema.org/a/WdA4olIWsk8l4nzQiV0e88ayo.png)](https://asciinema.org/a/WdA4olIWsk8l4nzQiV0e88ayo)
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


[mkfile_bad]: http://blog.metaobject.com/2017/02/mkfile8-is-severely-syscall-limited-on.html