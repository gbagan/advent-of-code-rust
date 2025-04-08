use ahash::{HashMap, HashMapExt};
use itertools::Itertools;
use crate::util::{foreach_permutation, parser::*};

fn parse_line(s: &str) -> (&str, &str, u32) {
    let (city1, _, city2, _, dist) = s.split(' ').next_tuple().unwrap();
    let dist = dist.try_unsigned().unwrap();
    (city1, city2, dist)
}

pub fn solve(input: &str) -> (u32, u32) {
    let travels: Vec<_> = input.lines().map(parse_line).collect();
    let mut dict = HashMap::new();

    let mut i = 0;

    for (city1, city2, _) in &travels {
        if !dict.contains_key(city1) {
            dict.insert(city1, i);
            i += 1;
        }
        if !dict.contains_key(city2) {
            dict.insert(city2, i);
            i += 1;
        }
    }
    let n = dict.len();

    let mut table = vec![0; n*n];
    for (city1, city2, dist) in &travels {
        let i = dict[city1];
        let j = dict[city2];
        table[i * n + j] = *dist;
        table[j * n + i] = *dist;
    }

    let mut min_travel = u32::MAX;
    let mut max_travel = 0;

    let mut init: Vec<usize> = (1..n).collect();
    foreach_permutation(&mut init, |perm| {
    //for perm in (1..n).permutations(n-1) {
        if perm[0] > perm[n-2] {
            return
        }
        let mut sum = table[perm[0]];
        let mut min = sum;
        let mut max = sum; 
        for (i, j) in perm.iter().tuple_windows() {
            let edge = table[i*n+j];
            sum += edge;
            min = min.min(edge);
            max = max.max(edge);
        }
        let edge = table[perm[n-2]];
        sum += edge;
        min = min.min(edge);
        max = max.max(edge);
        min_travel = min_travel.min(sum - max);
        max_travel = max_travel.max(sum - min);
    });
    (min_travel, max_travel)
}