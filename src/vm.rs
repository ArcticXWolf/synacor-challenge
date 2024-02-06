use std::collections::VecDeque;

use crate::{debugger::Debugger, opcodes::Instruction};

pub const HEAP_SIZE: usize = 1 << 15; // 15-bit space
pub const MAX_ADDRESS: u16 = HEAP_SIZE as u16 - 1;
pub const AMOUNT_REGISTERS: usize = 8;
pub const REGISTER_ADDRESS_START: u16 = MAX_ADDRESS + 1;
pub const REGISTER_ADDRESS_END: u16 = REGISTER_ADDRESS_START + AMOUNT_REGISTERS as u16 - 1;

pub struct Memory {
    pub heap: [u16; HEAP_SIZE],
    pub registers: [u16; AMOUNT_REGISTERS],
    pub stack: Vec<u16>,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            heap: [0; HEAP_SIZE],
            registers: [0; AMOUNT_REGISTERS],
            stack: vec![],
        }
    }
}

impl Memory {
    pub fn read(&self, value: &u16) -> u16 {
        match value {
            0..=MAX_ADDRESS => *value,
            REGISTER_ADDRESS_START..=REGISTER_ADDRESS_END => {
                let v = self.registers[*value as usize - HEAP_SIZE];
                return if (REGISTER_ADDRESS_START..=REGISTER_ADDRESS_END).contains(&v) {
                    self.read(&v)
                } else {
                    v
                };
            }
            _ => panic!("Read violation - read at {}", value),
        }
    }

    pub fn write(&mut self, address: &u16, value: u16) {
        match address {
            REGISTER_ADDRESS_START..=REGISTER_ADDRESS_END => {
                self.registers[*address as usize - HEAP_SIZE] =
                    if (REGISTER_ADDRESS_START..=REGISTER_ADDRESS_END).contains(&value) {
                        self.read(&value)
                    } else {
                        value
                    };
            }
            _ => panic!("Write violation - write {} at {}", value, address),
        }
    }

    pub fn mem_read(&self, address: &u16) -> u16 {
        match address {
            0..=MAX_ADDRESS => self.heap[*address as usize],
            REGISTER_ADDRESS_START..=REGISTER_ADDRESS_END => {
                self.registers[*address as usize - HEAP_SIZE]
            }
            _ => panic!("Memory read violation - read at {}", address),
        }
    }

    pub fn mem_write(&mut self, address: &u16, value: u16) {
        match address {
            0..=MAX_ADDRESS => self.heap[*address as usize] = value,
            REGISTER_ADDRESS_START..=REGISTER_ADDRESS_END => {
                self.registers[*address as usize - HEAP_SIZE] = value
            }
            _ => panic!("Memory write violation - write {} at {}", value, address),
        }
    }
}

#[derive(Default)]
pub struct VirtualMachine {
    pub halted: bool,
    pub program_counter: u16,
    pub memory: Memory,
    pub stdin_buffer: VecDeque<u8>,
}

impl VirtualMachine {
    pub fn reset(&mut self) {
        self.halted = Default::default();
        self.program_counter = Default::default();
        self.memory = Default::default();
    }

    pub fn load_data(&mut self, program: &[u16]) {
        for (offset, value) in program.iter().enumerate() {
            self.memory.heap[offset] = *value;
        }
    }

    pub fn get_stdin(&mut self) -> u8 {
        if let Some(character) = self.stdin_buffer.pop_front() {
            return character;
        }

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");

        for c in input.chars() {
            self.stdin_buffer.push_back(c as u8);
        }

        if let Some(character) = self.stdin_buffer.pop_front() {
            return character;
        }
        panic!("did not get any characters")
    }

    pub fn fetch(&self) -> &[u16] {
        &self.memory.heap[(self.program_counter as usize)..(self.program_counter as usize + 4)]
    }

    pub fn decode(&self, fetched_memory: &[u16]) -> Instruction {
        match Instruction::try_from(fetched_memory) {
            Ok(instruction) => instruction,
            Err(x) => panic!("{:?}", x),
        }
    }

    pub fn execute(&mut self, instruction: Instruction, debugger: Option<&mut Debugger>) {
        instruction.execute(self, debugger)
    }

    pub fn run(&mut self, mut debugger: Option<Debugger>) {
        while !self.halted {
            if let Some(debug) = debugger.as_mut() {
                if debug.break_next || debug.breakpoints.contains(&self.program_counter) {
                    debug.break_next = false;
                    debug.interrupt(self);
                }
            }

            let fetched_memory = self.fetch();
            let instruction = self.decode(fetched_memory);
            self.execute(instruction, debugger.as_mut());
        }

        if let Some(debug) = debugger.as_mut() {
            debug.interrupt(self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::VirtualMachine;

    #[test]
    fn test_load_program_into_memory() {
        let program: [u16; 6] = [9, 32768, 32769, 4, 19, 32768];
        let mut vm = VirtualMachine::default();
        vm.load_data(&program);

        for (offset, value) in program.iter().enumerate() {
            assert_eq!(vm.memory.heap[offset], *value);
        }
    }
}
