use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let line = line.as_bytes();
        let solution2 = joltage::<12>(line);
        let solution1 = joltage::<2>(&solution2);
        p1 += solution1.to_unsigned::<u64>();
        p2 += solution2.to_unsigned::<u64>();
    }
    
    (p1, p2)
}

fn joltage<const N: usize>(line: &[u8]) -> [u8; N] {
    let mut solution = [0; N];
    let end = line.len() - N;
    solution.copy_from_slice(&line[end..]);
    for &c in line[..end].iter().rev() {
        update_solution(&mut solution, c);
    }
    solution
}

fn update_solution<const N: usize>(solution: &mut [u8; N], mut b: u8) {
    for b2 in solution {
        if *b2 > b {
            break;
        }
        b = std::mem::replace(b2, b);
    }
}