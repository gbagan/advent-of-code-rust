use crate::util::parser::*;
use ahash::HashMap;

#[derive(Clone, Copy)] 
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

enum Job {
    Integer(u64),
    Calc(usize, Operator, usize)
}

use {Job::*, Operator::*};

pub fn solve(input: &str) -> (u64, u64) {
    let lines: Vec<_> = input.lines().collect();

    let index_map: HashMap<_, _> = lines
        .iter()
        .enumerate()
        .map(|(index, line)| (&line.as_bytes()[0..4], index))
        .collect();
    
    let mut jobs: Vec<_> = lines
        .iter()
        .map(|line| {
            let line = line.as_bytes();
            if line.len() == 17 {
                let left = index_map[&line[6..10]];
                let right = index_map[&line[13..17]];
                let op = match line[11] {
                    b'+' => Add,
                    b'-' => Sub,
                    b'*' => Mul,
                    b'/' => Div,
                    _ => panic!("invalid operator"),
                };
                Calc(left, op, right)
            } else {
                Integer((&line[6..]).to_unsigned())
            }
        })
        .collect();

    let mut results = vec![0; jobs.len()];
    let root = index_map[b"root".as_slice()];
    let humn = index_map[b"humn".as_slice()];

    let p1 = compute(&jobs, &mut results, root);

    // part 2

    let mut humn_table = vec![false; jobs.len()];
    contains_humn(&jobs, &mut humn_table, root, humn);

    jobs[root] = match jobs[root] {
        Calc(left, _, right) => Calc(left, Sub, right),
        _ => unreachable!(),
    };

    let mut result = 0;
    let mut current = root;
    let p2 = loop {
        match jobs[current] {
            Integer(_) => break result,
            Calc(left, op, right) => {
                if humn_table[left] {
                    let right = results[right];
                    result = match op {
                        Add => result - right,
                        Sub => result + right,
                        Mul => result / right,
                        Div => result * right,
                    };
                    current = left;
                } else {
                    let left = results[left];
                    result = match op {
                        Add => result - left,
                        Sub => left - result,
                        Mul => result / left,
                        Div => left / result,
                    };
                    current = right;
                }
            }
        }
    };

    (p1, p2)
}

fn compute(jobs: &[Job], results: &mut[u64], root: usize) -> u64 {
    let result = match jobs[root] {
        Integer(n) => n,
        Calc(left, op, right) => {
            let left = compute(jobs, results, left);
            let right = compute(jobs, results, right);
            match op {
                Add => left + right,
                Sub => left - right,
                Mul => left * right,
                Div => left / right,
            }
        }
    };
    results[root] = result;
    result
}

fn contains_humn(jobs: &[Job], humn_table: &mut [bool], root: usize, humn: usize) -> bool {
    let result = match jobs[root] {
        Integer(_) => root == humn,
        Calc(left, _, right) =>
            contains_humn(jobs, humn_table, left, humn)
            || contains_humn(jobs, humn_table, right, humn),
    };
    humn_table[root] = result;
    result
}