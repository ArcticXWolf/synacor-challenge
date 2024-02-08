pub mod opcodes;
use opcodes::Instruction;
use std::{collections::VecDeque, sync::mpsc, thread, time};

pub const HEAP_SIZE: usize = 1 << 15; // 15-bit space
pub const MAX_ADDRESS: u16 = HEAP_SIZE as u16 - 1;
pub const AMOUNT_REGISTERS: usize = 8;
pub const REGISTER_ADDRESS_START: u16 = MAX_ADDRESS + 1;
pub const REGISTER_ADDRESS_END: u16 = REGISTER_ADDRESS_START + AMOUNT_REGISTERS as u16 - 1;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct VirtualMachineSubscriber {
    pub tick_receiver: mpsc::Receiver<VirtualMachineSubscriptionTick>,
    pub update_sender: mpsc::Sender<VirtualMachineSubscriptionUpdate>,
}

impl VirtualMachineSubscriber {
    pub fn setup() -> (VirtualMachineSubscriber, VirtualMachineSubscription) {
        VirtualMachineSubscription::setup()
    }
}

#[derive(Debug)]
pub struct VirtualMachineSubscription {
    pub update_receiver: mpsc::Receiver<VirtualMachineSubscriptionUpdate>,
    pub tick_sender: mpsc::Sender<VirtualMachineSubscriptionTick>,
}

impl VirtualMachineSubscription {
    pub fn setup() -> (VirtualMachineSubscriber, VirtualMachineSubscription) {
        let (update_sender, update_receiver) = mpsc::channel();
        let (tick_sender, tick_receiver) = mpsc::channel();

        (
            VirtualMachineSubscriber {
                tick_receiver,
                update_sender,
            },
            VirtualMachineSubscription {
                update_receiver,
                tick_sender,
            },
        )
    }
}

pub struct VirtualMachineSubscriptionTick {
    pub additional_stdin: String,
}

#[derive(Debug)]
pub struct VirtualMachineSubscriptionUpdate {
    pub output_buffer: String,
    pub cycle: usize,
    pub registers: [u16; AMOUNT_REGISTERS],
    pub stack: Vec<u16>,
    pub current_instruction: Instruction,
    pub current_program_counter: u16,
}

impl Default for VirtualMachineSubscriptionUpdate {
    fn default() -> Self {
        Self {
            output_buffer: Default::default(),
            cycle: Default::default(),
            registers: [0; AMOUNT_REGISTERS],
            stack: vec![],
            current_instruction: Instruction::Noop,
            current_program_counter: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct VirtualMachine {
    pub halted: bool,
    pub cycle: usize,
    pub program_counter: u16,
    pub memory: Memory,
    pub stdin_buffer: VecDeque<u8>,
    pub output_buffer: String,
    pub subscriber: VirtualMachineSubscriber,
}

impl VirtualMachine {
    pub fn new(subscriber: VirtualMachineSubscriber) -> Self {
        Self {
            halted: Default::default(),
            cycle: Default::default(),
            program_counter: Default::default(),
            memory: Default::default(),
            stdin_buffer: Default::default(),
            output_buffer: Default::default(),
            subscriber: subscriber,
        }
    }

    pub fn reset(&mut self) {
        self.halted = Default::default();
        self.cycle = Default::default();
        self.program_counter = Default::default();
        self.memory = Default::default();
    }

    pub fn load_data(&mut self, program: &[u16]) {
        for (offset, value) in program.iter().enumerate() {
            self.memory.heap[offset] = *value;
        }
    }

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
            self.stdin_buffer.push_back(c as u8);
        }
    }

    pub fn get_subscription_update(&mut self) -> VirtualMachineSubscriptionUpdate {
        let fetched_memory = self.fetch();
        let instruction = self.decode(fetched_memory);
        VirtualMachineSubscriptionUpdate {
            output_buffer: self.output_buffer.clone(),
            cycle: self.cycle,
            registers: self.memory.registers.clone(),
            stack: self.memory.stack.clone(),
            current_instruction: instruction,
            current_program_counter: self.program_counter,
        }
    }

    pub fn cycle(&mut self) {
        self.handle_subscriber();

        let fetched_memory = self.fetch();
        let instruction = self.decode(fetched_memory);
        self.execute(instruction);

        self.cycle += 1;
    }

    pub fn run(&mut self) {
        while !self.halted {
            self.cycle();
            thread::yield_now();
        }
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
