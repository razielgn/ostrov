use ostrov::compiler::compile;
use ostrov::instructions::Instruction;
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
fn variable_assignment() {
    assert_eq!(
        vec!(
            Instruction::load_reference("x".to_string()),
            Instruction::load_constant(integer(23)),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(set! x 23)")
    );

    assert_eq!(
        vec!(
            Instruction::load_unspecified(),
            Instruction::assignment("x".to_string()),
        ),
        parse_and_compile("(define x)")
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
