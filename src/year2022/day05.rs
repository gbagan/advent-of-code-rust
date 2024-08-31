use itertools::Itertools;
use crate::util::parser::*;

pub fn solve(input: &str) -> Option<(String, String)> {
    let (input1, input2) = input.split_once("\n\n")?;
    let input1: Vec<_> = input1.lines().rev().collect();
    let width = (input1[0].len() + 1) / 4;
    let mut stacks = vec![vec!(); width];
    for line in input1[1..].iter() {
        for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
            if c != ' ' {
                stacks[i].push(c);
            }
        }
    }
    let mut stacks2 = stacks.clone();

    for (amount, from, to) in input2.iter_unsigned::<usize>().tuples() {
        let from = from - 1;
        let to = to - 1;
        move_crates::<true>(&mut stacks, amount, from, to);
        move_crates::<false>(&mut stacks2, amount, from, to);
    }

    let p1 = stacks.iter().filter_map(|stack| stack.last()).collect();
    let p2 = stacks2.iter().filter_map(|stack| stack.last()).collect();

    Some((p1, p2))
}

fn move_crates<const REVERSE: bool>(stacks: &mut [Vec<char>], amount: usize, from: usize, to: usize) {
    let n = stacks[from].len() - amount;
    let [rfrom, rto] = stacks.get_many_mut([from, to]).unwrap();
    let moved = rfrom.drain(n..);
    if REVERSE {
        rto.extend(moved.rev());  
    } else {    
        rto.extend(moved);
    }
}