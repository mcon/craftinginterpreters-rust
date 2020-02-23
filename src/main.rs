extern crate argparse;
extern crate loxrust;

use std::io::{self, Read};
use argparse::{ArgumentParser, Store};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use loxrust::parser::Parser;
use loxrust::interpreter::{interpret};

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

pub fn run_prompt() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        println!("> {}", line);
        run(line);
    }
}

pub fn run_file(source_file: &String) {
    let file = File::open(source_file).expect("failed to open file");

    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("failed to read file contents to buffer");

    println!("contents: {}", contents);
    run(contents);
}

pub fn run(source: String) {
    let mut scanner = loxrust::scanner::Scanner::new(source);
    {
        let tokens = scanner.scan_tokens();
        let mut parser = Parser::new(tokens);
        let parse_result = parser.parse();

        match parse_result {
            Ok(stmts) => {
                let mut output_string = String::new();
                // Debug the AST being produced
                for stmt in &stmts {
                    let ast = loxrust::ast::ast_printer(&mut output_string, stmt);
                    println!("Resulting AST: {:?}", ast);
                }

                match interpret(&stmts) {
                    Ok(_0) => {},
                    Err(e) => {println!("{}", e)},
                }
            }
            Err(err) => println!("Errors in statement: {:?}", err)
        }
    }
}