use std::collections::HashSet;

pub fn parse(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> Option<u32> {
    let origin: (i32, i32) = (0, 0);

    let positions = input.chars().scan(origin, |acc, dir| {
        match dir {
            '<' => acc.0 -= 1,
            '>' => acc.0 += 1,
            '^' => acc.1 -= 1,
            'v' => acc.1 += 1,
            _ => panic!("invalid direction: {dir}"),
        }
        Some (acc.clone())
    });
    let mut visited: HashSet<(i32, i32)> = HashSet::from_iter(positions);
    visited.insert(origin);
    Some(visited.len() as u32)
}

pub fn part2(input: &str) -> Option<u32> {
    let origin: (isize, isize) = (0, 0);

    let positions = input.chars().enumerate().scan((origin, origin), |(acc1, acc2), (i, dir)| {
        if i % 2 == 0 {
            match dir {
                '<' => acc1.0 -= 1,
                '>' => acc1.0 += 1,
                '^' => acc1.1 -= 1,
                'v' => acc1.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }
            Some (acc1.clone())
        } else {
            match dir {
                '<' => acc2.0 -= 1,
                '>' => acc2.0 += 1,
                '^' => acc2.1 -= 1,
                'v' => acc2.1 += 1,
                _ => panic!("invalid direction: {dir}"),
            }
            Some (acc2.clone())
        }    
    });
    let mut visited: HashSet<(isize, isize)> = HashSet::from_iter(positions);
    visited.insert(origin);
    Some(visited.len() as u32)
}