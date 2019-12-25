#![feature(never_type)]
#![feature(box_patterns)]
mod ast;
mod evaluator;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use std::io;

#[cfg_attr(tarpaulin, skip)]
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
