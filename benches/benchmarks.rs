extern crate ostrov;
extern crate test;

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
        assert_eq!(runtime.eval_str(input), runtime.eval_str("3"));
    })
}

#[bench]
fn procedure_evaluation(b: &mut Bencher) {
    b.iter(|| {
        let input = "
            (define (fact n)
                (if (= n 1)
                    1
                    (* n (fact (- n 1)))))
            (fact 5)
        ";

        let mut runtime = Runtime::new();
        assert_eq!(runtime.eval_str(input), runtime.eval_str("'fact 120"));
    })
}
