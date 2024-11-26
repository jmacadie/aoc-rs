use super::{LineSegment, Point, event::Event, events::Events, segments::Segments};

pub struct Solver<const N: usize> {
    events: Events,
    segments: Segments<N>,
    intersections: usize,
}

impl<const N: usize> Solver<N> {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn new(data: &[Option<LineSegment>; N]) -> Self {
        let mut events = Events::default();
        data.iter()
            .enumerate()
            .filter(|(_, segment)| segment.is_some())
            .map(|(seg_id, segment)| (seg_id, segment.as_ref().unwrap()))
            .for_each(|(seg_id, segment)| {
                events.push(Event::Start(segment.from, seg_id));
                events.push(Event::End(segment.to, seg_id));
            });
        let all = std::array::from_fn(|i| (data[i].clone(), None));
        let segments = Segments::<N>::new(all, [0; N], 0);
        Self {
            events,
            segments,
            intersections: 0,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn run(&mut self) -> usize {
        let mut last = Point::default();
        while let Some(e) = self.events.pop() {
            match e {
                Event::Start(p, seg_id) => {
                    let sorted_idx = self.segments.add(seg_id, p);
                    if let Some(next) = self.segments.get_next_id(sorted_idx) {
                        let next_ls = self.segments.get(next);
                        let ls = self.segments.get(seg_id);
                        if let Some(loc) = ls.intersect(next_ls) {
                            assert!(loc.x > p.x);
                            self.events.push(Event::Intersection(loc, seg_id, next));
                        }
                    }
                    if let Some(prev_id) = self.segments.get_prev_id(sorted_idx) {
                        let prev_ls = self.segments.get(prev_id);
                        let ls = self.segments.get(seg_id);
                        if let Some(loc) = ls.intersect(prev_ls) {
                            assert!(loc.x > p.x);
                            self.events.push(Event::Intersection(loc, prev_id, seg_id));
                        }
                    }
                    last = p;
                }
                Event::End(p, seg_id) => {
                    let sorted_idx = self.segments.get_pos(seg_id);
                    assert_eq!(self.segments.get_active_idx(sorted_idx), seg_id);
                    if let Some(next_id) = self.segments.get_next_id(sorted_idx)
                        && let Some(prev_id) = self.segments.get_prev_id(sorted_idx)
                    {
                        let next_ls = self.segments.get(next_id);
                        let prev_ls = self.segments.get(prev_id);
                        if let Some(loc) = next_ls.intersect(prev_ls)
                            && loc.x > p.x
                        {
                            self.events.push(Event::Intersection(loc, prev_id, next_id));
                        }
                    }
                    self.segments.del(sorted_idx);
                    last = p;
                }
                Event::Intersection(p, s1, s2) => {
                    self.intersections += 1;
                    let mut lower = self.segments.get_pos(s1);
                    let mut upper = self.segments.get_pos(s2);
                    let lower_seg;
                    let upper_seg = if lower > upper {
                        std::mem::swap(&mut lower, &mut upper);
                        lower_seg = s2;
                        s1
                    } else {
                        lower_seg = s1;
                        s2
                    };
                    assert_eq!(lower + 1, upper);
                    self.segments.swap(lower, upper);
                    if let Some(next_id) = self.segments.get_next_id(upper) {
                        let next_ls = self.segments.get(next_id);
                        let ls = self.segments.get(lower_seg);
                        if let Some(loc) = ls.intersect(next_ls)
                            && loc.x > p.x
                        {
                            self.events
                                .push(Event::Intersection(loc, lower_seg, next_id));
                        }
                    }
                    if let Some(prev_id) = self.segments.get_prev_id(lower) {
                        let prev_ls = self.segments.get(prev_id);
                        let ls = self.segments.get(upper_seg);
                        if let Some(loc) = ls.intersect(prev_ls)
                            && loc.x > p.x
                        {
                            self.events
                                .push(Event::Intersection(loc, prev_id, upper_seg));
                        }
                    }
                    assert!(p.x >= last.x);
                    last = p;
                }
            }
        }
        self.intersections
    }
}
