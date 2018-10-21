#![feature(never_type)]
extern crate ascii;
#[macro_use]
extern crate lazy_static;
pub mod token;
pub mod lexer;
pub mod repl;

use std::io;



fn main() {
    println!("Hello! This is the Monky programming language!");
    println!("Feel free to type in commands!");
    if let Err(err) = repl::start(io::BufReader::new(io::stdin()), io::stdout()) {
        eprintln!("[ERROR] failed to read line");
        eprintln!("{:?}", err);
    }
}
