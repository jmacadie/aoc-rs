#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one::<8, 184>(data));
    println!("Part 2: {}", part_two(data));
}

fn part_one<const N: usize, const C: usize>(data: &str) -> u32 {
    let w = Weights::<N>::new::<C>(data);
    println!("{w:?}");
    find_min_path(&w, 0, 1, 0, u32::MAX)
}

const fn part_two(_data: &str) -> usize {
    0
}

fn find_min_path<const N: usize>(
    weights: &Weights<N>,
    current_node: usize,
    visited: u8,
    current_distance: u32,
    best_distance: u32,
) -> u32 {
    let mut best = best_distance;
    for nd in weights.0[current_node]
        .iter()
        .filter(|&nd| (visited >> nd.0 .0) & 1 == 0)
    {
        let (next_node, next_distance) = nd.0;
        let distance = current_distance + next_distance;
        // best can't be beaten, return it straight away
        if distance > best {
            return best;
        }
        // on last node & better than best so return found distance
        if visited.count_ones() == (N - 1).try_into().unwrap() {
            return distance;
        }
        //     If distance + next_distance
        //        + min_distance_from_next_node
        //        + x min_distances_left_in_weights (x remaining nodes to visit)
        //        > best then continue
        let next_visited = visited | (1 << next_node);
        // This node can't beat the best distance
        // ... but another node might, so continue onto the next one
        /*if distance + 1 > best {
            continue;
        }*/
        best = find_min_path(weights, next_node, next_visited, distance, best);
    }
    best
}

#[derive(Debug, Clone, Copy)]
struct NodeDistance((usize, u32));

impl NodeDistance {
    const fn new() -> Self {
        Self((0, 0))
    }
}

impl PartialEq for NodeDistance {
    fn eq(&self, other: &Self) -> bool {
        self.0 .1 == other.0 .1
    }
}
impl Eq for NodeDistance {}

impl Ord for NodeDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0 .1.cmp(&other.0 .1)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Weights<const N: usize>([[NodeDistance; N]; N]);

impl<const N: usize> Weights<N> {
    fn new<const C: usize>(data: &str) -> Self {
        let data = data.as_bytes();
        // TODO: This could totally be parallelised
        let mut output = std::array::from_fn(|i| {
            let start = b'0' + u8::try_from(i).unwrap();
            let mut row = Self::find_weights_from::<C>(data, start);
            row.sort();
            row
        });
        output.sort();
        Self(output)
    }

    const fn get_value<const C: usize>(data: &[u8], p: Point) -> u8 {
        let posn = p.0 * C + p.1;
        data[posn]
    }

    fn find_weights_from<const C: usize>(data: &[u8], start: u8) -> [NodeDistance; N] {
        let mut visited = Visited::<C>::new();
        let mut frontier = Frontier::new();
        let mut next = Frontier::new();
        let max_node = b'0' + u8::try_from(N).unwrap();
        let mut step = 0;
        let mut output = [NodeDistance::new(); N];

        let p = Self::get_start_location::<C>(data, start);
        frontier.push(p);

        while frontier.size > 0 {
            while let Some(p) = frontier.pop() {
                match Self::get_value::<C>(data, p) {
                    b'#' => (), // Wall: do nothing
                    b'.' => {
                        // Empty space
                        if !visited.contains(p) {
                            visited.insert(p);
                            next.add_neighbours(p);
                        }
                    }
                    x if x >= b'0' && x <= max_node => {
                        // Found one of the other nodes
                        if !visited.contains(p) {
                            visited.insert(p);
                            next.add_neighbours(p);
                            let idx: usize = (x - b'0').into();
                            output[idx] = NodeDistance((idx, step));
                        }
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
    data: [Point; 500],
    size: usize,
}

impl Frontier {
    const fn new() -> Self {
        Self {
            data: [Point(0, 0); 500],
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
        let data = include_str!("test.txt");
        assert_eq!(0, part_one::<5, 12>(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test.txt");
        assert_eq!(0, part_two(data));
    }
}
