use crate::lexer::Lexer;
use crate::token::Token;
use ascii::AsciiString;
use std::io;
use std::str::FromStr;

const PROMPT: &str = ">> ";

pub fn start<R, W>(mut reader: R, mut writer: W) -> io::Result<!>
where
    R: io::BufRead,
    W: io::Write,
{
    loop {
        write!(writer, "{}", PROMPT).unwrap();
        writer.flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if let Ok(ascii_line) = AsciiString::from_str(&line) {
            let mut lex = Lexer::new(ascii_line);
            let mut tok;
            while {
                tok = lex.next_token();
                tok != Token::EOF
            } {
                writeln!(writer, "{:?}", tok).unwrap();
            }
        } else {
            writeln!(writer, "[ERROR] please input only ASCII string").unwrap();
        }
    }
}
