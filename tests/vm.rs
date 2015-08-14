use ostrov::vm::VM;
use ostrov::instructions::Instruction;
use helpers::*;
use helpers::ast::*;

#[test]
fn execute_load_constant() {
    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(integer(1)),
            Instruction::load_constant(integer(2)),
            Instruction::load_constant(integer(3)),
        );

        assert_eq!(
            Ok(vm.memory.integer(3)),
            vm.execute(instr.into_iter())
        );
    }

    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(bool(false)),
            Instruction::load_constant(bool(true)),
            Instruction::load_constant(bool(false)),
        );

        assert_eq!(
            Ok(vm.memory.b_false()),
            vm.execute(instr.into_iter())
        );
    }
}

#[test]
fn execute_jump() {
    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(integer(23)),
            Instruction::jump(1),
            Instruction::load_constant(integer(42)),
        );

        assert_eq!(
            Ok(vm.memory.integer(23)),
            vm.execute(instr.into_iter())
        );
    }
}

#[test]
fn execute_jump_on_false() {
    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(bool(false)),
            Instruction::jump_on_false(1),
            Instruction::load_constant(integer(23)),
        );

        assert_eq!(
            Ok(vm.memory.b_false()),
            vm.execute(instr.into_iter())
        );
    }

    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(bool(false)),
            Instruction::jump_on_false(2),
            Instruction::load_constant(integer(1)),
            Instruction::jump(1),
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_false(2),
            Instruction::load_constant(integer(1)),
            Instruction::jump(1),
            Instruction::load_constant(integer(2)),
        );

        assert_eq!(
            Ok(vm.memory.integer(1)),
            vm.execute(instr.into_iter())
        );
    }
}

#[test]
fn execute_jump_on_true() {
    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(bool(false)),
            Instruction::jump_on_true(1),
            Instruction::load_constant(integer(23)),
        );

        assert_eq!(
            Ok(vm.memory.integer(23)),
            vm.execute(instr.into_iter())
        );
    }

    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_constant(bool(true)),
            Instruction::jump_on_true(1),
            Instruction::load_constant(integer(23)),
        );

        assert_eq!(
            Ok(vm.memory.b_true()),
            vm.execute(instr.into_iter())
        );
    }
}

#[test]
fn execute_load_reference() {
    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_reference("a".to_string()),
        );

        assert_eq!(
            Err(unbound_variable_error("a")),
            vm.execute(instr.into_iter())
        );
    }

    {
        let mut vm = VM::new();
        let instr = vec!(
            Instruction::load_reference("a".to_string()),
        );

        vm.env.set("a".to_string(), vm.memory.integer(1));

        assert_eq!(
            Ok(vm.memory.integer(1)),
            vm.execute(instr.into_iter())
        );
    }
}

#[test]
fn execute_assignment() {
    let mut vm = VM::new();
    let instr = vec!(
        Instruction::load_constant(integer(1)),
        Instruction::assignment("x".to_string()),
        Instruction::load_constant(integer(2)),
        Instruction::load_reference("x".to_string()),
    );

    assert_eq!(
        Ok(vm.memory.integer(1)),
        vm.execute(instr.into_iter())
    );
}

#[test]
fn execute_load_unspecified() {
    let mut vm = VM::new();
    let instr = vec!(
        Instruction::load_constant(integer(1)),
        Instruction::load_unspecified(),
    );

    assert_eq!(
        Ok(vm.memory.unspecified()),
        vm.execute(instr.into_iter())
    );
}

#[test]
fn execute_apply() {
    let mut vm = VM::new();
    let instr = vec!(
        Instruction::frame(),
        Instruction::load_reference("+".to_string()),
        Instruction::apply(),
    );

    assert_eq!(
        Ok(vm.memory.integer(0)),
        vm.execute(instr.into_iter())
    );
}

#[test]
fn execute_argument() {
    let mut vm = VM::new();
    let instr = vec!(
        Instruction::frame(),
        Instruction::load_constant(integer(2)),
        Instruction::argument(),
        Instruction::load_reference("+".to_string()),
        Instruction::apply(),
    );

    assert_eq!(
        Ok(vm.memory.integer(2)),
        vm.execute(instr.into_iter())
    );
}

#[test]
fn execute_nested_arguments() {
    let mut vm = VM::new();
    let instr = vec!(
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
    );

    assert_eq!(
        Ok(vm.memory.integer(4)),
        vm.execute(instr.into_iter())
    );
}

#[test]
fn illegal_frame_apply() {
    let mut vm = VM::new();
    let instr = vec!(
        Instruction::load_reference("+".to_string()),
        Instruction::apply(),
    );

    assert_eq!(
        Err(cannot_pop_last_frame()),
        vm.execute(instr.into_iter())
    );
}
