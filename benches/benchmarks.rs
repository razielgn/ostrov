extern crate ostrov;
extern crate test;

use ostrov::runtime::Runtime;

use test::Bencher;

static NESTED_IFS: &'static str = "
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

#[bench]
fn nested_evaluation(b: &mut Bencher) {
    let mut runtime = Runtime::new();

    b.iter(|| {
        assert_eq!(runtime.eval_str(NESTED_IFS), runtime.eval_str("3"));
    })
}

#[bench]
fn nested_evaluation_bytecode(b: &mut Bencher) {
    let mut runtime = Runtime::new();

    b.iter(|| {
        assert_eq!(runtime.eval_str(NESTED_IFS), runtime.eval_str("3"));
    })
}

#[bench]
fn procedure_evaluation(b: &mut Bencher) {
    let input = "
        (define (fact n)
            (if (= n 1)
                1
                (* n (fact (- n 1)))))
        (fact 5)
    ";

    let mut runtime = Runtime::new();

    b.iter(|| {
        assert_eq!(
            runtime.eval_str(input).unwrap()[1],
            runtime.eval_str("120").unwrap()[0]
        );
    })
}
