use eval::eval;
use parser::parse;

use std::io;

pub fn repl() {
    let mut input = io::stdin();

    loop {
        print!("> ");

        match input.read_line() {
            Ok(line) => {
                match parse(line.as_slice()) {
                    Ok(exprs) => {
                        for expr in exprs.iter() {
                            match eval(expr) {
                                Ok(value)  => println!("=> {}", value),
                                Err(error) => println!("{}", error),
                            }
                        }
                    },
                    Err(error) => println!("Parse error: {}", error),
                }
            },
            Err(error) => {
                match error.kind {
                    io::IoErrorKind::EndOfFile => break,
                    _ => panic!("{}", error),
                }
            },
        }
    }
}
