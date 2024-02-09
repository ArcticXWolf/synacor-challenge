use std::fmt::Display;

use crate::vm::{
    memory::{HEAP_SIZE, MAX_ADDRESS},
    VirtualMachine,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Halt,
    Set(u16, u16),
    Push(u16),
    Pop(u16),
    Equality(u16, u16, u16),
    GreaterThan(u16, u16, u16),
    Jump(u16),
    JumpIfNonZero(u16, u16),
    JumpIfZero(u16, u16),
    Add(u16, u16, u16),
    Mult(u16, u16, u16),
    Mod(u16, u16, u16),
    And(u16, u16, u16),
    Or(u16, u16, u16),
    Not(u16, u16),
    Load(u16, u16),
    Store(u16, u16),
    Call(u16),
    Return,
    Out(u16),
    In(u16),
    Noop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecoderError {
    Invalid(u16),
    NotImplemented(u16),
    ParameterMissing,
    Empty,
}

impl TryFrom<&[u16]> for Instruction {
    type Error = DecoderError;

    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        let a = value.iter().nth(1).ok_or(Self::Error::ParameterMissing);
        let b = value.iter().nth(2).ok_or(Self::Error::ParameterMissing);
        let c = value.iter().nth(3).ok_or(Self::Error::ParameterMissing);

        match value.first() {
            Some(0) => Ok(Self::Halt),
            Some(1) => Ok(Self::Set(*a?, *b?)),
            Some(2) => Ok(Self::Push(*a?)),
            Some(3) => Ok(Self::Pop(*a?)),
            Some(4) => Ok(Self::Equality(*a?, *b?, *c?)),
            Some(5) => Ok(Self::GreaterThan(*a?, *b?, *c?)),
            Some(6) => Ok(Self::Jump(*a?)),
            Some(7) => Ok(Self::JumpIfNonZero(*a?, *b?)),
            Some(8) => Ok(Self::JumpIfZero(*a?, *b?)),
            Some(9) => Ok(Self::Add(*a?, *b?, *c?)),
            Some(10) => Ok(Self::Mult(*a?, *b?, *c?)),
            Some(11) => Ok(Self::Mod(*a?, *b?, *c?)),
            Some(12) => Ok(Self::And(*a?, *b?, *c?)),
            Some(13) => Ok(Self::Or(*a?, *b?, *c?)),
            Some(14) => Ok(Self::Not(*a?, *b?)),
            Some(15) => Ok(Self::Load(*a?, *b?)),
            Some(16) => Ok(Self::Store(*a?, *b?)),
            Some(17) => Ok(Self::Call(*a?)),
            Some(18) => Ok(Self::Return),
            Some(19) => Ok(Self::Out(*a?)),
            Some(20) => Ok(Self::In(*a?)),
            Some(21) => Ok(Self::Noop),
            Some(&x) if x <= 21 => Err(Self::Error::NotImplemented(x)),
            Some(&x) => Err(Self::Error::Invalid(x)),
            None => Err(Self::Error::Empty),
        }
    }
}

impl Instruction {
    pub fn memnonic(&self) -> &'static str {
        match self {
            Self::Halt => "HALT",
            Self::Set(_, _) => "SET",
            Self::Push(_) => "PUSH",
            Self::Pop(_) => "POP",
            Self::Equality(_, _, _) => "EQ",
            Self::GreaterThan(_, _, _) => "GT",
            Self::Jump(_) => "JMP",
            Self::JumpIfNonZero(_, _) => "JT",
            Self::JumpIfZero(_, _) => "JF",
            Self::Add(_, _, _) => "ADD",
            Self::Mult(_, _, _) => "MULT",
            Self::Mod(_, _, _) => "MOD",
            Self::And(_, _, _) => "AND",
            Self::Or(_, _, _) => "OR",
            Self::Not(_, _) => "NOT",
            Self::Load(_, _) => "RMEM",
            Self::Store(_, _) => "WMEM",
            Self::Call(_) => "CALL",
            Self::Return => "RET",
            Self::Out(_) => "OUT",
            Self::In(_) => "IN",
            Self::Noop => "NOOP",
        }
    }
    pub fn byte_length(&self) -> usize {
        match self {
            Self::Halt => 1,
            Self::Set(_, _) => 3,
            Self::Push(_) => 2,
            Self::Pop(_) => 2,
            Self::Equality(_, _, _) => 4,
            Self::GreaterThan(_, _, _) => 4,
            Self::Jump(_) => 2,
            Self::JumpIfNonZero(_, _) => 3,
            Self::JumpIfZero(_, _) => 3,
            Self::Add(_, _, _) => 4,
            Self::Mult(_, _, _) => 4,
            Self::Mod(_, _, _) => 4,
            Self::And(_, _, _) => 4,
            Self::Or(_, _, _) => 4,
            Self::Not(_, _) => 3,
            Self::Load(_, _) => 3,
            Self::Store(_, _) => 3,
            Self::Call(_) => 2,
            Self::Return => 1,
            Self::Out(_) => 2,
            Self::In(_) => 2,
            Self::Noop => 1,
        }
    }

    pub fn execute(&self, vm: &mut VirtualMachine) {
        match self {
            // 0
            Self::Halt => {
                vm.halted = true;
            }
            // 1
            Self::Set(register, value) => {
                vm.memory.write(register, *value);
                vm.program_counter += self.byte_length() as u16;
            }
            // 2
            Self::Push(value) => {
                vm.memory.stack.push((vm.memory.read(value), None));
                vm.program_counter += self.byte_length() as u16;
            }
            // 3
            Self::Pop(register) => {
                let (value, _) = vm.memory.stack.pop().unwrap();
                vm.memory.write(register, value);
                vm.program_counter += self.byte_length() as u16;
            }
            // 4
            Self::Equality(address, operand1, operand2) => {
                let result = if vm.memory.read(operand1) == vm.memory.read(operand2) {
                    1
                } else {
                    0
                };
                vm.memory.write(address, result);
                vm.program_counter += self.byte_length() as u16;
            }
            // 5
            Self::GreaterThan(address, operand1, operand2) => {
                let result = if vm.memory.read(operand1) > vm.memory.read(operand2) {
                    1
                } else {
                    0
                };
                vm.memory.write(address, result);
                vm.program_counter += self.byte_length() as u16;
            }
            // 6
            Self::Jump(address) => {
                vm.program_counter = vm.memory.read(address);
            }
            // 7
            Self::JumpIfNonZero(compare, jump_address) => {
                if vm.memory.read(compare) != 0 {
                    vm.program_counter = vm.memory.read(jump_address);
                } else {
                    vm.program_counter += self.byte_length() as u16;
                }
            }
            // 8
            Self::JumpIfZero(compare, jump_address) => {
                if vm.memory.read(compare) == 0 {
                    vm.program_counter = vm.memory.read(jump_address);
                } else {
                    vm.program_counter += self.byte_length() as u16;
                }
            }
            // 9
            Self::Add(address, operand1, operand2) => {
                let mut result =
                    vm.memory.read(operand1) as usize + vm.memory.read(operand2) as usize;
                result %= HEAP_SIZE;
                vm.memory.write(address, result as u16);
                vm.program_counter += self.byte_length() as u16;
            }
            // 10
            Self::Mult(address, operand1, operand2) => {
                let mut result =
                    vm.memory.read(operand1) as usize * vm.memory.read(operand2) as usize;
                result %= HEAP_SIZE;
                vm.memory.write(address, result as u16);
                vm.program_counter += self.byte_length() as u16;
            }
            // 11
            Self::Mod(address, operand1, operand2) => {
                let result = vm.memory.read(operand1) % vm.memory.read(operand2);
                vm.memory.write(address, result);
                vm.program_counter += self.byte_length() as u16;
            }
            // 12
            Self::And(address, operand1, operand2) => {
                let result = vm.memory.read(operand1) & vm.memory.read(operand2);
                vm.memory.write(address, result);
                vm.program_counter += self.byte_length() as u16;
            }
            // 13
            Self::Or(address, operand1, operand2) => {
                let result = vm.memory.read(operand1) | vm.memory.read(operand2);
                vm.memory.write(address, result);
                vm.program_counter += self.byte_length() as u16;
            }
            // 14
            Self::Not(address, operand) => {
                let result = !vm.memory.read(operand) & MAX_ADDRESS;
                vm.memory.write(address, result);
                vm.program_counter += self.byte_length() as u16;
            }
            // 15
            Self::Load(register, address) => {
                let mem_value = vm.memory.mem_read(&vm.memory.read(address));
                vm.memory.write(register, mem_value);
                vm.program_counter += self.byte_length() as u16;
            }
            // 16
            Self::Store(address, register_or_value) => {
                let value = vm.memory.read(register_or_value);
                vm.memory.mem_write(&vm.memory.read(address), value);
                vm.program_counter += self.byte_length() as u16;
            }
            // 17
            Self::Call(address) => {
                vm.memory.stack.push((
                    vm.program_counter + self.byte_length() as u16,
                    Some(vm.memory.read(address)),
                ));
                vm.program_counter = vm.memory.read(address);
            }
            // 18
            Self::Return => {
                let (value, _) = vm.memory.stack.pop().unwrap();
                vm.program_counter = value;
            }
            // 19
            Self::Out(character_raw) => {
                let character = vm.memory.read(character_raw) as u8 as char;
                vm.output_buffer.push(character);
                vm.program_counter += self.byte_length() as u16;
            }
            // 20
            Self::In(address) => {
                let character = vm.get_stdin() as u16;
                vm.memory.write(address, character);
                vm.program_counter += self.byte_length() as u16;
            }
            // 21
            Self::Noop => {
                vm.program_counter += self.byte_length() as u16;
            }
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/*
#[cfg(test)]
mod tests {
    use crate::vm::VirtualMachine;

    use super::{DecoderError, Instruction};

    #[test]
    fn test_construct_instruction() {
        let program: [u16; 5] = [21, 19, 65, 0, 200];
        assert_eq!(Instruction::try_from(&program[0..4]), Ok(Instruction::Noop));
        assert_eq!(
            Instruction::try_from(&program[1..5]),
            Ok(Instruction::Out(65))
        );
        assert_eq!(
            Instruction::try_from(&program[2..5]),
            Err(DecoderError::Invalid(65))
        );
        assert_eq!(Instruction::try_from(&program[3..5]), Ok(Instruction::Halt));
        assert_eq!(
            Instruction::try_from(&program[4..5]),
            Err(DecoderError::Invalid(200))
        );
    }

    #[test]
    fn test_instruction_halt() {
        let mut vm = VirtualMachine::default();
        let program: [u16; 1] = [0];
        vm.load_data(&program);
        let instruction = Instruction::Halt;
        instruction.execute(&mut vm);
        assert!(vm.halted);
    }

    #[test]
    fn test_instructions_jump() {
        let mut vm = VirtualMachine::default();
        let program: [u16; 2] = [0, 11];
        vm.load_data(&program);

        vm.program_counter = 100;
        let instruction = Instruction::Jump(300);
        instruction.execute(&mut vm);
        assert_eq!(vm.program_counter, 300);

        vm.program_counter = 100;
        let instruction = Instruction::JumpIfNonZero(0, 300);
        instruction.execute(&mut vm);
        assert_eq!(vm.program_counter, 103);

        vm.program_counter = 100;
        let instruction = Instruction::JumpIfNonZero(1, 300);
        instruction.execute(&mut vm);
        assert_eq!(vm.program_counter, 300);

        vm.program_counter = 100;
        let instruction = Instruction::JumpIfZero(0, 300);
        instruction.execute(&mut vm);
        assert_eq!(vm.program_counter, 300);

        vm.program_counter = 100;
        let instruction = Instruction::JumpIfZero(1, 300);
        instruction.execute(&mut vm);
        assert_eq!(vm.program_counter, 103);
    }
}
*/
