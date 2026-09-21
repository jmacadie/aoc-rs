#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(dead_code)]

use std::fmt::Display;

pub fn main() {
    let data = include_str!("input.txt");
    let g = Graph::<590>::new(data);
    println!("Part 1: {}", part_one::<590>(&g));
    println!("Part 2: {}", part_two::<590>(&g));
}

fn part_one<const N: usize>(g: &Graph<N>) -> usize {
    g.paths(0, "you").0
}

fn part_two<const N: usize>(g: &Graph<N>) -> usize {
    let (dac, i) = g.paths(0, "dac");
    let (fft, i) = g.paths(i, "fft");
    let (svr, _) = g.paths(i, "svr");
    dac * fft * svr
}

#[derive(Debug)]
struct Graph<const N: usize> {
    data: [Node; N],
}

impl<const N: usize> Display for Graph<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for node in self.data {
            writeln!(f, "{node}")?;
        }
        Ok(())
    }
}

impl<const N: usize> Graph<N> {
    fn new(input: &'static str) -> Self {
        let mut data = [Node::empty(); N];
        let mut added = [false; N];
        data[0].set_name("out");
        let mut size = 1;
        loop {
            let mut temp_size = size;
            for (i, l) in input.lines().enumerate() {
                if added[i] {
                    continue;
                }
                let (root, children) = l.split_once(": ").unwrap();
                if let Some((children, len)) =
                    Self::resolve_children(&data[..size], children.split_whitespace())
                {
                    data[temp_size].set_name(root);
                    data[temp_size].set_children(children, len);
                    temp_size += 1;
                    added[i] = true;
                }
            }
            if temp_size == N {
                break;
            }
            size = temp_size;
        }
        Self { data }
    }

    fn resolve_children(
        data: &[Node],
        test: impl Iterator<Item = &'static str>,
    ) -> Option<([usize; 25], usize)> {
        let mut children_idx = [0; 25];
        let mut len = 0;
        for child_str in test {
            let (i, _) = data
                .iter()
                .enumerate()
                .rev()
                .find(|&(_, n)| n.name == child_str)?;
            children_idx[len] = i;
            len += 1;
        }
        Some((children_idx, len))
    }

    fn paths(&self, from: usize, target: &str) -> (usize, usize) {
        let mut paths = [0; N];
        paths[from] = 1;
        for (i, node) in self.data.iter().enumerate().skip(from + 1) {
            let path_count = node.iter().map(|idx| paths[idx]).sum();
            if node.name == target {
                return (path_count, i);
            }
            paths[i] = path_count;
        }
        unreachable!()
    }
}

#[derive(Copy, Clone, Debug)]
struct Node {
    name: &'static str,
    children: [usize; 25],
    len: usize,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {:?}", self.name, &self.children[..self.len])?;
        Ok(())
    }
}

impl Node {
    const fn empty() -> Self {
        Self {
            name: "",
            children: [0; 25],
            len: 0,
        }
    }

    const fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    const fn set_children(&mut self, children: [usize; 25], len: usize) {
        self.children = children;
        self.len = len;
    }

    fn iter(&self) -> impl Iterator<Item = usize> {
        self.children.iter().take(self.len).copied()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test_1.txt");
        let g = Graph::<10>::new(data);
        assert_eq!(5, part_one::<10>(&g));
    }

    #[test]
    fn two() {
        let data = include_str!("test_2.txt");
        let g = Graph::<14>::new(data);
        assert_eq!(2, part_two::<14>(&g));
    }
}
