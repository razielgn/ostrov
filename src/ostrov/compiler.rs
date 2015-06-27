use instructions::Instruction;
use ast::AST;
use errors::Error;
use std::collections::LinkedList;

pub type Bytecode = LinkedList<Instruction>;

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
    let ref fun = list[0];

    if let &AST::Atom(ref fun) = fun {
        let args = &list[1..];

        match &**fun {
            "if"     => emit_if(args),
            "quote"  => emit_constant(&args[0]),
            "set!"   => emit_set(args),
            "define" => emit_define(args),
            _        => emit_apply(fun, args),
        }
    } else {
        Err(Error::MalformedExpression)
    }
}

fn emit_if(args: &[AST]) -> Result<Bytecode, Error> {
    let mut condition = try!(compile_single(&args[0]));
    let mut then      = try!(compile_single(&args[1]));
    let mut else_     = try!(compile_single(&args[2]));

    let mut instructions = LinkedList::new();
    instructions.append(&mut condition);
    instructions.push_back(Instruction::jump_on_false(then.len() + 1));
    instructions.append(&mut then);
    instructions.push_back(Instruction::jump(else_.len()));
    instructions.append(&mut else_);
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
    if let AST::Atom(ref name) = args[0] {
        let mut instructions = LinkedList::new();
        instructions.push_back(Instruction::load_unspecified());
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
