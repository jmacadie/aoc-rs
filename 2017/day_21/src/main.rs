#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod grid2;
mod grid2x2;
mod grid3;
mod grid_map;

use crate::grid3::Grid3;
use crate::grid_map::GridMap;
use itertools::Itertools;

// The raw u16 for the starting `Grid3`
// . # .
// . . #
// # # #
const START: u16 = 0b0000_0000_0101_1110;

pub fn main() {
    let data = include_str!("input.txt");
    match GridMap::new(data) {
        Ok(g) => {
            println!("Part 1: {}", part_one(&g));
            println!("Part 2: {}", part_two(&g));
        }
        Err(e) => {
            println!("Cannot parse the input data: {e}");
        }
    }
}

fn part_one(grid_map: &GridMap) -> u32 {
    grid_map
        .get_3x3(START.into())
        .iter()
        .flat_map(|g| grid_map.get_2x3(*g))
        .map(Grid3::num_on)
        .sum()
}

fn part_two(grid_map: &GridMap) -> u32 {
    fn expand(data: &[(Grid3, u32)], grid_map: &GridMap) -> Vec<(Grid3, u32)> {
        data.iter()
            .flat_map(|(g, c)| grid_map.get_3x3(*g).into_iter().map(move |gi| (gi, c)))
            .sorted_unstable_by_key(|&(g, _)| g)
            .group_by(|&(g, _)| g)
            .into_iter()
            .map(|(grid, found)| (grid, found.map(|(_, c)| c).sum()))
            .collect()
    }

    let mut grids = vec![(START.into(), 1)];
    for _ in 0..6 {
        grids = expand(&grids, grid_map);
    }

    grids.into_iter().map(|(g, c)| c * g.num_on()).sum()
}
