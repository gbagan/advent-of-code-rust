use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for n in input.iter_unsigned::<u32>() {
        let m = n / 3 - 2;
        p1 += m;
        p2 += part2(m);
    }

    (p1, p2)
}

fn part2(mut n: u32) -> u32 {
    let mut sum = n;
    while n >= 6 {
        n = n / 3 - 2;
        sum += n;
    }
    sum
}