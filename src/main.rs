extern crate argparse;

use std::io::{self, Read};
use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
mod scanner;

fn main() {
    let mut file_name = String::new();

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Execute code written in lox");
        ap.refer(&mut file_name)
            .add_argument("file", Store, "File containing lox code to run");
        ap.parse_args_or_exit();
    }
    match file_name.len() {
        0 => run_prompt(),
        _ => run_file(&file_name)
    }
}

fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("> {}", line);
        run(line);
    }
}

fn run_file(source_file: &String) {
    let file = File::open(source_file).expect("failed to open file");

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("failed to read file contents to buffer");

    println!("contents: {}", contents);
    run(contents);
}

fn run(source: String) {
    let mut scanner = scanner::Scanner::new(source);
    let tokens = scanner.scan_tokens();
}