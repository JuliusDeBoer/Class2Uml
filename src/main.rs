use std::{env, process};
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("ERROR: No files provided");
        process::exit(1);
    }

    // TODO: Support multiple files
    let filename = args[1].clone();

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    };

    dbg!(file);
}
