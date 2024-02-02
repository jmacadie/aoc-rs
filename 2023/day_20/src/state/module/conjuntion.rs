use crate::module;
use crate::pulse::Type;

#[derive(Debug)]
pub(super) struct ConjunctionState {
    pub(super) data: Vec<(module::Idx, Type)>,
}

impl ConjunctionState {
    pub(super) fn new() -> Self {
        Self {
            data: Vec::with_capacity(10),
        }
    }

    pub(super) fn add(&mut self, idx: module::Idx) {
        self.data.push((idx, Type::Low));
    }

    fn all_high(&self) -> bool {
        self.data.iter().all(|&(_, p)| p == Type::High)
    }

    pub(super) fn recieve(&mut self, from: module::Idx, pulse: Type) -> Type {
        if let Some((_, p)) = self.data.iter_mut().find(|(i, _)| *i == from) {
            *p = pulse;
        } else {
            unreachable!();
        };
        if self.all_high() {
            Type::Low
        } else {
            Type::High
        }
    }
}
