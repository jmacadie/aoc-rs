#![warn(clippy::all, clippy::pedantic)]

use took::{Timer, Took};

const RUNS: usize = 100;

fn main() {
    println!("Benchmarking all days with {} runs...", RUNS);

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

    times.iter().for_each(|t| Took::from_std(t.1).describe(t.0));
    Took::from_std(times.into_iter().map(|(_, t)| t).sum()).describe("everything");
}

fn jobs() -> &'static [(fn(), &'static str)] {
    &[
        (day_01::bench, "Day 1"),
        (day_02::bench, "Day 2"),
        (day_03::bench, "Day 3"),
        (day_04::bench, "Day 4"),
        (day_05::bench, "Day 5"),
        (day_06::bench, "Day 6"),
    ]
}
