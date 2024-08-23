use crate::util::grid::Grid;

fn parse_grid(input: &str) -> Option<(usize, usize)> {
    let grid = Grid::parse(input);
    let mut rows: Vec<u32> = Vec::with_capacity(grid.height);
    let mut columns: Vec<u32> = Vec::with_capacity(grid.width);
    for _ in 0..grid.height {
        rows.push(0);
    }
    for _ in 0..grid.width {
        columns.push(0);
    }
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


    Some((p1, p2))
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


pub fn parse(input: &str) -> Option<(usize, usize)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for (a, b) in input.split("\n\n").filter_map(parse_grid) {
        p1 += a;
        p2 += b;
    };
    Some((p1, p2))
}

pub fn part1(input: &(usize, usize)) -> Option<usize> {
    Some(input.0)
}

pub fn part2(input: &(usize, usize)) -> Option<usize> {
    Some(input.1)
}