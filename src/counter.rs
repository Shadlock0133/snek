pub struct Counter {
    counter: u32,
    trigger: u32,
}

impl Counter {
    pub fn new(trigger: u32) -> Self {
        Self {
            counter: 0,
            trigger,
        }
    }

    pub fn set_trigger(&mut self, trigger: u32) {
        self.trigger = trigger;
    }

    pub fn checked_inc(&mut self) -> bool {
        self.counter += 1;
        if self.counter >= self.trigger {
            self.counter = 0;
            true
        } else {
            false
        }
    }
}