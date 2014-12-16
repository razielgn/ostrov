use runtime::Runtime;

use std::io;

pub fn repl(args: Vec<String>) {
    let mut runtime = Runtime::new();

    if args.len() > 1 {
        let path = Path::new(&args[1]);

        match runtime.eval_file(&path) {
            Ok(ref expr) if !expr.is_empty() => println!("{}", expr.last().unwrap()),
            Ok(ref _expr) => println!(""),
            Err(error) => println!("{}", error),
        }
    } else {
        enter_repl(&mut runtime);
    }
}

fn enter_repl(runtime: &mut Runtime) {
    let mut input = io::stdin();

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
