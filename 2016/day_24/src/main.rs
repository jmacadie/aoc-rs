#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use itertools::Itertools;
use std::thread;

pub fn main() {
    const N: usize = 8;
    let data = include_str!("input.txt");
    let w = Weights::<N>::new::<184>(data);
    println!("Part 1: {}", part_one::<N>(&w));
    println!("Part 2: {}", part_two::<N>(&w));
}

fn part_one<const N: usize>(weights: &Weights<N>) -> u32 {
    find_min_path(weights, 0, 1, 0, u32::MAX, false)
}

fn part_two<const N: usize>(weights: &Weights<N>) -> u32 {
    find_min_path(weights, 0, 1, 0, u32::MAX, true)
}

fn find_min_path<const N: usize>(
    weights: &Weights<N>,
    current_node: usize,
    visited: u8,
    current_distance: u32,
    best_distance: u32,
    go_home: bool,
) -> u32 {
    let mut best = best_distance;
    for &(next_node, next_distance) in weights.get_remaining_from(current_node, visited) {
        let distance = current_distance + next_distance;
        // Best can't be beaten, return it straight away
        if distance > best {
            return best;
        }
        // On last node
        if visited.count_ones() == (N - 1).try_into().unwrap() {
            // If we don't need to go back home, we're done
            // We're on the last node & it's better than the best found to date
            if !go_home {
                return distance;
            }
            // We do need to go back home
            // Add that in and test if we beat best
            let home_distance = weights.get_home_distance(next_node);
            if distance + home_distance < best {
                return distance + home_distance;
            }
            // Doesn't beat the best
            continue;
        }
        // Add next_node to visted
        let next_visited = visited | (1 << next_node);
        // This next node can't beat the best distance
        // ... but another node might, so continue onto the next one
        if distance + min_poss_remaining(weights, next_node, next_visited, go_home) > best {
            continue;
        }
        best = find_min_path(weights, next_node, next_visited, distance, best, go_home);
    }
    best
}

fn min_poss_remaining<const N: usize>(
    weights: &Weights<N>,
    current_node: usize,
    visited: u8,
    go_home: bool,
) -> u32 {
    // Get the minimum hop from current node
    let &(next_node, next_distance) = weights
        .get_remaining_from(current_node, visited)
        .next()
        .unwrap();
    // Find the x min remaining weights
    // this may well not be possible but it doesn't matter for a lower bound
    let next_visited = visited | 1 << next_node;
    let min_remaining = weights.get_min_hops(next_visited);
    // Include distance home too
    let home_distance = if go_home {
        weights.get_min_home(visited)
    } else {
        0
    };
    // Return the sum
    next_distance + min_remaining + home_distance
}

#[derive(Debug)]
struct Weights<const N: usize>([[(usize, u32); N]; N]);

impl<const N: usize> Weights<N> {
    fn new<const C: usize>(data: &'static str) -> Self {
        let data = data.as_bytes();

        let mut output = [[(0, 0); N]; N];
        let mut handles = Vec::with_capacity(N);

        for i in 0..N {
            let handle = thread::spawn(move || {
                let start = b'0' + u8::try_from(i).unwrap();
                let mut row = Self::find_weights_from::<C>(data, start);
                row.sort_unstable_by_key(|&(_, d)| d);
                row
            });

            handles.push(handle);
        }

        for (handle, out) in handles.into_iter().zip(output.iter_mut()) {
            *out = handle.join().unwrap();
        }

        Self(output)
    }

    const fn get_value<const C: usize>(data: &[u8], p: Point) -> u8 {
        let posn = p.0 * C + p.1;
        data[posn]
    }

    fn find_weights_from<const C: usize>(data: &[u8], start: u8) -> [(usize, u32); N] {
        let mut visited = Visited::<C>::new();
        let mut frontier = Frontier::new();
        let mut next = Frontier::new();
        let max_node = b'0' + u8::try_from(N).unwrap();
        let mut step = 0;
        let mut output = [(0, 0); N];
        let mut found = 0;

        let p = Self::get_start_location::<C>(data, start);
        frontier.push(p);

        while frontier.size > 0 {
            while let Some(p) = frontier.pop() {
                if visited.contains(p) {
                    continue;
                }
                match Self::get_value::<C>(data, p) {
                    b'#' => (), // Wall: do nothing
                    b'.' => {
                        // Empty space
                        visited.insert(p);
                        next.add_neighbours(p);
                    }
                    x if x >= b'0' && x <= max_node => {
                        // Found one of the nodes
                        let idx: usize = (x - b'0').into();
                        output[idx] = (idx, step);
                        found += 1;
                        if found == N {
                            return output;
                        }
                        visited.insert(p);
                        next.add_neighbours(p);
                    }
                    _ => unreachable!(),
                }
            }
            step += 1;
            frontier = next;
            next = Frontier::new();
        }

        output
    }

    fn get_remaining_from(
        &self,
        current_node: usize,
        visited: u8,
    ) -> impl Iterator<Item = &(usize, u32)> {
        self.0[current_node]
            .iter()
            .filter(move |&(i, _)| (visited >> i) & 1 == 0)
    }

    fn get_min_hops(&self, visited: u8) -> u32 {
        let remaining_nodes = N - usize::try_from(visited.count_ones()).unwrap();
        self.0
            .iter()
            .enumerate()
            .filter(move |(i, _)| (visited >> i) & 1 == 0)
            .flat_map(|(i, r)| {
                r.iter()
                    .filter(move |&(j, _)| (visited >> j) & 1 == 0 && j > &i)
                    .map(|(_, d)| d)
            })
            .sorted_unstable()
            .take(remaining_nodes)
            .sum()
    }

    fn get_home_distance(&self, node: usize) -> u32 {
        self.0[0]
            .iter()
            .find(|&&(n, _)| n == node)
            .unwrap_or_else(|| panic!("Expected to find {node}, but couldn't"))
            .1
    }

    fn get_min_home(&self, visited: u8) -> u32 {
        self.0[0]
            .iter()
            .filter(|&(i, _)| (visited >> i) & 1 == 0)
            .map(|&(_, d)| d)
            .min()
            .unwrap_or(0)
    }

    fn get_start_location<const C: usize>(data: &[u8], start: u8) -> Point {
        let raw_position = data.iter().position(|&ch| ch == start).unwrap_or_else(|| {
            panic!(
                "Expected to find {} in the map but couldn't",
                char::from_u32(start.into()).unwrap()
            )
        });
        let row = raw_position / C;
        let col = raw_position % C;
        Point(row, col)
    }
}

// Specialised (& noddy) vector for this problem
// Designed to stay on the stack
#[derive(Debug)]
struct Frontier {
    data: [Point; 200],
    size: usize,
}

impl Frontier {
    const fn new() -> Self {
        Self {
            data: [Point(0, 0); 200],
            size: 0,
        }
    }

    fn add_neighbours(&mut self, p: Point) {
        // Since input is bounded by walls, don't need to do bounds checks
        self.push(Point(p.0 - 1, p.1));
        self.push(Point(p.0, p.1 - 1));
        self.push(Point(p.0 + 1, p.1));
        self.push(Point(p.0, p.1 + 1));
    }

    fn push(&mut self, p: Point) {
        *self.data.get_mut(self.size).unwrap() = p;
        self.size += 1;
    }

    fn pop(&mut self) -> Option<Point> {
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        Some(self.data[self.size])
    }
}

// Specialised (& noddy) hash table for this problem
// Since the row count is less than 64, storing each column of the map as a single u64.
// Can use the bits of each number to flag whether the corresponding row has been visited.
// Designed to stay on the stack
#[derive(Debug)]
struct Visited<const C: usize> {
    data: [u64; C],
}

impl<const C: usize> Visited<C> {
    const fn new() -> Self {
        Self { data: [0; C] }
    }

    fn insert(&mut self, p: Point) {
        let new = 1 << p.0;
        let col_data = self
            .data
            .get_mut(p.1)
            .unwrap_or_else(|| panic!("Point {p:?} is outside the bounds of the map"));
        *col_data |= new;
    }

    fn contains(&self, p: Point) -> bool {
        let col_data = self
            .data
            .get(p.1)
            .unwrap_or_else(|| panic!("Point {p:?} is outside the bounds of the map"));
        (*col_data >> p.0) & 1 == 1
    }
}

/// Tuple: (row, column)
#[derive(Debug, Clone, Copy)]
struct Point(usize, usize);

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        const N: usize = 5;
        let data = include_str!("test.txt");
        let w = Weights::<N>::new::<12>(data);
        assert_eq!(14, part_one::<N>(&w));
    }

    #[test]
    fn two() {
        const N: usize = 5;
        let data = include_str!("test.txt");
        let w = Weights::<N>::new::<12>(data);
        assert_eq!(20, part_two::<5>(&w));
    }
}
