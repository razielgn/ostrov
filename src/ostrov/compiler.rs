use ast::AST;
use ast::AST::*;
use errors::RuntimeError;
use instructions::Instruction::*;
use instructions::{ArgumentsType, Bytecode, Instruction};

pub fn compile(ast: &[AST]) -> Result<Bytecode, RuntimeError> {
    let mut instructions = vec![];

    for ast_value in ast {
        instructions.append(&mut try!(compile_single(ast_value)));
    }

    Ok(instructions)
}

pub fn compile_single(ast: &AST) -> Result<Bytecode, RuntimeError> {
    match *ast {
        Integer(..) | Bool(..) => emit_constant(ast),
        Atom(ref atom) => emit_reference(atom),
        List(ref list) => emit_application(list),
        _ => Err(RuntimeError::MalformedExpression),
    }
}

fn emit_single_instr(instruction: Instruction) -> Result<Bytecode, RuntimeError> {
    let mut bytecode = vec![];
    bytecode.push(instruction);
    Ok(bytecode)
}

fn emit_constant(value: &AST) -> Result<Bytecode, RuntimeError> {
    emit_single_instr(LoadConstant(value.clone()))
}

fn emit_reference(atom: &str) -> Result<Bytecode, RuntimeError> {
    emit_single_instr(LoadReference(atom.into()))
}

fn emit_application(list: &[AST]) -> Result<Bytecode, RuntimeError> {
    if list.is_empty() {
        return Err(RuntimeError::MalformedExpression);
    }

    let head = &list[0];
    let tail = &list[1..];

    if let Atom(ref special_form) = *head {
        let args = tail;

        match special_form.as_ref() {
            "if" => return emit_if(args),
            "and" => return emit_and(args),
            "or" => return emit_or(args),
            "quote" => return emit_constant(&args[0]),
            "set!" => return emit_set(args),
            "define" => return emit_define(args),
            "lambda" => return emit_lambda(args),
            "let" => return emit_let(args),
            _ => (),
        }
    }

    emit_apply(head, tail)
}

fn emit_if(args: &[AST]) -> Result<Bytecode, RuntimeError> {
    if args.len() < 2 || args.len() > 3 {
        return Err(RuntimeError::BadArity(Some("if".into())));
    }

    let mut instructions = vec![];

    instructions.append(&mut try!(compile_single(&args[0])));

    let mut then = try!(compile_single(&args[1]));
    instructions.push(JumpOnFalse(then.len() + 1));
    instructions.append(&mut then);

    if args.len() == 3 {
        let mut else_ = try!(compile_single(&args[2]));
        instructions.push(Jump(else_.len()));
        instructions.append(&mut else_);
    } else {
        instructions.push(Jump(1usize));
        instructions.push(LoadUnspecified);
    }

    Ok(instructions)
}

fn emit_and(args: &[AST]) -> Result<Bytecode, RuntimeError> {
    emit_logical_op(args, true, JumpOnFalse)
}

fn emit_or(args: &[AST]) -> Result<Bytecode, RuntimeError> {
    emit_logical_op(args, false, JumpOnTrue)
}

fn emit_logical_op<F>(
    args: &[AST],
    default: bool,
    instruction: F,
) -> Result<Bytecode, RuntimeError>
where
    F: Fn(usize) -> Instruction,
{
    let mut instructions = vec![];

    if args.is_empty() {
        instructions.push(LoadConstant(Bool(default)));
    } else {
        let mut compiled_args = Vec::with_capacity(args.len());
        let mut sizes = Vec::with_capacity(args.len());

        for arg in args {
            let compiled_single = try!(compile_single(arg));
            sizes.push(compiled_single.len());
            compiled_args.push(compiled_single);
        }

        sizes.reverse();

        let mut jumps: Vec<usize> = sizes
            .iter()
            .enumerate()
            .map(|(i, size)| {
                let sum_of_previouses: usize = sizes.iter().take(i).sum();
                size + sum_of_previouses + i
            })
            .rev()
            .skip(1)
            .collect();

        jumps.push(0);

        for (mut compiled_arg, jump) in
            compiled_args.into_iter().zip(jumps.into_iter())
        {
            instructions.append(&mut compiled_arg);
            instructions.push(instruction(jump));
        }
        instructions.pop();
    }

    Ok(instructions)
}

fn emit_set(args: &[AST]) -> Result<Bytecode, RuntimeError> {
    if args.len() != 2 {
        return Err(RuntimeError::BadArity(Some("set!".into())));
    }

    if let Atom(ref name) = args[0] {
        let mut argument = try!(compile_single(&args[1]));
        let mut instructions = vec![];
        instructions.append(&mut argument);
        instructions.push(Replace(name.clone()));
        Ok(instructions)
    } else {
        Err(RuntimeError::MalformedExpression)
    }
}

fn emit_define(args: &[AST]) -> Result<Bytecode, RuntimeError> {
    if args.is_empty() {
        return Err(RuntimeError::MalformedExpression);
    }

    let mut instructions = vec![];

    match args[0] {
        Atom(ref name) => {
            if args.len() == 2 {
                instructions.append(&mut try!(compile_single(&args[1])));
            } else {
                instructions.push(LoadUnspecified);
            }

            instructions.push(Assignment(name.clone()));
        }
        List(ref list) if !list.is_empty() => {
            let name = try!(unpack_atom(&list[0]));
            let arg_names = &list[1..];
            let body = &args[1..];

            let mut lambda = vec![List(arg_names.to_vec())];
            for x in body {
                lambda.push(x.clone());
            }
            instructions.append(&mut try!(emit_lambda(&lambda)));

            instructions.push(Assignment(name));
        }
        DottedList(ref list, ref extra) if !list.is_empty() => {
            let name = try!(unpack_atom(&list[0]));
            let arg_names = &list[1..];
            let body = &args[1..];

            let mut lambda = Vec::new();

            if arg_names.is_empty() {
                lambda.push(*extra.clone());
            } else {
                lambda.push(DottedList(arg_names.to_vec(), extra.clone()));
            }

            for x in body {
                lambda.push(x.clone());
            }
            instructions.append(&mut try!(emit_lambda(&lambda)));

            instructions.push(Assignment(name));
        }
        _ => return Err(RuntimeError::MalformedExpression),
    }

    Ok(instructions)
}

fn emit_apply(head: &AST, args: &[AST]) -> Result<Bytecode, RuntimeError> {
    let mut instructions = vec![];
    instructions.push(Frame);

    for arg in args {
        instructions.append(&mut try!(compile_single(arg)));
        instructions.push(Argument);
    }

    instructions.append(&mut try!(compile_single(head)));
    instructions.push(Apply);
    Ok(instructions)
}

fn emit_lambda(args_: &[AST]) -> Result<Bytecode, RuntimeError> {
    let body = &args_[1..];
    if body.is_empty() {
        return Err(RuntimeError::MalformedExpression);
    }

    let mut instructions = vec![];

    let compiled_body = try!(compile(body));
    let (args, args_type) = try!(function_arguments(&args_[0]));
    instructions.push(Close {
        args,
        args_type,
        body: compiled_body,
    });

    Ok(instructions)
}

fn emit_let(args_: &[AST]) -> Result<Bytecode, RuntimeError> {
    if args_.len() < 2 {
        return Err(RuntimeError::BadArity(Some("let".into())));
    }

    let mut instructions = vec![];

    if let List(ref bindings) = *args_.first().unwrap() {
        let mut params = Vec::with_capacity(bindings.len());

        instructions.push(Frame);

        for binding in bindings.iter() {
            match *binding {
                List(ref binding) if binding.len() == 2 => {
                    params.push(try!(unpack_atom(&binding[0])));

                    instructions.append(&mut try!(compile_single(&binding[1])));
                    instructions.push(Argument);
                }
                _ => return Err(RuntimeError::MalformedExpression),
            }
        }

        let mut inner_instr = vec![];
        for body in &args_[1..] {
            inner_instr.append(&mut try!(compile_single(body)));
        }

        instructions.push(Close {
            args: params,
            args_type: ArgumentsType::Fixed,
            body: inner_instr,
        });

        instructions.push(Apply);
    } else {
        return Err(RuntimeError::MalformedExpression);
    }

    Ok(instructions)
}

fn function_arguments(
    ast: &AST,
) -> Result<(Vec<String>, ArgumentsType), RuntimeError> {
    match *ast {
        List(ref list) => {
            let mut atoms = Vec::with_capacity(list.len());

            for atom in list {
                let arg = try!(unpack_atom(atom));
                atoms.push(arg.clone());
            }

            Ok((atoms, ArgumentsType::Fixed))
        }
        DottedList(ref list, ref extra) => {
            let mut atoms = Vec::with_capacity(list.len() + 1);

            for atom in list {
                let arg = try!(unpack_atom(atom));
                atoms.push(arg.clone());
            }

            let arg = try!(unpack_atom(extra));
            atoms.push(arg);

            Ok((atoms, ArgumentsType::Variable))
        }
        Atom(ref arg) => Ok((vec![arg.clone()], ArgumentsType::Any)),
        _ => Err(RuntimeError::MalformedExpression),
    }
}

fn unpack_atom(value: &AST) -> Result<String, RuntimeError> {
    if let Atom(ref atom) = *value {
        Ok(atom.clone())
    } else {
        Err(RuntimeError::MalformedExpression)
    }
}

#[cfg(test)]
mod test {
    use super::compile;
    use ast::AST::*;
    use instructions::Instruction::*;
    use instructions::{ArgumentsType, Instruction};
    use parser::parse;

    fn parse_and_compile(input: &str) -> Vec<Instruction> {
        let ast = parse(input).expect(&format!("failed to parse {:?}", input));
        compile(&ast).expect(&format!("failed to compile {:?}", input))
    }

    #[test]
    fn constants_values_one_integer() {
        assert_eq!(vec![LoadConstant(Integer(1))], parse_and_compile("1"));
    }

    #[test]
    fn constants_values_two_integers() {
        assert_eq!(
            vec![LoadConstant(Integer(1)), LoadConstant(Integer(2))],
            parse_and_compile("1 2")
        );
    }

    #[test]
    fn constants_values_boolean() {
        assert_eq!(vec![LoadConstant(Bool(true))], parse_and_compile("#t"));
    }

    #[test]
    fn if_one_arg() {
        assert_eq!(
            vec![
                LoadConstant(Bool(true)),
                JumpOnFalse(2),
                LoadConstant(Integer(1)),
                Jump(1),
                LoadUnspecified,
            ],
            parse_and_compile("(if #t 1)")
        );
    }

    #[test]
    fn if_two_args() {
        assert_eq!(
            vec![
                LoadConstant(Bool(true)),
                JumpOnFalse(2),
                LoadConstant(Integer(1)),
                Jump(1),
                LoadConstant(Integer(2)),
            ],
            parse_and_compile("(if #t 1 2)")
        );

        assert_eq!(
            vec![
                LoadConstant(Bool(false)),
                JumpOnFalse(2),
                LoadConstant(Integer(1)),
                Jump(1),
                LoadConstant(Bool(true)),
                JumpOnFalse(2),
                LoadConstant(Integer(1)),
                Jump(1),
                LoadConstant(Integer(2)),
            ],
            parse_and_compile(
                "(if
                   (if #f 1 #t)
                     1
                     2)",
            )
        );
    }

    #[test]
    fn and() {
        assert_eq!(vec![LoadConstant(Bool(true))], parse_and_compile("(and)"));

        assert_eq!(vec![LoadConstant(Integer(1))], parse_and_compile("(and 1)"));

        assert_eq!(
            vec![
                Frame,
                LoadReference("+".into()),
                Apply,
                JumpOnFalse(7),
                LoadConstant(Bool(true)),
                JumpOnFalse(5),
                Frame,
                LoadReference("+".into()),
                Apply,
                JumpOnFalse(1),
                LoadConstant(Bool(false)),
            ],
            parse_and_compile("(and (+) #t (+) #f)")
        );
    }

    #[test]
    fn or() {
        assert_eq!(vec![LoadConstant(Bool(false))], parse_and_compile("(or)"));

        assert_eq!(vec![LoadConstant(Integer(1))], parse_and_compile("(or 1)"));

        assert_eq!(
            vec![
                Frame,
                LoadReference("+".into()),
                Apply,
                JumpOnTrue(7),
                LoadConstant(Bool(true)),
                JumpOnTrue(5),
                Frame,
                LoadReference("+".into()),
                Apply,
                JumpOnTrue(1),
                LoadConstant(Bool(false)),
            ],
            parse_and_compile("(or (+) #t (+) #f)")
        );
    }

    #[test]
    fn quote() {
        assert_eq!(
            vec![LoadConstant(Atom("a".into()))],
            parse_and_compile("'a")
        );

        assert_eq!(vec![LoadConstant(List(vec![]))], parse_and_compile("'()"));
    }

    #[test]
    fn variable_referencing() {
        assert_eq!(vec![LoadReference("+".into())], parse_and_compile("+"));
    }

    #[test]
    fn set_bang() {
        assert_eq!(
            vec![LoadConstant(Integer(23)), Replace("x".into())],
            parse_and_compile("(set! x 23)")
        );
    }

    #[test]
    fn define_with_nothing() {
        assert_eq!(
            vec![LoadUnspecified, Assignment("x".into())],
            parse_and_compile("(define x)")
        );
    }

    #[test]
    fn define_with_constant() {
        assert_eq!(
            vec![LoadConstant(Integer(25)), Assignment("x".into())],
            parse_and_compile("(define x 25)")
        );
    }

    #[test]
    fn define_with_lambda_fixed() {
        assert_eq!(
            vec![
                Close {
                    args: vec!["a".into()],
                    args_type: ArgumentsType::Fixed,
                    body: vec![
                        LoadReference("a".into()),
                        LoadReference("b".into()),
                        LoadReference("c".into()),
                    ],
                },
                Assignment("x".into()),
            ],
            parse_and_compile("(define (x a) a b c)")
        );
    }

    #[test]
    fn define_with_lambda_variable() {
        assert_eq!(
            vec![
                Close {
                    args: vec!["y".into(), "z".into()],
                    args_type: ArgumentsType::Variable,
                    body: vec![LoadReference("z".into())],
                },
                Assignment("x".into()),
            ],
            parse_and_compile("(define (x y . z) z)")
        );
    }

    #[test]
    fn define_with_lambda_any() {
        assert_eq!(
            vec![
                Close {
                    args: vec!["z".into()],
                    args_type: ArgumentsType::Any,
                    body: vec![LoadReference("z".into())],
                },
                Assignment("x".into()),
            ],
            parse_and_compile("(define (x . z) z)")
        );
    }

    #[test]
    fn lambda_fixed() {
        assert_eq!(
            vec![Close {
                args: vec!["x".into(), "y".into(), "z".into()],
                args_type: ArgumentsType::Fixed,
                body: vec![
                    LoadReference("x".into()),
                    LoadReference("y".into()),
                    LoadReference("z".into()),
                ],
            }],
            parse_and_compile("(lambda (x y z) x y z)")
        );
    }

    #[test]
    fn lambda_variable() {
        assert_eq!(
            vec![Close {
                args: vec!["x".into(), "y".into(), "z".into()],
                args_type: ArgumentsType::Variable,
                body: vec![LoadReference("x".into())],
            }],
            parse_and_compile("(lambda (x y . z) x)")
        );
    }

    #[test]
    fn lambda_any() {
        assert_eq!(
            vec![Close {
                args: vec!["x".into()],
                args_type: ArgumentsType::Any,
                body: vec![LoadReference("x".into())],
            }],
            parse_and_compile("(lambda x x)")
        );
    }

    #[test]
    fn let_() {
        assert_eq!(
            vec![
                Frame,
                LoadConstant(Integer(1)),
                Argument,
                LoadConstant(Integer(2)),
                Argument,
                Close {
                    args: vec!["a".into(), "b".into()],
                    args_type: ArgumentsType::Fixed,
                    body: vec![
                        Frame,
                        LoadReference("a".into()),
                        Argument,
                        LoadReference("b".into()),
                        Argument,
                        LoadReference("+".into()),
                        Apply,
                    ],
                },
                Apply,
            ],
            parse_and_compile(
                "(let ((a 1)
                       (b 2))
                   (+ a b))",
            )
        );
    }

    #[test]
    fn function_application() {
        assert_eq!(
            vec![Frame, LoadReference("+".into()), Apply],
            parse_and_compile("(+)")
        );

        assert_eq!(
            vec![
                Frame,
                LoadConstant(Integer(1)),
                Argument,
                LoadConstant(Integer(2)),
                Argument,
                LoadConstant(Integer(3)),
                Argument,
                LoadReference("+".into()),
                Apply,
            ],
            parse_and_compile("(+ 1 2 3)")
        );

        assert_eq!(
            vec![
                Frame,
                Frame,
                LoadConstant(Integer(1)),
                Argument,
                LoadConstant(Integer(2)),
                Argument,
                LoadReference("+".into()),
                Apply,
                Argument,
                Frame,
                LoadConstant(Integer(4)),
                Argument,
                LoadConstant(Integer(3)),
                Argument,
                LoadReference("-".into()),
                Apply,
                Argument,
                LoadReference("+".into()),
                Apply,
            ],
            parse_and_compile("(+ (+ 1 2) (- 4 3))")
        );
    }
}
