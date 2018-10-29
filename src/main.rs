#![feature(never_type)]
extern crate ascii;
#[macro_use]
extern crate lazy_static;
pub mod ast;
pub mod lexer;
pub mod repl;
pub mod token;

use std::io;

fn main() {
    println!("Hello! This is the Monky programming language!");
    println!("Feel free to type in commands!");
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    if let Err(err) = repl::start(stdin_lock, io::stdout()) {
        eprintln!("[ERROR] failed to read line");
        eprintln!("{:?}", err);
    }
}
