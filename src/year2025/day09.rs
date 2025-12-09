use crate::util::{grid::Grid, iter::*, parallel::*, parser::*};
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
    let mut xs: Vec<_> = points.iter().map(|p| p.0).collect();
    let mut ys: Vec<_> = points.iter().map(|p| p.1).collect();
    xs.sort_unstable();
    ys.sort_unstable();
    let xpositions: HashMap<_, _> =
        xs.iter().step_by(2).enumerate().map(|(i, &x)| (x, i+1)).collect();
    let ypositions: HashMap<_, _> =
        ys.iter().step_by(2).enumerate().map(|(i, &y)| (y, i+1)).collect();

    let indices: Vec<_> = points
        .iter()
        .map(|(x, y)| (xpositions[x], ypositions[y]))
        .collect();

    let mut grid = Grid::new(xpositions.len()+2, ypositions.len()+2, b'@');

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

    //println!("{}", grid.draw());

    (0..points.len()-1)
        .par_iter()
        .map(|i| {
            let mut max = 0;
            let (x1, y1) = points[i];
            let (px1, py1) = indices[i];
            'outer: for j in i+1..points.len() {
                let (x2, y2) = points[j];
                let (px2, py2) = indices[j];
                let xmin = px1.min(px2);
                let xmax = px1.max(px2);
                let ymin = py1.min(py2);
                let ymax = py1.max(py2);
                for px in xmin..xmax+1 {
                    if grid[(px, ymin)] == b'.' || grid[(px, ymax)] == b'.' {
                        continue 'outer;
                    }
                }
                for py in ymin..ymax+1 {
                    if grid[(xmin, py)] == b'.' || grid[(xmin, py)] == b'.' {
                        continue 'outer;
                    }
                }
                max = max.max((x1.abs_diff(x2) + 1) * (y1.abs_diff(y2) + 1));
            }
            max
        }).reduce(|| 0, |&a, &b| a.max(b))
}

fn flood_grid(grid: &mut Grid<u8>) {
    grid[(0, 0)] = b'.';
    let mut stack = vec![(0, 0)];
    while let Some((x, y)) = stack.pop() {
        for (x2, y2) in [(x-1, y-1), (x-1, y), (x-1, y+1), (x, y-1), (x, y+1),
            (x+1, y-1), (x+1, y), (x+1, y+1)]
        {
            if x2 < grid.width && y2 < grid.height && grid[(x2, y2)] == b'@' {
                grid[(x2, y2)] =  b'.';
                stack.push((x2, y2));
            }    
        }
    }
}