use instructions::{Instruction, Bytecode};
use ast::AST;
use errors::Error;
use std::collections::LinkedList;

pub fn compile(ast: &Vec<AST>) -> Result<Bytecode, Error> {
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

    let ref fun = list[0];

    if let &AST::Atom(ref fun) = fun {
        let args = &list[1..];

        match &**fun {
            "if"     => emit_if(args),
            "and"    => emit_and(args),
            "or"     => emit_or(args),
            "quote"  => emit_constant(&args[0]),
            "set!"   => emit_set(args),
            "define" => emit_define(args),
            "lambda" => emit_lambda(args),
            _        => emit_apply(fun, args),
        }
    } else {
        Err(Error::MalformedExpression)
    }
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
    let mut argument = try!(compile_single(&args[1]));

    if let AST::Atom(ref name) = args[0] {
        let mut instructions = LinkedList::new();
        instructions.push_back(Instruction::load_reference(name.clone()));
        instructions.append(&mut argument);
        instructions.push_back(Instruction::assignment(name.clone()));
        Ok(instructions)
    } else {
        Err(Error::MalformedExpression)
    }
}

fn emit_define(args: &[AST]) -> Result<Bytecode, Error> {
    if args.is_empty() {
        return Err(Error::MalformedExpression);
    }

    if let AST::Atom(ref name) = args[0] {
        let mut instructions = LinkedList::new();

        if args.len() == 2 {
            instructions.append(&mut try!(compile_single(&args[1])));
        } else {
            instructions.push_back(Instruction::load_unspecified());
        }

        instructions.push_back(Instruction::assignment(name.clone()));
        Ok(instructions)
    } else {
        Err(Error::MalformedExpression)
    }
}

fn emit_apply(fun: &String, args: &[AST]) -> Result<Bytecode, Error> {
    let mut instructions = LinkedList::new();
    instructions.push_back(Instruction::frame());

    for arg in args {
        instructions.append(&mut try!(compile_single(arg)));
        instructions.push_back(Instruction::argument());
    }

    instructions.append(&mut try!(emit_reference(fun)));
    instructions.push_back(Instruction::apply());
    Ok(instructions)
}

fn emit_lambda(args_: &[AST]) -> Result<Bytecode, Error> {
    let args = try!(list_of_atoms(&args_[0]));
    let compiled_body = try!(compile_single(&args_[1]));

    let mut instructions = LinkedList::new();
    instructions.push_back(Instruction::close(args, compiled_body));
    Ok(instructions)
}

fn list_of_atoms(ast: &AST) -> Result<Vec<String>, Error> {
    if let AST::List(ref list) = *ast {
        let mut atoms = Vec::with_capacity(list.len());

        for atom in list {
            if let AST::Atom(ref atom) = *atom {
                atoms.push(atom.clone());
            } else {
                return Err(Error::MalformedExpression);
            }
        }

        Ok(atoms)
    } else {
        Err(Error::MalformedExpression)
    }
}
