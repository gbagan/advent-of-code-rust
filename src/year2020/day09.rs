use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let numbers: Vec<u64> = input.iter_unsigned().collect();
    let p1 = part1::<26>(&numbers);
    let p2 = part2(&numbers, p1);
    (p1, p2)
}

fn part1<const N: usize>(numbers: &[u64])-> u64 {
    let table = numbers
        .array_windows::<N>()
        .find(|&slice| {
            let last = slice[N-1];
            for i in 0..N-2 {
                for j in i+1..N-1 {
                    if slice[i] + slice[j] == last {
                        return false
                    }
                }
            }
            true
        }).unwrap();

    table[N-1]
}

fn part2(numbers: &[u64], target: u64) -> u64 {
    let n = numbers.len();
    let mut start = 0;
    let mut end = 2;
    let mut sum = numbers[0] + numbers[1];
    while sum != target && end < n {
        if sum < target {
            sum += numbers[end];
            end += 1;
        } else {
            sum -= numbers[start];
            start += 1;
        }
    }
    assert!(sum == target);
    let slice = &numbers[start..end];
    slice.iter().min().unwrap() + slice.iter().max().unwrap()
}