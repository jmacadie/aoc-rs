# Advent of Code solutions

This repo is built to host all my solutions to all the advent of code puzzles.
All days are written in Rust.
I only started in 2021 and did my first year in Python, because I didn't know any better!
In fairness, Python is a great language to do AoC, but I discovered Rust shortly after finishing that year and switched allegiance

## Structure

Each day is it's own binary which can be found in subfolders.
Go here to run (or test) the days and get the actual answers.

* (2015)[2015]
* (2016)[2016]
* 2017 - _not yet started_
* 2018 - _not yet started_
* 2019 - _not yet started_
* 2020 - _not yet started_
* 2021 - _not yet started_, although I do have (a python repo)[https://github.com/jmacadie/AdventOfCode]
* (2022)[2022]

In addition to the days themselves, this top-level folder contains a command line runner application that
will time profile the days in a selected year

I also have a sub-application that will look for any missing input files and download them automatically.
For more info, see (here)[inputs]

Finally, I have a (template folder)[template] with a template of a day for quickly getting another day added.
There's not much of interest in here

## How to run

To run any day, you need (Rust installed)[https://www.rust-lang.org/tools/install] :wink:

Then, at the root or in a day folder, run `cargo run --release`

Most days have tests based on the example cases given in the problem description. To run these, run `cargo test`

# To add a new day

Not the most streamlined process, this may change:

1. Copy the template folder to the new day e.g. `cp -r template 2015/day_01`
2. Edit the package name in the Cargo.toml file for the newly copied day. The name format is `day_yyyy_dd`
3. Uncomment the newly added day in (the top-level app's Cargo.toml)[Cargo.toml]
4. Update the relevant array in (src/days.rs)[src/days.rs] to include the new day
5. Go to the inputs folder and run the input script to download the new day's input: `cd inputs && cargo run --release`
6. Go back to the root folder and run the main app to check the new day shows and runs: `cd .. && cargo run --release`
7. Navigate to the new day & start coding :persevere:

Adding a new year will be a bit more work over and above these steps

## Performance

Stats generated on an Azure VM running Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz

<details>
  <summary>2015</summary>

  All Days = 555.24ms

  | Day | Runtime | Percentage of year |
  |---|---|---|
  |  Day 1 |   14.80 μs |    0% |
  |  Day 2 |  154.80 μs |    0% |
  |  Day 3 |  674.90 μs |    0% |
  |  Day 4 |  282.74 ms |   50% |
  |  Day 5 |  331.40 μs |    0% |
  |  Day 6 |   25.19 ms |    4% |
  |  Day 7 |   16.48 ms |    2% |
  |  Day 8 |   40.60 μs |    0% |
  |  Day 9 |   20.01 ms |    3% |
  | Day 10 |   62.95 ms |   11% |
  | Day 11 |   52.00 ms |    9% |
  | Day 12 |  177.30 μs |    0% |
  | Day 13 |    3.16 ms |    0% |
  | Day 14 |  249.40 μs |    0% |
  | Day 15 |  165.70 μs |    0% |
  | Day 16 |  209.30 μs |    0% |
  | Day 17 |    7.24 ms |    1% |
  | Day 18 |   47.98 ms |    8% |
  | Day 19 |   51.70 μs |    0% |
  | Day 20 |  451.30 μs |    0% |
  | Day 21 |  478.30 μs |    0% |
  | Day 22 |   26.53 ms |    4% |
  | Day 23 |   14.30 μs |    0% |
  | Day 24 |    7.95 ms |    1% |
  | Day 25 |    1.50 μs |    0% |

</details>

<details>
  <summary>2016</summary>

  All Days = 105.50 μs

  | Day | Runtime | Percentage of year |
  |---|---|---|
  |  Day 1 |   60.50 μs |   57% |
  |  Day 2 |   45.00 μs |   42% |

</details>

<details>
  <summary>2018</summary>

  _No solutions yet written_

</details>

<details>
  <summary>2019</summary>

  _No solutions yet written_

</details>

<details>
  <summary>2020</summary>

  _No solutions yet written_

</details>

<details>
  <summary>2021</summary>

  _No solutions yet written_

</details>

<details>
  <summary>2022</summary>

  All Days = 2.86s

  | Day | Runtime | Percentage of year |
  |---|---|---|
  |  Day 1 |   98.30 μs |    0% |
  |  Day 2 |  152.80 μs |    0% |
  |  Day 3 |  135.00 μs |    0% |
  |  Day 4 |  209.70 μs |    0% |
  |  Day 5 |  139.40 μs |    0% |
  |  Day 6 |   13.70 μs |    0% |
  |  Day 7 |   55.10 μs |    0% |
  |  Day 8 |  109.30 μs |    0% |
  |  Day 9 |  654.40 μs |    0% |
  | Day 10 |   27.30 μs |    0% |
  | Day 11 |   11.20 ms |    0% |
  | Day 12 |  545.70 μs |    0% |
  | Day 13 |  235.70 μs |    0% |
  | Day 14 |   15.86 ms |    0% |
  | Day 15 |   39.50 μs |    0% |
  | Day 16 |     1.01 s |   35% |
  | Day 17 |  397.60 μs |    0% |
  | Day 18 |  183.06 ms |    6% |
  | Day 19 |  170.59 ms |    5% |
  | Day 20 |  178.75 ms |    6% |
  | Day 21 |    3.60 ms |    0% |
  | Day 22 |    3.10 ms |    0% |
  | Day 23 |  242.24 ms |    8% |
  | Day 24 |     1.03 s |   36% |
  | Day 25 |   15.30 μs |    0% |

</details>
