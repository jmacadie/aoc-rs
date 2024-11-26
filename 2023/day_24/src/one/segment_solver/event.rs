use std::cmp::Ordering;

use super::super::point::Point;
use super::SegID;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Start(Point, SegID),
    End(Point, SegID),
    Intersection(Point, SegID, SegID),
}

impl Event {
    const fn loc(&self) -> Point {
        match self {
            Self::Start(p, _) | Self::End(p, _) | Self::Intersection(p, _, _) => *p,
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.loc();
        let b = other.loc();
        match b.x.partial_cmp(&a.x) {
            Some(Ordering::Less) => Ordering::Less,
            Some(Ordering::Greater) => Ordering::Greater,
            Some(Ordering::Equal) => {
                b.y.partial_cmp(&a.y)
                    .expect("co-ordinates with comparable values")
            }
            None => unreachable!(),
        }
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Event {}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.loc() == other.loc()
    }
}
