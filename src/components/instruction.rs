/// LC-3 has 16 opcodes, each instruction 16 bits long — first 4 bits store the OpCode, rest are saved for parameters

/// This file includes every single instruction: br, add, ld, st, jsr, and, ldr, str, rti, not, ldi, sti, jmp, res, lea, trap

use super::vm::VM; 

use std::io;
use std::io::Read;
use std::io::Write;
use std::process;

#[derive(Debug)] // default debug functionality
pub enum OpCode {
    BR = 0, // branch
    ADD,    // add
    LD,     // load
    ST,     // store
    JSR,    // jump register
    AND,    // bitwise and
    LDR,    // load register
    STR,    // store register
    RTI,    // unused
    NOT,    // bitwise not
    LDI,    // load indirect
    STI,    // store indirect
    JMP,    // jump
    RES,    // reserved (unused)
    LEA,    // load effective address
    TRAP,   // execute trap
}

// TRAP Codes
pub enum TrapCode {
    // keyboard mapping
    Getc = 0x20,
    // output chars
    Out = 0x21,
    // output string
    Puts = 0x22,
    // input string
    In = 0x23,
    // output a byte string
    Putsp = 0x24,
    // halts program
    Halt = 0x25,
}

pub fn execute_instruction(instr: u16, vm: &mut VM) {
    // Extract OpCode from instruction
    let op_code = get_opcode(&instr);

    match op_code {
        Some(OpCode::ADD) => add(instr, vm),
        Some(OpCode::AND) => and(instr, vm),
        Some(OpCode::NOT) => not(instr, vm),
        Some(OpCode::BR) => br(instr, vm),
        Some(OpCode::JMP) => jmp(instr, vm),
        Some(OpCode::JSR) => jsr(instr, vm),
        Some(OpCode::LD) => ld(instr, vm),
        Some(OpCode::LDI) => ldi(instr, vm),
        Some(OpCode::LDR) => ldr(instr, vm),
        Some(OpCode::LEA) => lea(instr, vm),
        Some(OpCode::ST) => st(instr, vm),
        Some(OpCode::STI) => sti(instr, vm),
        Some(OpCode::STR) => str(instr, vm),
        Some(OpCode::TRAP) => trap(instr, vm),
        _ => {}
    }
}

// Each instruction is 16 bits long, l4 store opcode, rest store parameters
pub fn get_opcode(instruction: &u16) -> Option<OpCode> {
    match instruction >> 12 {
        0 => Some(OpCode::BR),
        1 => Some(OpCode::ADD),
        2 => Some(OpCode::LD),
        3 => Some(OpCode::ST),
        4 => Some(OpCode::JSR),
        5 => Some(OpCode::AND),
        6 => Some(OpCode::LDR),
        7 => Some(OpCode::STR),
        8 => Some(OpCode::RTI),
        9 => Some(OpCode::NOT),
        10 => Some(OpCode::LDI),
        11 => Some(OpCode::STI),
        12 => Some(OpCode::JMP),
        13 => Some(OpCode::RES),
        14 => Some(OpCode::LEA),
        15 => Some(OpCode::TRAP),
        _ => None,
    }
}

pub fn add(instruction: u16, vm: &mut VM) {
    // Get destination address using bitwise operation to shift binary for DR.
    let dr = (instruction >> 9) & 0x7;

    // First operand — move 6
    let sr1 = (instruction >> 6) & 0x7;

    // Check if we're in immediate mode or register mode (imm_flag)
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);

        // set as u32 to prevent overflow
        let val: u32 = imm5 as u32 + vm.registers.get(sr1) as u32;

        // result of sum set from target register
        vm.registers.update(dr, val as u16);
    } else {
        // 2nd needs to be extracted in this case
        let sr2 = instruction & 0x7;

        // rest is normal
        
        let val: u32 = vm.registers.get(sr1) as u32 + vm.registers.get(sr2) as u32;

        vm.registers.update(dr, val as u16);
    }

    // dr last operation
    vm.registers.update_r_cond_register(dr);
}

/* 
The address is determined by sign-extending bits [8:0] to 16 bits and adding it to the incremented PC. The content stored in memory at this computed address represents the data to be loaded into DR, with condition codes set accordingly.
*/
pub fn ldi(instruction: u16, vm: &mut VM) {
    // Get the direct register
    let dr = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    // This sum addresses a location in memory — contains another value: the address of the value to load
    let first_read = vm.read_memory(vm.registers.pc + pc_offset);

    // Read the resulting address and update the DR.
    let resulting_address = vm.read_memory(first_read);
    vm.registers.update(dr, resulting_address);
    vm.registers.update_r_cond_register(dr);
}

// Normal `and` functionality
pub fn and(instruction: u16, vm: &mut VM) {
    // Get the direct register encoded in the instruction
    let dr = (instruction >> 9) & 0x7;

    let sr1 = (instruction >> 6) & 0x7;
    let imm_flag = (instruction >> 5) & 0x1;

    if imm_flag == 1 {
        let imm5 = sign_extend(instruction & 0x1F, 5);
        // execute and store bitwise value in the DR.
        vm.registers.update(dr, vm.registers.get(sr1) & imm5);
    } else {
        let sr2 = instruction & 0x7;
        // same as above
        vm.registers.update(dr, vm.registers.get(sr1) & vm.registers.get(sr2));
    }

    vm.registers.update_r_cond_register(dr);
}

// Binary negation
pub fn not(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;
    let sr1 = (instruction >> 6) & 0x7;
    vm.registers.update(dr, !vm.registers.get(sr1));

    vm.registers.update_r_cond_register(dr);
}

// The branching operation: redirect a location within assembly code depending on bit conditions [11:9]
pub fn br(instruction: u16, vm: &mut VM) {
    let pc_offset = sign_extend((instruction) & 0x1ff, 9);

    let cond_flag = (instruction >> 9) & 0x7;

    // combine '001', xor '010', xor '100' stored in the condition register w/ instruction
    if cond_flag & vm.registers.cond != 0 {
        let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
        vm.registers.pc = val as u16;
    }

}

// The program unconditionally jumps to the location specified by the contents of the base register.

// typical assembly classifications
pub fn jmp(instruction: u16, vm: &mut VM) {
    // base_reg will either be an arbitrary register or the register 7 (`111`) — `RET` operation.
    let base_reg = (instruction >> 6) & 0x7;
    vm.registers.pc = vm.registers.get(base_reg);
}

// Save the he incremented PC in R7, load with subroutine instruction to cause unconditional jump
pub fn jsr(instruction: u16, vm: &mut VM) {
    // base register
    let base_reg = (instruction >> 6) & 0x7;

    let long_pc_offset = sign_extend(instruction & 0x7ff, 11);

    let long_flag = (instruction >> 11) & 1;

    // Save the incremented PC in R7
    vm.registers.r7 = vm.registers.pc;

    if long_flag != 0 {
        // the address to jump from PCOffset11
        let val: u32 = vm.registers.pc as u32 + long_pc_offset as u32;
        vm.registers.pc = val as u16;
    } else {
        // address to jump to in the base register
        vm.registers.pc = vm.registers.get(base_reg);
    }
}

/* 
An address is computed by sign-extending bits [8:0] to 16 bits and adding this value to the incremented PC: contents into DR, condition codes set.
*/
pub fn ld(instruction: u16, vm: &mut VM) {
    // Get the direct register encoded in the instruction (see `add` fn for more in-depth details)
    let dr = (instruction >> 9) & 0x7;

    // Grab the PCOffset and sign extend it
    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    let mem: u32 = pc_offset as u32 + vm.registers.pc as u32;

    // Read the value from the place where the memory above was computed
    let value = vm.read_memory(mem as u16);

    // Save that value to the direct register and update the condition register
    vm.registers.update(dr, value);
    vm.registers.update_r_cond_register(dr);
}

// Load base + offset
pub fn ldr(instruction: u16, vm: &mut VM) {
    // Get the direct register encoded in the instruction (see `add` fn for more in-depth details)
    let dr = (instruction >> 9) & 0x7;

    // Grab the base register
    let base_reg = (instruction >> 6) & 0x7;

    // Grab the offset and sign extend it
    let offset = sign_extend(instruction & 0x3F, 6);

    // Compute the memory location to be loaded
    let val: u32 = vm.registers.get(base_reg) as u32 + offset as u32;

    // Read the value at that memory location
    let mem_value = vm.read_memory(val as u16).clone();

    // Update the register with the loaded value and update the condition register
    vm.registers.update(dr, mem_value);
    vm.registers.update_r_cond_register(dr);
}

pub fn lea(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    let val: u32 = vm.registers.pc as u32 + pc_offset as u32;

    vm.registers.update(dr, val as u16);

    vm.registers.update_r_cond_register(dr);
}

pub fn st(instruction: u16, vm: &mut VM) {
    let sr = (instruction >> 9) & 0x7;

    // Grab the PC offset and sign extend it
    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    // add current PC to PC offset and convert to avoid overflow
    let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;

    // Store the value in the register being passed at above instructed address
    vm.write_memory(val as usize, vm.registers.get(sr));
}


pub fn sti(instruction: u16, vm: &mut VM) {
    let sr = (instruction >> 9) & 0x7;

    let pc_offset = sign_extend(instruction & 0x1ff, 9);

    let val: u32 = vm.registers.pc as u32 + pc_offset as u32;
    let val: u16 = val as u16;

    // This is the difference between STI and ST
    let address = vm.read_memory(val) as usize;

    vm.write_memory(address, vm.registers.get(sr));
}

pub fn str(instruction: u16, vm: &mut VM) {
    let dr = (instruction >> 9) & 0x7;

    let base_reg = (instruction >> 6) & 0x7;

    let offset = sign_extend(instruction & 0x3F, 6);

    let val: u32 = vm.registers.get(base_reg) as u32 + offset as u32;
    let val: u16 = val as u16;
    vm.write_memory(val as usize, vm.registers.get(dr));
}

// I/O device interaction

// figure out what exactly is accessed and how the parts work together
pub fn trap(instruction: u16, vm: &mut VM) {
    match instruction & 0xFF {
        0x20 => {
            // Get character
            let mut buffer = [0; 1];
            std::io::stdin().read_exact(&mut buffer).unwrap();
            vm.registers.r0 = buffer[0] as u16;
        }
        0x21 => {
            // Write out character
            let c = vm.registers.r0 as u8;
            print!("{}", c as char);
        }
        0x22 => {
            let mut index = vm.registers.r0;
            let mut c = vm.read_memory(index);
            while c != 0x0000 {
                print!("{}", (c as u8) as char);
                index += 1;
                c = vm.read_memory(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        0x23 => {
            // take input, print prompt and read a char (y/n typically), ASCII encoded into R0 + clear the high 8bits of R0
            print!("Enter a  character : ");
            io::stdout().flush().expect("failed to flush");
            let char = std::io::stdin()
                .bytes()
                .next()
                .and_then(|result| result.ok())
                .map(|byte| byte as u16)
                .unwrap();
            vm.registers.update(0, char);
        }
        0x24 => {
            // Putsp — packed string
            let mut index = vm.registers.r0;
            let mut c = vm.read_memory(index);
            while c != 0x0000 {
                let c1 = ((c & 0xFF) as u8) as char;
                print!("{}", c1);
                let c2 = ((c >> 8) as u8) as char;
                if c2 != '\0' {
                    print!("{}", c2);
                }
                index += 1;
                c = vm.read_memory(index);
            }
            io::stdout().flush().expect("failed to flush");
        }
        0x25 => {
            println!("HALT detected");
            io::stdout().flush().expect("failed to flush");
            process::exit(1);
        }
        _ => {
            process::exit(1);
        }
    }
}

pub fn sign_extend(mut x: u16, bit_count: u8) -> u16 {
    // transform original bits to 16bit

    // test value
    if (x >> (bit_count - 1)) & 1 != 0 {
        x |= 0xFFFF << bit_count;
    }
    // return as is given positive
    x
}