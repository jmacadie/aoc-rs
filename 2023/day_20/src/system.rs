use crate::module;
use crate::pulse;
use crate::Modules;
use crate::Pulses;
use crate::Records;
use crate::State;

const RUNS: usize = 1000;

#[derive(Debug)]
pub struct System<const N: usize> {
    modules: Modules<N>,
    state: State<N>,
    stack: Pulses,
    start: module::Idx,
    output: module::Idx,
    states: Records,
    presses: usize,
    low: usize,
    high: usize,
}

impl<const N: usize> System<N> {
    pub fn new(input: &'static str) -> Self {
        let modules: Modules<N> = input.into();
        let state: State<N> = (&modules).into();
        let start = modules.start();
        let output = modules.end();
        Self {
            modules,
            state,
            stack: Pulses::new(),
            start,
            output,
            states: Records::new(),
            presses: 0,
            low: 0,
            high: 0,
        }
    }

    pub fn run(&mut self) -> usize {
        while self.presses < RUNS {
            self.push_button();
            let state = self.state.num();
            if let Some(cycle_start) = self.states.contains(state) {
                return self.output_value(cycle_start);
            }
            self.states.add(state, self.low, self.high);
        }
        self.low * self.high
    }

    fn output_value(&self, cycle_start: usize) -> usize {
        let cycle_length = self.presses - cycle_start;
        let cycles = (RUNS - cycle_start) / cycle_length;
        let tail_length = RUNS - cycle_start - cycle_length * cycles;

        let start_low = self.states[cycle_start].low;
        let start_high = self.states[cycle_start].high;
        let cycle_low = self.low - start_low;
        let cycle_high = self.high - start_high;
        let tail_low = self.states[cycle_start + tail_length].low - start_low;
        let tail_high = self.states[cycle_start + tail_length].high - start_high;

        let lows = start_low + cycle_low * cycles + tail_low;
        let highs = start_high + cycle_high * cycles + tail_high;

        lows * highs
    }

    pub fn find_output_cycle(&mut self) -> usize {
        let output_input = self
            .modules
            .iter()
            .position(|m| m.iter().any(|&o| o == self.output))
            .unwrap_or_else(|| unreachable!());
        let mut signals: Vec<(usize, Option<usize>)> = self
            .modules
            .iter()
            .enumerate()
            .filter(|&(_, m)| m.iter().any(|&o| o == output_input))
            .map(|(i, _)| (i, None))
            .collect();
        while !signals.iter().all(|&(_, cycle)| cycle.is_some()) && self.presses < 10000 {
            self.push_button_2(&mut signals);
        }
        println!("{signals:?}");
        signals
            .iter()
            .map(|(_, cycle)| cycle.unwrap_or_else(|| unreachable!()))
            .product()
    }

    fn push_button(&mut self) {
        self.presses += 1;
        self.stack.add(0, self.start, pulse::Type::Low);
        self.step();
    }

    fn step(&mut self) {
        if let Some(pulse) = self.stack.pop() {
            match pulse.flavour {
                pulse::Type::Low => self.low += 1,
                pulse::Type::High => self.high += 1,
            }
            if let Some(next_pulse) = self.state[pulse.to].recieve(pulse.from, pulse.flavour) {
                self.modules[pulse.to].iter().for_each(|&next| {
                    self.stack.add(pulse.to, next, next_pulse);
                });
            };
            self.step();
        }
    }

    fn push_button_2(&mut self, signals: &mut [(usize, Option<usize>)]) {
        self.presses += 1;
        self.stack.add(0, self.start, pulse::Type::Low);
        self.step_2(signals);
    }

    fn step_2(&mut self, signals: &mut [(usize, Option<usize>)]) {
        if let Some(pulse) = self.stack.pop() {
            if let Some(next_pulse) = self.state[pulse.to].recieve(pulse.from, pulse.flavour) {
                self.modules[pulse.to].iter().for_each(|&next| {
                    if next == self.output && pulse.flavour == pulse::Type::High {
                        signals
                            .iter_mut()
                            .filter(|(input, _)| *input == pulse.from)
                            .for_each(|(_, c)| *c = Some(self.presses));
                    }
                    self.stack.add(pulse.to, next, next_pulse);
                });
            };
            self.step_2(signals);
        }
    }
}
