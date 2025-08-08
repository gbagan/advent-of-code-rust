use crate::util::grid::Grid;
const NB_STEPS: u64 = 26_501_365;

pub fn solve(input: &str) -> (u64, u64) {
    let grid = Grid::parse_with_padding(input, b'#');
    let start = grid.width / 2 + grid.width *  (grid.height / 2);
    
    let (even_inside, odd_inside, even_outside, odd_outside) =
            bfs(&grid, &[start], 65, u64::MAX);
    let even = even_inside + even_outside;
    let odd = odd_inside + odd_outside;
    let p1 = even_inside;

    let n = NB_STEPS / (grid.width-2) as u64;
    let remainder = NB_STEPS % (grid.width-2) as u64;
    // debug_assert_eq!(remainder * 2 + 1, grid.height as u64);

    let corners = [
        grid.width + 1,
        2 * grid.width - 2,
        grid.width * (grid.height - 2) + 1,
        grid.width * (grid.height - 1) - 2,
    ];


    let (even_corner,..) = bfs(&grid, &corners, remainder-1, remainder-1);
    let odd_corner = odd_outside;
    let p2 = (n+1) * (n+1) * odd + n * n * even + n * even_corner - (n+1) * odd_corner;

    (p1, p2)
}

fn bfs(grid: &Grid<u8>, starts: &[usize], inside_limit: u64, limit: u64) -> (u64, u64, u64, u64) {
    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();
    let mut seen = vec![false; grid.width * grid.height];
    let mut dist = 0;
    for &start in starts {
        queue1.push(start);
        seen[start] = true;
    }
    let mut even_inside = 0;
    let mut even_outside = 0;
    let mut odd_inside = 0;
    let mut odd_outside = 0;
    
    while !queue1.is_empty() && dist <= limit {
        for &index in &queue1 {
            seen[index] = true;
            if dist <= inside_limit {
                if dist.is_multiple_of(2) {
                    even_inside += 1;
                } else {
                    odd_inside += 1;
                }
            } else if dist.is_multiple_of(2) {
                even_outside += 1;
            } else {
                odd_outside += 1;
            }
            for next in [index-1, index+1, index-grid.width, index+grid.width] {
                if grid[next] == b'.' {
                    if !seen[next] {
                        queue2.push(next);
                        seen[next] = true;
                    }
                }
            }
        }
        dist += 1;
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();
    }
    (even_inside, odd_inside, even_outside, odd_outside)
}