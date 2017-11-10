Very small library to create files. It's an alternative to macOS's
`mkfile(8)` that works on everything that has rust's std.
I have no idea how fast or perfomant this is, so good luck.

# Arguments

* `how_many`       — How many megabytes to write. It can be either in
megabytes or gigabytes

* `buffer_size`    — Size of chuck in megabytes. Default value is 16. Meaning empty vector with a size of 16 megabytes will be create in memory

* `write_mode`     — It can either implicitly flush after each chuck or
just once at the end.

* `file`           — A Writer to write to. Doesn't have to be a file.

# Example

```rust
use cavity::{fill, Bytes, WriteMode};
use std::fs::File;
let mut f = File::create("wat.test").unwrap();
fill(Bytes::MegaBytes(5), None, WriteMode::FlushEvery, &mut f).unwrap();
```
