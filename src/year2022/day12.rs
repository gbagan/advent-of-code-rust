use std::collections::VecDeque;
use crate::util::grid::Grid;

pub fn solve(input: &str) -> (u32, u32) {
    let mut grid = Grid::parse(input).unwrap();
    let start = grid.vec.iter().position(|&c| c == b'E').unwrap();
    grid[start] = b'z';
    let mut p2 = u32::MAX;
    let mut p1 = 0;
    let mut queue = VecDeque::new();
    let mut seen = vec![false; grid.width * grid.height];

    queue.push_back((start, 0));
    while let Some((idx, dist)) = queue.pop_front() {
        let c1 = grid[idx];
        if c1 == b'a' {
            p2 = p2.min(dist);
        }
        if c1 == b'S' {
            p1 = dist;
            break; 
        }
        if seen[idx] {
            continue;
        }
        seen[idx] = true;

        grid.foreach_adjacent_index(idx, |idx2| {
            if can_climb(c1, grid[idx2]) {
                queue.push_back((idx2, dist + 1));
            }
        })
    } 
    (p1, p2)
}

fn can_climb(c1: u8, c2: u8) -> bool {
    match (c1, c2) {
        (b'a', b'S') => true,
        _ => c1 != b'E' && c1 <= c2 + 1
    }
}
