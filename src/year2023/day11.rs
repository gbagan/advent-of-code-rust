// use the following formula
// given a non decreasing sequence v_1, ... v_n
// sum_{1 <= i < j <= n} |v_i - v_j| = sum_{1 <= i <= n} v_i * (2i - n - 1)

use crate::util::grid::Grid;

pub fn solve(input: &str) -> (i64, i64){
    let grid = Grid::parse(input).unwrap();
    let mut galaxy_xs: Vec<i64> = Vec::new();
    let mut galaxy_ys: Vec<i64> = Vec::new();
    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    let mut nb_empty_rows = 0;
    let mut nb_empty_cols = 0;
    
    for y in 0..grid.height {
        let mut is_empty = true;
        for x in 0..grid.width {
            if grid[(x, y)] == b'#' {
                galaxy_xs.push(x as i64);
                galaxy_ys.push(y as i64);
                is_empty = false;
            }
        }
        if is_empty {
            nb_empty_rows += 1;
        }
        empty_rows.push(nb_empty_rows);
    }

    for x in 0..grid.width {
        let mut is_empty = true;
        for y in 0..grid.height {
            if grid[(x, y)] == b'#' {
                is_empty = false;
            }
        }
        if is_empty {
            nb_empty_cols += 1;
        }
        empty_cols.push(nb_empty_cols);
    }

    galaxy_xs.sort_unstable();
    galaxy_ys.sort_unstable();
    let n = galaxy_xs.len();

    let mut p1 = 0;
    let mut p2 = 0;
    
    for i in 0..n {
        let c = 2 * (i as i64) - (n as i64) + 1;
        let expansion = empty_cols[galaxy_xs[i] as usize] + empty_rows[galaxy_ys[i] as usize];
        p1 += c * (galaxy_xs[i] + galaxy_ys[i] + expansion); 
        p2 += c * (galaxy_xs[i] + galaxy_ys[i] + 999_999 * expansion);
    }

    (p1, p2)
}