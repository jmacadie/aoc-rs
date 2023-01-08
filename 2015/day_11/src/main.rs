#![warn(clippy::all, clippy::pedantic)]

use itertools::Itertools;

pub fn main() {
    print!("Part 1: ");
    print_password(part_one("vzbxkghb"));

    print!("Part 2: ");
    print_password(part_two("vzbxkghb"));
}

fn part_one(data: &str) -> Password {
    let password = new_password(data);
    next_valid_password(password)
}

fn part_two(data: &str) -> Password {
    let password = new_password(data);
    let password = next_valid_password(password);
    next_valid_password(password)
}

type Password = [u8; 8];

fn next_valid_password(mut password: Password) -> Password {
    next_password(&mut password);
    while !is_valid(password) {
        next_password(&mut password);
    }
    password
}

fn print_password(password: Password) {
    for elem in password.into_iter().rev() {
        if elem != 0 {
            let c = char::from_u32(u32::from(elem)).unwrap();
            print!("{c}");
        }
    }
    println!();
}

fn new_password(data: &str) -> Password {
    let mut password = Password::default();
    for (p, b) in password.iter_mut().zip(data.as_bytes().iter().rev()) {
        *p = *b;
    }
    password
}

fn next_password(curr: &mut Password) {
    for (i, elem) in curr.iter_mut().enumerate() {
        match elem {
            b'z' => *elem = b'a',
            b'h' | b'k' | b'n' => {
                *elem += 2;
                for item in curr.iter_mut().take(i) {
                    *item = b'a';
                }
                return;
            }
            _ => {
                *elem += 1;
                return;
            }
        }
    }
}

fn is_valid(password: Password) -> bool {
    contains_straight(password) && two_pairs(password)
}

fn contains_straight(password: Password) -> bool {
    for (a, b, c) in password.into_iter().tuple_windows::<(_, _, _)>() {
        if a == b + 1 && b == c + 1 {
            return true;
        }
    }
    false
}

fn two_pairs(password: Password) -> bool {
    password
        .into_iter()
        .dedup_with_count()
        .filter(|(c, _)| c >= &2)
        .count()
        >= 2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        assert_eq!(
            [b'a', b'a', b'f', b'f', b'd', b'c', b'b', b'a'],
            part_one("abcdefgh")
        );
    }
}
