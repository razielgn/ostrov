use instructions::{Instruction, Bytecode, ArgumentsType};
use ast::AST;
use errors::Error;
use std::collections::LinkedList;

pub fn compile(ast: &[AST]) -> Result<Bytecode, Error> {
    let mut instructions = LinkedList::new();

    for ast_value in ast {
        instructions.append(
            &mut try!(compile_single(ast_value))
        );
    }

    Ok(instructions)
}

pub fn compile_single(ast: &AST) -> Result<Bytecode, Error> {
    match ast {
        &AST::Integer(..) | &AST::Bool(..) =>
            emit_constant(ast),
        &AST::Atom(ref atom) =>
            emit_reference(atom),
        &AST::List(ref list) =>
            emit_application(list),
        _ =>
            Err(Error::MalformedExpression),
    }
}

fn emit_single_instr(instruction: Instruction) -> Result<Bytecode, Error> {
    let mut bytecode = LinkedList::new();
    bytecode.push_back(instruction);
    Ok(bytecode)
}

fn emit_constant(value: &AST) -> Result<Bytecode, Error> {
    emit_single_instr(
        Instruction::load_constant(value.clone())
    )
}

fn emit_reference(atom: &String) -> Result<Bytecode, Error> {
    emit_single_instr(
        Instruction::load_reference(atom.clone())
    )
}

fn emit_application(list: &Vec<AST>) -> Result<Bytecode, Error> {
    if list.is_empty() {
        return Err(Error::MalformedExpression);
    }

    let head = &list[0];
    let tail = &list[1..];

    if let &AST::Atom(ref special_form) = head {
        let args = tail;

        match special_form.as_ref() {
            "if"     => return emit_if(args),
            "and"    => return emit_and(args),
            "or"     => return emit_or(args),
            "quote"  => return emit_constant(&args[0]),
            "set!"   => return emit_set(args),
            "define" => return emit_define(args),
            "lambda" => return emit_lambda(args),
            "let"    => return emit_let(args),
            _        => (),
        }
    }

    emit_apply(head, tail)
}

fn emit_if(args: &[AST]) -> Result<Bytecode, Error> {
    if args.len() < 2 || args.len() > 3 {
        return Err(Error::BadArity(Some("if".to_owned())));
    }

    let mut instructions = LinkedList::new();

    instructions.append(&mut try!(compile_single(&args[0])));

    let mut then = try!(compile_single(&args[1]));
    instructions.push_back(Instruction::jump_on_false(then.len() + 1));
    instructions.append(&mut then);

    if args.len() == 3 {
        let mut else_ = try!(compile_single(&args[2]));
        instructions.push_back(Instruction::jump(else_.len()));
        instructions.append(&mut else_);
    } else {
        instructions.push_back(Instruction::jump(1usize));
        instructions.push_back(Instruction::load_unspecified());
    }

    Ok(instructions)
}

fn emit_and(args: &[AST]) -> Result<Bytecode, Error> {
    emit_logical_op(args, true, Instruction::jump_on_false)
}

fn emit_or(args: &[AST]) -> Result<Bytecode, Error> {
    emit_logical_op(args, false, Instruction::jump_on_true)
}

fn emit_logical_op<F>(args: &[AST], default: bool, instruction: F) -> Result<Bytecode, Error>
    where F: Fn(usize) -> Instruction
{
    let mut instructions = LinkedList::new();

    if args.len() == 0 {
        instructions.push_back(Instruction::load_constant(AST::Bool(default)));
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
                let sum_of_previouses = sizes.iter().take(i).fold(0, |acc, n| acc + n);
                size + sum_of_previouses + i
            })
            .rev()
            .skip(1)
            .collect();

        jumps.push(0);

        for (mut compiled_arg, jump) in compiled_args.into_iter().zip(jumps.into_iter()) {
            instructions.append(&mut compiled_arg);
            instructions.push_back(instruction(jump));
        }
        instructions.pop_back();
    }

    Ok(instructions)
}

fn emit_set(args: &[AST]) -> Result<Bytecode, Error> {
    if args.len() != 2 {
        return Err(Error::BadArity(Some("set!".to_owned())));
    }

    if let AST::Atom(ref name) = args[0] {
        let mut argument = try!(compile_single(&args[1]));
        let mut instructions = LinkedList::new();
        instructions.append(&mut argument);
        instructions.push_back(Instruction::replace(name.clone()));
        Ok(instructions)
    } else {
        Err(Error::MalformedExpression)
    }
}

fn emit_define(args: &[AST]) -> Result<Bytecode, Error> {
    if args.is_empty() {
        return Err(Error::MalformedExpression);
    }

    let mut instructions = LinkedList::new();

    match args[0] {
        AST::Atom(ref name) => {
            if args.len() == 2 {
                instructions.append(&mut try!(compile_single(&args[1])));
            } else {
                instructions.push_back(Instruction::load_unspecified());
            }

            instructions.push_back(Instruction::assignment(name.clone()));
        }
        AST::List(ref list) if list.len() > 0 => {
            let name = try!(unpack_atom(&list[0]));
            let ref arg_names = list[1..];
            let ref body = args[1..];

            let mut lambda = vec!(AST::List(arg_names.to_vec()));
            for x in body {
                lambda.push(x.clone());
            }
            instructions.append(&mut try!(emit_lambda(&lambda)));

            instructions.push_back(Instruction::assignment(name));
        }
        AST::DottedList(ref list, ref extra) if list.len() > 0 => {
            let name = try!(unpack_atom(&list[0]));
            let ref arg_names = list[1..];
            let ref body = args[1..];

            let mut lambda = Vec::new();

            if arg_names.len() > 0 {
                lambda.push(AST::DottedList(arg_names.to_vec(), extra.clone()));
            } else {
                lambda.push(*extra.clone());
            }

            for x in body {
                lambda.push(x.clone());
            }
            instructions.append(&mut try!(emit_lambda(&lambda)));

            instructions.push_back(Instruction::assignment(name));
        }
        _ =>
            return Err(Error::MalformedExpression)
    }

    Ok(instructions)
}

fn emit_apply(head: &AST, args: &[AST]) -> Result<Bytecode, Error> {
    let mut instructions = LinkedList::new();
    instructions.push_back(Instruction::frame());

    for arg in args {
        instructions.append(&mut try!(compile_single(arg)));
        instructions.push_back(Instruction::argument());
    }

    instructions.append(&mut try!(compile_single(head)));
    instructions.push_back(Instruction::apply());
    Ok(instructions)
}

fn emit_lambda(args_: &[AST]) -> Result<Bytecode, Error> {
    let mut instructions = LinkedList::new();

    let compiled_body = try!(compile(&args_[1..]));
    let (args, args_type) = try!(function_arguments(&args_[0]));
    instructions.push_back(Instruction::close(args, args_type, compiled_body));

    Ok(instructions)
}

fn emit_let(args_: &[AST]) -> Result<Bytecode, Error> {
    if args_.len() < 2 {
        return Err(Error::BadArity(Some("let".to_owned())));
    }

    let mut instructions = LinkedList::new();

    if let &AST::List(ref bindings) = args_.first().unwrap() {
        let mut params = Vec::with_capacity(bindings.len());

        instructions.push_back(Instruction::frame());

        for binding in bindings.iter() {
            match binding {
                &AST::List(ref binding) if binding.len() == 2 => {
                    params.push(try!(unpack_atom(&binding[0])));

                    instructions.append(&mut try!(compile_single(&binding[1])));
                    instructions.push_back(Instruction::argument());
                }
                _ =>
                    return Err(Error::MalformedExpression),
            }
        }

        let mut inner_instr = LinkedList::new();
        for body in &args_[1..] {
            inner_instr.append(&mut try!(compile_single(body)));
        }

        instructions.push_back(Instruction::close(
            params,
            ArgumentsType::Fixed,
            inner_instr,
        ));

        instructions.push_back(Instruction::apply());
    } else {
        return Err(Error::MalformedExpression);
    }

    Ok(instructions)
}

fn function_arguments(ast: &AST) -> Result<(Vec<String>, ArgumentsType), Error> {
    match *ast {
        AST::List(ref list) => {
            let mut atoms = Vec::with_capacity(list.len());

            for atom in list {
                let arg = try!(unpack_atom(atom));
                atoms.push(arg.clone());
            }

            Ok((atoms, ArgumentsType::Fixed))
        }
        AST::DottedList(ref list, ref extra) => {
            let mut atoms = Vec::with_capacity(list.len() + 1);

            for atom in list {
                let arg = try!(unpack_atom(atom));
                atoms.push(arg.clone());
            }

            let arg = try!(unpack_atom(extra));
            atoms.push(arg);

            Ok((atoms, ArgumentsType::Variable))
        }
        AST::Atom(ref arg) => {
            Ok((vec!(arg.clone()), ArgumentsType::Any))
        }
        _ =>
            Err(Error::MalformedExpression)
    }
}

fn unpack_atom(value: &AST) -> Result<String, Error> {
    if let &AST::Atom(ref atom) = value {
        Ok(atom.to_string())
    } else {
        Err(Error::MalformedExpression)
    }
}
