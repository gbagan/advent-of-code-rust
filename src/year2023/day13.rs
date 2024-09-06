use anyhow::*;
use crate::util::grid::Grid;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for pair in input.split("\n\n").map(parse_grid) {
        let (a, b) = pair?;
        p1 += a;
        p2 += b;
    };
    Ok((p1, p2))
}

fn parse_grid(input: &str) -> Result<(usize, usize)> {
    let grid = Grid::parse(input);
    let mut rows: Vec<u32> = vec![0; grid.height];
    let mut columns: Vec<u32> = vec![0; grid.width];
    for x in 0..grid.width {
        for y in 0..grid.height {
            let b = if grid[(x, y)] == b'#' {1} else {0};
            columns[x] = columns[x] * 2 + b;
            rows[y] = rows[y] * 2 + b;
        }
    }
    let p1 = if let Some (v) = reflect(&columns) {
        v
    } else if let Some(v) = reflect(&rows) {
        v * 100
    } else {
        0
    };
    let p2 = if let Some (v) = reflect2(&columns) {
        v
    } else if let Some(v) = reflect2(&rows) {
        v * 100
    } else {
        0
    };


    Ok((p1, p2))
}

fn reflect(encoding: &[u32]) -> Option<usize> {
    let n = encoding.len();
    (1..n).find(|&i| (0..i.min(n - i)).all(|j|
        encoding[i - j - 1] == encoding[i + j])
    )
}

fn reflect2(encoding: &[u32]) -> Option<usize> {
    let n = encoding.len();
    (1..n).find(|&i| {
        let mut one_diff = false;
        for j in 0..i.min(n - i) {
            let diff = encoding[i - j - 1] ^ encoding[i + j];
            if diff != 0 {
                if diff & (diff-1) != 0 || one_diff {
                    return false;
                } else {
                    one_diff = true;
                }

            }
        }
        one_diff
    })
}