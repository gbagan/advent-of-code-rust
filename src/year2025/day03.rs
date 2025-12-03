pub fn solve(input: &str) -> (u32, u64) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let line = line.as_bytes();
        p1 += solve_p1(line);
        p2 += joltage::<12>(line);
    }
    
    (p1, p2)
}

fn solve_p1(line: &[u8]) -> u32 {
    let n = line.len();
    let idx1 = (0..n-1).rev().max_by_key(|&i| line[i]).unwrap();
    let idx2 = (idx1+1..n).max_by_key(|&i| line[i]).unwrap();
    10 * (line[idx1] - b'0') as u32 + (line[idx2] - b'0') as u32
}

fn joltage<const N: usize>(line: &[u8]) -> u64 {
    let mut total = 0;
    let mut bytes = line;
    for i in (0..N).rev() {
        let idx = (0..bytes.len()-i).rev().max_by_key(|&i| bytes[i]).unwrap();
        total = 10 * total + (bytes[idx] - b'0') as u64;
        bytes = &bytes[idx+1..]
    }
    total
}