use std::collections::HashSet;

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let bytes = input.trim().as_bytes();
    Some((part1(bytes), part2(bytes)))
}

pub fn part1(bytes: &[u8]) -> usize {
    let origin: (i32, i32) = (0, 0);

    let positions = bytes.iter().scan(origin, |acc, dir| {
        match dir {
            b'<' => acc.0 -= 1,
            b'>' => acc.0 += 1,
            b'^' => acc.1 -= 1,
            b'v' => acc.1 += 1,
            _ => panic!("invalid direction: {dir}"),
        }
        Some(*acc)
    });
    let mut visited: HashSet<(i32, i32)> = HashSet::from_iter(positions);
    visited.insert(origin);
    visited.len()
}

pub fn part2(bytes: &[u8]) -> usize {
    let origin: (isize, isize) = (0, 0);

    let positions = bytes.iter().enumerate().scan((origin, origin), |(acc1, acc2), (i, dir)| {
        if i % 2 == 0 {
            match dir {
                b'<' => acc1.0 -= 1,
                b'>' => acc1.0 += 1,
                b'^' => acc1.1 -= 1,
                b'v' => acc1.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }
            Some (*acc1)
        } else {
            match dir {
                b'<' => acc2.0 -= 1,
                b'>' => acc2.0 += 1,
                b'^' => acc2.1 -= 1,
                b'v' => acc2.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }
            Some (*acc2)
        }    
    });
    let mut visited: HashSet<(isize, isize)> = HashSet::from_iter(positions);
    visited.insert(origin);
    visited.len()
}