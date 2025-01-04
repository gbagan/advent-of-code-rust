use crate::util::grid::Grid;
use ahash::{HashSet, HashSetExt};

const WIDTH: usize = 47;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse_with_padding2::<45, 45>(input, b'#');
    let grid = grid.vec;

    let mut p1 = 0;
    let mut p2 = 0;
    let mut stack = Vec::with_capacity(16);
    let mut level9 = HashSet::with_capacity(16);

    for (i, &c) in grid.iter().enumerate() {
        if c == b'0' {
            let (a, b) = unsafe { hike_score(&grid, i, &mut stack, &mut level9) };
            p1 += a;
            p2 += b;
        }
    }
    
    (p1, p2)
}

unsafe fn hike_score(grid: &[u8], start: usize, stack: &mut Vec<(usize, u8)>, level9: &mut HashSet<usize>) -> (u32, u32) {
    let mut rating = 0;
    stack.push((start , b'0'));
    while let Some((current, level)) = stack.pop() {
        if level == b'9' {
            rating += 1;
            level9.insert(current);
            continue;
        }
        let next = current - 1;
        if *grid.get_unchecked(next) == level + 1 {
            stack.push((next, level+1));
        }
        
        let next = current + 1;
        if *grid.get_unchecked(next) == level + 1 {
            stack.push((next, level+1));
        }

        let next = current - WIDTH;
        if *grid.get_unchecked(next) == level + 1 {
            stack.push((next, level+1));
        }
        
        let next = current + WIDTH;
        if *grid.get_unchecked(next) == level + 1 {
            stack.push((next, level+1));
        }
    }
    let score = level9.len();
    level9.clear();
    stack.clear();
    (score as u32, rating)
}