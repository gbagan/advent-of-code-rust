pub fn solve(input: &str) -> (i32, usize) {
    let input = input.trim().as_bytes();
    let p1 = part1(input);
    let p2 = part2(input);
    (p1, p2)
}

pub fn part1(input: &[u8]) -> i32 {
    let mut n = 0;
    
    for &c in input {
        n += (c == b'(') as i32
    }
    2 * n - input.len() as i32
}

pub fn part2(input: &[u8]) -> usize {
    let mut n = 0;
    
    for (i, &c) in input.iter().enumerate() {
        if c == b'(' {
            n += 1;
        } else {
            n -= 1;
            if n < 0 {
                return i+1;
            }
        }
    }
    unreachable!();
}