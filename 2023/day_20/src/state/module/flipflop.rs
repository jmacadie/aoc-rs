use crate::pulse::Type;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FlipFlopState {
    On,
    Off,
}

impl FlipFlopState {
    pub(super) fn recieve(&mut self, pulse: Type) -> Option<Type> {
        match (pulse, *self) {
            (Type::Low, Self::On) => {
                *self = Self::Off;
                Some(Type::Low)
            }
            (Type::Low, Self::Off) => {
                *self = Self::On;
                Some(Type::High)
            }
            (Type::High, _) => None,
        }
    }
}
