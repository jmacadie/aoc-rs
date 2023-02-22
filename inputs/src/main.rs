#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use reqwest::blocking::Client;
use reqwest::header;
use std::env;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let headers = get_headers();
    if headers.is_err() {
        println!("Missing the environment variable: 'AOC_SESSION'");
        println!("Cannot download personal inputs without it. Please extract it from a current web session.");
        println!("The session can be set (in bash/zsh) with:");
        println!("export AOC_SESSION=x");
        println!("Replace the x with your actual session value");
        return Ok(());
    }
    let headers = headers.unwrap();

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
    let path_str = format!("../{year}/day_{day:02}");
    let path = PathBuf::from(&path_str);
    if !path.exists() {
        //println!("No folder for {year} day {day}. Skipping to next.");
        return Ok(());
    }

    let input_path = format!("{path_str}/src/input.txt");
    let path = PathBuf::from(&input_path);
    //println!("{} exists: {}", input_path, path.exists());
    if !path.exists() {
        println!("Found missing data for {year}, day {day}. Downloading from website...");
        let data = get_day_input(year, day, client)?;
        let mut f = std::fs::File::create(path)?;
        f.write_all(data.as_bytes())?;
    }

    Ok(())
}

fn get_day_input(year: u16, day: u8, client: &Client) -> reqwest::Result<String> {
    let input_url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let res = client.get(input_url).send()?;
    res.text()
}

fn get_headers() -> Result<header::HeaderMap, Box<dyn Error>> {
    let session = format!("session={}", env::var("AOC_SESSION")?);
    let mut headers = header::HeaderMap::new();
    let value = header::HeaderValue::from_str(&session)?;
    headers.insert("Cookie", value);
    Ok(headers)
}
