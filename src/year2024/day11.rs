use ahash::{HashMap, HashMapExt};
use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut indices = HashMap::with_capacity(5000);
    let mut new_values = Vec::new();
    let mut occurences = Vec::new();

    for value in input.iter_unsigned::<u64>() {
        if let Some(&index) = indices.get(&value) {
            occurences[index] += 1;
        } else {
            indices.insert(value, indices.len());
            new_values.push(value);
            occurences.push(1);
        }
    }

    let mut stones = Vec::with_capacity(5000);

    for _ in 0..25 {
        (new_values, occurences) = blink(&new_values, &occurences, &mut stones, &mut indices);
    }

    let p1 = occurences.iter().sum();

    for _ in 0..50 {
        (new_values, occurences) = blink(&new_values, &occurences, &mut stones, &mut indices);
    }

    let p2 = occurences.iter().sum();

    Ok((p1, p2))
}

pub fn blink(values: &[u64], occurences: &[u64], stones: &mut Vec<(usize, usize)>, indices: &mut HashMap<u64, usize>)
        -> (Vec<u64>, Vec<u64>) {
    let mut new_values = Vec::with_capacity(150);

    let mut index_of = |value| {
        let size = indices.len();
        *indices.entry(value).or_insert_with(|| {
            new_values.push(value);
            size
        })
    };

    let it = values.iter().map(|&value| {
        if value == 0 {
            (index_of(1), usize::MAX)
        } else if let Some((prefix, suffix)) = split_number(value) {
            (index_of(prefix), index_of(suffix))
        } else {
            (index_of(value * 2024), usize::MAX)
        }
    });
    stones.extend(it);
    
    let mut next_occurences = vec![0; indices.len()];

    for (&(first, second), &occurence) in stones.iter().zip(occurences) {
        next_occurences[first] += occurence;
        if second != usize::MAX {
            next_occurences[second] += occurence;
        }
    }

    (new_values, next_occurences)
}

#[inline]
fn split_number(n: u64) -> Option<(u64, u64)> {
    let m = n.ilog10() + 1;
    if m & 1 == 0 {
        let p = 10u64.pow(m >> 1);
        Some((n / p, n % p))
    } else {
        None
    }
}