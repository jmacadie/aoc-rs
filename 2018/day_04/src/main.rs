#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{collections::HashMap, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<1118>(data));
    println!("Part 2: {}", part_two::<1118>(data));
}

fn part_one<const N: usize>(data: &str) -> usize {
    let log = Log::<N>::new(data);
    let records = log.to_guard_records();
    let (_, guard, times) = records.iter().fold(
        (0, 0, [0; 60]),
        |(max, max_guard, max_times), (&guard, &times)| {
            let total = times.iter().sum();
            if total > max {
                (total, guard, times)
            } else {
                (max, max_guard, max_times)
            }
        },
    );
    let (minute, _) = times.iter().enumerate().max_by_key(|(_, &t)| t).unwrap();
    usize::from(guard) * minute
}

fn part_two<const N: usize>(data: &str) -> usize {
    let log = Log::<N>::new(data);
    let records = log.to_guard_records();
    let (guard, time, _) = records.iter().fold(
        (0, 0, 0),
        |(max_guard, max_min, max_count), (&guard, &times)| {
            let (time, count) = times.iter().enumerate().max_by_key(|(_, &c)| c).unwrap();
            if count > &max_count {
                (guard, time, *count)
            } else {
                (max_guard, max_min, max_count)
            }
        },
    );
    usize::from(guard) * time
}

type GuardRecord = [u16; 60];
type GuardNumber = u16;

#[derive(Debug)]
struct Log<const N: usize> {
    data: [LogEntry; N],
}

impl<const N: usize> Log<N> {
    fn new(data: &str) -> Self {
        let mut lines = data.lines();
        let mut data: [LogEntry; N] =
            std::array::from_fn(|_| lines.next().unwrap().parse().unwrap());
        data.sort_unstable_by_key(|e| e.timestamp);
        Self { data }
    }

    fn to_guard_records(&self) -> HashMap<GuardNumber, GuardRecord> {
        let mut records = HashMap::new();
        let mut guard = 0;
        let mut start = 0;
        for entry in &self.data {
            match entry.event {
                Event::BeginsShift(g) => guard = g,
                Event::FallsAsleep => {
                    if entry.timestamp.hour > 0 {
                        start = 0;
                    } else {
                        start = entry.timestamp.minute;
                    }
                }
                Event::WakesUp => {
                    records
                        .entry(guard)
                        .and_modify(|record: &mut GuardRecord| {
                            for i in start..entry.timestamp.minute {
                                record[usize::from(i)] += 1;
                            }
                        })
                        .or_insert_with(|| {
                            std::array::from_fn(|i| {
                                u16::from(i >= start.into() && i < entry.timestamp.minute.into())
                            })
                        });
                }
            }
        }
        records
    }
}

#[derive(Debug)]
struct LogEntry {
    timestamp: Timestamp,
    event: Event,
}

impl FromStr for LogEntry {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (timestamp, event) = s
            .split_once("] ")
            .ok_or_else(|| format!("Cannot split the timestamp from the event entry: {s}"))?;
        let timestamp = timestamp.trim_start_matches('[').parse()?;
        let event = event.parse()?;
        Ok(Self { timestamp, event })
    }
}

#[derive(Debug)]
enum Event {
    BeginsShift(GuardNumber),
    FallsAsleep,
    WakesUp,
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "falls asleep" => Ok(Self::FallsAsleep),
            "wakes up" => Ok(Self::WakesUp),
            _ => {
                let mut parts = s.split(' ');
                let (Some("Guard"), Some(guard_number), Some("begins"), Some("shift"), None) = (
                    parts.next(),
                    parts.next(),
                    parts.next(),
                    parts.next(),
                    parts.next(),
                ) else {
                    return Err(format!("Guard beginning shift is badly formatted: {s}"));
                };
                let guard_number = guard_number
                    .trim_start_matches('#')
                    .parse()
                    .map_err(|_| format!("Cannot covert the gaurd number: {guard_number}"))?;
                Ok(Self::BeginsShift(guard_number))
            }
        }
    }
}

#[derive(Eq, Debug, Clone, Copy)]
struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl Ord for Timestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for Timestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year
            && self.month == other.month
            && self.day == other.day
            && self.hour == other.hour
            && self.minute == other.minute
    }
}

impl FromStr for Timestamp {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, time) = s
            .split_once(' ')
            .ok_or_else(|| format!("Cannot split timestamp into date and time parts: {s}"))?;
        let (year, monthday) = date
            .split_once('-')
            .ok_or_else(|| format!("Cannot split year from date: {date}"))?;
        let (month, day) = monthday
            .split_once('-')
            .ok_or_else(|| format!("Cannot split month and day: {monthday}"))?;
        let year = year
            .parse()
            .map_err(|_| format!("Year is not a number: {year}"))?;
        let month = month
            .parse()
            .map_err(|_| format!("Month is not a number: {month}"))?;
        let day = day
            .parse()
            .map_err(|_| format!("Day is not a number: {day}"))?;
        let (hour, minute) = time
            .split_once(':')
            .ok_or_else(|| format!("Cannot split time into hours and minutes: {time}"))?;
        let hour = hour
            .parse()
            .map_err(|_| format!("Hour is not a number: {hour}"))?;
        let minute = minute
            .parse()
            .map_err(|_| format!("Minute is not a number: {minute}"))?;
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(240, part_one::<17>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(4455, part_two::<17>(data));
    }
}
