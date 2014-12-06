extern crate ostrov;
extern crate test;

use ostrov::ast::AST;
use ostrov::runtime::Runtime;

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

        let mut runtime = Runtime::new();

        match runtime.eval_str(input) {
            Ok(exprs)  => assert_eq!(AST::Integer(3), exprs[0]),
            Err(error) => panic!(format!("{}", error)),
        }
    })
}
