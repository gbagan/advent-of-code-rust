use anyhow::*;
use crate::util::{grid::*, heap::*};

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse(input)?;
    let start = grid.width * (grid.height-2) + 1;
    let end = grid.width * 2 - 2;

    let mut distances = vec![(u32::MAX, u32::MAX); grid.vec.len()];

    let p1 = part1(&grid, &mut distances, start, end).context("Part 1: No solution found")?;
    let p2 = part2(grid.width, &distances, end);
    Ok((p1, p2))
}

fn part1(grid: &Grid<u8>, distances: &mut [(u32, u32)], start: usize, end: usize) -> Option<u32> {
    let width = grid.width;
    let grid = &grid.vec;
    let up = 0usize.wrapping_sub(width);
    let mut queue = MinHeap::with_capacity(1024);
    queue.push(1, (start, 1));
    distances[start] = (1, u32::MAX);
        
    while let Some((dist, (node, direction))) = queue.pop() {
        let is_horizontal = direction == 1 || direction == usize::MAX;
        if node == end {
            if is_horizontal {
                distances[node].0 =  dist;
            } else {
                distances[node].1 =  dist;
            }
            return Some(dist-1);
        }
        if is_horizontal && distances[node].0 != dist || !is_horizontal && distances[node].1 != dist {
            continue;
        }
       
        let next = node + 1;
        if direction != usize::MAX && grid[next] != b'#' {
            let nextdist = dist + if direction == 1 { 1 } else { 1001 };
            if nextdist < distances[next].0 {
                distances[next].0 = nextdist;
                queue.push(nextdist, (node + 1, 1));
            }
        }

        let next = node.wrapping_add(usize::MAX);
        if direction != 1 && grid[next] != b'#' {
            let nextdist = dist + if direction == usize::MAX { 1 } else { 1001 };
            if nextdist < distances[next].0 {
                distances[next].0 = nextdist;
                queue.push(nextdist, (next, usize::MAX));
            }
        }
        let next = node + width;
        if direction != up && grid[next] != b'#' {
            let nextdist = dist + if direction == width { 1 } else { 1001 };
            if nextdist < distances[next].1 {
                distances[next].1 = nextdist;
                queue.push(nextdist, (next, width));
            }
        }
        let next = node.wrapping_add(up);
        if direction != width && grid[next] != b'#' {
            let nextdist = dist + if direction == up { 1 } else { 1001 };
            if nextdist < distances[next].1 {
                distances[next].1 = nextdist;
                queue.push(nextdist, (next, up));
            }
        }
    }
    None
}

fn part2(width: usize, distances: & [(u32, u32)], end: usize) -> u32 {
    let mut stack = Vec::new();
    let (d1, d2) = distances[end];
    if d1 <= d2 {
        stack.push((end, true));
    }
    if d2 <= d1 {
        stack.push((end, false));
    }

    let mut seen = vec![(false, false); distances.len()];

    while let Some((node, is_horizontal)) = stack.pop() {
        if is_horizontal {
            if seen[node].0 {
                continue;
            }
            seen[node].0 = true;

            let dist = distances[node];
            
            let next = node + 1;
            if distances[next].0 + 1 == dist.0 {
                stack.push((next, true))
            }
            if distances[next].1 + 1001 == dist.0 {
                stack.push((next, false))
            }
            
            let next = node - 1;
            if distances[next].0 + 1 == dist.0 {
                stack.push((next, true))
            }
            if distances[next].1 + 1001 == dist.0 {
                stack.push((next, false))
            }


        } else { // vertical
            if seen[node].1 {
                continue;
            }
            seen[node].1 = true;

            let dist = distances[node];
            
            let next = node + width;
            if distances[next].1 + 1 == dist.1 {
                stack.push((next, false))
            }
            if distances[next].0 + 1001 == dist.1 {
                stack.push((next, true))
            }
            
            let next = node - width;
            if distances[next].1 + 1 == dist.1 {
                stack.push((next, false))
            }
            if distances[next].0 + 1001 == dist.1 {
                stack.push((next, true))
            }

        }
    }

    let mut counter = 0;
    for (h, w) in seen {
        if h || w {
            counter += 1;
        }
    }
    counter
}