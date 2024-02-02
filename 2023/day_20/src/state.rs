mod module;
pub mod records;

use crate::module::Type;
use crate::Modules;

use std::ops::{Index, IndexMut};

type StateNum = u64;

#[derive(Debug, Default)]
pub struct Record {
    state: StateNum,
    pub low: usize,
    pub high: usize,
}

#[derive(Debug)]
pub struct State<const N: usize> {
    data: [module::State; N],
}

impl<const N: usize> From<&Modules<N>> for State<N> {
    fn from(value: &Modules<N>) -> Self {
        let mut mod_iter = value.iter();
        let mut data: [module::State; N] = std::array::from_fn(|_| {
            mod_iter
                .next()
                .map_or_else(|| unreachable!(), |m| m.flavour.into())
        });
        value.iter().enumerate().for_each(|(idx, m)| {
            m.iter().for_each(|&output_idx| {
                if value[output_idx].flavour == Type::Conjunction {
                    data[output_idx].add(idx);
                }
            });
        });
        Self { data }
    }
}

impl<const N: usize> Index<usize> for State<N> {
    type Output = module::State;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize> IndexMut<usize> for State<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize> State<N> {
    pub fn num(&self) -> StateNum {
        self.data.iter().fold(0, |num, s| s.num(num))
    }
}
