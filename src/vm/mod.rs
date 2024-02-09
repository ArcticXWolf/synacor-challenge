pub mod memory;
pub mod opcodes;
pub mod subscription;
use memory::Memory;
use opcodes::Instruction;
use std::{collections::VecDeque, fs, thread};

use self::subscription::{
    VirtualMachineSubscriber, VirtualMachineSubscriptionTick, VirtualMachineSubscriptionUpdate,
};

pub const HISTORY_FILE_PATH: &'static str = "./history.txt";

#[derive(Debug, Default)]
pub struct VirtualMachineSavestate {
    pub paused: bool,
    pub halted: bool,
    pub cycle: usize,
    pub program_counter: u16,
    pub memory: Memory,
    pub stdin_history: String,
    pub stdin_buffer: VecDeque<u8>,
    pub output_buffer: String,
}

#[derive(Debug)]
pub struct VirtualMachine {
    pub step_once: bool,
    pub paused: bool,
    pub halted: bool,
    pub cycle: usize,
    pub program_counter: u16,
    pub memory: Memory,
    pub stdin_history: String,
    pub stdin_buffer: VecDeque<u8>,
    pub output_buffer: String,
    pub subscriber: VirtualMachineSubscriber,
    pub save_state: VirtualMachineSavestate,
}

// Creation & setup
impl VirtualMachine {
    pub fn new(subscriber: VirtualMachineSubscriber) -> Self {
        Self {
            step_once: Default::default(),
            paused: Default::default(),
            halted: Default::default(),
            cycle: Default::default(),
            program_counter: Default::default(),
            memory: Default::default(),
            stdin_history: Default::default(),
            stdin_buffer: Default::default(),
            output_buffer: Default::default(),
            subscriber: subscriber,
            save_state: VirtualMachineSavestate::default(),
        }
    }

    pub fn load_data(&mut self, program: &[u16]) {
        for (offset, value) in program.iter().enumerate() {
            self.memory.heap[offset] = *value;
        }
    }
}

// Run
impl VirtualMachine {
    pub fn get_stdin(&mut self) -> u8 {
        while self.stdin_buffer.is_empty() {
            self.handle_subscriber_blocking();
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

    pub fn execute(&mut self, instruction: Instruction) {
        instruction.execute(self)
    }

    pub fn cycle(&mut self) {
        self.handle_subscriber();

        let fetched_memory = self.fetch();
        let instruction = self.decode(fetched_memory);
        self.execute(instruction);

        if self.program_counter == 5511 {
            self.paused = true;
        }

        self.cycle += 1;
    }

    pub fn run(&mut self) {
        loop {
            while (!self.halted && !self.paused) || self.step_once {
                self.cycle();
                thread::yield_now();
                self.step_once = false;
            }

            self.handle_subscriber_blocking();
        }
    }
}

// Save&Load
impl VirtualMachine {
    pub fn get_state(&mut self) -> VirtualMachineSavestate {
        VirtualMachineSavestate {
            paused: self.paused.clone(),
            halted: self.halted.clone(),
            cycle: self.cycle.clone(),
            program_counter: self.program_counter.clone(),
            stdin_history: self.stdin_history.clone(),
            stdin_buffer: self.stdin_buffer.clone(),
            memory: self.memory.clone(),
            output_buffer: self.output_buffer.clone(),
        }
    }

    pub fn load_state(&mut self) {
        self.paused = self.save_state.paused.clone();
        self.halted = self.save_state.halted.clone();
        self.cycle = self.save_state.cycle.clone();
        self.program_counter = self.save_state.program_counter.clone();
        self.stdin_history = self.save_state.stdin_history.clone();
        self.stdin_buffer = self.save_state.stdin_buffer.clone();
        self.memory = self.save_state.memory.clone();
        self.output_buffer = self.save_state.output_buffer.clone();
    }

    pub fn write_out_history(&self) {
        fs::write(HISTORY_FILE_PATH, self.stdin_history.clone()).expect("Could not write file");
    }
}

// Subscriber
impl VirtualMachine {
    pub fn handle_subscriber(&mut self) {
        if let Ok(tick) = self.subscriber.tick_receiver.try_recv() {
            self.handle_subscriber_tick(tick);
            let update = self.get_subscription_update();
            let _ = self.subscriber.update_sender.send(update);
        }
    }

    pub fn handle_subscriber_blocking(&mut self) {
        if let Ok(tick) = self.subscriber.tick_receiver.recv() {
            self.handle_subscriber_tick(tick);
            let update = self.get_subscription_update();
            let _ = self.subscriber.update_sender.send(update);
        }
    }

    pub fn handle_subscriber_tick(&mut self, tick: VirtualMachineSubscriptionTick) {
        for c in tick.additional_stdin.chars() {
            self.output_buffer.push(c);
            self.stdin_buffer.push_back(c as u8);
            self.stdin_history.push(c);
        }

        if tick.save_state {
            self.save_state = self.get_state();
        }

        if tick.load_state {
            self.load_state();
        }

        if tick.write_history {
            self.write_out_history();
        }

        if tick.toggle_pause {
            self.paused = !self.paused;
        }

        if let Some(register_idx) = tick.set_register_id {
            if register_idx <= self.memory.registers.len() {
                self.memory.registers[register_idx] = tick.set_register_value;
            }
        }

        if tick.step_once {
            self.step_once = true;
        }
    }

    pub fn get_subscription_update(&mut self) -> Box<VirtualMachineSubscriptionUpdate> {
        let fetched_memory = self.fetch();
        let instruction = self.decode(fetched_memory);
        Box::new(VirtualMachineSubscriptionUpdate {
            current_instruction: instruction,
            savestate: self.get_state(),
        })
    }
}

/*
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
*/
