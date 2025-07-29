// dynamic programming

use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u64) {
    let mut numbers: Vec<_> = input.iter_unsigned().collect();
    numbers.push(0);
    numbers.sort_unstable();
    let p1 = part1(&numbers);
    let p2 = part2(&numbers);

    (p1, p2)
}

fn part1(numbers: &[usize]) -> u32 {
    let mut one_jolt = 0;
    let mut three_jolt = 0;
    for &[x, y] in numbers.array_windows() {
        if y - x == 1 {
            one_jolt += 1;
        } else if y - x == 3 {
            three_jolt += 1;
        }
    }
    one_jolt * (1+three_jolt)
}

fn part2(numbers: &[usize]) -> u64 {
    let largest = numbers[numbers.len()-1];
    let n = largest + 1;
    let mut count = vec![0; n];
    count[0] = 1;
    count[1] = (numbers[1] == 1) as u64;
    count[2] = (numbers[1] == 1 || numbers[2] == 1) as u64 * (count[0] + count[1]);
    for &i in numbers {
        if i >= 3 {
            count[i] = count[i-1] + count[i-2] + count[i-3];
        }
    }

    count[largest]
}