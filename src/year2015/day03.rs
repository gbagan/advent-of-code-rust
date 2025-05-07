use ahash::{HashSet, HashSetExt};

pub fn solve(input: &str) -> (usize, usize) {
    let input = input.trim().as_bytes();
    let p1 = part1(input);
    let p2 = part2(input);
    (p1, p2)
}

pub fn part1(input: &[u8]) -> usize {
    let mut position: (i32, i32) = (0, 0);
    let mut seen = HashSet::new();
    seen.insert(position);

    for &c in input.iter() {
        match c {
            b'<' => position.0 -= 1,
            b'>' => position.0 += 1,
            b'^' => position.1 -= 1,
            _ => position.1 += 1,
        }
        seen.insert(position);
    }

    seen.len()
}

pub fn part2(input: &[u8]) -> usize {
    let mut position1 = (0i32, 0i32);
    let mut position2 = (0i32, 0i32);
    let mut seen = HashSet::new();
    seen.insert(position1);

    for (i, &c) in input.iter().enumerate() {
        if i & 1 == 0 {
            match c {
                b'<' => position1.0 -= 1,
                b'>' => position1.0 += 1,
                b'^' => position1.1 -= 1,
                _ => position1.1 += 1,
            }
            seen.insert(position1);
        } else {
            match c {
                b'<' => position2.0 -= 1,
                b'>' => position2.0 += 1,
                b'^' => position2.1 -= 1,
                _ => position2.1 += 1,
            }
            seen.insert(position2);
        }
    }

    seen.len()
}