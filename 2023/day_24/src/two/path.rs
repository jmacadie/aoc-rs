use std::str::FromStr;

pub struct Path {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub u: i64,
    pub v: i64,
    pub w: i64,
}

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s
            .split_once(" @ ")
            .ok_or_else(|| format!("Cannot split {s} into position & velocity"))?;
        let mut parts = pos.split(", ");
        let (Some(x), Some(y), Some(z), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(format!("Cannot split {pos} into three parts"));
        };
        let mut parts = vel.split(", ");
        let (Some(u), Some(v), Some(w), None) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        else {
            return Err(format!("Cannot split {vel} into three parts"));
        };
        let x = x
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse {x} into a number"))?;
        let y = y
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse {y} into a number"))?;
        let z = z
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse {z} into a number"))?;
        let u = u
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse {u} into a number"))?;
        let v = v
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse {v} into a number"))?;
        let w = w
            .parse::<i64>()
            .map_err(|_| format!("Cannot parse {x} into a number"))?;
        Ok(Self { x, y, z, u, v, w })
    }
}
