// A* algorithm

use anyhow::*;
use crate::util::{coord::Coord, grid::Grid};

type Point = Coord<usize>;

pub fn solve(input: &str) -> Result<(u16, u16)> {
    let grid= Grid::parse_with_padding(input, b'#')?;
    let p1 = astar(&grid, 1, 3).context("Part 1: No solution found")?;
    let p2 = astar(&grid, 4, 10).context("Part 2: No solution found")?;
    Ok((p1, p2))
}

const VERTICAL: usize = 0;
const HORIZONTAL: usize = 1;

fn astar(grid: &Grid<u8>, min_dist: usize, max_dist: usize) -> Option<u16> {
    let start = grid.width + 1;
    let goal = Point::new(grid.width - 2, grid.height - 2);
    let mut heuristic = vec![0; grid.width * grid.height];
    for y in 1..grid.height - 1 {
        for x in 1..grid.width - 1 {
            let p = Point::new(x, y);
            let diff = goal - p;
            let dist = p.manhattan(goal);
            let penalty = diff.x.abs_diff(diff.y) * min_dist / max_dist;
            heuristic[y * grid.width + x] = dist + penalty;
        }
    }
    
    let max_heuristic = heuristic.iter().max()?;
    let mut queue = BucketQueue::new(*max_heuristic + 100);
    let mut costs = Grid::new(grid.width, grid.height, [u16::MAX; 2]);
    queue.insert(heuristic[start], (start, HORIZONTAL, 0));
    queue.insert(heuristic[start], (start, VERTICAL, 0));

    let goal = (grid.height - 1) * grid.width - 2;
    while let Some((_, (node, direction, cost))) = queue.pop() {
        if node == goal {
            return Some(cost);
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
                    queue.insert(next_cost as usize + h, (next, next_dir, next_cost));
                }
            }
        }
    } 

    None
}


struct BucketQueue<A> {
    min: Option<usize>,
    buckets: Vec<Vec<A>>,
}

impl <A> BucketQueue<A> {
    fn new(n: usize) -> BucketQueue<A> {
        let buckets = (0..n).map(|_| vec!()).collect();
        BucketQueue { buckets, min: None }
    }

    fn insert(&mut self, key: usize, val: A) {
        if key >= self.buckets.len() {
            self.buckets
                .resize_with(key + 1, || Vec::new());
        }
        
        self.buckets[key].push(val);
        if self.min.map(|m| m > key).unwrap_or(true) {
            self.min = Some(key);
        }
    }

    fn pop(&mut self) -> Option<(usize, A)> {
        if let Some(min) = self.min {
            let v = self.buckets[min].pop()?;
            self.min = (min..self.buckets.len()).find(|&i| !self.buckets[i].is_empty());
            Some((min, v))
        } else {
            None
        }
    }
}