#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use gag::Gag;
use took::{Timer, Took};

const RUNS: usize = 10;

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
        (day_2022_01::main, "Day 1"),
        (day_2022_02::main, "Day 2"),
        (day_2022_03::main, "Day 3"),
        (day_2022_04::main, "Day 4"),
        (day_2022_05::main, "Day 5"),
        (day_2022_06::main, "Day 6"),
        (day_2022_07::main, "Day 7"),
        (day_2022_08::main, "Day 8"),
        (day_2022_09::main, "Day 9"),
        (day_2022_10::main, "Day 10"),
        (day_2022_11::main, "Day 11"),
        (day_2022_12::main, "Day 12"),
        (day_2022_13::main, "Day 13"),
        (day_2022_14::main, "Day 14"),
        (day_2022_15::main, "Day 15"),
        (day_2022_16::main, "Day 16"),
        (day_2022_17::main, "Day 17"),
        (day_2022_18::main, "Day 18"),
        (day_2022_19::main, "Day 19"),
        (day_2022_20::main, "Day 20"),
        (day_2022_21::main, "Day 21"),
        (day_2022_22::main, "Day 22"),
        (day_2022_23::main, "Day 23"),
        (day_2022_24::main, "Day 24"),
        (day_2022_25::main, "Day 25"),
    ]
}
