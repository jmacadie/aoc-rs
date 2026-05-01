#[must_use]
pub const fn gcd(mut u: u32, v: i32) -> u32 {
    let mut v = v.unsigned_abs();
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        #[allow(clippy::manual_swap)]
        if u > v {
            // mem::swap(&mut u, &mut v);
            let temp = u;
            u = v;
            v = temp;
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}
