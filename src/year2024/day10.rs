use crate::util::grid::Grid;
use ahash::{HashSet, HashSetExt};

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse_with_padding(input, b'#').unwrap();
    let width = grid.width;
    let grid = grid.vec;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = Vec::with_capacity(16);
    let mut level0 = HashSet::with_capacity(16);

    for (i, &c) in grid.iter().enumerate() {
        if c == b'9' {
            let (a, b) = unsafe { hike_score(&grid, width, i, &mut stack, &mut level0) };
            p1 += a;
            p2 += b;
        }
    }
    
    (p1, p2)
}

#[inline(always)]
unsafe fn hike_score(grid: &[u8], width: usize, start: usize, stack: &mut Vec<(usize, u8)>, level0: &mut HashSet<usize>) -> (u32, u32) {
    let mut rating = 0;
    stack.push((start , b'9'));
    while let Some((current, level)) = stack.pop() {
        if level == b'0' {
            rating += 1;
            level0.insert(current);
            continue;
        }
        let next = current - 1;
        if *grid.get_unchecked(next) == level - 1 {
            stack.push((next, level-1));
        }
        
        let next = current + 1;
        if *grid.get_unchecked(next) == level - 1 {
            stack.push((next, level-1));
        }

        let next = current - width;
        if *grid.get_unchecked(next) == level - 1 {
            stack.push((next, level-1));
        }
        
        let next = current + width;
        if *grid.get_unchecked(next) == level - 1 {
            stack.push((next, level-1));
        }
    }
    let score = level0.len();
    level0.clear();

    (score as u32, rating)
}