#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    let (even, odd, outer) = run(data);
    println!("Part 1: {}", part_one(even));
    println!("Part 2: {}", part_two(even, odd, outer));
}

const fn part_one(even: usize) -> usize {
    even
}

const fn part_two(even: usize, odd: usize, outer: usize) -> usize {
    const STEPS: usize = 26_501_365;
    let n = (STEPS - 65) / 131;
    even * n * n + odd * (n + 1) * (n + 1) + outer * n * (n + 1)
}

fn run(data: &'static str) -> (usize, usize, usize) {
    const F: usize = 131;
    const V: usize = 393; // 3 * F, not sure how to do this without hard-coding
    let farm: Farm<F> = data.into();
    let start = Point {
        x: (V - 1) / 2,
        y: (V - 1) / 2,
    };
    let mut visit_odd = Visited::<V>::new();
    let mut visit_even = Visited::<V>::new();
    let mut frontier = vec![start];
    for idx in (0..2).cycle().take((F - 1) / 2) {
        let visited = if idx == 0 {
            &mut visit_odd
        } else {
            &mut visit_even
        };
        frontier = step(frontier, visited, &farm);
    }
    let even = visit_even.count();
    let odd = visit_odd.count();
    for idx in (0..2).cycle().skip(1).take(F) {
        let visited = if idx == 0 {
            &mut visit_odd
        } else {
            &mut visit_even
        };
        frontier = step(frontier, visited, &farm);
    }
    let outer = (visit_even.count() - 4 * odd - even) / 2;
    (even, odd, outer)
}

fn step<const V: usize, const F: usize>(
    mut frontier: Vec<Point>,
    visited: &mut Visited<V>,
    farm: &Farm<F>,
) -> Vec<Point> {
    let mut next = Vec::with_capacity(frontier.len() * 4);
    while let Some(p) = frontier.pop() {
        for next_p in p.neighbours() {
            if !visited.contains(next_p) && farm.is_garden(next_p) {
                visited.add(next_p);
                next.push(next_p);
            }
        }
    }
    next
}

#[allow(dead_code)]
fn print<const V: usize, const F: usize>(farm: &Farm<F>, visited: &Visited<V>) {
    for y in 0..V {
        for x in 0..V {
            let p = Point { x, y };
            if visited.contains(p) {
                print!("O");
            } else {
                match farm.get(p) {
                    b'S' => print!("S"),
                    b'#' => print!("#"),
                    b'.' => print!("."),
                    _ => unreachable!(),
                }
            }
        }
        println!();
    }
}

struct Farm<const N: usize> {
    map: &'static [u8],
}

impl<const N: usize> From<&'static str> for Farm<N> {
    fn from(value: &'static str) -> Self {
        Self {
            map: value.as_bytes(),
        }
    }
}

impl<const N: usize> Farm<N> {
    fn is_garden(&self, p: Point) -> bool {
        match self.get(p) {
            b'.' | b'S' => true,
            b'#' => false,
            _ => unreachable!(),
        }
    }

    const fn get(&self, p: Point) -> u8 {
        self.map[Self::idx(p)]
    }

    const fn idx(p: Point) -> usize {
        let y = p.y % N;
        let x = p.x % N;
        (N + 1) * y + x
    }
}

struct Visited<const N: usize> {
    data: [[bool; N]; N],
}

impl<const N: usize> Visited<N> {
    const fn new() -> Self {
        Self {
            data: [[false; N]; N],
        }
    }

    const fn contains(&self, p: Point) -> bool {
        self.data[p.y][p.x]
    }

    fn add(&mut self, p: Point) {
        self.data[p.y][p.x] = true;
    }

    fn count(&self) -> usize {
        self.data.iter().flatten().filter(|&&b| b).count()
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    const fn neighbours(self) -> [Self; 4] {
        [
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
        ]
    }
}

#[cfg(test)]
mod tests {}
