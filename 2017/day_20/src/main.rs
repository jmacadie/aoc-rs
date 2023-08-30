#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::ops::{Add, AddAssign, Mul};
use std::str::FromStr;

use itertools::Itertools;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one(data: &str) -> usize {
    let iter = data.lines().map(|l| l.parse::<Particle>().unwrap());
    let min_acceleration = iter.clone().map(|p| p.acceleration.size()).min().unwrap();

    iter.enumerate()
        .filter(|(_, p)| p.acceleration.size() == min_acceleration)
        .map(|(i, mut p)| {
            p.step(1_000);
            (i, p)
        })
        .min_by_key(|(_, p)| p.velocity.size())
        .unwrap()
        .0
}

fn part_two(data: &str) -> usize {
    let mut particles: Vec<_> = data
        .lines()
        .map(|l| l.parse::<Particle>().unwrap())
        .collect();
    particles = remove_collisions(&particles);
    // TODO: can we ve a bit smarter with this upper limit?
    for _ in 0..500 {
        particles.iter_mut().for_each(|p| p.step(1));
        particles = remove_collisions(&particles);
    }
    particles.len()
}

fn remove_collisions(particles: &[Particle]) -> Vec<Particle> {
    particles
        .iter()
        .sorted_unstable()
        .dedup_with_count()
        .filter(|&(c, _)| c == 1)
        .map(|(_, p)| p.clone())
        .collect()
}

#[derive(Debug, Eq, Clone)]
struct Particle {
    position: ThreeDim,
    velocity: ThreeDim,
    acceleration: ThreeDim,
}

impl Particle {
    fn step(&mut self, num: i32) {
        let triangle = num * (num + 1) / 2;
        self.position += num * self.velocity + triangle * self.acceleration;
        self.velocity += num * self.acceleration;
    }
}

impl FromStr for Particle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.split(", ");
        let (Some(p), Some(v), Some(a), None) = (data.next(), data.next(), data.next(), data.next()) else {
            return Err(format!("{s} not formed of three parts, seperated by commas"));
        };
        let p = p.trim_start_matches("p=").parse()?;
        let v = v.trim_start_matches("v=").parse()?;
        let a = a.trim_start_matches("a=").parse()?;
        Ok(Self {
            position: p,
            velocity: v,
            acceleration: a,
        })
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position
            .size()
            .cmp(&other.position.size())
            .then(self.position.x.cmp(&other.position.x))
            .then(self.position.y.cmp(&other.position.y))
            .then(self.position.z.cmp(&other.position.z))
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct ThreeDim {
    x: i32,
    y: i32,
    z: i32,
}

impl ThreeDim {
    const fn size(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl FromStr for ThreeDim {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.trim_start_matches('<').trim_end_matches('>').split(',');
        let (Some(x), Some(y), Some(z), None) = (data.next(), data.next(), data.next(), data.next()) else {
            return Err(format!("{s} not in three parts, seperated by commas"));
        };
        let x = x
            .parse()
            .map_err(|_| format!("{x} cannot be pasrsed into a number"))?;
        let y = y
            .parse()
            .map_err(|_| format!("{y} cannot be pasrsed into a number"))?;
        let z = z
            .parse()
            .map_err(|_| format!("{z} cannot be pasrsed into a number"))?;
        Ok(Self { x, y, z })
    }
}

impl Mul<ThreeDim> for i32 {
    type Output = ThreeDim;

    fn mul(self, rhs: ThreeDim) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<i32> for ThreeDim {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.y * rhs,
        }
    }
}

impl Add for ThreeDim {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for ThreeDim {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
