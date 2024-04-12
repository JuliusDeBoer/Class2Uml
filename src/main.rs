use std::{env, fs, process};

/// The minimum supported version that this program supports
const MINIMUM_SUPPORTED_VERSION: u16 = 61;
/// The maximum supported version that this program supports
const MAXIMUM_SUPPORTED_VERSION: u16 = u16::MAX;

/// Checks the magic number of a file to check if it is a java class file
fn is_class(data: &[u8]) -> bool {
    const MAGIC_NUMBER: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

    if data.len() < 4 {
        return false;
    }

    let first_four_bytes = &data[..4];

    first_four_bytes == MAGIC_NUMBER
}

/// Returns the version of a given java class file. Where the first number is
/// the minor version. And the last is the major.
fn get_version(data: &[u8]) -> Result<(u16, u16), &str> {
    if data.len() < 7 {
        return Err("File not long enough. Is it a valid java class file?");
    }

    let minor: u16 = (data[4] as u16 >> 8) + data[5] as u16;
    let major: u16 = (data[6] as u16 >> 8) + data[7] as u16;

    Ok((minor, major))
}

fn parse_file(data: &[u8]) {
    if !is_class(data) {
        eprintln!("ERROR: Provided file is not a java class file");
        return;
    }

    let major_version = match get_version(data) {
        Ok(v) => v.1,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    };

    // TODO: Add a flag to ignore this check
    if !(MINIMUM_SUPPORTED_VERSION..=MAXIMUM_SUPPORTED_VERSION).contains(&major_version) {
        eprintln!(
            "ERROR: Java class file version {} is not supported",
            major_version
        );
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("ERROR: No files provided");
        process::exit(1);
    }

    // TODO: Support multiple files
    let filename = args[1].clone();

    let data = match fs::read(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    };

    parse_file(&data);
}
