extern crate proc_macro;
extern crate core;
extern crate itertools;

// TODO: Split out the library from the REPL and any interpreter frontend.
pub mod scanner;
pub mod ast;
pub mod parser;
pub mod interpreter;
pub mod environment;