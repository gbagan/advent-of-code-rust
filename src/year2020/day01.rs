use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let numbers: Vec<_> = input.iter_unsigned().filter(|&n| n < 2020).collect();
    let mut table = vec![0; 2022];
    let mut round = 0;

    let p1 = sum2(&numbers, &mut table, &mut round, 2020).context("Part 1: No solution found")?;
    let p2 = part2(&numbers, &mut table, &mut round).context("Part 2: No solution found")?;

    Ok((p1, p2))
}

fn part2(numbers: &[usize], table: &mut [u32], round: &mut u32) -> Option<usize> {
    for i in 0..numbers.len()-2 {
        if let Some(j) = sum2(&numbers[i+1..], table, round, 2020 - numbers[i]) {
            return Some(numbers[i] * j)
        }
    }
    None
}

    
fn sum2(numbers: &[usize], table: &mut [u32], round: &mut u32, goal: usize) -> Option<usize> {
    *round += 1;
    let round = *round;
    for &n in numbers {
        if n < goal {
            if table[goal-n] == round {
                return Some(n * (goal - n));
            } 
            table[n] = round;
        }
    }
    None
}