use super::{LineSegment, Point};
use super::{SegID, SortID};

pub struct Segments<const N: usize> {
    all: [(Option<LineSegment>, Option<SortID>); N],
    active: [SegID; N],
    active_count: usize,
}

impl<const N: usize> Segments<N> {
    #[must_use]
    pub const fn new(
        all: [(Option<LineSegment>, Option<SortID>); N],
        active: [SegID; N],
        active_count: usize,
    ) -> Self {
        Self {
            all,
            active,
            active_count,
        }
    }

    #[must_use]
    pub const fn get_active_idx(&self, idx: usize) -> SegID {
        self.active[idx]
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const fn get(&self, segment: SegID) -> &LineSegment {
        self.all[segment].0.as_ref().unwrap()
    }

    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub const fn get_pos(&self, segment: SegID) -> usize {
        self.all[segment]
            .1
            .expect("The segment has a position in the active array")
    }

    #[must_use]
    pub const fn get_prev_id(&self, sorted_idx: SortID) -> Option<SegID> {
        if sorted_idx == 0 {
            return None;
        }
        Some(self.active[sorted_idx - 1])
    }

    #[must_use]
    pub const fn get_next_id(&self, sorted_idx: SortID) -> Option<SegID> {
        if (sorted_idx + 1) >= self.active_count {
            return None;
        }
        Some(self.active[sorted_idx + 1])
    }

    fn find(&self, p: Point) -> SortID {
        fn find_inner(
            p: Point,
            search: &[SegID],
            segments: &[(Option<LineSegment>, Option<SortID>)],
            start_index: SortID,
        ) -> SortID {
            let mid_idx = search.len() / 2;
            let seg_id = search[mid_idx];
            let mid_value = segments[seg_id]
                .0
                .as_ref()
                .unwrap()
                .point_at_x(p.x)
                .unwrap()
                .y;

            if search.len() == 1 {
                if p.y > mid_value {
                    return start_index + 1;
                }
                return start_index;
            }
            if search.len() == 2 && p.y > mid_value {
                return start_index + 2;
            }

            if p.y > mid_value {
                let idx = mid_idx + 1;
                find_inner(p, &search[idx..], segments, start_index + idx)
            } else {
                let idx = mid_idx;
                find_inner(p, &search[..idx], segments, start_index)
            }
        }

        if self.active_count == 0 {
            return 0;
        }

        find_inner(p, &self.active[..self.active_count], &self.all, 0)
    }

    pub fn add(&mut self, segment: SegID, p: Point) -> SortID {
        let position = self.find(p);
        (position..self.active_count).rev().for_each(|i| {
            self.increment(i);
            self.active[i + 1] = self.active[i];
        });
        self.all[segment].1 = Some(position);
        self.active[position] = segment;
        self.active_count += 1;
        position
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn del(&mut self, position: SortID) {
        assert!(self.all[self.active[position]].1.is_some());
        self.all[self.active[position]].1 = None;
        (position..self.active_count).for_each(|i| {
            if i > position {
                self.decrement(i);
            }
            self.active[i] = self.active[i + 1];
        });
        self.active_count -= 1;
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn swap(&mut self, lower: SortID, upper: SortID) {
        assert_eq!(lower + 1, upper); // Should be true?
        self.increment(lower);
        self.decrement(upper);
        self.active.swap(lower, upper);
    }

    fn increment(&mut self, location: SortID) {
        if let Some(x) = self.all[self.active[location]].1.as_mut() {
            *x += 1;
        } else {
            unreachable!();
        }
    }

    fn decrement(&mut self, location: SortID) {
        if let Some(x) = self.all[self.active[location]].1.as_mut() {
            *x -= 1;
        } else {
            unreachable!();
        }
    }
}
