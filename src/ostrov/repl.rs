use runtime::Runtime;

use std::io;

pub fn repl() {
    let mut input = io::stdin();

    let mut runtime = Runtime::new();

    loop {
        print!("> ");

        match input.read_line() {
            Ok(line) => {
                match runtime.eval_str(line.as_slice()) {
                    Ok(exprs) => {
                        for expr in exprs.iter() {
                            println!("=> {}", expr);
                        }
                    },
                    Err(error) => println!("Error: {}", error),
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
