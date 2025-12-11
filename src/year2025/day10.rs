use std::collections::VecDeque;
use crate::util::{grid::Grid, parallel::*, parser::*};

struct Input {
    size: usize,
    goal: u32,
    buttons: Vec<u32>,
    matrix: Grid<i32>,
    part2: Vec<i32>,
}

fn parse_line(line: &str) -> Input {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    let lights = tokens[0].as_bytes();
    let lights = &lights[1..lights.len()-1];
    let mut goal = 0;
    for (i, &light) in lights.iter().enumerate() {
        if light == b'#' {
            goal |= 1 << i;
        }
    }
    let size = lights.len();
    let buttons: Vec<u32> = tokens[1..tokens.len()-1]
        .iter()
        .map(|token| token.iter_unsigned::<u32>().fold(0, |acc, i| acc | 1 << i))
        .collect();
    
    let matrix = Grid::generate(buttons.len(), size, |c, r| {
        ((buttons[c] & 1 << r) != 0) as i32
    });

    let part2: Vec<_> = tokens[tokens.len()-1].iter_unsigned().collect();

    Input { goal, size, buttons, matrix, part2 }
}

fn part1(input: &Input) -> u32 {
    let mut seen = vec![false; 1 << input.size];
    seen[input.goal as usize] = true;
    let mut queue = VecDeque::new();
    queue.push_back((input.goal, 0, 0));

    while let Some((current, index, steps)) = queue.pop_front() {
        if current == 0 {
            return steps
        }
        for j in index..input.buttons.len() {
            let next = current ^ input.buttons[j];
            if !seen[next as usize] {
                seen[next as usize] = true;
                queue.push_back((next, index+1, steps+1));
            }
        }
    }

    unreachable!();
}


fn part2(input: &Input) -> i32 {
    let (mat, is_pivot) = gauss(&input.matrix, &input.part2);
    
    let min_joltage: Vec<_> = (0..input.matrix.width).map(|i| 
        (0..input.matrix.height).map(|j| {
            if input.matrix[(i, j)] == 1 { input.part2[j] } else { i32::MAX }
        }).min().unwrap()
    ).collect();

    let mut start_row = mat.height - 1;
    while (0..mat.width-1).all(|i| mat[(i, start_row)] == 0) {
        start_row -= 1;
    }

    let mut solution = vec![0; mat.width-1];
    let mut res: Vec<_> = (0..mat.height)
        .map(|i| mat[(mat.width-1, i)])
        .collect();

    let non_pivot_count = is_pivot.iter().filter(|&&b| !b).count();
    //let limit = if non_pivot_count <= 2 { 200 } else { 30 };

    let a = explore(&mat, &is_pivot, &min_joltage, &mut solution, &mut res, start_row, mat.width-2);
    if a == i32::MAX {
    //println!("ouaf {} {} {} {}", input.matrix.width, matrix.height, buttons.len(), part2.len());
    for row in 0..input.matrix.height {
        for col in 0..input.matrix.width {
            //println!("meuh {row} {col} {} {}", matrix.height, matrix.width);
            print!("{} ", input.matrix[(col, row)]);
        }
        println!("{}", input.part2[row]);
    }
    println!("{non_pivot_count}");


    for row in 0..mat.height {
        for col in 0..mat.width {
            //println!("meuh {row} {col} {} {}", matrix.height, matrix.width);
            print!("{} ", mat[(col, row)]);
        }
        println!("");
    }
    println!("");

    }


    a
}

pub fn solve(input: &str) -> (u32, i32) {
    let lines: Vec<_> = input.lines().collect();
    lines
        .par_iter()
        .map(|line| {
            let inp = parse_line(line);
            let p1 = part1(&inp);
            let p2 = part2(&inp);
            (p1, p2)
        }).reduce(|| (0, 0), |(a, b), (c, d)| (a+c, b+d))
}

fn gauss(matrix: &Grid<i32>, b: &[i32]) -> (Grid<i32>, Vec<bool>) {
    let n = matrix.height;
    let m = matrix.width;
    let mut is_pivot = vec![false; matrix.width];

    let mut mat = Grid::generate(m+1, n, |c, r|
        if c == m { b[r] } else { matrix[(c, r)] }
    );

    let mut current_row = 0;

    for col in 0..m {
        if current_row >= n {
            break;
        } 
        let Some(pivot_row) = (current_row..n)
            .find(|&row| mat[(col, row)] != 0) else {
            continue
        };

        for i in 0..m+1 {
            let tmp = mat[(i, current_row)];
            mat[(i, current_row)] = mat[(i, pivot_row)];
            mat[(i, pivot_row)] = tmp;
        }

        is_pivot[col] = true;

        for row in current_row+1..n {
            let f = mat[(col, row)];
            if f != 0 {
                let f = mat[(col, row)];
                let pivot_val = mat[(col, current_row)];

                for j in col..m+1 {
                    mat[(j, row)] = pivot_val * mat[(j, row)] - f * mat[(j, current_row)];
                }
            }
        }
        current_row += 1
    }

    (mat, is_pivot)
}

fn explore(mat: &Grid<i32>, is_pivot: &[bool], min_joltage: &[i32], solution: &mut [i32], b: &mut [i32],
                mut row: usize, mut col: usize) -> i32
{
    loop {    
        if col == usize::MAX {
            return solution.iter().sum()
        }
        if !is_pivot[col] {
            break;
        }

        let v = mat[(col, row)];
        let u = b[row];
        if u % v != 0 {
            return i32::MAX;
        }
        let w = u / v;
        if w < 0 {
            return i32::MAX;
        }
        //let mut solution = solution.to_vec();
        //let mut res = res.to_vec();
        solution[col] = w;
        for row2 in 0..row {
            b[row2] -= w * mat[(col, row2)];
        }
        row -= 1;
        col -= 1;
    }
    let mut best_solution = i32::MAX;
    let mut solution2 = solution.to_vec();
    let mut b2 = b.to_vec();

    for w in 0..min_joltage[col]+1 {
        solution2.copy_from_slice(&solution);
        b2.copy_from_slice(&b);
        solution2[col] = w;
        for row2 in 0..row+1 {
            b2[row2] -= w * mat[(col, row2)];
        }
        best_solution = best_solution.min(
            explore(mat, is_pivot, min_joltage, &mut solution2, &mut b2, row, col-1)
        );
    }
    best_solution
}