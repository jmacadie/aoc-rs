#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod bench;
mod days;

use bench::bench;
use days::Year;
use std::{
    io::{self, BufRead, Write},
    time::Duration,
};
use took::Took;

// https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
// https://stackoverflow.com/questions/69981449/how-do-i-print-colored-text-to-the-terminal-in-rust#answer-69982036
const ANSI_GREY: &str = "\x1b[38;5;243m";
const ANSI_BLUE: &str = "\x1b[38;5;4m";
const ANSI_PURPLE: &str = "\x1b[38;5;5m";
const ANSI_RESET: &str = "\x1b[0m";
const ANSI_ERASE_IN_LINE: &str = "\x1b[2K";
const ANSI_PREVIOUS_LINE: &str = "\x1b[F";

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
    clear_days_menu(year)?;

    if day == 0 {
        run_all_days(year)?;
    } else {
        let (txt, d) = run_day(year, day);
        Took::from_std(d).describe(txt);
    }

    Ok(())
}

fn run_all_days(year: Year) -> io::Result<()> {
    let mut results = Vec::new();
    for day in 1..=days::count(year) {
        let (_, day_txt) = days::get(year, day);
        let output = format!("Calculating {day_txt}...");
        replace_current_line(&output)?;
        results.push(run_day(year, day));
    }
    replace_current_line("")?;

    let total = results.iter().map(|&(_, d)| d).sum();
    // Calculate percentages
    println!("|{:=>7}==={:=>10}==={:=>5}=|", "", "", "");
    println!("|{:^7} | {:^10} | {:^5} |", "Day", "Time", "%");
    println!("|{:=>7}=+={:=>10}=+={:=>5}=|", "", "", "");
    for &(txt, d) in &results {
        let pcnt = div_duration_pcnt(d, total);
        let formatted = Took::from_std(d);
        println!("|{txt:>7} | {formatted:>10} | {pcnt:>4}% |");
        println!("|{:->7}-+-{:->10}-+-{:->4}--|", "", "", "");
    }

    println!();
    Took::from_std(total).describe("All days");

    Ok(())
}

const fn div_duration_pcnt(numerator: Duration, denominator: Duration) -> u128 {
    100 * numerator.as_nanos() / denominator.as_nanos()
}

fn run_day(year: Year, day: usize) -> (&'static str, Duration) {
    let (f, txt) = days::get(year, day);
    let res = bench(f);
    (txt, res)
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
            "8" | "2022" => {
                if confirm_year("2022")? {
                    return Ok(Some(Year::Y2022));
                }
            }
            "9" | "2023" | "" => {
                if confirm_year("2023")? {
                    return Ok(Some(Year::Y2023));
                }
            }
            "h" | "help" => show_years(),
            "q" | "quit" | "exit" => return Ok(None),
            _ => {
                println!("Bad option: You typed '{buffer}'");
                show_short_options();
            }
        }
    }
}

fn confirm_year(year: &str) -> io::Result<bool> {
    let mut handle = io::stdout().lock();
    write!(handle, "Running {year}. Please confirm [y]/n  ")?;
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
                println!("Bad option: You typed '{buffer}'");
                show_short_options();
            }
        }
    }
}

fn confirm_day(year: Year, day: usize) -> io::Result<bool> {
    let mut handle = io::stdout().lock();
    if day == 0 {
        write!(handle, "Running all days in {year}. Please confirm [y]/n  ")?;
    } else {
        let (_, day_str) = days::get(year, day);
        write!(handle, "Running {year}, {day_str}. Please confirm [y]/n  ")?;
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
    println!("Choose a day to run from {year}:");
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
        Year::Y2023 => show_days_inner(days::DAYS_2023.iter()),
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
            println!("{idx}: {txt}");
        } else {
            println!(" {idx}: {txt}");
        }
    }
}

fn clear_days_menu(year: Year) -> io::Result<()> {
    let lines = days::count(year) + 10;
    clear_previous_lines(lines)?;
    Ok(())
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
        0 => println!("{ANSI_GREY} {index}:   {year} (no days available){ANSI_RESET}"),
        1 => println!(" {index}:   {year} (1 day available)"),
        d => println!(" {index}:   {year} ({d} days available)"),
    }
}

fn show_short_options() {
    println!("Type 'h' or 'help' to show the options again");
    println!("Type 'q' or 'exit' to quit");
}

fn welcome() {
    println!();
    println!("{ANSI_BLUE}Welcome to jmacadie's AoC runner{ANSI_RESET}");
    println!("================================");
    println!("{ANSI_GREY}https://github.com/jmacadie/aoc-rs{ANSI_RESET}");
    println!();
    println!("This tool is used to performance profile (run-time only) my solutions.");
    println!("All days are written in Rust.");
    println!("Individual days are their own binaries and can be run (to get the answers) by");
    println!(
        "navigating to the year & day (e.g. {ANSI_PURPLE}2015/day_25{ANSI_RESET}) and using '{ANSI_PURPLE}cargo run{ANSI_RESET}' there"
    );
    println!();
}

fn clear_previous_lines(num: usize) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "{ANSI_ERASE_IN_LINE}")?;
    for _ in 0..num {
        write!(handle, "{ANSI_PREVIOUS_LINE}{ANSI_ERASE_IN_LINE}")?;
    }
    handle.flush()?;

    Ok(())
}

fn replace_current_line(new_line: &str) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    write!(handle, "{ANSI_ERASE_IN_LINE}\r")?;
    write!(handle, "{new_line}")?;
    handle.flush()?;

    Ok(())
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
