use runtime::Runtime;

use std::old_io;

pub fn repl(args: Vec<String>) {
    let mut runtime = Runtime::new();

    if args.len() > 1 {
        let path = Path::new(&args[1]);

        match runtime.eval_file(&path) {
            Ok(_)      => (),
            Err(error) => println!("{:?}", error),
        }
    } else {
        enter_repl(&mut runtime);
    }
}

fn enter_repl(runtime: &mut Runtime) {
    let mut input = old_io::stdin();

    loop {
        print!("> ");

        match input.read_line() {
            Ok(line) => {
                match line.as_slice() {
                    "exit\n" => break,
                    "dump-heap\n" => {
                        runtime.dump_heap();
                    }
                    line => {
                        match runtime.eval_str(line) {
                            Ok(exprs) => {
                                for expr in exprs.iter() {
                                    println!("=> {}", expr);
                                }
                            },
                            Err(error) => println!("Error: {:?}", error),
                        }
                    }
                }
            },
            Err(error) => {
                match error.kind {
                    old_io::IoErrorKind::EndOfFile => break,
                    _ => panic!("{:?}", error),
                }
            },
        }
    }
}
