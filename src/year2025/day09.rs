use crate::util::{grid::Grid, iter::*, parser::*};
use ahash::HashMap;

pub fn solve(input: &str) -> (u64, u64) {
    let points: Vec<(u64, u64)> = input.iter_unsigned().tuples().collect();
    let p1 = part1(&points);
    let p2 = part2(&points);
    (p1, p2)
}

fn part1(points: &[(u64, u64)]) -> u64 {
    let mut p1 = 0;
    for (i, &(x1, y1)) in points[..points.len()-1].iter().enumerate() {
        for &(x2, y2) in &points[i+1..] {
            p1 = p1.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
        }
    }
    p1
}

fn part2(points: &[(u64, u64)]) -> u64 {
    let mut xs = Vec::with_capacity(2*points.len());
    let mut ys = Vec::with_capacity(2*points.len());
    for &(x, y) in points {
        xs.push(x);
        xs.push(x+1);
        ys.push(y);
        ys.push(y+1);
    }

    //let mut xs: Vec<_> = points.iter().map(|p| p.0).collect();
    //let mut ys: Vec<_> = points.iter().map(|p| p.1).collect();
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    let x_index: HashMap<_, _> =
        xs.iter().enumerate().map(|(i, &x)| (x, i+1)).collect();
    let y_index: HashMap<_, _> =
        ys.iter().enumerate().map(|(i, &y)| (y, i+1)).collect();

    let indices: Vec<_> = points
        .iter()
        .map(|(x, y)| (x_index[x], y_index[y]))
        .collect();

    let mut grid = Grid::new(x_index.len()+1, y_index.len()+1, b'*');

    let mut fill_line = |(x1, y1): (usize, usize), (x2, y2): (usize, usize)| {
        if x1 == x2 {
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            for y in ymin..ymax+1 {
                grid[(x1, y)] = b'#';
            }
        } else { // y1 == y2
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            for x in xmin..xmax+1 {
                grid[(x, y1)] = b'#';
            }
        }
    };

    for &[p1, p2] in indices.array_windows() {
        fill_line(p1, p2);
    }
    fill_line(indices[0], indices[indices.len()-1]);

    flood_grid(&mut grid);

    let mut empty_table = Grid::new(grid.width, grid.height, 0);
    let w = empty_table.width;
    for j in 1..empty_table.height {
        for i in 1..empty_table.width {
            let index = j * empty_table.width + i;
            empty_table[index] =
                empty_table[index-1]
                + empty_table[index-w]
                - empty_table[index-w-1]
                + (grid[index] == b'.') as u32;
        }
    }

    let mut best_area = 0;
    for i in 0..points.len() - 1 {
        let (x1, y1) = indices[i];
        for j in i+1..points.len() {
            let (x2, y2) = indices[j];
            let xmin = x1.min(x2);
            let xmax = x1.max(x2);
            let ymin = y1.min(y2);
            let ymax = y1.max(y2);
            let empty_count =
                empty_table[(xmax, ymax)]
                + empty_table[(xmin-1, ymin-1)]
                - empty_table[(xmin-1, ymax)]
                - empty_table[(xmax, ymin-1)];
            if empty_count == 0 {
                let (x1, y1) = points[i];
                let (x2, y2) = points[j];
                best_area = best_area.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
            }
        }
    }
    best_area
}

fn flood_grid(grid: &mut Grid<u8>) {
    grid[(0, 0)] = b'.';
    let mut stack = vec![(0, 0)];
    while let Some((x, y)) = stack.pop() {
        for (x2, y2) in [(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1)]
        {
            if x2 < grid.width && y2 < grid.height && grid[(x2, y2)] == b'*' {
                grid[(x2, y2)] =  b'.';
                stack.push((x2, y2));
            }
        }
    }
}