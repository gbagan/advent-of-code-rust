use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let lines: Vec<_> = input.lines().map(str::as_bytes).collect(); 
    let p1 = part1(&lines);
    let p2 = part2(&lines);

    (p1, p2)
}

fn part1(lines: &[&[u8]]) -> u64 {
    let mut numbers: Vec<_> = lines[..lines.len()-1]
        .iter()
        .map(|line| line.iter_unsigned::<u64>())
        .collect();
    let operators = lines[lines.len()-1].iter().filter(|&&c| c != b' ');

    operators.map(|&op|
        if op == b'+' {
            numbers.iter_mut().map(|it| it.next().unwrap()).sum::<u64>()
        } else {
            numbers.iter_mut().map(|it| it.next().unwrap()).product()   
        }
    ).sum()
}

fn part2(lines: &[&[u8]]) -> u64 {
    let ops = lines[lines.len()-1];
    
    let mut current_op = b'+';
    let mut current_problem = 0;
    let mut total = 0;

    for (i, &op) in ops.iter().enumerate() {
        if op != b' ' {
            current_op = op;
            total += current_problem;
            current_problem = if op == b'+' { 0 } else { 1 };
        }

        let mut current_number = 0;
        for &line in &lines[..lines.len()-1] {
            if line[i] != b' ' {
                current_number = 10 * current_number + (line[i] - b'0') as u64;
            }
        }
        if current_op == b'+' {
            current_problem += current_number;
        } else if current_number != 0 {
            current_problem *= current_number;
        }
    }
    total + current_problem
}