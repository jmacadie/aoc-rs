#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use program::Program;

const MAX_CHILDREN: usize = 7;

pub fn main() {
    let data = include_str!("input.txt");
    let programs = Programs::<'_, 1124>::from_input(data);
    println!("Part 1: {}", part_one::<1124>(&programs));
    println!("Part 2: {}", part_two::<1124>(&programs));
}

fn part_one<'inp, const N: usize>(p: &Programs<'inp, N>) -> &'inp str {
    p.root
}

fn part_two<const N: usize>(p: &Programs<'_, N>) -> u16 {
    if let Err((node, bad, correct)) = find_weights(p, p.root) {
        let idx = p.data.binary_search(&node.into()).unwrap();
        let p = p.data.get(idx).unwrap();
        return p.weight + correct - bad;
    }
    0
}

fn find_weights<'inp, const N: usize>(
    programs: &Programs<'inp, N>,
    root: Name<'inp>,
) -> Result<u16, (Name<'inp>, u16, u16)> {
    let mut children = [("", 0); MAX_CHILDREN];
    let idx = programs.data.binary_search(&root.into()).unwrap();
    let p = programs.data.get(idx).unwrap();
    let mut weight = p.weight;

    for (child, weights) in p.children().zip(children.iter_mut()) {
        let c_weight = find_weights(programs, child)?;
        weight += c_weight;
        *weights = (child, c_weight);
    }

    if let Some(((bad_program, bad_weight), correct_weight)) = get_bad_weight(children) {
        return Err((bad_program, bad_weight, correct_weight));
    }

    Ok(weight)
}

fn get_bad_weight(children: [(&str, u16); MAX_CHILDREN]) -> Option<((&str, u16), u16)> {
    // There's no bad number if there are no children
    // Quit straight out
    if children[0].1 == 0 {
        return None;
    }
    let a = children[0];
    let mut b = (Name::default(), 0);
    let mut confirmed = false;
    for &child in children.iter().skip(1).filter(|&(_, v)| *v > 0) {
        if child.1 == a.1 {
            if b.1 > 0 {
                return Some((b, a.1));
            }
            confirmed = true;
        } else {
            if confirmed {
                return Some((child, a.1));
            }
            if b.1 > 0 {
                return Some((a, b.1));
            }
            b = child;
        }
    }

    // No mismatches found, therefore no bad weights to report
    None
}

type Name<'inp> = &'inp str;

#[derive(Debug)]
struct Programs<'inp, const N: usize> {
    data: [Program<'inp>; N],
    root: Name<'inp>,
}

impl<'inp, const N: usize> Programs<'inp, N> {
    fn from_input(input: &'inp str) -> Self {
        let mut l = input.lines().map(Program::from_line);
        let mut data = std::array::from_fn(|_| l.next().unwrap());
        data.sort_unstable();
        let root = Self::find_root(&data);
        Self { data, root }
    }

    fn find_root(data: &[Program<'inp>]) -> Name<'inp> {
        let mut candidates: Vec<&str> = Vec::with_capacity(N);
        for p in data.iter().filter(|&p| p.has_children()) {
            for c in p.children() {
                if let Ok(pos) = candidates.binary_search(&c) {
                    candidates.remove(pos);
                }
            }
            candidates.push(p.name);
        }
        for p in data.iter().filter(|&p| p.has_children()) {
            for c in p.children() {
                if let Ok(pos) = candidates.binary_search(&c) {
                    candidates.remove(pos);
                }
            }
        }
        candidates[0]
    }
}

mod program {

    use crate::Name;
    use crate::MAX_CHILDREN;

    #[derive(Debug, Clone, Copy)]
    struct ProgramChildren<'inp> {
        data: [Name<'inp>; MAX_CHILDREN],
        count: usize,
    }

    impl<'inp> ProgramChildren<'inp> {
        const fn new() -> Self {
            Self {
                data: [""; MAX_CHILDREN],
                count: 0,
            }
        }

        pub fn from_str(line: &'inp str) -> Self {
            let mut data = [""; MAX_CHILDREN];
            let mut count = 0;
            for (child, data) in line.split(", ").zip(data.iter_mut()) {
                *data = child;
                count += 1;
            }
            Self { data, count }
        }

        pub const fn len(&self) -> usize {
            self.count
        }
    }

    pub struct ChildrenIter<'it, 'inp: 'it> {
        index: usize,
        data: &'it [Name<'inp>],
    }

    impl<'it, 'inp> Iterator for ChildrenIter<'it, 'inp> {
        type Item = Name<'inp>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.data.len() {
                return None;
            }
            self.index += 1;
            Some(self.data[self.index - 1])
        }
    }

    impl<'it, 'inp: 'it> ProgramChildren<'inp> {
        pub fn iter(&'it self) -> ChildrenIter<'it, 'inp> {
            ChildrenIter {
                index: 0,
                data: &self.data[..self.count],
            }
        }
    }

    impl<'it, 'inp> IntoIterator for &'it ProgramChildren<'inp> {
        type Item = Name<'inp>;
        type IntoIter = ChildrenIter<'it, 'inp>;

        fn into_iter(self) -> Self::IntoIter {
            self.iter()
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Program<'inp> {
        pub name: Name<'inp>,
        pub weight: u16,
        children: ProgramChildren<'inp>,
    }

    impl<'inp> Program<'inp> {
        pub fn from_line(line: &'inp str) -> Self {
            if let Some((main, children)) = line.split_once(" -> ") {
                let (name, weight) = Self::parse_main(main);
                let children = ProgramChildren::from_str(children);
                Self {
                    name,
                    weight,
                    children,
                }
            } else {
                let (name, weight) = Self::parse_main(line);
                let children = ProgramChildren::new();
                Self {
                    name,
                    weight,
                    children,
                }
            }
        }

        fn parse_main(main: &'inp str) -> (&'inp str, u16) {
            let Some((name, weight)) = main.split_once(' ') else {panic!("Badly formatted main section {main}")};
            let weight = weight
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse()
                .expect("Badly formatted weight: {weight}");
            (name, weight)
        }

        pub const fn has_children(&self) -> bool {
            self.children.len() > 0
        }
    }

    impl<'it, 'inp: 'it> Program<'inp> {
        pub fn children(&'it self) -> ChildrenIter<'it, 'inp> {
            self.children.iter()
        }
    }

    impl<'inp> Default for Program<'inp> {
        fn default() -> Self {
            Self {
                name: "",
                weight: 0,
                children: ProgramChildren::new(),
            }
        }
    }

    impl<'inp> PartialEq for Program<'inp> {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl<'inp> Eq for Program<'inp> {}

    impl<'inp> Ord for Program<'inp> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.name.cmp(other.name)
        }
    }

    impl<'inp> PartialOrd for Program<'inp> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<'inp> From<&'inp str> for Program<'inp> {
        fn from(value: &'inp str) -> Self {
            Self {
                name: value,
                weight: 0,
                children: ProgramChildren::new(),
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let p = Programs::<'_, 13>::from_input(data);
        assert_eq!("tknk", part_one::<13>(&p));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let p = Programs::<'_, 13>::from_input(data);
        assert_eq!(60, part_two::<13>(&p));
    }
}
