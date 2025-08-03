use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, usize) {
    let claims: Vec<_> = input.iter_unsigned().array_chunks().map(Claim::parse).collect();

    let mut grid = vec![0u64; 16000];
    let mut overlap = vec![0u64; 16000];

    for claim in &claims {
        for index in (claim.start..claim.end).step_by(16) {
            overlap[index] |= claim.block1 & grid[index];
            grid[index] |= claim.block1;
            if claim.block2 != 0 {
                overlap[index+1] |= claim.block2 & grid[index+1];
                grid[index+1] |= claim.block2;
            }
        }
    }

    let p1 = overlap.iter().map(|x| x.count_ones()).sum();

    let p2 = 1 + claims.iter().position(|claim| {
        (claim.start..claim.end).step_by(16).all(|index|
            claim.block1 & overlap[index] == 0 
            && (claim.block2 == 0 || claim.block2 & overlap[index+1] == 0)
        )
    }).unwrap();

    (p1, p2)
}


struct Claim {
    start: usize,
    end: usize,
    block1: u64,
    block2: u64,
}

impl Claim {
    fn parse(input: [usize; 5]) -> Self {
        let [_, x, y, width, height] = input;
        let quot = x / 64;
        let rem = x % 64;
        let start = y * 16 + quot;
        let end = (y + height) * 16 + quot;
        let block1 = ((1 << width) - 1) << rem;
        let block2 = (1 << (rem + width).saturating_sub(64)) - 1;

        Self { start, end, block1, block2 }
    }
}