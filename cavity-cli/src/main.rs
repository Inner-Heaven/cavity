extern crate clap;
use clap::{App, Arg};

extern crate cavity;

use cavity::{Bytes, WriteMode, fill};
use std::fs::File;
use std::path::Path;

fn has_suffix(val: &String) -> bool {
    let last_char = val.chars().last().unwrap();
    match last_char {
        'K' | 'M' | 'G' | 'k' | 'm' | 'g' => true,
        _ => false,
    }
}

fn is_size(val: String) -> Result<(), String> {
    let mut mut_val = val.clone();
    if has_suffix(&mut_val) {
        mut_val.pop().unwrap();
    }
    if mut_val.parse::<usize>().is_err() {
        Err(String::from("Incorrect size"))
    } else {
        Ok(())
    }
}

fn parse_size(mut val: String) -> Bytes {
    let mut suffix = None;
    if has_suffix(&val) {
        suffix = val.pop();
    }
    let size = val.parse::<usize>().unwrap();

    match suffix {
        Some('K') | Some('k') => Bytes::KiloBytes(size),
        Some('M') | Some('m') => Bytes::MegaBytes(size),
        Some('G') | Some('g') => Bytes::GigaBytes(size),
        _ => Bytes::KiloBytes(size),
    }
}
fn main() {
    let matches = App::new("Cavity")
        .version("1.1.0")
        .author("Andrey Cherkashin <with.out@me.com>")
        .about("Create files padded with zeros")
        .arg(Arg::with_name("force")
                 .short("f")
                 .long("force")
                 .help("Overwrite file if exist."))
        .arg(Arg::with_name("mode")
                 .short("m")
                 .long("mode")
                 .help("Which mode to run in. `every` to flush every chunk and `once` to \
                        flush at the end.")
                 .default_value("once")
                 .takes_value(true)
                 .possible_values(&["every", "once"]))
        .arg(Arg::with_name("chunk")
                 .short("cs")
                 .long("chuck-size")
                 .help("Size of a chunk to write")
                 .default_value("512K")
                 .takes_value(true)
                 .validator(is_size))
        .arg(Arg::with_name("SIZE")
                 .value_name("SIZE")
                 .help("Size of a file to create. The default size unit is kilobytes, but \
                        the following suffixes are allowed: K, M, G.")
                 .required(true)
                 .validator(is_size)
                 .index(1))
        .arg(Arg::with_name("FILE")
                 .value_name("FILE")
                 .help("Path to file")
                 .required(true)
                 .index(2))
        .get_matches();


    let mode = match matches.value_of("mode") {
        Some("every") => WriteMode::FlushEvery,
        Some("once") => WriteMode::FlushOnce,
        _ => unreachable!(),
    };

    let mut size_mut = matches.value_of("SIZE").unwrap().into();
    let size = parse_size(size_mut);

    let mut cs_mut = matches.value_of("chunk").unwrap().into();
    let cs = parse_size(cs_mut);


    let path = Path::new(matches.value_of("FILE").unwrap());
    if !matches.is_present("force") && path.exists() {
        panic!("File exists. Use  --force to overwrite.");
    }
    let mut file = File::create(path).unwrap();
    fill(size, Some(cs), mode, &mut file);
}
