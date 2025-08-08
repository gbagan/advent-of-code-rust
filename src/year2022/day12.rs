use crate::util::grid::Grid;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse_with_padding(input, b'#');
    let width = grid.width;
    let mut grid = grid.vec;
    let start = grid.iter().position(|&c| c == b'E').unwrap();
    grid[start] = b'z';
    let mut p2 = u32::MAX;
    let mut p1 = 0;
    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();
    let mut seen = vec![false; grid.len()];
    seen[start] = true;
    let mut dist = 0;

    queue1.push(start);
    'outer: while !queue1.is_empty() {
        for &idx in &queue1 {
            let c1 = grid[idx];
            if c1 == b'a' {
                p2 = p2.min(dist);
            }
            if c1 == b'S' {
                p1 = dist;
                break 'outer; 
            }
            seen[idx] = true;

            let idx2 = idx-1;
            if !seen[idx2] && can_climb(c1, grid[idx2]) {
                seen[idx2] = true;
                queue2.push(idx2);
            }
            let idx2 = idx+1;
            if !seen[idx2] && can_climb(c1, grid[idx2]) {
                seen[idx2] = true;
                queue2.push(idx2);
            }
            let idx2 = idx+width;
            if !seen[idx2] && can_climb(c1, grid[idx2]) {
                seen[idx2] = true;
                queue2.push(idx2);
            }
            let idx2 = idx-width;
            if !seen[idx2] && can_climb(c1, grid[idx2]) {
                seen[idx2] = true;
                queue2.push(idx2);
            }
        }
        dist += 1;
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();
    }
    (p1, p2)
}

fn can_climb(c1: u8, c2: u8) -> bool {
    match (c1, c2) {
        (_, b'#') => false,
        (b'a', b'S') => true,
        _ => c1 <= c2 + 1
    }
}
