// part 1: BFS, part 2: DFS

use crate::util::parser::*;

const SIZE: usize = 73;
const START: usize = SIZE+1;
const END: usize =  (SIZE-1) * SIZE - 2;

pub fn solve(input: &str) -> (u32, String) {
    let bytes: Vec<_> = input.iter_unsigned().array_chunks().collect();

    let p1 = part1(&bytes);
    let p2 = part2(&bytes);

    (p1, p2)
}

fn part1(bytes: &[[u8; 2]]) -> u32 {
    let mut grid = vec![b'.'; SIZE*SIZE];
    for i in 0..SIZE {
        grid[i] = b'#';
        grid[i*SIZE] = b'#';
        grid[(i+1)*SIZE-1] = b'#';
        grid[SIZE*(SIZE-1)+i] = b'#';
    }

    for &[x, y] in &bytes[..1024] {
        grid[(y as usize + 1) * SIZE + x as usize + 1] = b'#';
    }

    let mut queue1 = Vec::with_capacity(1000);
    let mut queue2 = Vec::with_capacity(1000);
    queue1.push(START);
    grid[START] = b'#';

    let mut dist = 0;
    loop {
        for &node in &queue1 {
            if node == END {
                return dist;
            }
            for next in [node + 1, node - 1, node + 73, node - 73] {
                if grid[next] == b'.' {
                    grid[next] = b'#';
                    queue2.push(next);
                }
            }
        }
        dist += 1;
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();
    }
}

fn part2(bytes: &[[u8; 2]]) -> String {
    let mut grid = vec![b'.'; SIZE*SIZE];
    for i in 0..SIZE {
        grid[i] = b'#';
        grid[i*SIZE] = b'#';
        grid[(i+1)*SIZE-1] = b'#';
        grid[SIZE*(SIZE-1)+i] = b'#';
    }

    for &[x, y] in bytes {
        grid[(y as usize + 1) * SIZE + x as usize + 1] = b'#';
    }

    let mut stack = Vec::with_capacity(1000);
    dfs(&mut grid, &mut stack, START);

    let &[x, y] = bytes.iter().rfind(|&&[x, y]| {
        let node = (y as usize + 1) * SIZE + x as usize + 1;
        grid[node] = b'.';
        (grid[node-1] == b'$' || grid[node+1] == b'$' || grid[node-SIZE] == b'$' || grid[node+SIZE] == b'$')
                && dfs(&mut grid, &mut stack, node)
    }).unwrap();
    format!("{x},{y}")
}

#[inline]
fn dfs(grid: &mut [u8], stack: &mut Vec<usize>, start: usize) -> bool {
    stack.push(start);
    grid[start] = b'$';
    while let Some(node) = stack.pop() {
        if node == END {
            return true;
        }
        grid[node] = b'$';
        for next in [node + 1, node - 1, node + SIZE, node - SIZE] {
            if grid[next] == b'.' {
                grid[node] = b'$';
                stack.push(next);
            }
        }
    }
    false
}