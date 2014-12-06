extern crate ostrov;
extern crate test;

use ostrov::eval::eval;
use ostrov::parser::parse;
use ostrov::env::Env;

use test::Bencher;

#[bench]
fn nested_evaluation(b: &mut Bencher) {
    b.iter(|| {
        let input = "
            (if
                (if
                    (if
                        (if
                            (if
                                (if
                                    (> 1 2 3 4 5 6 7 8 9 10)
                                    (= 2 2 2 2 2 2 2 2 2 2)
                                    #f
                                )
                                (= 2 2 2 2 2 2 2 2 2 2)
                                #f
                            )
                            (= 2 2 2 2 2 2 2 2 2 2)
                            #f
                        )
                        (= 2 2 2 2 2 2 2 2 2 2)
                        #f
                    )
                    (= 2 2 2 2 2 2 2 2 2 2)
                    #f
                )
                2
                3
            )
        ";

        let mut env = Env::new();

        match parse(input) {
            Ok(ast) => match eval(ast.iter().last().unwrap(), &mut env) {
                Ok(_actual) => {},
                Err(error)  => panic!(format!("{}", error)),
            },
            Err(error) => panic!(format!("{}", error)),
        }
    })
}
