#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use reqwest::blocking::Client;
use reqwest::header;
use std::env;
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let Ok(headers) = headers() else {
        println!("Missing the environment variable: 'AOC_SESSION'");
        println!("Cannot download personal inputs without it. Please extract it from a current web session.");
        println!("The session can be set (in bash/zsh) with:");
        println!("export AOC_SESSION=x");
        println!("Replace the x with your actual session value");
        return Ok(());
    };

    match in_aoc_repo() {
        Err(e) => {
            println!("Could not run git command, is it installed?");
            println!("{e}");
            return Ok(());
        }
        Ok(false) => {
            println!(
                "Don't appear to be in the advent of code repo. This command won't work outside."
            );
            return Ok(());
        }
        Ok(true) => (), // continue
    }

    let root = root_dir()?;

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    // Loop through all days
    for year in 2015..=2030 {
        for day in 1..=25 {
            process_day(year, day, &root, &client)?;
        }
    }
    Ok(())
}

fn in_aoc_repo() -> Result<bool, Box<dyn Error>> {
    const REMOTE: &str = "origin\tgit@github.com:jmacadie/aoc-rs.git (fetch)";
    let cmd = Command::new("git").args(["remote", "-v"]).output()?;
    if !cmd.status.success() {
        return Ok(false);
    }
    let out = String::from_utf8(cmd.stdout)?;
    if out.lines().next().unwrap_or("") == REMOTE {
        Ok(true)
    } else {
        Ok(false)
    }
}

fn root_dir() -> Result<String, Box<dyn Error>> {
    let cmd = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if !cmd.status.success() {
        return Ok(String::from("Not a git directory"));
    }
    let mut result = cmd.stdout.as_slice();
    while result.ends_with(&[b'\n']) || result.ends_with(&[b'\r']) {
        (_, result) = result.split_last().unwrap();
    }
    Ok(String::from_utf8(result.to_vec())?)
}

fn process_day(year: u16, day: u8, root: &str, client: &Client) -> Result<(), Box<dyn Error>> {
    let path = format!("{root}/{year}/day_{day:02}");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_in_aoc_repo() {
        let out = in_aoc_repo();
        assert!(out.is_ok());
        let out = out.unwrap();
        assert!(out);
    }

    #[test]
    fn test_root_dir() {
        let out = root_dir();
        assert!(out.is_ok());
        let out = out.unwrap();

        let dir = std::env::current_dir().unwrap();
        assert_eq!(dir.parent(), Some(std::path::Path::new(&out)));
    }
}
