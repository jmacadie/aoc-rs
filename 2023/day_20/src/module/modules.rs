use super::{Idx, Module, Type};

use std::ops::Index;
use std::slice::Iter;

#[derive(Debug)]
pub struct Modules<const N: usize> {
    data: [Module; N],
}

impl<const N: usize> From<&'static str> for Modules<N> {
    fn from(value: &'static str) -> Self {
        let mut lines = value.lines().chain(std::iter::once("output"));
        let mut data: [Module; N] =
            std::array::from_fn(|_| lines.next().map_or_else(|| unreachable!(), Into::into));
        value
            .lines()
            .map(|l| {
                if let Some((_, outputs)) = l.split_once(" -> ") {
                    outputs
                } else {
                    unreachable!()
                }
            })
            .enumerate()
            .for_each(|(idx, outputs)| {
                outputs.split(',').map(str::trim).for_each(|o| {
                    if let Some(output_idx) = data.iter().position(|m| m.name() == o) {
                        data[idx].add(output_idx);
                    } else {
                        // This must be an output node
                        data[idx].add(N - 1);
                    }
                });
            });
        Self { data }
    }
}

impl<const N: usize> Index<usize> for Modules<N> {
    type Output = Module;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> Modules<N> {
    pub fn iter(&self) -> Iter<Module> {
        self.data.iter()
    }

    fn find(&self, module: Type) -> Option<Idx> {
        self.data.iter().position(|m| m.flavour == module)
    }

    pub fn start(&self) -> Idx {
        self.find(Type::Broadcaster)
            .unwrap_or_else(|| unreachable!())
    }

    pub fn end(&self) -> Idx {
        self.find(Type::Output).unwrap_or_else(|| unreachable!())
    }
}
