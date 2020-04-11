use crate::evaluator::eval;
use crate::lexer::Lexer;
use crate::object::Env;
use crate::parser::Parser;
use std::io;

const PROMPT: &str = ">> ";

#[cfg_attr(tarpaulin, skip)]
pub fn start<R, W>(mut reader: R, mut writer: W) -> io::Result<!>
where
    R: io::BufRead,
    W: io::Write,
{
    let mut env = Env::new();
    loop {
        write!(writer, "{}", PROMPT)?;
        writer.flush()?;
        let mut line = String::new();
        reader.read_line(&mut line)?;
        if let Ok(mut lex) = Lexer::new(line) {
            let mut p = Parser::new(&mut lex);
            let program = p.parse_program();

            if !p.errors.is_empty() {
                writer = print_parse_errors(writer, p.errors)?;
                continue;
            }

            if let Some(val) = eval(program, &mut env) {
                writeln!(writer, "{}", val)?
            }
        } else {
            writeln!(writer, "[ERROR] please input only ASCII string").unwrap();
        }
    }
}

fn print_parse_errors<W: io::Write>(mut writer: W, errors: Vec<String>) -> io::Result<W> {
    for err in errors {
        writeln!(writer, "\t{}", err)?
    }
    Ok(writer)
}
