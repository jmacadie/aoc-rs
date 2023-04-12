# Advent of Code solutions

This repo is built to host my solutions to [advent of code puzzles](https://adventofcode.com/).
All days are written in Rust.

Generally speaking, I'm trying to stick to standard library.
This is a learning exercise for me & I don't want an external crate to magically do all the work for me.
I do make an exception for [itertools](https://docs.rs/itertools/latest/itertools/), is which is de facto part of the standard library as far as I'm concerned.
On odd days I'll use other crates too, e.g. the [MD-5 crate](https://docs.rs/md-5/latest/md5/) for [2015/day_04](2015/day_04/src/main.rs).

I only started in 2021 and did my first year in Python, because I didn't know any better!
In fairness, Python is a great language to do AoC, but I discovered Rust shortly after finishing that year and switched allegiance.

## Structure

Each day is it's own binary which can be found in subfolders.
Go here to run (or test) the days, and get the actual answers.

* [2015](2015)
* [2016](2016)
* 2017 - _not yet started_
* 2018 - _not yet started_
* 2019 - _not yet started_
* 2020 - _not yet started_, I do have some solutions [here](wip/year_2020) from when I was starting out.
They need to be re-written in the standard format and generally made less bad
* 2021 - _not yet started_, although I do have [a python repo](https://github.com/jmacadie/AdventOfCode)
* [2022](2022)

In addition to the days themselves, this top-level folder contains a command line runner application that
will time profile the days in a selected year.

I also have a sub-application that will look for any missing input files and download them automatically.
For more info, see [here](inputs).

Finally, I have a [template folder](template) with a template of a day for quickly getting another day added.
There's not much of interest in here.

## How to run

To start, you need :crab: [Rust installed](https://www.rust-lang.org/tools/install) :crab:

* **Performance profiler**: at the root directory, run `cargo run --release`
N.B. This won't output any answers, just the calc runtime
* **Run a day**: navigate to the relevant directory, e.g. `cd 2015/day_01`, and then `cargo run --release`
* **Test a day**: Most days have tests based on the example cases given in the problem description.
To run these, navigate to the day directory & run `cargo test`

## To add a new day

Not the most streamlined process, this may change:

1. Copy the template folder to the new day e.g. `cp -r template 2015/day_01`
2. Edit the package name in the Cargo.toml file for the newly copied day. The name format is `day_yyyy_dd`
3. Uncomment the newly added day in [the top-level app's Cargo.toml](Cargo.toml)
4. Update the relevant array in [src/days.rs](src/days.rs) to include the new day.
Note you will need to increase the size of the array as well as uncommenting the day line
5. Download the inputs by running `cargo aoc-inputs`. This needs to be installed first, see [here](inputs) for more info
6. From the root folder, run the main app to check the new day shows up and runs without error `cargo run --release`
7. Navigate to the new day & start coding :persevere:

Adding a new year will be a bit more work over and above these steps

## Performance

Stats generated on an Azure VM running Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz

<details>
  <summary>2015</summary>

  **All Days -- 555.24ms**

  | Day | Runtime | Percentage of year |
  |---|---|---|
  |  [Day 1](2015/day_01/src/main.rs) |   14.80 μs |    0% |
  |  [Day 2](2015/day_02/src/main.rs) |  154.80 μs |    0% |
  |  [Day 3](2015/day_03/src/main.rs) |  674.90 μs |    0% |
  |  [Day 4](2015/day_04/src/main.rs) |  282.74 ms |   50% |
  |  [Day 5](2015/day_05/src/main.rs) |  331.40 μs |    0% |
  |  [Day 6](2015/day_06/src/main.rs) |   25.19 ms |    4% |
  |  [Day 7](2015/day_07/src/main.rs) |   16.48 ms |    2% |
  |  [Day 8](2015/day_08/src/main.rs) |   40.60 μs |    0% |
  |  [Day 9](2015/day_09/src/main.rs) |   20.01 ms |    3% |
  | [Day 10](2015/day_10/src/main.rs) |   62.95 ms |   11% |
  | [Day 11](2015/day_11/src/main.rs) |   52.00 ms |    9% |
  | [Day 12](2015/day_12/src/main.rs) |  177.30 μs |    0% |
  | [Day 13](2015/day_13/src/main.rs) |    3.16 ms |    0% |
  | [Day 14](2015/day_14/src/main.rs) |  249.40 μs |    0% |
  | [Day 15](2015/day_15/src/main.rs) |  165.70 μs |    0% |
  | [Day 16](2015/day_16/src/main.rs) |  209.30 μs |    0% |
  | [Day 17](2015/day_17/src/main.rs) |    7.24 ms |    1% |
  | [Day 18](2015/day_18/src/main.rs) |   47.98 ms |    8% |
  | [Day 19](2015/day_19/src/main.rs) |   51.70 μs |    0% |
  | [Day 20](2015/day_20/src/main.rs) |  451.30 μs |    0% |
  | [Day 21](2015/day_21/src/main.rs) |  478.30 μs |    0% |
  | [Day 22](2015/day_22/src/main.rs) |   26.53 ms |    4% |
  | [Day 23](2015/day_23/src/main.rs) |   14.30 μs |    0% |
  | [Day 24](2015/day_24/src/main.rs) |    7.95 ms |    1% |
  | [Day 25](2015/day_25/src/main.rs) |    1.50 μs |    0% |

</details>

<details>
  <summary>2016</summary>

  **All Days -- 15.00 s** _(part-completed)_

  | Day | Runtime | Percentage of year |
  |---|---|---|
  |  [Day 1](2016/day_01/src/main.rs) |  60.50 μs |    0% |
  |  [Day 2](2016/day_02/src/main.rs) |  45.00 μs |    0% |
  |  [Day 3](2016/day_03/src/main.rs) | 346.30 μs |    0% |
  |  [Day 4](2016/day_04/src/main.rs) |   1.95 ms |    0% |
  |  [Day 5](2016/day_05/src/main.rs) |   5.52  s |   36% |
  |  [Day 6](2016/day_06/src/main.rs) | 211.10 μs |    0% |
  |  [Day 7](2016/day_07/src/main.rs) |   2.09 ms |    0% |
  |  [Day 8](2016/day_08/src/main.rs) |  44.90 μs |    0% |
  |  [Day 9](2016/day_09/src/main.rs) |  31.90 μs |    0% |
  | [Day 10](2016/day_10/src/main.rs) |  87.10 μs |    0% |
  | [Day 11](2016/day_11/src/main.rs) |  46.63 ms |    0% |
  | [Day 12](2016/day_12/src/main.rs) |   3.40 μs |    0% |
  | [Day 13](2016/day_13/src/main.rs) |  57.10 μs |    0% |
  | [Day 14](2016/day_14/src/main.rs) |   9.51 s  |   63% |
  | [Day 15](2016/day_15/src/main.rs) |   8.60 μs |    0% |
  | [Day 16](2016/day_16/src/main.rs) |   2.60 μs |    0% |

</details>

<details>
  <summary>2017</summary>

  _No solutions yet written_

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

  **All Days -- 2.86s**

  | Day | Runtime | Percentage of year |
  |---|---|---|
  |  [Day 1](2022/day_01/src/main.rs) |   98.30 μs |    0% |
  |  [Day 2](2022/day_02/src/main.rs) |  152.80 μs |    0% |
  |  [Day 3](2022/day_03/src/main.rs) |  135.00 μs |    0% |
  |  [Day 4](2022/day_04/src/main.rs) |  209.70 μs |    0% |
  |  [Day 5](2022/day_05/src/main.rs) |  139.40 μs |    0% |
  |  [Day 6](2022/day_06/src/main.rs) |   13.70 μs |    0% |
  |  [Day 7](2022/day_07/src/main.rs) |   55.10 μs |    0% |
  |  [Day 8](2022/day_08/src/main.rs) |  109.30 μs |    0% |
  |  [Day 9](2022/day_09/src/main.rs) |  654.40 μs |    0% |
  | [Day 10](2022/day_10/src/main.rs) |   27.30 μs |    0% |
  | [Day 11](2022/day_11/src/main.rs) |   11.20 ms |    0% |
  | [Day 12](2022/day_12/src/main.rs) |  545.70 μs |    0% |
  | [Day 13](2022/day_13/src/main.rs) |  235.70 μs |    0% |
  | [Day 14](2022/day_14/src/main.rs) |   15.86 ms |    0% |
  | [Day 15](2022/day_15/src/main.rs) |   39.50 μs |    0% |
  | [Day 16](2022/day_16/src/main.rs) |     1.01 s |   35% |
  | [Day 17](2022/day_17/src/main.rs) |  397.60 μs |    0% |
  | [Day 18](2022/day_18/src/main.rs) |  183.06 ms |    6% |
  | [Day 19](2022/day_19/src/main.rs) |  170.59 ms |    5% |
  | [Day 20](2022/day_20/src/main.rs) |  178.75 ms |    6% |
  | [Day 21](2022/day_21/src/main.rs) |    3.60 ms |    0% |
  | [Day 22](2022/day_22/src/main.rs) |    3.10 ms |    0% |
  | [Day 23](2022/day_23/src/main.rs) |  242.24 ms |    8% |
  | [Day 24](2022/day_24/src/main.rs) |     1.03 s |   36% |
  | [Day 25](2022/day_25/src/main.rs) |   15.30 μs |    0% |

</details>
