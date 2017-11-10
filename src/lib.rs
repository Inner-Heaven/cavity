/// Very small library to create files. It's an alternative to macOS's
/// `mkfile(8)` that works on everything that has rust's std.
/// I have no idea how fast or perfomant this is, so good luck.
///
/// # Arguments
/// * `how_many`       — How many megabytes to write. It can be either in
/// megabytes or gigabytes
///  * `buffer_size`    — Size of chuck in megabytes. Default value is 16. Meaning empty vector with a size of 16 megabytes will be create in memory
/// * `write_mode`     — It can either implicitly flush after each chuck or
/// just once at the end.
///  * `file`           — A Writer to write to. Doesn't have to be a file.
/// # Example
///
/// ```rust,no_run
/// use cavity::{fill, Bytes, WriteMode};
/// use std::fs::File;
/// let mut f = File::create("wat.test").unwrap();
/// fill(Bytes::MegaBytes(5), None, WriteMode::FlushEvery, &mut f).unwrap();
/// ```

use std::io::{Result as IoResult, Write};

/// Default buffer size.
static BUF_SIZE_MB: usize = 16;

static MEGABYTE_NUL: [u8; 1000 * 1000] = [0; 1000 * 1000];

/// What writing mode to use.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum WriteMode {
    /// Call to flush only once at the end.
    FlushOnce,
    /// Can to flush every write.
    FlushEvery,
}
/// Represents size. Please note that it's using proper notation, so 1 kilobyte
/// is 1000 bytes.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
pub enum Bytes {
    MegaBytes(usize),
    GigaBytes(usize),
}

impl Bytes {
    /// Return size in bytes.
    pub fn as_bytes(&self) -> usize {
        return match *self {
                   Bytes::MegaBytes(e) => e * 1000 * 1000,
                   Bytes::GigaBytes(e) => e * 1000 * 1000 * 1000,
               };
    }
    /// Return as size in megabyges.
    pub fn as_megabytes(&self) -> usize {
        return match *self {
                   Bytes::MegaBytes(e) => e,
                   Bytes::GigaBytes(e) => 1000 / e,
               };
    }
}

/// Fill writer with as many zeroes as you want. First it writes in chunks of
/// buffer_size, then it writes megabyte by megabyte.
pub fn fill<W: Write>(how_many: Bytes,
                      buffer_size: Option<Bytes>,
                      write_mode: WriteMode,
                      file: &mut W)
                      -> IoResult<()> {
    let buf_size = buffer_size.unwrap_or(Bytes::MegaBytes(BUF_SIZE_MB));
    let mut buf = Vec::with_capacity(buf_size.as_bytes());
    for _ in 0..buf_size.as_megabytes() {
        buf.extend_from_slice(&MEGABYTE_NUL);
    }

    let number_of_writes = (how_many.as_megabytes() as f64 / BUF_SIZE_MB as f64).floor() as usize;

    for _ in 0..number_of_writes {
        match write_mode {
            WriteMode::FlushEvery => file.write_all(&buf).map(|_| ())?,
            WriteMode::FlushOnce => file.write(&buf).map(|_| ())?,
        }
    }

    let megabytes_left = how_many.as_megabytes() - (buf_size.as_megabytes() * number_of_writes);

    for _ in 0..megabytes_left {
        file.write_all(&MEGABYTE_NUL).map(|_| ())?;
    }
    file.flush()
}
#[cfg(test)]
mod tests {
    use {Bytes, WriteMode, fill};
    use std::fs::File;

    #[test]
    fn it_works() {
        let mut f = File::create("wat.test").unwrap();
        fill(Bytes::MegaBytes(5), None, WriteMode::FlushEvery, &mut f).unwrap();
    }
}
