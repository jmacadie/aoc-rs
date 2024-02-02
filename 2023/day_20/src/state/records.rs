use super::{Record, StateNum};
use std::ops::Index;

#[derive(Debug)]
pub struct Records {
    data: Vec<Record>,
}

impl Records {
    pub fn new() -> Self {
        let data = vec![Record::default()];
        Self { data }
    }

    pub fn contains(&self, state: StateNum) -> Option<usize> {
        self.data.iter().position(|sr| sr.state == state)
    }

    pub fn add(&mut self, state: StateNum, low: usize, high: usize) {
        self.data.push(Record { state, low, high });
    }
}

impl Index<usize> for Records {
    type Output = Record;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
