use crate::util::{coord::Coord, grid::Grid};

pub fn parse(input: &str) -> Option<(i64, i64)>{
    let grid = Grid::parse(input);
    let mut galaxies = Vec::new();
    let mut empty_rows: Vec<i64> = Vec::new();
    let mut empty_cols: Vec<i64> = Vec::new();
    let mut nb_empty_rows = 0;
    let mut nb_empty_cols = 0;

    for y in 0..grid.height {
        let mut is_empty = true;
        for x in 0..grid.width {
            if grid[(x, y)] == b'#' {
                galaxies.push(Coord::new(x as i32, y as i32));
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

    let mut p1 = 0;
    let mut p2 = 0;

    for g1 in &galaxies {
        for g2 in &galaxies {
            let dist =  g1.manhattan(&g2) as i64;
            let expansion = (empty_cols[g1.x as usize] - empty_cols[g2.x as usize]).abs()
                            + (empty_rows[g1.y as usize] - empty_rows[g2.y as usize]).abs();
            p1 += dist + expansion;
            p2 += dist + 999_999 * expansion;
        }
    }

    Some((p1/2, p2/2))
}


pub fn part1(input: &(i64, i64)) -> Option<i64> {
    Some(input.0)
}

pub fn part2(input: &(i64, i64)) -> Option<i64> {
    Some(input.1)
}