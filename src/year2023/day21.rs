use crate::util::{coord::Coord, grid::Grid};
use std::collections::VecDeque;

const NB_STEPS: u64 = 26_501_365;

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let grid = Grid::parse(input);
    debug_assert!(grid.height == grid.width); 
    let start = Coord::new(grid.width as i32 / 2, grid.height as i32 / 2);
    debug_assert_eq!(grid[start], b'S');
    
    let (even_inside, odd_inside, even_outside, odd_outside) =
            bfs(&grid, &[start], 65, u64::MAX);
    let even = even_inside + even_outside;
    let odd = odd_inside + odd_outside;
    let p1 = even_inside;

    let n = NB_STEPS / grid.width as u64;
    let remainder = NB_STEPS % grid.width as u64;
    debug_assert_eq!(remainder * 2 + 1, grid.height as u64);

    let corners = vec!(
        Coord::new(0, 0),
        Coord::new(0, grid.height as i32 - 1),
        Coord::new(grid.width as i32 - 1, 0),
        Coord::new(grid.width as i32 - 1, grid.height as i32 - 1),
    );


    let (even_corner,..) = bfs(&grid, &corners, remainder-1, remainder-1);
    let odd_corner = odd_outside;
    let p2 = (n+1) * (n+1) * odd + n * n * even + n * even_corner - (n+1) * odd_corner;

    Some((p1, p2))
}

fn bfs(grid: &Grid<u8>, starts: &[Coord], inside_limit: u64, limit: u64) -> (u64, u64, u64, u64) {
    let mut queue = VecDeque::new();
    for &start in starts {
        queue.push_back((start, 0));
    }
    let mut seen = Grid::new(grid.width, grid.height, false);
    let mut even_inside = 0;
    let mut even_outside = 0;
    let mut odd_inside = 0;
    let mut odd_outside = 0;
    while let Some((node, dist)) = queue.pop_front() {
        if dist > limit {
            break;
        }
        if seen[node] {
            continue;            
        }
        seen[node] = true;
        if dist <= inside_limit {
            if (node.x + node.y) % 2 == 0 {
                even_inside += 1;
            } else {
                odd_inside += 1;
            }
        } else if (node.x + node.y) % 2 == 0 {
             even_outside += 1;
        } else {
            odd_outside += 1;
        }
        for nbor in node.adjacent() {
            if grid.contains(nbor) && grid[nbor] == b'.' {
                queue.push_back((nbor, dist +1));
            }
        }
    }
    (even_inside, odd_inside, even_outside, odd_outside)
}