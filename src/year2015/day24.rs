// dynamic programming

pub fn parse(input: &str) -> Option<Vec<usize>> {
    Some(input.lines().filter_map(|line| line.parse().ok()).collect())
}

pub fn solve(numbers: &[usize], k: usize) -> Option<u128> {
    let m = numbers.len();
    let n = numbers.iter().sum::<usize>() / k;
    let size = (m+1) * (n+1);
    let mut table: Vec<Option<u128>> = vec![None; size];
    table[0] = Some(1);
    for i in 1..=m {
        let v = numbers[i-1];
        for j in 0..=n {
            let index = i * (n+1) + j;
            table[index] =
                if v > j {
                    table[index - (n+1)]
                } else {
                    match (table[index - (n+1)], table[index - (n+1) - v]) {
                        (x, None) => x,
                        (None, Some(x)) => Some(v as u128 *x),
                        (Some(x), Some(y)) => Some(x.min(v as u128 *y))
                    }
                }
        }
    }
    table[size-1]
}

pub fn part1(numbers: &[usize]) -> Option<u128> {
    solve(numbers, 3)
}

pub fn part2(numbers: &[usize]) -> Option<u128> {
    solve(numbers, 4)
}