#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

use crate::grid2::Grid2;
use crate::grid2x2::Grid2x2;
use crate::grid3::Grid3;

#[derive(Debug)]
pub struct GridMap {
    g2_g3: [Grid3; 16],
    g3_g2x2: [Grid2x2; 512],    // 1 iteration
    g3_g2x3: [[Grid3; 4]; 512], // 2 iterations
    g3_g3x3: [[Grid3; 9]; 512], // 3 iterations
}

impl GridMap {
    pub fn new(data: &str) -> Result<Self, String> {
        let mut g2_g3 = [Grid3::default(); 16];
        let mut g3_g2x2 = [Grid2x2::default(); 512];

        // Read in the explicit written data first
        for line in data.lines() {
            let (from, to) = line.split_once(" => ").ok_or_else(|| {
                format!("Expect to be able to split the input rule with ' => ': {line}")
            })?;
            if from.len() == 5 {
                // 2x2 to 3x3 map
                let from: Grid2 = from.parse()?;
                let to: Grid3 = to.parse()?;

                for g in from.equivalent_set() {
                    g2_g3[g.to_index()] = to;
                }
            } else {
                // 3x3 to 4x4 map
                let from: Grid3 = from.parse()?;
                let to: Grid2x2 = to.parse()?;

                for g in from.equivalent_set() {
                    g3_g2x2[g.to_index()] = to;
                }
            }
        }

        // Now add the implied mappings for the next two steps
        let g3_g2x3 = [[Grid3::default(); 4]; 512];
        let g3_g3x3 = [[Grid3::default(); 9]; 512];

        let mut output = Self {
            g2_g3,
            g3_g2x2,
            g3_g2x3,
            g3_g3x3,
        };

        output.gen_2x3();
        output.gen_3x3();

        Ok(output)
    }

    pub fn get_2x3(&self, source: Grid3) -> [Grid3; 4] {
        self.g3_g2x3[source.to_index()]
    }

    pub fn get_3x3(&self, source: Grid3) -> [Grid3; 9] {
        self.g3_g3x3[source.to_index()]
    }

    fn gen_2x3(&mut self) {
        for (next, prev) in self.g3_g2x3.iter_mut().zip(self.g3_g2x2.iter()) {
            for (inner_next, inner_prev) in next.iter_mut().zip(prev.data.iter()) {
                *inner_next = self.g2_g3[inner_prev.to_index()];
            }
        }
    }

    fn gen_3x3(&mut self) {
        for (next, prev) in self.g3_g3x3.iter_mut().zip(self.g3_g2x3.iter()) {
            let prev_3x2 = Self::gen_3x2(*prev);
            for (inner_next, inner_prev) in next.iter_mut().zip(prev_3x2.iter()) {
                *inner_next = self.g2_g3[inner_prev.to_index()];
            }
        }
    }

    // Same bit set but sliced up into 9 grids of 2x2 rather than 4 grids of 3x3
    // There's no other way round (that I can think of) than to individually allocate the source
    // bits to the target bit locations.
    // Sometimes we get a quick win where we can allocate runs of bits
    fn gen_3x2(g2x3: [Grid3; 4]) -> [Grid2; 9] {
        let mut next = [Grid2::default(); 9];
        let mut ring: u16;

        // Top left
        ring = g2x3[0].data >> 4 & 0x000c; // 8, 7 -> 4, 3
        ring |= g2x3[0].data >> 7 & 0x0002; // 9 -> 2
        ring |= g2x3[0].data & 0x0001; // 1 -> 1
        next[0] = ring.into();

        // Top middle
        ring = g2x3[0].data >> 2 & 0x0008; // 6 -> 4
        ring |= g2x3[1].data >> 5 & 0x0004; // 8 -> 3
        ring |= g2x3[1].data << 1 & 0x0002; // 1 -> 2
        ring |= g2x3[0].data >> 4 & 0x0001; // 5 -> 1
        next[1] = ring.into();

        // Top right
        ring = g2x3[1].data >> 3 & 0x000e; // 7, 6, 5 -> 4, 3, 2
        ring |= g2x3[1].data >> 8 & 0x0001; // 9 -> 1
        next[2] = ring.into();

        // Middle left
        ring = g2x3[0].data << 2 & 0x0008; // 2 -> 4
        ring |= g2x3[0].data & 0x0004; // 3 -> 3
        ring |= g2x3[2].data >> 5 & 0x0002; // 7 -> 2
        ring |= g2x3[2].data >> 7 & 0x0001; // 8 -> 1
        next[3] = ring.into();

        // Middle middle
        ring = g2x3[0].data & 0x0008; // 4 -> 4
        ring |= g2x3[1].data << 1 & 0x0004; // 2 -> 3
        ring |= g2x3[3].data >> 6 & 0x0002; // 8 -> 2
        ring |= g2x3[2].data >> 5 & 0x0001; // 6 -> 1
        next[4] = ring.into();

        // Middle right
        ring = g2x3[1].data << 1 & 0x0008; // 3 -> 4
        ring |= g2x3[1].data >> 1 & 0x0004; // 4 -> 3
        ring |= g2x3[3].data >> 4 & 0x0002; // 6 -> 2
        ring |= g2x3[3].data >> 6 & 0x0001; // 7 -> 1
        next[5] = ring.into();

        // Bottom left
        ring = g2x3[2].data << 3 & 0x0008; // 1 -> 4
        ring |= g2x3[2].data >> 6 & 0x0004; // 9 -> 3
        ring |= g2x3[2].data >> 1 & 0x0003; // 3, 2 -> 2, 1
        next[6] = ring.into();

        // Bottom middle
        ring = g2x3[2].data >> 1 & 0x0008; // 5 -> 4
        ring |= g2x3[3].data << 2 & 0x0004; // 1 -> 3
        ring |= g2x3[3].data & 0x0002; // 2 -> 2
        ring |= g2x3[2].data >> 3 & 0x0001; // 4 -> 1
        next[7] = ring.into();

        // Bottom right
        ring = g2x3[3].data >> 5 & 0x0008; // 9 -> 4
        ring |= g2x3[3].data >> 2 & 0x0007; // 5, 4, 3 -> 3, 2, 1
        next[8] = ring.into();

        next
    }
}
