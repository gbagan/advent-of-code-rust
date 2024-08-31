pub fn solve(input: &str) -> Option<(u32, u32)> {
    let jumps: Vec<_> = input.lines().filter_map(|line| line.parse().ok()).collect();
    Some((part1(&jumps), part2(&jumps)))
}

fn part1(jumps: &[i32]) -> u32 {
    let mut jumps = jumps.to_vec();
    let n = jumps.len() as i32;

    let mut steps = 0;
    let mut offset = 0;
    while offset < n {
        let tmp = offset + jumps[offset as usize];
        jumps[offset as usize] += 1;
        offset = tmp;
        steps += 1;
    }
    steps
}

fn part2(jumps: &[i32]) -> u32 {
    let mut jumps = jumps.to_vec();
    let n = jumps.len() as i32;

    let mut steps = 0;
    let mut offset = 0;
    while offset < n {
        let offset2 = jumps[offset as usize];
        if offset2 >= 3 {
            jumps[offset as usize] -= 1;
        } else {
            jumps[offset as usize] += 1;
        }
        offset += offset2;
        steps += 1;
    }
    steps
}