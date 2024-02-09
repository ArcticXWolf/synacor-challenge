pub const HEAP_SIZE: usize = 1 << 15; // 15-bit space
pub const MAX_ADDRESS: u16 = HEAP_SIZE as u16 - 1;
pub const AMOUNT_REGISTERS: usize = 8;
pub const REGISTER_ADDRESS_START: u16 = MAX_ADDRESS + 1;
pub const REGISTER_ADDRESS_END: u16 = REGISTER_ADDRESS_START + AMOUNT_REGISTERS as u16 - 1;

#[derive(Debug, Clone)]
pub struct Memory {
    pub heap: [u16; HEAP_SIZE],
    pub registers: [u16; AMOUNT_REGISTERS],
    pub stack: Vec<(u16, Option<u16>)>, // value and Call
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
