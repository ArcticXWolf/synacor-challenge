use std::io::Write;

use crate::vm::VirtualMachine;

#[derive(Default)]
pub struct Debugger {
    pub collected_output: String,
    pub breakpoints: Vec<u16>,
    pub break_next: bool,
}

impl Debugger {
    pub fn push_output(&mut self, character: char) {
        self.collected_output.push(character);
    }

    pub fn interrupt(&mut self, vm: &VirtualMachine) {
        loop {
            let fetched_memory = vm.fetch();
            let instruction = vm.decode(fetched_memory);
            println!();
            println!(
                "Breakpoint at {}: {} | {:?}",
                vm.program_counter, instruction, vm.memory.registers
            );
            print!("> ");
            std::io::stdout().flush().unwrap();

            let command_line: Vec<String> = Self::read_command()
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();

            if let Some(command) = command_line.first() {
                match command.trim() {
                    "output" => {
                        println!("{}", self.collected_output);
                    }
                    "break" => {
                        let address = command_line.iter().nth(1).unwrap().parse::<u16>().unwrap();
                        if self.breakpoints.contains(&address) {
                            self.breakpoints.retain(|&a| a == address);
                        } else {
                            self.breakpoints.push(address);
                        }
                    }
                    "mem" => {
                        let address_start =
                            command_line.iter().nth(1).unwrap().parse::<u16>().unwrap();
                        let address_end =
                            command_line.iter().nth(2).unwrap().parse::<u16>().unwrap();
                        for i in address_start..address_end {
                            println!("{}: {}", i, vm.memory.read(&i));
                        }
                    }
                    "stack" => {
                        for (i, value) in vm.memory.stack.iter().enumerate() {
                            println!("{}: {}", i, value);
                        }
                    }
                    "run" => break,
                    "step" => {
                        self.break_next = true;
                        break;
                    }
                    "quit" | "exit" => panic!("Exited early"),
                    _ => println!("Unknown command"),
                }
            } else {
                self.break_next = true;
                break;
            }
        }
    }

    fn read_command() -> String {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        input
    }
}
