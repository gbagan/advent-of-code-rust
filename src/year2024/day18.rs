use anyhow::*;
use crate::util::parser::*;
use std::collections::VecDeque;

const SIZE: usize = 73;
const START: usize = SIZE+1;
const END: usize =  (SIZE-1) * SIZE - 2;

pub fn solve(input: &str) -> Result<(u32, String)> {
    let bytes: Vec<_> = input.iter_unsigned().array_chunks().collect();

    let p1 = part1(&bytes).context("Part 1: No solution found")?;
    let p2 = part2(&bytes).context("Part 2: No solution found")?;

    Ok((p1, p2))
}

fn part1(bytes: &[[u8; 2]]) -> Option<u32> {
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

    let mut queue = VecDeque::with_capacity(1000);
    queue.push_back((0, START));

    while let Some((dist, node)) = queue.pop_front() {
        if node == END {
            return Some(dist);
        }
        if grid[node] == b'#' {
            continue;
        }
        grid[node] = b'#';
        for next in [node + 1, node - 1, node + 73, node - 73] {
            if grid[next] == b'.' {
                queue.push_back((dist+1, next));
            }
        }
    }
    None
}

fn part2(bytes: &[[u8; 2]]) -> Option<String> {
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
    dfs_aux(&mut grid, &mut stack, START);

    bytes.iter().rev().find(|&&[x, y]| {
        let node = (y as usize + 1) * SIZE + x as usize + 1;
        grid[node] = b'.';
        (grid[node-1] == b'$' || grid[node+1] == b'$' || grid[node-SIZE] == b'$' || grid[node+SIZE] == b'$')
                && dfs_aux(&mut grid, &mut stack, node)
    }).map(|[x, y]| format!("{x},{y}"))
}

#[inline]
fn dfs_aux(grid: &mut [u8], stack: &mut Vec<usize>, node: usize) -> bool {
    stack.push(node);
    while let Some(node) = stack.pop() {
        if node == END {
            return true;
        }
        if grid[node] != b'.' {
            continue;
        }
        grid[node] = b'$';
        for next in [node + 1, node - 1, node + SIZE, node - SIZE] {
            if grid[next] == b'.' {
                stack.push(next);
            }
        }
    }
    false
}