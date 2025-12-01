use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Part([u32; 4]);

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim_start_matches('{').trim_end_matches('}').split(',');
        let (Some(x), Some(m), Some(a), Some(s), None) = (
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
            parts.next(),
        ) else {
            return Err(format!("Part doesn't come with exactly 4 categories: {s}"));
        };
        let x = x
            .trim_start_matches("x=")
            .parse()
            .map_err(|_| format!("x category is badly formatted: {s}"))?;
        let m = m
            .trim_start_matches("m=")
            .parse()
            .map_err(|_| format!("m category is badly formatted: {s}"))?;
        let a = a
            .trim_start_matches("a=")
            .parse()
            .map_err(|_| format!("a category is badly formatted: {s}"))?;
        let s = s
            .trim_start_matches("s=")
            .parse()
            .map_err(|_| format!("s category is badly formatted: {s}"))?;
        Ok(Self([x, m, a, s]))
    }
}

impl Part {
    pub fn rating(self) -> u32 {
        self.0.into_iter().sum()
    }
}

impl IntoIterator for Part {
    type Item = u32;
    type IntoIter = std::array::IntoIter<u32, 4>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Iter<'a> {
    part: &'a Part,
    index: usize,
}

impl Part {
    pub const fn iter(&self) -> Iter<'_> {
        Iter {
            part: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.part.0.get(self.index);
        self.index += 1;
        result.copied()
    }
}

impl<'a> IntoIterator for &'a Part {
    type Item = u32;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
