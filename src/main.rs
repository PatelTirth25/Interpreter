mod ast;
mod scanner;
mod token;

use std::{
    env::args,
    fs::read_to_string,
    io::{self},
};

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- (.nz file)");
        return;
    }
    let buffer = read_file(&args[1]).expect("Error Reading file!");
    println!("Content: {:?}", buffer);
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let buffer = read_to_string(path)?;
    Ok(buffer)
}
