#![warn(clippy::all, clippy::pedantic)]

use gag::Gag;
use took::{Timer, Took};

const RUNS: usize = 100;

fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);
    let print_gag = Gag::stdout().unwrap();

    let times: Vec<_> = jobs()
        .iter()
        .map(|j| {
            (
                j.1,
                (0..RUNS)
                    .map(|_| {
                        let took = Timer::new();
                        j.0();
                        took.took().into_std()
                    })
                    .min()
                    .unwrap(),
            )
        })
        .collect();

    drop(print_gag);
    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("everything");
}

fn jobs() -> &'static [(fn(), &'static str)] {
    &[
        (day_01::main, "Day 1"),
        (day_02::main, "Day 2"),
        (day_03::main, "Day 3"),
        (day_04::main, "Day 4"),
        (day_05::main, "Day 5"),
        (day_06::main, "Day 6"),
        (day_07::main, "Day 7"),
        (day_08::main, "Day 8"),
        (day_09::main, "Day 9"),
        (day_10::main, "Day 10"),
        (day_11::main, "Day 11"),
        (day_12::main, "Day 12"),
        (day_13::main, "Day 13"),
        (day_14::main, "Day 14"),
        //(day_15::main, "Day 15"),
    ]
}
