#![warn(clippy::all, clippy::pedantic)]

pub mod common;
pub mod year_2015;
pub mod year_2016;
pub mod year_2017;
pub mod year_2018;
pub mod year_2019;
pub mod year_2020;
pub mod year_2021;
pub mod year_2022;

use year_2020::day_10;

fn main() {
    day_10::run();
}
