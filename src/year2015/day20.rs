use itertools::iterate;

const PRIMES: [u32; 12] = [ 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let goal = input.trim().parse().ok()?;
    let p1 = part1(goal / 10, PRIMES.len());
    let p2 = part2(goal);
    Some((p1, p2))
}

fn part1(goal: u32, prime_index: usize) -> u32 {
    if prime_index == 0 {
        return goal;
    }
    let p = PRIMES[prime_index-1];
    if prime_index == 0 { return goal }
    iterate((1, 1), |(p_power, p_sum)| (p_power * p, p_sum + p_power * p))
        .take_while(|&(_, p_sum)| p_sum <= goal)
        .map(|(p_power, p_sum)| p_power * part1(goal.div_ceil(p_sum), prime_index - 1))
        .min()
        .unwrap_or(0)
}

fn part2_number(n: u32) -> u32 {
    (1..50).filter_map(|i| (n % i == 0).then_some(n / i)).sum()

}

pub fn part2(goal: u32) -> u32 {
    let mut i = 0;
    loop {
        if 11 * part2_number(i) >= goal {
            return i;
        }
        i += 2 * 3 * 5 * 7;
    }
}