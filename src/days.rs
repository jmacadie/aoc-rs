use std::fmt::Display;

pub const YEARS: [Year; 8] = [
    Year::Y2015,
    Year::Y2016,
    Year::Y2017,
    Year::Y2018,
    Year::Y2019,
    Year::Y2020,
    Year::Y2021,
    Year::Y2022,
];

#[derive(Debug, Clone, Copy)]
pub enum Year {
    Y2015,
    Y2016,
    Y2017,
    Y2018,
    Y2019,
    Y2020,
    Y2021,
    Y2022,
}

impl Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Y2015 => write!(f, "2015")?,
            Self::Y2016 => write!(f, "2016")?,
            Self::Y2017 => write!(f, "2017")?,
            Self::Y2018 => write!(f, "2018")?,
            Self::Y2019 => write!(f, "2019")?,
            Self::Y2020 => write!(f, "2020")?,
            Self::Y2021 => write!(f, "2021")?,
            Self::Y2022 => write!(f, "2022")?,
        }
        Ok(())
    }
}

pub fn count(year: Year) -> usize {
    match year {
        Year::Y2015 => DAYS_2015.len(),
        Year::Y2016 => DAYS_2016.len(),
        Year::Y2017 => DAYS_2017.len(),
        Year::Y2018 => DAYS_2018.len(),
        Year::Y2019 => DAYS_2019.len(),
        Year::Y2020 => DAYS_2020.len(),
        Year::Y2021 => DAYS_2021.len(),
        Year::Y2022 => DAYS_2022.len(),
    }
}

pub fn get(year: Year, day: usize) -> (fn(), &'static str) {
    match year {
        Year::Y2015 => DAYS_2015[day - 1],
        Year::Y2016 => DAYS_2016[day - 1],
        Year::Y2017 => DAYS_2017[day - 1],
        Year::Y2018 => DAYS_2018[day - 1],
        Year::Y2019 => DAYS_2019[day - 1],
        Year::Y2020 => DAYS_2020[day - 1],
        Year::Y2021 => DAYS_2021[day - 1],
        Year::Y2022 => DAYS_2022[day - 1],
    }
}

pub const DAYS_2022: [(fn(), &str); 25] = [
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
];

pub const DAYS_2021: [(fn(), &str); 0] = [
    //(day_2021_01::main, "Day 1"),
    //(day_2021_02::main, "Day 2"),
    //(day_2021_03::main, "Day 3"),
    //(day_2021_04::main, "Day 4"),
    //(day_2021_05::main, "Day 5"),
    //(day_2021_06::main, "Day 6"),
    //(day_2021_07::main, "Day 7"),
    //(day_2021_08::main, "Day 8"),
    //(day_2021_09::main, "Day 9"),
    //(day_2021_10::main, "Day 10"),
    //(day_2021_11::main, "Day 11"),
    //(day_2021_12::main, "Day 12"),
    //(day_2021_13::main, "Day 13"),
    //(day_2021_14::main, "Day 14"),
    //(day_2021_15::main, "Day 15"),
    //(day_2021_16::main, "Day 16"),
    //(day_2021_17::main, "Day 17"),
    //(day_2021_18::main, "Day 18"),
    //(day_2021_19::main, "Day 19"),
    //(day_2021_20::main, "Day 20"),
    //(day_2021_21::main, "Day 21"),
    //(day_2021_22::main, "Day 22"),
    //(day_2021_23::main, "Day 23"),
    //(day_2021_24::main, "Day 24"),
    //(day_2021_25::main, "Day 25"),
];

pub const DAYS_2020: [(fn(), &str); 0] = [
    //(day_2020_01::main, "Day 1"),
    //(day_2020_02::main, "Day 2"),
    //(day_2020_03::main, "Day 3"),
    //(day_2020_04::main, "Day 4"),
    //(day_2020_05::main, "Day 5"),
    //(day_2020_06::main, "Day 6"),
    //(day_2020_07::main, "Day 7"),
    //(day_2020_08::main, "Day 8"),
    //(day_2020_09::main, "Day 9"),
    //(day_2020_10::main, "Day 10"),
    //(day_2020_11::main, "Day 11"),
    //(day_2020_12::main, "Day 12"),
    //(day_2020_13::main, "Day 13"),
    //(day_2020_14::main, "Day 14"),
    //(day_2020_15::main, "Day 15"),
    //(day_2020_16::main, "Day 16"),
    //(day_2020_17::main, "Day 17"),
    //(day_2020_18::main, "Day 18"),
    //(day_2020_19::main, "Day 19"),
    //(day_2020_20::main, "Day 20"),
    //(day_2020_21::main, "Day 21"),
    //(day_2020_22::main, "Day 22"),
    //(day_2020_23::main, "Day 23"),
    //(day_2020_24::main, "Day 24"),
    //(day_2020_25::main, "Day 25"),
];

pub const DAYS_2019: [(fn(), &str); 0] = [
    //(day_2019_01::main, "Day 1"),
    //(day_2019_02::main, "Day 2"),
    //(day_2019_03::main, "Day 3"),
    //(day_2019_04::main, "Day 4"),
    //(day_2019_05::main, "Day 5"),
    //(day_2019_06::main, "Day 6"),
    //(day_2019_07::main, "Day 7"),
    //(day_2019_08::main, "Day 8"),
    //(day_2019_09::main, "Day 9"),
    //(day_2019_10::main, "Day 10"),
    //(day_2019_11::main, "Day 11"),
    //(day_2019_12::main, "Day 12"),
    //(day_2019_13::main, "Day 13"),
    //(day_2019_14::main, "Day 14"),
    //(day_2019_15::main, "Day 15"),
    //(day_2019_16::main, "Day 16"),
    //(day_2019_17::main, "Day 17"),
    //(day_2019_18::main, "Day 18"),
    //(day_2019_19::main, "Day 19"),
    //(day_2019_20::main, "Day 20"),
    //(day_2019_21::main, "Day 21"),
    //(day_2019_22::main, "Day 22"),
    //(day_2019_23::main, "Day 23"),
    //(day_2019_24::main, "Day 24"),
    //(day_2019_25::main, "Day 25"),
];

pub const DAYS_2018: [(fn(), &str); 0] = [
    //(day_2018_01::main, "Day 1"),
    //(day_2018_02::main, "Day 2"),
    //(day_2018_03::main, "Day 3"),
    //(day_2018_04::main, "Day 4"),
    //(day_2018_05::main, "Day 5"),
    //(day_2018_06::main, "Day 6"),
    //(day_2018_07::main, "Day 7"),
    //(day_2018_08::main, "Day 8"),
    //(day_2018_09::main, "Day 9"),
    //(day_2018_10::main, "Day 10"),
    //(day_2018_11::main, "Day 11"),
    //(day_2018_12::main, "Day 12"),
    //(day_2018_13::main, "Day 13"),
    //(day_2018_14::main, "Day 14"),
    //(day_2018_15::main, "Day 15"),
    //(day_2018_16::main, "Day 16"),
    //(day_2018_17::main, "Day 17"),
    //(day_2018_18::main, "Day 18"),
    //(day_2018_19::main, "Day 19"),
    //(day_2018_20::main, "Day 20"),
    //(day_2018_21::main, "Day 21"),
    //(day_2018_22::main, "Day 22"),
    //(day_2018_23::main, "Day 23"),
    //(day_2018_24::main, "Day 24"),
    //(day_2018_25::main, "Day 25"),
];

pub const DAYS_2017: [(fn(), &str); 0] = [
    //(day_2017_01::main, "Day 1"),
    //(day_2017_02::main, "Day 2"),
    //(day_2017_03::main, "Day 3"),
    //(day_2017_04::main, "Day 4"),
    //(day_2017_05::main, "Day 5"),
    //(day_2017_06::main, "Day 6"),
    //(day_2017_07::main, "Day 7"),
    //(day_2017_08::main, "Day 8"),
    //(day_2017_09::main, "Day 9"),
    //(day_2017_10::main, "Day 10"),
    //(day_2017_11::main, "Day 11"),
    //(day_2017_12::main, "Day 12"),
    //(day_2017_13::main, "Day 13"),
    //(day_2017_14::main, "Day 14"),
    //(day_2017_15::main, "Day 15"),
    //(day_2017_16::main, "Day 16"),
    //(day_2017_17::main, "Day 17"),
    //(day_2017_18::main, "Day 18"),
    //(day_2017_19::main, "Day 19"),
    //(day_2017_20::main, "Day 20"),
    //(day_2017_21::main, "Day 21"),
    //(day_2017_22::main, "Day 22"),
    //(day_2017_23::main, "Day 23"),
    //(day_2017_24::main, "Day 24"),
    //(day_2017_25::main, "Day 25"),
];

pub const DAYS_2016: [(fn(), &str); 12] = [
    (day_2016_01::main, "Day 1"),
    (day_2016_02::main, "Day 2"),
    (day_2016_03::main, "Day 3"),
    (day_2016_04::main, "Day 4"),
    (day_2016_05::main, "Day 5"),
    (day_2016_06::main, "Day 6"),
    (day_2016_07::main, "Day 7"),
    (day_2016_08::main, "Day 8"),
    (day_2016_09::main, "Day 9"),
    (day_2016_10::main, "Day 10"),
    (day_2016_11::main, "Day 11"),
    (day_2016_12::main, "Day 12"),
    //(day_2016_13::main, "Day 13"),
    //(day_2016_14::main, "Day 14"),
    //(day_2016_15::main, "Day 15"),
    //(day_2016_16::main, "Day 16"),
    //(day_2016_17::main, "Day 17"),
    //(day_2016_18::main, "Day 18"),
    //(day_2016_19::main, "Day 19"),
    //(day_2016_20::main, "Day 20"),
    //(day_2016_21::main, "Day 21"),
    //(day_2016_22::main, "Day 22"),
    //(day_2016_23::main, "Day 23"),
    //(day_2016_24::main, "Day 24"),
    //(day_2016_25::main, "Day 25"),
];

pub const DAYS_2015: [(fn(), &str); 25] = [
    (day_2015_01::main, "Day 1"),
    (day_2015_02::main, "Day 2"),
    (day_2015_03::main, "Day 3"),
    (day_2015_04::main, "Day 4"),
    (day_2015_05::main, "Day 5"),
    (day_2015_06::main, "Day 6"),
    (day_2015_07::main, "Day 7"),
    (day_2015_08::main, "Day 8"),
    (day_2015_09::main, "Day 9"),
    (day_2015_10::main, "Day 10"),
    (day_2015_11::main, "Day 11"),
    (day_2015_12::main, "Day 12"),
    (day_2015_13::main, "Day 13"),
    (day_2015_14::main, "Day 14"),
    (day_2015_15::main, "Day 15"),
    (day_2015_16::main, "Day 16"),
    (day_2015_17::main, "Day 17"),
    (day_2015_18::main, "Day 18"),
    (day_2015_19::main, "Day 19"),
    (day_2015_20::main, "Day 20"),
    (day_2015_21::main, "Day 21"),
    (day_2015_22::main, "Day 22"),
    (day_2015_23::main, "Day 23"),
    (day_2015_24::main, "Day 24"),
    (day_2015_25::main, "Day 25"),
];
