use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "catr", version = "0.1.0", author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>", about = "Rust cat")]
pub struct Args {
    #[arg(value_name = "FILE", help = "Input file(s) [default: -]", default_value = "-")]
    files: Vec<String>,
    #[arg(short = 'n', long = "number", help = "Number lines", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,
    #[arg(short = 'b', long = "number-nonblank", help = "Number nonblank lines")]
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn run(args: Args) -> MyResult<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename)
        }
    }

    Ok(())
}
