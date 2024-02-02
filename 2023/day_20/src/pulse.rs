use crate::module;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Pulses {
    queue: VecDeque<Pulse>,
}

impl Pulses {
    pub const fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn add(&mut self, from: module::Idx, to: module::Idx, flavour: Type) {
        self.queue.push_back(Pulse { from, to, flavour });
    }

    pub fn pop(&mut self) -> Option<Pulse> {
        self.queue.pop_front()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pulse {
    pub from: module::Idx,
    pub to: module::Idx,
    pub flavour: Type,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Low,
    High,
}
