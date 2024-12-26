// A* algorithm

use anyhow::*;
use crate::util::{coord::Coord, grid::Grid};

type Point = Coord<usize>;

pub fn solve(input: &str) -> Result<(u16, u16)> {
    let grid= Grid::parse_with_padding(input, b'#')?;
    let p1 = astar(&grid, 1, 3);
    let p2 = astar(&grid, 4, 10);
    Ok((p1, p2))
}

const VERTICAL: usize = 0;
const HORIZONTAL: usize = 1;

fn astar(grid: &Grid<u8>, min_dist: usize, max_dist: usize) -> u16 {
    let start = grid.width + 1;
    let goal = Point::new(grid.width - 2, grid.height - 2);
    let mut heuristic = vec![0; grid.width * grid.height];
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let p = Point::new(x, y);
            let dist = p.manhattan(goal);
            heuristic[y * grid.width + x] = dist; // + penalty;
        }
    }
    
    let mut queue = vec![vec!(); 100];
    let mut costs = Grid::new(grid.width, grid.height, [u16::MAX; 2]);
    queue[heuristic[start] % 100].push((start, HORIZONTAL, 0));
    queue[heuristic[start] % 100].push((start, VERTICAL, 0));

    let goal = (grid.height - 1) * grid.width - 2;
    let mut index = 0;

    loop {
        while let Some((node, direction, cost)) = queue[index].pop() {
            if node == goal {
                return cost;
            }
            if cost > costs[node][direction] {
                continue;
            }

            let dirs = if direction == VERTICAL {
                [0usize.wrapping_sub(grid.width), grid.width]}
            else {
                [usize::MAX, 1] // [-1, 1]
            };
            for dir2 in dirs {
                let mut psum = 0;
                let mut next = node;
                let next_dir = 1 - direction;
                for i in 1..max_dist+1 {
                    next += dir2;
                    if grid[next] == b'#' {
                        break;
                    }
                    psum += grid[next] - b'0';
                    if i < min_dist {
                        continue;
                    }
                    let h = heuristic[next];
                    let next_cost = cost + psum as u16;
                    if next_cost < costs[next][next_dir] {
                        costs[next][next_dir] = next_cost;
                        queue[(next_cost as usize + h) % 100].push((next, next_dir, next_cost));
                    }
                }
            }
        }
        index += 1;
        if index == 100 {
            index = 0;
        }
    }
}