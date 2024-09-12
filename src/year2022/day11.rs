use anyhow::*;
use itertools::Itertools;
use num_integer::Integer;
use crate::util::parser::*;
use std::sync::Mutex;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};


pub enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

struct Monkey {
    operation: Operation,
    divided_by: u64,
    if_true: usize,
    if_false: usize
}

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut items = vec!();
    let mut monkeys = vec!();
    let mut lines = input.lines();
    let mut i = 0;
    while let Some((_, line2, line3, line4, line5, line6)) = lines.next_tuple() {
        for j in line2.iter_unsigned() {
            items.push((i, j));
        }
        let operation =
            if let Result::Ok(val) = line3.try_unsigned() {
                match line3.as_bytes()[23] {
                    b'+' => Operation::Add(val),
                    b'*' => Operation::Multiply(val),
                    c => bail!("unexpected character {}", c as char),
                }
            } else {
                Operation::Square
            };
        let divided_by = line4.try_unsigned().unwrap();
        let if_true = line5.try_unsigned().unwrap();
        let if_false = line6.try_unsigned().unwrap();
        monkeys.push(Monkey{operation, divided_by, if_true, if_false});
        i += 1;
        lines.next();
    }

    let p1 = part1(&monkeys, &items);
    let p2 = part2(&monkeys, &items);

    Ok((p1, p2))
}

fn part1(monkeys: &[Monkey], items: &[(usize, u64)]) -> u64 {
    let mut business = vec![0; monkeys.len()];
    for item in items {
        let item_business = simulate_item(monkeys, *item, 20, |n| n / 3);
        for (b, ib) in business.iter_mut().zip(item_business) {
            *b += ib;
        }
    }
    business.sort_unstable();
    business.iter().rev().take(2).product()
}

fn part2(monkeys: &[Monkey], items: &[(usize, u64)]) -> u64 {
    let mutex = Mutex::new(vec![0; monkeys.len()]);
    let counter = AtomicUsize::new(0);

    let lcm = monkeys.iter().fold(1, |acc, monkey| acc.lcm(&monkey.divided_by));
    
    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(monkeys, items, lcm, &counter, &mutex));
        }
    });    
    let mut business = mutex.into_inner().unwrap();
    
    business.sort_unstable();
    business.iter().rev().take(2).product()
}

fn worker(monkeys: &[Monkey], items: &[(usize, u64)], lcm: u64, counter: &AtomicUsize, mutex: &Mutex<Vec<u64>>) {
    loop {
        let i = counter.fetch_add(1, Ordering::Relaxed);
        if i >= items.len() {
            break;
        }
        let item_business = simulate_item(monkeys, items[i], 10_000, |n| n % lcm);
        let mut business = mutex.lock().unwrap();
        for (i, v) in item_business.iter().enumerate() {
            business[i] += v;
        }
    }
}

fn simulate_item(monkeys: &[Monkey], item: (usize, u64), rounds: usize, func: impl Fn(u64) -> u64) -> Vec<u64> {
    let mut business = vec![0; monkeys.len()];
    let (mut owner, mut item) = item;
    let mut i = 0;
    while i < rounds {
        let worry = match monkeys[owner].operation {
            Operation::Square => item * item,
            Operation::Multiply(y) => item * y,
            Operation::Add(y) => item + y,
        };
        item = func(worry);

        let to = if item % monkeys[owner].divided_by == 0 {
                monkeys[owner].if_true
            } else { 
                monkeys[owner].if_false
            };
        if to < owner {
            i += 1;
        }
        business[owner] += 1;
        owner = to;
    }
    business
}