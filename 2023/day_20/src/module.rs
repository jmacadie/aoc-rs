pub mod modules;

pub type Idx = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Broadcaster,
    FlipFlop,
    Conjunction,
    Output,
}

#[derive(Debug)]
pub struct Module {
    name: &'static str,
    pub flavour: Type,
    outputs: Vec<Idx>,
}

impl Module {
    fn new(name: &'static str, flavour: Type) -> Self {
        Self {
            name,
            flavour,
            outputs: Vec::with_capacity(5),
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Idx> {
        self.outputs.iter()
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub fn add(&mut self, output: Idx) {
        self.outputs.push(output);
    }
}

impl From<&'static str> for Module {
    fn from(value: &'static str) -> Self {
        match value.chars().next() {
            Some('%') => {
                if let Some((name, _)) = value.split_once(' ') {
                    let name = name.trim_start_matches('%');
                    Self::new(name, Type::FlipFlop)
                } else {
                    unreachable!()
                }
            }
            Some('&') => {
                if let Some((name, _)) = value.split_once(' ') {
                    let name = name.trim_start_matches('&');
                    Self::new(name, Type::Conjunction)
                } else {
                    unreachable!()
                }
            }
            Some('b') => Self::new("broadcaster", Type::Broadcaster),
            Some('o') => Self::new("output", Type::Output),
            _ => unreachable!(),
        }
    }
}
