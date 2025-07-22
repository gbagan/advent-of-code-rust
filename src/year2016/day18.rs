pub fn solve(input: &str) -> (u32, u32) {
    let input = input.trim().as_bytes();
    let len = input.len();
    assert!(len <= 128);

    let mut odds = 0;
    let mut evens = 0;
    for &[c1, c2] in input.array_chunks() {
        evens = 2 * evens + (c1 == b'^') as u64;
        odds = 2 * odds + (c2 == b'^') as u64;
    }

    dbg!(len);

    let p1 = count_safes::<40>(evens, odds, len);
    let p2 = count_safes::<400000>(evens, odds, len);
    (p1, p2)
}

pub fn count_safes<const N: usize>(mut evens: u64, mut odds: u64, len: usize) -> u32 {
    let mask = (1 << (len/2)) - 1;
    let mut traps = evens.count_ones() + odds.count_ones();

    for _ in 0..N-1 {
        (evens, odds) = (odds ^ (odds >> 1), (evens ^ (evens << 1)) & mask);
        traps += evens.count_ones() + odds.count_ones();
    }

    (len * N) as u32 - traps
}
