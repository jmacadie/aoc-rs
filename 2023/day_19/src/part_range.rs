use crate::part::Part;
use crate::workflows::{Bound, ConditionTest, PartCategory};

#[derive(Debug, Clone)]
pub struct PartRange {
    ranges: [Range; 4],
}

impl PartRange {
    pub const fn new() -> Self {
        Self {
            ranges: [Range::new(); 4],
        }
    }

    pub fn apply(&self, test: ConditionTest) -> (Option<Self>, Option<Self>) {
        let idx = match test.category {
            PartCategory::ExtremelyCool => 0,
            PartCategory::Musical => 1,
            PartCategory::Aerodynamical => 2,
            PartCategory::Shiny => 3,
        };
        let if_true;
        let if_false;
        match test.bound {
            Bound::Less => {
                if_true = self.ranges[idx].add_to(test.val);
                if_false = self.ranges[idx].add_from(test.val - 1);
            }
            Bound::Greater => {
                if_true = self.ranges[idx].add_from(test.val);
                if_false = self.ranges[idx].add_to(test.val + 1);
            }
        }
        let if_true = if_true.map(|r| {
            let mut out = self.clone();
            out.ranges[idx] = r;
            out
        });
        let if_false = if_false.map(|r| {
            let mut out = self.clone();
            out.ranges[idx] = r;
            out
        });
        (if_true, if_false)
    }

    pub fn within(&self, part: Part) -> bool {
        self.ranges
            .iter()
            .zip(part.iter())
            .all(|(&r, p)| r.within(p))
    }

    pub fn combinations(&self) -> u64 {
        self.ranges.into_iter().map(Range::size).product()
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    from: Option<u32>,
    to: Option<u32>,
}

impl Range {
    const fn new() -> Self {
        Self {
            from: None,
            to: None,
        }
    }

    fn add_from(self, new: u32) -> Option<Self> {
        if self.to.is_some_and(|to| to <= new + 1) {
            return None;
        }
        let from = self.from.map_or(new, |prev| std::cmp::max(prev, new));
        Some(Self {
            from: Some(from),
            to: self.to,
        })
    }

    fn add_to(self, new: u32) -> Option<Self> {
        if self.from.is_some_and(|from| new <= from + 1) {
            return None;
        }
        let to = self.to.map_or(new, |prev| std::cmp::min(prev, new));
        Some(Self {
            from: self.from,
            to: Some(to),
        })
    }

    fn within(self, val: u32) -> bool {
        (self.from.is_some_and(|x| val > x) || self.from.is_none())
            && (self.to.is_some_and(|x| val < x) || self.to.is_none())
    }

    fn size(self) -> u64 {
        (self.to.unwrap_or(4001) - self.from.unwrap_or(0) - 1).into()
    }
}
