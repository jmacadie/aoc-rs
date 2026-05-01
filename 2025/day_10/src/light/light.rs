use std::{fmt::Display, num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Light {
    target: u16,
    buttons: Vec<u16>,
}

impl Light {
    #[must_use]
    pub fn find_min_presses(&self) -> usize {
        let mut frontier = Vec::new();
        let mut next_frontier = vec![0];
        let mut counter = 0;
        let mut seen = [false; 1024];
        loop {
            std::mem::swap(&mut frontier, &mut next_frontier);
            counter += 1;
            while let Some(next) = frontier.pop() {
                for b in &self.buttons {
                    // XOR to press button
                    let new = next ^ b;
                    if new == self.target {
                        return counter;
                    }
                    let idx = usize::from(new);
                    if seen[idx] {
                        continue;
                    }
                    seen[idx] = true;
                    next_frontier.push(new);
                }
            }
        }
    }
}

impl Display for Light {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Target:")?;
        writeln!(f, " {:010b}", self.target)?;
        writeln!(f, "Buttons:")?;
        for b in &self.buttons {
            writeln!(f, " {b:010b}")?;
        }
        Ok(())
    }
}

impl FromStr for Light {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target_txt, rest) = s
            .split_once(' ')
            .ok_or_else(|| format!("Cannot split {s}"))?;
        let (buttons_txt, _) = rest
            .rsplit_once(' ')
            .ok_or_else(|| format!("Cannot split off the joltage at the end of {rest}"))?;

        let target = target_txt
            .char_indices()
            .filter(|&(_, c)| c == '#')
            .fold(0u16, |acc, (i, _)| acc | 1 << (i - 1));

        let mut buttons = Vec::new();
        let make_button = |numbers: &str| {
            let mut button: u16 = 0u16;
            for num in numbers.trim_prefix('(').trim_suffix(')').split(',') {
                let parsed = num.parse::<u16>()?;
                button |= 1 << parsed;
            }
            Ok(button)
        };
        for button in buttons_txt.split(' ') {
            let b = make_button(button).map_err(|_: ParseIntError| {
                format!("Cannot format button text into a u16: {button}")
            })?;
            buttons.push(b);
        }

        Ok(Self { target, buttons })
    }
}
