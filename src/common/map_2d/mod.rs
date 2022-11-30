use std::{slice, vec};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}

pub struct Map<P> {
    data: Vec<Vec<P>>,
    height: usize,
    width: usize,
}

impl<P> Map<P> {
    #[must_use]
    pub fn new(data: Vec<Vec<P>>) -> Self {
        let height = data.len();
        let width = data[0].len();
        Self {
            data,
            height,
            width,
        }
    }

    #[must_use]
    pub fn val(&self, loc: Point) -> Option<&P> {
        if !self.contains(loc) {
            return None;
        }
        Some(&self.data[loc.y][loc.x])
    }

    pub fn update(&mut self, loc: Point, val: P) {
        if self.contains(loc) {
            self.data[loc.y][loc.x] = val;
        }
    }

    #[must_use]
    pub fn neighbours(&self, loc: Point, diagonal: bool) -> Vec<Point> {
        let mut out = Vec::new();
        for (x_offset, y_offset) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            if let Some(p) = self.offset(loc, x_offset, y_offset) {
                out.push(p);
            }
        }
        if diagonal {
            for (x_offset, y_offset) in [(-1, -1), (1, -1), (1, 1), (-1, 1)] {
                if let Some(p) = self.offset(loc, x_offset, y_offset) {
                    out.push(p);
                }
            }
        }
        out
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn neighbours_val(&self, loc: Point, diagonal: bool) -> Vec<(Point, &P)> {
        let mut out = Vec::new();
        for p in self.neighbours(loc, diagonal) {
            out.push((p, self.val(p).unwrap()));
        }
        out
    }

    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn offset(&self, loc: Point, x_offset: i32, y_offset: i32) -> Option<Point> {
        let new_x: i32 = i32::try_from(loc.x).unwrap() + x_offset;
        let new_y: i32 = i32::try_from(loc.y).unwrap() + y_offset;
        if new_x < 0 || new_y < 0 {
            return None;
        }
        let new_x = new_x.try_into().unwrap();
        let new_y = new_y.try_into().unwrap();
        if new_x >= self.width || new_y >= self.height {
            return None;
        }
        Some(Point { x: new_x, y: new_y })
    }

    #[must_use]
    pub fn contains(&self, loc: Point) -> bool {
        loc.x < self.width && loc.y < self.height
    }
}

pub struct IntoIter<P> {
    curr_row: vec::IntoIter<P>,
    rem_rows: vec::IntoIter<Vec<P>>,
    x: usize,
    y: usize,
}

impl<P> Iterator for IntoIter<P> {
    type Item = (Point, P);

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_row.next() {
            Some(val) => {
                let p = Point {
                    x: self.x,
                    y: self.y,
                };
                self.x += 1;
                Some((p, val))
            }
            None => match self.rem_rows.next() {
                Some(vec) => {
                    self.curr_row = vec.into_iter();
                    self.x = 0;
                    self.y += 1;
                    let p = Point {
                        x: self.x,
                        y: self.y,
                    };
                    self.x += 1;
                    Some((p, self.curr_row.next().unwrap()))
                }
                None => None,
            },
        }
    }
}

impl<P> IntoIterator for Map<P> {
    type Item = (Point, P);
    type IntoIter = IntoIter<P>;

    fn into_iter(self) -> Self::IntoIter {
        let mut rows = self.data.into_iter();
        let curr_row = rows.next().unwrap().into_iter();
        IntoIter {
            curr_row,
            rem_rows: rows,
            x: 0,
            y: 0,
        }
    }
}

pub struct Iter<'a, P> {
    curr_row: &'a [P],
    rem_rows: &'a [Vec<P>],
    x: usize,
    y: usize,
}

impl<'a, P> Iterator for Iter<'a, P> {
    type Item = (Point, &'a P);

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr_row.split_first() {
            Some((val, tail)) => {
                let p = Point {
                    x: self.x,
                    y: self.y,
                };
                self.x += 1;
                self.curr_row = tail;
                Some((p, val))
            }
            None => match self.rem_rows.split_first() {
                Some((next_row, tail)) => {
                    self.x = 0;
                    self.y += 1;
                    let p = Point {
                        x: self.x,
                        y: self.y,
                    };
                    self.x += 1;
                    self.rem_rows = tail;
                    let (val, row) = next_row.as_slice().split_first()?;
                    self.curr_row = row;
                    Some((p, val))
                }
                None => None,
            },
        }
    }
}

impl<'a, P> Map<P> {
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn iter(&'a self) -> Iter<'a, P> {
        let rows = self.data.as_slice();
        let (curr_row, rem_rows) = rows.split_first().unwrap();
        let curr_row = curr_row.as_slice();
        Iter {
            curr_row,
            rem_rows,
            x: 0,
            y: 0,
        }
    }
}

impl<'a, P> IntoIterator for &'a Map<P> {
    type Item = (Point, &'a P);
    type IntoIter = Iter<'a, P>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
