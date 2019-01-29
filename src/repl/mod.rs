use ascii::AsciiString;
use std::io;
use std::str::FromStr;
use crate::token::Token;
use crate::lexer::Lexer;
use crate::parser::Parser;

const PROMPT: &str = ">> ";

#[cfg_attr(tarpaulin, skip)]
pub fn start<R, W>(mut reader: R, mut writer: W) -> io::Result<!>
where
    R: io::BufRead,
    W: io::Write,
{
    loop {
        write!(writer, "{}", PROMPT)?;
        writer.flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if let Ok(ascii_line) = AsciiString::from_str(&line) {
            let mut lex = Lexer::new(ascii_line);
            let mut p = Parser::new(&mut lex);
            let program = p.parse_program();

            if p.errors.len() != 0 {
                writer = print_parse_errors(writer, p.errors)?;
                continue
            }

            writeln!(writer, "{}", program)?
        } else {
            writeln!(writer, "[ERROR] please input only ASCII string").unwrap();
        }
    }
}


fn print_parse_errors<W: io::Write>(mut writer: W, errors: Vec<String>) -> io::Result<W> {
    for err in errors {
        write!(writer, "\t{}\n", err)?
    }
    Ok(writer)
}