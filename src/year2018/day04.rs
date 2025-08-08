use ahash::{HashMap, HashMapExt};

use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut lines: Vec<_> = input.lines().map(str::as_bytes).collect();
    lines.sort_unstable();

    let mut guards = HashMap::new();
    let mut start = 0;
    let mut guard = 0u32;

    for line in lines {
        match line.len() {
            31 => { // falls asleep
                start = (&line[15..17]).to_unsigned();
            }, 
            27 => { // wakes up
                let end = (&line[15..17]).to_unsigned();
                let minutes = guards.entry(guard).or_insert_with(|| [0u32; 60]);
                for minute in minutes.iter_mut().take(end).skip(start) {
                    *minute += 1
                };
            },
            _ => { // shift
                guard = (&line[26..line.len()-13]).to_unsigned();
            }
        }
    }

    let p1 = solve_strategy(&guards, |minutes| minutes.iter().sum());
    let p2 = solve_strategy(&guards, |minutes| *minutes.iter().max().unwrap());

    (p1, p2)
}

fn solve_strategy(guards: &HashMap<u32, [u32; 60]>, strategy: impl Fn(&[u32; 60]) -> u32) -> u32 {
    let (guard, minutes) = guards
        .iter()
        .max_by_key(|(_, minutes)| strategy(minutes))
        .unwrap();

    guard * minutes.iter().enumerate().max_by_key(|p| p.1).unwrap().0 as u32
}