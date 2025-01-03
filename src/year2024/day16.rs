use crate::util::grid::*;
use std::collections::VecDeque;

pub fn solve(input: &str) -> (u32, u32) {
    let grid = Grid::parse(input).unwrap();
    let start = grid.width * (grid.height-2) + 1;
    let end = grid.width * 2 - 2;

    let mut distances = vec![[u32::MAX; 2]; grid.vec.len()];

    let p1 = part1(&grid, &mut distances, start, end);
    let p2 = part2(grid.width, &distances, end);
    (p1, p2)
}

fn part1(grid: &Grid<u8>, distances: &mut [[u32; 2]], start: usize, end: usize) -> u32 {
    let width = grid.width;
    let grid = &grid.vec;
    let up = 0usize.wrapping_sub(width);
    
    let mut queue: VecDeque<(u32, usize, usize)> = VecDeque::with_capacity(1024);
    let mut todo = Vec::new();
    let mut next_todo = Vec::new();

    todo.push((1, start, 1));
    
    let directions = [1, usize::MAX, width, up];

    loop {
        let mut index = 0;
        loop {
            let (dist, node, direction) =
                if index >= todo.len() {
                    match queue.pop_front() {
                        Some(n) => n,
                        _ => break,
                    }
                } else {
                    match queue.front() {
                        Some(&tuple) if tuple.0 <= todo[index].0 => {
                            queue.pop_front();
                            tuple
                        },
                        _ => {
                            let tuple= todo[index];
                            index += 1;
                            tuple
                        }
                    }
                };
            let is_horizontal = direction == 1 || direction == usize::MAX;
            if is_horizontal {
                if distances[node][0] != u32::MAX {
                    continue;
                }
                distances[node][0] = dist;
            } else {
                if distances[node][1] != u32::MAX {
                    continue;
                }
                distances[node][1] = dist;
            }
            if node == end {
                return dist-1;
            }

            for next_direction in directions {
                let next = node.wrapping_add(next_direction);
                if direction.wrapping_add(next_direction) == 0 || grid[next] == b'#' {
                    continue
                } else if direction == next_direction {
                    queue.push_back((dist + 1, next, next_direction));
                } else {
                    next_todo.push((dist + 1001, next, next_direction));
                }
            }
        }
        (todo, next_todo) = (next_todo, todo);
        next_todo.clear();
    }
}

fn part2(width: usize, distances: & [[u32; 2]], end: usize) -> u32 {
    let mut stack = Vec::new();
    let mut seen = vec![(false, false); distances.len()];

    let [d1, d2] = distances[end];
    if d1 <= d2 {
        stack.push((end, true));
        seen[end].0 = true;
    }
    if d2 <= d1 {
        stack.push((end, false));
        seen[end].1 = true;
    }

    while let Some((node, is_horizontal)) = stack.pop() {
        if is_horizontal {
            let dist = distances[node];
            
            let next = node + 1;
            if distances[next][0].wrapping_add(1) == dist[0] && !seen[next].0 {
                seen[next].0 = true;
                stack.push((next, true))
            }
            if distances[next][1].wrapping_add(1001) == dist[0] && !seen[next].1 {
                seen[next].1 = true;
                stack.push((next, false))
            }
            
            let next = node - 1;
            if distances[next][0].wrapping_add(1) == dist[0] && !seen[next].0 {
                seen[next].0 = true;
                stack.push((next, true))
            }
            if distances[next][1].wrapping_add(1001) == dist[0] && !seen[next].1 {
                seen[next].1 = true;
                stack.push((next, false))
            }
        } else { // vertical
            let dist = distances[node];
            
            let next = node + width;
            if distances[next][1].wrapping_add(1) == dist[1] && !seen[next].1 {
                seen[next].1 = true;
                stack.push((next, false))
            }
            if distances[next][0].wrapping_add(1001) == dist[1] && !seen[next].0 {
                seen[next].0 = true;
                stack.push((next, true))
            }
            
            let next = node - width;
            if distances[next][1].wrapping_add(1) == dist[1] && !seen[next].1 {
                seen[next].1 = true;
                stack.push((next, false))
            }
            if distances[next][0].wrapping_add(1001) == dist[1] && !seen[next].0  {
                seen[next].0 = true;
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