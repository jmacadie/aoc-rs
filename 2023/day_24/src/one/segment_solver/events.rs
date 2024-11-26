use std::collections::BinaryHeap;

use super::event::Event;

pub struct Events {
    priority_queue: BinaryHeap<Event>,
    last: Option<Event>,
}

impl Events {
    pub fn pop(&mut self) -> Option<Event> {
        let mut value = self.priority_queue.pop();
        while value.is_some() && value == self.last {
            value = self.priority_queue.pop();
        }
        self.last = value;
        value
    }

    pub fn push(&mut self, item: Event) {
        self.priority_queue.push(item);
    }
}

impl Default for Events {
    fn default() -> Self {
        Self {
            priority_queue: BinaryHeap::with_capacity(100),
            last: None,
        }
    }
}
