fn part1(n: usize) -> usize {
    let mut v = Vec::with_capacity(2018);
    v.push(0);
    let mut pos = 0;
    for i in 1..=2017 {
        pos = 1 + (pos + n) % i;
        v.insert(pos, i);
    }
    v[(pos+1)%2018]
}

fn part2(steps: usize) -> usize {
    let steps = steps + 1;
    let mut val_after_0 = 0;
    let mut pos = 0;
    let mut i = 1;
    while i <= 50_000_000usize {
        if pos == 0 {
            val_after_0 = i;
        }
        let skip = (i - pos).div_ceil(steps);
        i += skip;
        pos = (pos + skip * steps) % i;
    }
    val_after_0
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let n = input.trim().parse().ok()?;
    Some((part1(n), part2(n)))
}