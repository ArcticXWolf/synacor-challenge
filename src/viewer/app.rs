use crate::vm::subscription::{
    VirtualMachineSubscription, VirtualMachineSubscriptionTick, VirtualMachineSubscriptionUpdate,
};

#[derive(Debug)]
pub enum Page {
    Output,
    MemoryView,
}

/// Application.
#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    pub current_input: String,
    pub active_page: Page,
    pub memory_page_scroll: usize,
    pub virtual_machine_subscription: VirtualMachineSubscription,
    pub next_tick_to_send: VirtualMachineSubscriptionTick,
    pub last_update: Box<VirtualMachineSubscriptionUpdate>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(virtual_machine_subscription: VirtualMachineSubscription) -> Self {
        Self {
            should_quit: false,
            current_input: String::default(),
            active_page: Page::Output,
            memory_page_scroll: 0,
            virtual_machine_subscription: virtual_machine_subscription,
            next_tick_to_send: VirtualMachineSubscriptionTick::default(),
            last_update: Box::new(VirtualMachineSubscriptionUpdate::default()),
        }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&mut self) {
        let _ = self
            .virtual_machine_subscription
            .tick_sender
            .send(self.next_tick_to_send.clone());

        self.next_tick_to_send = VirtualMachineSubscriptionTick::default();
    }

    pub fn update(&mut self) {
        if let Ok(update) = self.virtual_machine_subscription.update_receiver.try_recv() {
            self.last_update = update;
        }
    }

    pub fn toggle_page(&mut self) {
        self.active_page = match self.active_page {
            Page::Output => Page::MemoryView,
            Page::MemoryView => Page::Output,
        }
    }

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
