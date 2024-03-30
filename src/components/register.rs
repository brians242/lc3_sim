const PC_START: u16 = 0x3000;

// LC-3 has 10 registers -- 8 general-purpose registers, 1 program counter, and one condition flag.
// The program counter stores a uint as the memory address of the executed instruction.
pub struct Registers {
    pub r0: u16,        // general-purpose register
    pub r1: u16,        // general-purpose register
    pub r2: u16,        // general-purpose register
    pub r3: u16,        // general-purpose register
    pub r4: u16,        // general-purpose register
    pub r5: u16,        // general-purpose register
    pub r6: u16,        // general-purpose register
    pub r7: u16,        // general-purpose register
    pub pc: u16,        // program counter
    pub cond: u16,      // condition flag
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            r0: 0,        // general-purpose register
            r1: 0,        // general-purpose register
            r2: 0,        // general-purpose register
            r3: 0,        // general-purpose register
            r4: 0,        // general-purpose register
            r5: 0,        // general-purpose register
            r6: 0,        // general-purpose register
            r7: 0,        // general-purpose register
            pc: PC_START, // program counter
            cond: 0,      // condition flag
        }
    }

    pub fn update(&mut self, index: u16, value: u16) {
        match index {
            0 => self.r0 = value,
            1 => self.r1 = value,
            2 => self.r2 = value,
            3 => self.r3 = value,
            4 => self.r4 = value,
            5 => self.r5 = value,
            6 => self.r6 = value,
            7 => self.r7 = value,
            8 => self.pc = value,
            9 => self.cond = value,
            _ => panic!("Index out of bound"),
        }
    }

    pub fn get(&self, index: u16) -> u16 {
        match index {
            0 => self.r0,
            1 => self.r1,
            2 => self.r2,
            3 => self.r3,
            4 => self.r4,
            5 => self.r5,
            6 => self.r6,
            7 => self.r7,
            8 => self.pc,
            9 => self.cond,
            _ => panic!("Index out of bound. "),
        }
    }

    // Update the condition register based on the value inside the register `r`.
    pub fn update_r_cond_register(&mut self, r: u16) {
        if self.get(r) == 0 {
            self.update(9, ConditionFlag::ZRO as u16);
        } else if (self.get(r) >> 15) != 0 {
            // a 1 in the left-most bit indicates negative
            self.update(9, ConditionFlag::NEG as u16);
        } else {
            self.update(9, ConditionFlag::POS as u16);
        }
    }
}

// The RCOND register stores condition flags representing information about the most recent computation.
enum ConditionFlag {
    POS = 1 << 0,
    ZRO = 1 << 1,
    NEG = 1 << 2,
}