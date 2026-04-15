#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use std::{cmp::Reverse, collections::BinaryHeap, fmt::Display, str::FromStr};

pub fn main() {
    let data = include_str!("input.txt");
    let (points, mut edges, mut find) = prep::<1000>(data);

    println!("Part 1: {}", part_one::<1000>(1000, &mut edges, &mut find));
    println!("Part 2: {}", part_two::<1000>(&points, edges, find));
}

#[derive(Default, Debug, Clone, Copy)]
struct Point3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "({}, {}, {})", self.x, self.y, self.z);
        Ok(())
    }
}

impl FromStr for Point3 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let (Some(x), Some(y), Some(z), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(format!("Could not parse {s} into three parts"));
        };
        let x = x
            .parse::<usize>()
            .map_err(|_| format!("Could not parse {x} into a number"))?;
        let y = y
            .parse::<usize>()
            .map_err(|_| format!("Could not parse {y} into a number"))?;
        let z = z
            .parse::<usize>()
            .map_err(|_| format!("Could not parse {z} into a number"))?;
        Ok(Self { x, y, z })
    }
}

impl Point3 {
    fn dist_sq(&self, other: &Self) -> usize {
        let dx = match self.x.cmp(&other.x) {
            std::cmp::Ordering::Less => other.x - self.x,
            std::cmp::Ordering::Greater => self.x - other.x,
            std::cmp::Ordering::Equal => 0,
        };
        let dy = match self.y.cmp(&other.y) {
            std::cmp::Ordering::Less => other.y - self.y,
            std::cmp::Ordering::Greater => self.y - other.y,
            std::cmp::Ordering::Equal => 0,
        };
        let dz = match self.z.cmp(&other.z) {
            std::cmp::Ordering::Less => other.z - self.z,
            std::cmp::Ordering::Greater => self.z - other.z,
            std::cmp::Ordering::Equal => 0,
        };
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct HeapElement {
    dist: usize,
    point_1: usize,
    point_2: usize,
}

impl HeapElement {
    const fn new(dist: usize, point_1: usize, point_2: usize) -> Self {
        Self {
            dist,
            point_1,
            point_2,
        }
    }

    const fn point_pair(&self) -> (usize, usize) {
        (self.point_1, self.point_2)
    }
}

impl Ord for HeapElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Flip ordering so we get a min heap
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for HeapElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct UnionFind<const N: usize> {
    root_pointers: [usize; N],
    size: [usize; N],
    disjoint_sets: usize,
}

impl<const N: usize> UnionFind<N> {
    fn new() -> Self {
        let root_pointers: [usize; N] = std::array::from_fn(|i| i);
        Self {
            root_pointers,
            size: [1; N],
            disjoint_sets: N,
        }
    }

    const fn find_root(&mut self, x: usize) -> usize {
        let mut current = x;
        // Pointer chasing loop
        // When an index points at itself, it is a root
        while current != self.root_pointers[current] {
            // While we are here ...
            // move current pointer to grandparent to improve pointer compaction
            self.root_pointers[current] = self.root_pointers[self.root_pointers[current]];
            current = self.root_pointers[current];
        }
        current
    }

    const fn union(&mut self, a: usize, b: usize) -> bool {
        let root_a = self.find_root(a);
        let root_b = self.find_root(b);

        if root_a == root_b {
            return false;
        }

        // Sort the trees by size
        let (small, big) = if self.size[root_a] < self.size[root_b] {
            (root_a, root_b)
        } else {
            (root_b, root_a)
        };

        // Add the smaller tree to the bigger
        self.root_pointers[small] = big;
        self.size[big] += self.size[small];
        self.disjoint_sets -= 1;

        true
    }

    fn root_sizes(&self) -> Vec<usize> {
        self.root_pointers
            .iter()
            .enumerate()
            .filter(|&(i, &x)| x == i)
            .map(|(_, &x)| self.size[x])
            .collect()
    }
}

fn prep<const N: usize>(data: &str) -> ([Point3; N], BinaryHeap<HeapElement>, UnionFind<N>) {
    let mut points = [Point3::default(); N];
    for (i, p) in data
        .lines()
        .map(|l| l.parse::<Point3>().unwrap())
        .enumerate()
    {
        points[i] = p;
    }

    let mut edges = BinaryHeap::with_capacity(N * (N - 1) / 2);

    for i in 0..N {
        for j in i + 1..N {
            let elem = HeapElement::new(points[i].dist_sq(&points[j]), i, j);
            edges.push(elem);
        }
    }

    let find = UnionFind::<N>::new();
    (points, edges, find)
}

fn part_one<const N: usize>(
    count: u32,
    edges: &mut BinaryHeap<HeapElement>,
    find: &mut UnionFind<N>,
) -> usize {
    for _ in 0..count {
        if let Some(e) = edges.pop() {
            let (a, b) = e.point_pair();
            find.union(a, b);
        }
    }

    let mut sizes = find.root_sizes();
    sizes.sort_unstable_by_key(|&x| Reverse(x));
    sizes.iter().take(3).product()
}

fn part_two<const N: usize>(
    points: &[Point3],
    mut edges: BinaryHeap<HeapElement>,
    mut find: UnionFind<N>,
) -> usize {
    while find.disjoint_sets > 1 {
        if let Some(e) = edges.pop() {
            let (a, b) = e.point_pair();
            if find.union(a, b) && find.disjoint_sets == 1 {
                return points[a].x * points[b].x;
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        let (_, mut edges, mut find) = prep::<20>(data);
        assert_eq!(40, part_one::<20>(10, &mut edges, &mut find));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        let (points, edges, find) = prep::<20>(data);
        assert_eq!(25272, part_two::<20>(&points, edges, find));
    }
}
