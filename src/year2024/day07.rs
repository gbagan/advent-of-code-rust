use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut row = Vec::new();

    for line in input.lines() {
        row.extend(line.iter_unsigned::<u64>());
        if has_solution::<false>(&row, row[0],row.len()-1) {
            p1 += row[0];
        } else if has_solution::<true>(&row, row[0],row.len()-1) {
            p2 += row[0];
        }
        row.clear();
    }

    (p1, p1+p2)
}

fn has_solution<const P2: bool>(row: &[u64], goal: u64, idx: usize) -> bool {
    let current = row[idx];
    if idx == 1 {
        return goal == current
    }
    if P2 {
        if let Some(goal2) = truncate_number(goal, current) {
            if has_solution::<P2>(row, goal2, idx - 1) {
                return true;
            }
        }
    }

    if goal % current == 0 && has_solution::<P2>(row, goal / current, idx - 1) {
        return true;
    }
    goal >= current && has_solution::<P2>(row, goal - current, idx - 1)
}

#[inline]
fn truncate_number(x: u64, y: u64) -> Option<u64> {
    let m =
        if y < 10 {
            10
        } else if y < 100 {
            100
        } else {
            1000
        };
    
    if x % m == y {
        Some(x / m)
    } else {
        None
    }
}