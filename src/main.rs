use std::{env, fs};

use debugger::Debugger;
use vm::VirtualMachine;

pub mod debugger;
pub mod opcodes;
pub mod vm;

fn transform_bytes_to_program_code(content: &[u8]) -> Vec<u16> {
    let mut program_code = vec![];
    for i in (0..content.len()).step_by(2) {
        let value = u16::from_le_bytes(content[i..(i + 2)].try_into().unwrap());
        program_code.push(value);
    }
    program_code
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Expecting a file path as argument");
    let content = fs::read(file_path).expect("Could not read file");
    let program = transform_bytes_to_program_code(&content);

    let mut debugger = Debugger::default();
    debugger.breakpoints.push(0);
    let mut vm = VirtualMachine::default();
    vm.load_data(&program);
    vm.run(None);
}
