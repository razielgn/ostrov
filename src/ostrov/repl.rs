use crate::{runtime::Runtime, values::Value};
use std::{
    io::{self, Write},
    path::Path,
};

pub fn repl(args: &[String]) {
    let mut runtime = Runtime::new();

    if args.len() > 1 {
        let path = Path::new(&args[1]);

        runtime.eval_file(path);
    } else {
        enter_repl(&mut runtime);
    }
}

fn enter_repl(runtime: &mut Runtime) {
    let input = io::stdin();

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        let mut line = String::new();
        match input.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => match &*line {
                "exit\n" => break,
                "dump-heap\n" => {
                    runtime.dump_heap();
                }
                line => match runtime.eval_str(line) {
                    Ok(exprs) => {
                        for expr in &exprs {
                            if **expr != Value::Unspecified {
                                println!("=> {}", expr);
                            }
                        }
                    }
                    Err(error) => println!("Error: {:?}", error),
                },
            },
            Err(error) => panic!("{:?}", error),
        }
    }
}
