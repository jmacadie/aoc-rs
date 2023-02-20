#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod bench;
mod days;

use bench::bench;
use days::Year;
use std::io::{self, BufRead, Write};

// https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
// https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust#answer-69982036
const ANSI_GREY: &str = "\x1b[38;5;243m";
const ANSI_BLUE: &str = "\x1b[38;5;4m";
const ANSI_RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    welcome();

    show_years();
    let year = pick_year()?;
    if year.is_none() {
        return Ok(());
    }
    let year = year.unwrap();

    let day = pick_day(year)?;
    if day.is_none() {
        return Ok(());
    }
    let day = day.unwrap();

    if day == 0 {
        run_all_days(year);
    } else {
        run_day(year, day);
    }

    Ok(())
}

fn run_all_days(year: Year) {
    for day in 1..=days::count(year) {
        run_day(year, day);
    }
}

fn run_day(year: Year, day: usize) {
    let (f, txt) = days::get(year, day);
    let res = bench(f);
    res.describe(txt);
}

fn pick_year() -> io::Result<Option<Year>> {
    let mut buffer = String::new();

    loop {
        buffer.truncate(0);
        io::stdin().lock().read_line(&mut buffer)?;
        trim_newline(&mut buffer);

        match buffer.as_str() {
            "1" | "2015" => {
                if confirm_year("2015")? {
                    return Ok(Some(Year::Y2015));
                }
            }
            "2" | "2016" => {
                if confirm_year("2016")? {
                    return Ok(Some(Year::Y2016));
                }
            }
            "3" | "2017" => {
                if confirm_year("2017")? {
                    return Ok(Some(Year::Y2017));
                }
            }
            "4" | "2018" => {
                if confirm_year("2018")? {
                    return Ok(Some(Year::Y2018));
                }
            }
            "5" | "2019" => {
                if confirm_year("2019")? {
                    return Ok(Some(Year::Y2019));
                }
            }
            "6" | "2020" => {
                if confirm_year("2020")? {
                    return Ok(Some(Year::Y2020));
                }
            }
            "7" | "2021" => {
                if confirm_year("2021")? {
                    return Ok(Some(Year::Y2021));
                }
            }
            "8" | "2022" | "" => {
                if confirm_year("2022")? {
                    return Ok(Some(Year::Y2022));
                }
            }
            "h" | "help" => show_years(),
            "q" | "quit" | "exit" => return Ok(None),
            _ => {
                println!("Bad option: You typed '{}'", buffer);
                show_short_options();
            }
        }
    }
}

fn confirm_year(year: &str) -> io::Result<bool> {
    let mut handle = io::stdout().lock();
    write!(handle, "Running {}. Please confirm [y]/n  ", year)?;
    handle.flush()?;

    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    trim_newline(&mut buffer);

    match buffer.to_lowercase().as_str() {
        "y" | "yes" | "" => Ok(true),
        _ => {
            write!(handle, "Cancelling... pick another year: ")?;
            handle.flush()?;
            Ok(false)
        }
    }
}

fn pick_day(year: Year) -> io::Result<Option<usize>> {
    show_days(year);
    let mut buffer = String::new();

    loop {
        buffer.truncate(0);
        io::stdin().lock().read_line(&mut buffer)?;
        trim_newline(&mut buffer);

        match buffer.as_str() {
            "h" | "help" => show_days(year),
            "q" | "quit" | "exit" => return Ok(None),
            "" | "0" => {
                if !confirm_day(year, 0)? {
                    continue;
                }
                return Ok(Some(0));
            }
            sel => {
                let sel_conv = sel.parse();
                if let Ok(sel_num) = sel_conv {
                    if sel_num <= days::count(year) {
                        if !confirm_day(year, sel_num)? {
                            continue;
                        }
                        return Ok(Some(sel_num));
                    }
                }
                println!("Bad option: You typed '{}'", buffer);
                show_short_options();
            }
        }
    }
}

fn confirm_day(year: Year, day: usize) -> io::Result<bool> {
    let mut handle = io::stdout().lock();
    if day == 0 {
        write!(
            handle,
            "Running all days in {}. Please confirm [y]/n  ",
            year
        )?;
    } else {
        let (_, day_str) = days::get(year, day);
        write!(
            handle,
            "Running {}, {}. Please confirm [y]/n  ",
            year, day_str
        )?;
    }
    handle.flush()?;

    let mut buffer = String::new();
    io::stdin().lock().read_line(&mut buffer)?;
    trim_newline(&mut buffer);

    match buffer.to_lowercase().as_str() {
        "y" | "yes" | "" => Ok(true),
        _ => {
            write!(handle, "Cancelling... pick another day: ")?;
            handle.flush()?;
            Ok(false)
        }
    }
}

fn show_days(year: Year) {
    println!();
    println!("Choose a day to run from {}:", year);
    println!(" 0: All Days (default)");
    match year {
        Year::Y2015 => show_days_inner(days::DAYS_2015.iter()),
        Year::Y2016 => show_days_inner(days::DAYS_2016.iter()),
        Year::Y2017 => show_days_inner(days::DAYS_2017.iter()),
        Year::Y2018 => show_days_inner(days::DAYS_2018.iter()),
        Year::Y2019 => show_days_inner(days::DAYS_2019.iter()),
        Year::Y2020 => show_days_inner(days::DAYS_2020.iter()),
        Year::Y2021 => show_days_inner(days::DAYS_2021.iter()),
        Year::Y2022 => show_days_inner(days::DAYS_2022.iter()),
    };
    println!();
    println!("Type the list number, but NOT the day value");
    println!("  - blank will select all days");
    show_short_options();
}

fn show_days_inner<'a, I>(days: I)
where
    I: Iterator<Item = &'a (fn(), &'a str)>,
{
    for ((_, txt), idx) in days.zip(1..) {
        if idx > 9 {
            println!("{}: {}", idx, txt);
        } else {
            println!(" {}: {}", idx, txt);
        }
    }
}

fn show_years() {
    println!("Choose a year to run:");
    for (year, idx) in days::YEARS.iter().zip(1..) {
        show_year_inner(idx, *year);
    }
    println!();
    println!("Type the list number, or the whole year");
    println!("  - blank will select the most recent year");
    show_short_options();
}

fn show_year_inner(index: u8, year: Year) {
    match days::count(year) {
        0 => println!(
            "{} {}:   {} (no days available){}",
            ANSI_GREY, index, year, ANSI_RESET
        ),
        1 => println!(" {}:   {} (1 day available)", index, year),
        d => println!(" {}:   {} ({} days available)", index, year, d),
    }
}

fn show_short_options() {
    println!("Type 'h' or 'help' to show the options again");
    println!("Type 'q' or 'exit' to quit");
}

fn welcome() {
    println!();
    println!(
        "{}Welcome to jmacadie's AoC runner{}",
        ANSI_BLUE, ANSI_RESET
    );
    println!("================================");
    println!(
        "{}https://github.com/jmacadie/aoc-rs{}",
        ANSI_GREY, ANSI_RESET
    );
    println!();
    println!("This tool is used to performance profile (run-time only) my solutions.");
    println!("All days are written in Rust.");
    println!("Individual days are their own binaries and can be run (to get the answers) by");
    println!("navigating to the year & day (e.g. $ cd 2015/day_25) and using 'cargo run' there");
    println!();
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
