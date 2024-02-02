mod conjuntion;
mod flipflop;

use crate::module;
use crate::pulse;

use super::StateNum;
use conjuntion::ConjunctionState;
use flipflop::FlipFlopState;

#[derive(Debug)]
pub struct State(StateInner);

impl From<module::Type> for State {
    fn from(value: module::Type) -> Self {
        Self(value.into())
    }
}

impl State {
    pub fn add(&mut self, input: module::Idx) {
        self.0.add(input);
    }

    pub fn recieve(&mut self, from: module::Idx, pulse: pulse::Type) -> Option<pulse::Type> {
        self.0.recieve(from, pulse)
    }

    pub(super) fn num(&self, mut num: StateNum) -> StateNum {
        match &self.0 {
            StateInner::Broadcaster | StateInner::Output => (),
            StateInner::FlipFlop(f) => {
                num <<= 1;
                if *f == FlipFlopState::On {
                    num |= 1;
                }
            }
            StateInner::Conjunction(c) => {
                for (_, p) in &c.data {
                    num <<= 1;
                    if *p == pulse::Type::High {
                        num |= 1;
                    }
                }
            }
        }
        num
    }
}

#[derive(Debug)]
enum StateInner {
    Broadcaster,
    FlipFlop(FlipFlopState),
    Conjunction(ConjunctionState),
    Output,
}

impl From<module::Type> for StateInner {
    fn from(value: module::Type) -> Self {
        match value {
            module::Type::Broadcaster => Self::Broadcaster,
            module::Type::FlipFlop => Self::FlipFlop(FlipFlopState::Off),
            module::Type::Conjunction => Self::Conjunction(ConjunctionState::new()),
            module::Type::Output => Self::Output,
        }
    }
}

impl StateInner {
    fn add(&mut self, idx: module::Idx) {
        if let Self::Conjunction(ref mut s) = self {
            s.add(idx);
        }
    }

    fn recieve(&mut self, from: module::Idx, pulse: pulse::Type) -> Option<pulse::Type> {
        match self {
            Self::Broadcaster => Some(pulse::Type::Low),
            Self::FlipFlop(ref mut s) => s.recieve(pulse),
            Self::Conjunction(ref mut s) => Some(s.recieve(from, pulse)),
            Self::Output => None,
        }
    }
}
