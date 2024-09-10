use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let numbers: Vec<u64> = input.iter_unsigned().collect();
    let p1 = part1::<26>(&numbers)?;
    let p2 = part2(&numbers, p1)?;
    Ok((p1, p2))
}

fn part1<const N: usize>(numbers: &[u64])-> Result<u64> {
    numbers
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
        })
        .map(|slice| slice[N-1])
        .context("Part 1: No solution found")
}

fn part2(numbers: &[u64], target: u64) -> Result<u64> {
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
    if sum == target {
        let slice = &numbers[start..end];
        Ok(slice.iter().min().unwrap() + slice.iter().max().unwrap())
    } else {
        bail!("Part 2: No solution found")
    }
}