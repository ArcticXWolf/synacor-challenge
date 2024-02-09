use std::sync::mpsc;

use super::{opcodes::Instruction, VirtualMachineSavestate};

#[derive(Debug)]
pub struct VirtualMachineSubscriber {
    pub tick_receiver: mpsc::Receiver<VirtualMachineSubscriptionTick>,
    pub update_sender: mpsc::Sender<Box<VirtualMachineSubscriptionUpdate>>,
}

impl VirtualMachineSubscriber {
    pub fn setup() -> (VirtualMachineSubscriber, VirtualMachineSubscription) {
        VirtualMachineSubscription::setup()
    }
}

#[derive(Debug)]
pub struct VirtualMachineSubscription {
    pub update_receiver: mpsc::Receiver<Box<VirtualMachineSubscriptionUpdate>>,
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

#[derive(Debug, Default, Clone)]
pub struct VirtualMachineSubscriptionTick {
    pub additional_stdin: String,
    pub save_state: bool,
    pub load_state: bool,
    pub write_history: bool,
    pub toggle_pause: bool,
    pub step_once: bool,
    pub set_register_id: Option<usize>,
    pub set_register_value: u16,
}

#[derive(Debug)]
pub struct VirtualMachineSubscriptionUpdate {
    pub current_instruction: Instruction,
    pub savestate: VirtualMachineSavestate,
}

impl Default for VirtualMachineSubscriptionUpdate {
    fn default() -> Self {
        Self {
            current_instruction: Instruction::Noop,
            savestate: VirtualMachineSavestate::default(),
        }
    }
}
