#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<6>(data));
    println!("Part 2: {}", part_two::<7>(data));
}

fn part_one<const N: usize>(data: &str) -> u32 {
    let mut s = Sculpture::<N>::new(data);
    s.solve()
}

fn part_two<const N: usize>(data: &str) -> u32 {
    let additional = data.to_owned() + "Disc #7 has 11 positions; at time=0, it is at position 0.";
    let mut s = Sculpture::<N>::new(&additional);
    s.solve()
}

#[derive(Debug)]
struct Sculpture<const N: usize> {
    discs: [Disc; N],
    step: u32,
    base: u32,
}

impl<const N: usize> Sculpture<N> {
    fn new(data: &str) -> Self {
        let mut lines = data.lines();
        let discs = std::array::from_fn(|_| Disc::new(lines.next().unwrap()));
        let base = discs
            .iter()
            .filter(|&d| d.aligned())
            .map(|d| d.modulo)
            .chain(std::iter::once(1))
            .product();
        Self {
            discs,
            step: 0,
            base,
        }
    }

    fn solve(&mut self) -> u32 {
        while self.discs.iter().any(|d| !d.aligned()) {
            self.solve_next();
        }
        self.step
    }

    fn solve_next(&mut self) {
        let mut solved = false;
        let mut rotation = 0;

        while !solved {
            rotation += self.base;
            for disc in self.discs.iter_mut().filter(|d| !d.aligned()) {
                disc.rotate(self.base);
                if disc.aligned() {
                    solved = true;
                }
            }
        }

        self.step += rotation;
        self.base = self
            .discs
            .iter()
            .filter(|&d| d.aligned())
            .map(|d| d.modulo)
            .product();
    }
}

#[derive(Debug, Clone, Copy)]
struct Disc {
    //id: u32,
    modulo: u32,
    position: u32,
}

impl Disc {
    fn new(line: &str) -> Self {
        let (id, rest) = line
            .trim_start_matches("Disc #")
            .split_once(" has ")
            .unwrap();
        let (modulo, rest) = rest.split_once(" positions;").unwrap();
        let (_, position) = rest.split_once(" position ").unwrap();

        let id = id.parse::<u32>().unwrap();
        let modulo = modulo.parse().unwrap();
        let position = position.trim_end_matches('.').parse::<u32>().unwrap();

        // Normalise position based on how far away from the drop it it
        let position = (position + id) % modulo;

        Self {
            //id,
            modulo,
            position,
        }
    }

    fn rotate(&mut self, steps: u32) {
        let mod_steps = steps % self.modulo;
        self.position = (self.position + mod_steps) % self.modulo;
    }

    const fn aligned(&self) -> bool {
        self.position == 0
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(5, part_one::<2>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(15, part_two::<3>(data));
    }
}
