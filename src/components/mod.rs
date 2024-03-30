pub mod instruction;
pub mod register;
pub mod vm;

use vm::VM;

pub const MEMORY_SIZE: usize = std::u16::MAX as usize;

pub fn execute_program(vm: &mut VM) {
    while vm.registers.pc < MEMORY_SIZE as u16 {
        let instruction = vm.read_memory(vm.registers.pc);

        // increment program counter
        vm.registers.pc += 1;

        instruction::execute_instruction(instruction, vm)
    }
}