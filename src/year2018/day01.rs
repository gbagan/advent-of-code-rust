use crate::util::parser::*;

pub fn solve(input: &str) -> (i32, i32) {
    let numbers: Vec<i32> = input.iter_signed().collect();
    let p1 = numbers.iter().sum::<i32>();

    let mut seen = Vec::with_capacity(numbers.len());    
    let mut frequency: i32 = 0;
    for (i, &n) in numbers.iter().enumerate() {
        seen.push((frequency.rem_euclid(p1), frequency, i));
        frequency += n;
    }

    seen.sort_unstable();

    let p2 = seen
        .array_chunks()
        .filter_map(|&[(rem1, freq1, idx1), (rem2, freq2, _)]|
            if rem1 != rem2 { None} else { Some((freq2 - freq1, idx1, freq2)) }
        ).min()
        .unwrap()
        .2;

    let s = "é";

    (p1, p2)
}