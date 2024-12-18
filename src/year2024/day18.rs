use anyhow::*;
use crate::util::parser::*;
use std::collections::VecDeque;

const SIZE: usize = 73;
const START: usize = SIZE+1;
const END: usize =  (SIZE-1) * SIZE - 2;

pub fn solve(input: &str) -> Result<(u32, String)> {
    let bytes: Vec<_> = input.iter_unsigned().array_chunks().collect();

    let p1 = part1(&bytes).context("Part 1: No solution found")?;
    let p2 = part2(&bytes);

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

    let mut uf: [u16; SIZE*SIZE] = std::array::from_fn(|i| i as u16);
    for i in 0..SIZE*SIZE {
        if grid[i] == b'#' {
            continue;
        }
        if i < SIZE*SIZE - 1 && grid[i+1] == b'.' {
            union(&mut uf, i, i+1);
        }
        if i < SIZE*SIZE - SIZE && grid[i+SIZE] == b'.' {
            union(&mut uf, i, i+SIZE);
        }
    }

    for i in (0..bytes.len()).rev() {
        if find(&mut uf, START) == find(&mut uf, END) {
            return format!("{},{}", bytes[i+1][0], bytes[i+1][1]); 
        }
        let [x, y] = bytes[i];
        let node = (y as usize + 1) * SIZE + x as usize + 1;
        if grid[node-1] == b'.' {
            union(&mut uf, node, node-1);
        }
        if grid[node+1] == b'.' {
            union(&mut uf, node, node+1);
        }
        if grid[node-SIZE] == b'.' {
            union(&mut uf, node, node-SIZE);
        }
        if grid[node+SIZE] == b'.' {
            union(&mut uf, node, node+SIZE);
        }
        grid[node] = b'.';
    }
    unreachable!();
}

fn find(uf: &mut[u16], node: usize) -> u16 {
    let parent = uf[node] as usize;
    if parent != node {
        uf[node] = find(uf, parent);
    }
    uf[node]
}

#[inline]
fn union(uf: &mut [u16], x: usize, y: usize) {
    let xroot = find(uf, x);
    let yroot = find(uf, y);
    if xroot != yroot {
        uf[xroot as usize] = yroot;
    }
}