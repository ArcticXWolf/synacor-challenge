use std::{env, fs, thread};

use vm::{subscription::VirtualMachineSubscription, VirtualMachine};

pub mod viewer;
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

    let (subscriber, subscription) = VirtualMachineSubscription::setup();

    let _handle = thread::spawn(move || {
        let mut vm = VirtualMachine::new(subscriber);
        vm.load_data(&program);

        if let Ok(content) = fs::read_to_string(vm::HISTORY_FILE_PATH) {
            for c in content.chars() {
                vm.stdin_buffer.push_back(c as u8);
            }
            vm.stdin_history = content;
        }

        vm.run();
    });

    let _ = viewer::main(subscription);
}
