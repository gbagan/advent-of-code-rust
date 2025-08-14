// https://www.reddit.com/r/adventofcode/comments/7kj35s/comment/dreucbm/
// bubble sort

use crate::util::parser::*;

pub fn solve(input:& str) -> (u64, usize) {
    let mut n: u64 = input.lines().nth(9).unwrap().try_unsigned().unwrap();

    let mut sequence: Vec<_> = (0..127).map(|_| {
        n = (n * 8505) % 0x7fffffff;
        n = (n * 129749 + 12345) % 0x7fffffff;
        n % 10000
    }).collect();

    let p1 = sequence[126];

    let mut done = false;
    let mut count = 0;

    while !done {
        done = true;
        for i in 0..126 - count {
            if sequence[i] < sequence[i+1] {
                sequence.swap(i, i+1);
                done = false;
            }
        }

        count += 1;
    }

    let p2 = 127 * count.div_ceil(2);

    (p1, p2)
}