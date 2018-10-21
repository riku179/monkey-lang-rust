use std::io;
use std::str::FromStr;
use ascii::AsciiString;
use lexer::Lexer;
use token;

const PROMPT: &'static str = ">> ";

pub fn start<R: io::BufRead, W: io::Write>(mut reader: R, _write: W) -> io::Result<!> {
    loop {
        print!("{}", PROMPT);
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if let Ok(ascii_line) = AsciiString::from_str(&line) {
            let mut lex = Lexer::new(ascii_line);
            let mut tok;
            while {
                tok = lex.next_token();
                tok.token_type != token::EOF
            } {
                println!("{:?}", tok);
            }
        } else {
            println!("[ERROR] please input only ASCII string");
        }
    }
}