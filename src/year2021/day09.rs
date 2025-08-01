use crate::util::grid::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut grid = Grid::parse_with_padding(input, b'9');
    let width = grid.width;
    let grid = &mut grid.vec;

    let mut p1 = 0;
    let mut basins =  Vec::with_capacity(300);

    for i in width..grid.len()-width {
        let node = grid[i];
        if node == b'9' || node >= grid[i-1] || node >= grid[i+1] || node >= grid[i-width] || node >= grid[i+width] {
            continue;
        }
        p1 += (node - b'0' + 1) as u32;
        basins.push(dfs(grid, width, i));
    }

    let n = basins.len();
    basins.select_nth_unstable(n-3);
    let p2 = basins.iter().rev().take(3).product();

    (p1, p2)
}

fn dfs(grid: &mut [u8], width: usize, node: usize) -> u32 {
    grid[node] = b'9'; 
    let mut count = 1;
    
    if grid[node+1] != b'9' {
        count += dfs(grid, width, node+1);
    }
    if grid[node-1] != b'9' {
        count += dfs(grid, width, node-1);
    }
    if grid[node+width] != b'9' {
        count += dfs(grid, width, node+width);
    }
    if grid[node-width] != b'9' {
        count += dfs(grid, width, node-width);
    }
    count
}