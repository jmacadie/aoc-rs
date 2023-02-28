#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use reqwest::blocking::Client;
use reqwest::header;
use std::env;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let Ok(headers) = headers() else {
        println!("Missing the environment variable: 'AOC_SESSION'");
        println!("Cannot download personal inputs without it. Please extract it from a current web session.");
        println!("The session can be set (in bash/zsh) with:");
        println!("export AOC_SESSION=x");
        println!("Replace the x with your actual session value");
        return Ok(());
    };

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Loop through all days
    for year in 2015..=2022 {
        for day in 1..=25 {
            process_day(year, day, &client)?;
        }
    }
    Ok(())
}

fn process_day(year: u16, day: u8, client: &Client) -> Result<(), Box<dyn Error>> {
    let path = format!("../{year}/day_{day:02}");
    if !PathBuf::from(&path).exists() {
        // Quit out if we don't have the day folder
        return Ok(());
    }

    let input_file = PathBuf::from(format!("{path}/src/input.txt"));
    if !input_file.exists() {
        println!("Found missing data for {year}, day {day}. Downloading from website...");
        let data = day_input(year, day, client)?;
        let mut f = std::fs::File::create(input_file)?;
        f.write_all(data.as_bytes())?;
    }

    Ok(())
}

fn day_input(year: u16, day: u8, client: &Client) -> reqwest::Result<String> {
    let input_url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let res = client.get(input_url).send()?;
    res.text()
}

fn headers() -> Result<header::HeaderMap, Box<dyn Error>> {
    let session = format!("session={}", env::var("AOC_SESSION")?);
    let mut headers = header::HeaderMap::new();
    let value = header::HeaderValue::from_str(&session)?;
    headers.insert("Cookie", value);
    Ok(headers)
}
