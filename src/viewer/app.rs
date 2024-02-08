use crate::vm::{
    VirtualMachineSubscription, VirtualMachineSubscriptionTick, VirtualMachineSubscriptionUpdate,
};

/// Application.
#[derive(Debug)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,

    pub virtual_machine_subscription: VirtualMachineSubscription,

    pub last_update: VirtualMachineSubscriptionUpdate,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(virtual_machine_subscription: VirtualMachineSubscription) -> Self {
        Self {
            should_quit: false,
            virtual_machine_subscription: virtual_machine_subscription,
            last_update: VirtualMachineSubscriptionUpdate::default(),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let _ =
            self.virtual_machine_subscription
                .tick_sender
                .send(VirtualMachineSubscriptionTick {
                    additional_stdin: String::new(),
                });
    }

    pub fn update(&mut self) {
        if let Ok(update) = self.virtual_machine_subscription.update_receiver.try_recv() {
            self.last_update = update;
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
