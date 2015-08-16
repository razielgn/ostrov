use ostrov::compiler::compile;
use ostrov::instructions::{Instruction, Bytecode, ArgumentsType};
use ostrov::parser::parse;
use helpers::ast::*;

use std::iter::FromIterator;

fn parse_and_compile(input: &str) -> Vec<Instruction> {
    Vec::from_iter(
        parse(input).
            and_then(|ast| compile(&ast)).
            unwrap().
            into_iter()
    )
}

fn bytecode(input: Vec<Instruction>) -> Bytecode {
    FromIterator::from_iter(input.into_iter())
}

#[test]
fn constants_values_one_integer() {
    assert_eq!(
        vec!(Instruction::load_constant(integer(1))),
        parse_and_compile("1")
    );
}

#[test]
fn constants_values_two_integers() {
    assert_eq!(
        vec!(
            Instruction::load_constant(integer(1)),
            Instruction::load_constant(integer(2)),
        ),
        parse_and_compile("1 2")
    );
}

#[test]
fn constants_values_boolean() {
    assert_eq!(
        vec!(
            Instruction::load_constant(bool(true))
        ),
        parse_and_compile("#t")
    );
}

#[test]
fn if_one_arg() {
    assert_eq!(
        vec!(
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_false(2),
            Instruction::load_constant(integer(1)),
            Instruction::jump(1),
            Instruction::load_unspecified(),
        ),
        parse_and_compile("(if #t 1)")
    );
}

#[test]
fn if_two_args() {
    assert_eq!(
        vec!(
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_false(2),
            Instruction::load_constant(integer(1)),
            Instruction::jump(1),
            Instruction::load_constant(integer(2)),
        ),
        parse_and_compile("(if #t 1 2)")
    );

    assert_eq!(
        vec!(
            Instruction::load_constant(bool(false)),
            Instruction::jump_on_false(2),
            Instruction::load_constant(integer(1)),
            Instruction::jump(1),
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_false(2),
            Instruction::load_constant(integer(1)),
            Instruction::jump(1),
            Instruction::load_constant(integer(2)),
        ),
        parse_and_compile("(if
                             (if #f 1 #t)
                             1
                             2)")
    );
}

#[test]
fn and() {
    assert_eq!(
        vec!(
            Instruction::load_constant(bool(true)),
        ),
        parse_and_compile("(and)")
    );

    assert_eq!(
        vec!(
            Instruction::load_constant(integer(1)),
        ),
        parse_and_compile("(and 1)")
    );

    assert_eq!(
        vec!(
            Instruction::frame(),
            Instruction::load_reference("+".to_owned()),
            Instruction::apply(),
            Instruction::jump_on_false(7),
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_false(5),
            Instruction::frame(),
            Instruction::load_reference("+".to_owned()),
            Instruction::apply(),
            Instruction::jump_on_false(1),
            Instruction::load_constant(bool(false)),
        ),
        parse_and_compile("(and (+) #t (+) #f)")
    );
}

#[test]
fn or() {
    assert_eq!(
        vec!(
            Instruction::load_constant(bool(false)),
        ),
        parse_and_compile("(or)")
    );

    assert_eq!(
        vec!(
            Instruction::load_constant(integer(1)),
        ),
        parse_and_compile("(or 1)")
    );

    assert_eq!(
        vec!(
            Instruction::frame(),
            Instruction::load_reference("+".to_owned()),
            Instruction::apply(),
            Instruction::jump_on_true(7),
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_true(5),
            Instruction::frame(),
            Instruction::load_reference("+".to_owned()),
            Instruction::apply(),
            Instruction::jump_on_true(1),
            Instruction::load_constant(bool(false)),
        ),
        parse_and_compile("(or (+) #t (+) #f)")
    );
}

#[test]
fn quote() {
    assert_eq!(
        vec!(
            Instruction::load_constant(atom("a")),
        ),
        parse_and_compile("'a")
    );

    assert_eq!(
        vec!(
            Instruction::load_constant(empty_list()),
        ),
        parse_and_compile("'()")
    );
}

#[test]
fn variable_referencing() {
    assert_eq!(
        vec!(
            Instruction::load_reference("+".to_string()),
        ),
        parse_and_compile("+")
    );
}

#[test]
fn set_bang() {
    assert_eq!(
        vec!(
            Instruction::load_constant(integer(23)),
            Instruction::replace("x".to_string()),
        ),
        parse_and_compile("(set! x 23)")
    );
}

#[test]
fn define_with_nothing() {
    assert_eq!(
        vec!(
            Instruction::load_unspecified(),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(define x)")
    );
}

#[test]
fn define_with_constant() {
    assert_eq!(
        vec!(
            Instruction::load_constant(integer(25)),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(define x 25)")
    );
}

#[test]
fn define_with_lambda_fixed() {
    assert_eq!(
        vec!(
            Instruction::close(
                vec!(
                    "a".to_string(),
                ),
                ArgumentsType::Fixed,
                bytecode(vec!(
                    Instruction::load_reference("a".to_string()),
                    Instruction::load_reference("b".to_string()),
                    Instruction::load_reference("c".to_string()),
                )),
            ),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(define (x a) a b c)")
    );
}

#[test]
fn define_with_lambda_variable() {
    assert_eq!(
        vec!(
            Instruction::close(
                vec!(
                    "y".to_string(),
                    "z".to_string(),
                ),
                ArgumentsType::Variable,
                bytecode(vec!(
                    Instruction::load_reference("z".to_string()),
                )),
            ),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(define (x y . z) z)")
    );
}

#[test]
fn define_with_lambda_any() {
    assert_eq!(
        vec!(
            Instruction::close(
                vec!(
                    "z".to_string(),
                ),
                ArgumentsType::Any,
                bytecode(vec!(
                    Instruction::load_reference("z".to_string()),
                )),
            ),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(define (x . z) z)")
    );
}

#[test]
fn lambda_fixed() {
    assert_eq!(
        vec!(
            Instruction::close(
                vec!(
                    "x".to_string(),
                    "y".to_string(),
                    "z".to_string(),
                ),
                ArgumentsType::Fixed,
                bytecode(vec!(
                    Instruction::load_reference("x".to_string()),
                    Instruction::load_reference("y".to_string()),
                    Instruction::load_reference("z".to_string()),
                )),
            ),
        ),
        parse_and_compile("(lambda (x y z) x y z)")
    );
}

#[test]
fn lambda_variable() {
    assert_eq!(
        vec!(
            Instruction::close(
                vec!(
                    "x".to_string(),
                    "y".to_string(),
                    "z".to_string(),
                ),
                ArgumentsType::Variable,
                bytecode(vec!(
                    Instruction::load_reference("x".to_string()),
                )),
            ),
        ),
        parse_and_compile("(lambda (x y . z) x)")
    );
}

#[test]
fn lambda_any() {
    assert_eq!(
        vec!(
            Instruction::close(
                vec!(
                    "x".to_string(),
                ),
                ArgumentsType::Any,
                bytecode(vec!(
                    Instruction::load_reference("x".to_string()),
                )),
            ),
        ),
        parse_and_compile("(lambda x x)")
    );
}

#[test]
fn let_() {
    assert_eq!(
        vec!(
            Instruction::frame(),
            Instruction::load_constant(integer(1)),
            Instruction::argument(),
            Instruction::load_constant(integer(2)),
            Instruction::argument(),
            Instruction::close(
                vec!(
                    "a".to_string(),
                    "b".to_string(),
                ),
                ArgumentsType::Fixed,
                bytecode(vec!(
                    Instruction::frame(),
                    Instruction::load_reference("a".to_string()),
                    Instruction::argument(),
                    Instruction::load_reference("b".to_string()),
                    Instruction::argument(),
                    Instruction::load_reference("+".to_string()),
                    Instruction::apply(),
                )),
            ),
            Instruction::apply(),
        ),
        parse_and_compile("(let ((a 1)
                                 (b 2))
                             (+ a b))")
    );
}

#[test]
fn function_application() {
    assert_eq!(
        vec!(
            Instruction::frame(),
            Instruction::load_reference("+".to_string()),
            Instruction::apply(),
        ),
        parse_and_compile("(+)")
    );

    assert_eq!(
        vec!(
            Instruction::frame(),
            Instruction::load_constant(integer(1)),
            Instruction::argument(),
            Instruction::load_constant(integer(2)),
            Instruction::argument(),
            Instruction::load_constant(integer(3)),
            Instruction::argument(),
            Instruction::load_reference("+".to_string()),
            Instruction::apply(),
        ),
        parse_and_compile("(+ 1 2 3)")
    );

    assert_eq!(
        vec!(
            Instruction::frame(),
            Instruction::frame(),
            Instruction::load_constant(integer(1)),
            Instruction::argument(),
            Instruction::load_constant(integer(2)),
            Instruction::argument(),
            Instruction::load_reference("+".to_string()),
            Instruction::apply(),
            Instruction::argument(),
            Instruction::frame(),
            Instruction::load_constant(integer(4)),
            Instruction::argument(),
            Instruction::load_constant(integer(3)),
            Instruction::argument(),
            Instruction::load_reference("-".to_string()),
            Instruction::apply(),
            Instruction::argument(),
            Instruction::load_reference("+".to_string()),
            Instruction::apply(),
        ),
        parse_and_compile("(+ (+ 1 2) (- 4 3))")
    );
}
