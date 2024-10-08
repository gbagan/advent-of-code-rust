use anyhow::*;
use crate::util::grid::Grid;
use std::collections::{HashSet, HashMap};
use lazy_static::lazy_static;

struct GridGraph {
    extremities: u32,
    horizontal: [[u32; 5]; 6],
    vertical: [[u32; 6]; 6],
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse_with_padding(input, b'#')?;
    let graph = compress_grid(&grid);
    let grid = graph_to_grid(&graph);
    let p1 = part1(&grid);
    let p2 = part2(&grid).context("Part 2: No solution found")?;
    Ok((p1, p2))
}

fn neighbors2 (grid: &Grid<u8>, idx: usize) -> Vec<usize> {
    if grid[idx] == b'#' {
        vec!()
    } else {
        [idx-1, idx+1, idx-grid.width, idx+grid.width]
        .iter()
        .filter(|&&n| grid[n] != b'#')
        .copied()
        .collect()
    }
}

fn follow_path(grid: &Grid<u8>, mut pos: usize, mut pred: usize, goal: usize) -> Option<(usize, u32)>
{
    let mut len = 1;
    loop {
        let nbors = neighbors2(grid, pos);
        match nbors.len() {
            1 => {
                let next = nbors[0];
                if next != pred {
                    pred = pos;
                    pos = next;
                    len += 1;
                } else if pos == goal {
                    return Some((pos, len))
                } else {
                    return None
                }
            },
            2 => {
                let next1 = nbors[0];
                let next2 = nbors[1];
                len += 1;
                let pos2 = if next1 == pred { next2 } else { next1 };
                pred = pos;
                pos = pos2;
            },
            _ => return Some((pos, len))
        }
    }
}

fn compress_grid(grid: &Grid<u8>) -> Vec<Vec<(usize, u32)>>
{
    let h = grid.height;
    let w = grid.width;
    let start = w+2;
    let goal = w * (h-1) - 3;
    let mut junctions = vec!();
    let mut n = 0;
    let grid2: Vec<_> = (0..h*w).map(|idx| {
        let nbors = neighbors2(grid, idx);
        if idx == start || idx == goal || nbors.len() > 2 {
            let m = n;
            n += 1;
            let nbors2 = nbors
                .iter()
                .filter_map(|&next| follow_path(grid, next, idx, goal))
                .collect();
            junctions.push(idx);
            (m, nbors2)
        } else {
            (n, vec!())
        }
    }).collect();
    junctions.iter().map(|&idx| {
        let nbors = &grid2[idx];
        nbors.1.iter().map(|&(idx2, len)| (grid2[idx2].0, len)).collect()
    }).collect()
}

fn graph_to_grid(graph: &[Vec<(usize, u32)>]) -> GridGraph {
    let start = 0;
    let next_to_start = graph[start][0].0;
    let goal = graph.len()-1;
    let extremities = graph[0][0].1 + graph[goal][0].1;

    let mut nodes = [[0; 6]; 6];
    let mut horizontal = [[0; 5]; 6];
    let mut vertical = [[0; 6]; 6];
    let mut visited = HashSet::new();
    nodes[0][0] = next_to_start;
    let mut current = next_to_start;
    
    let mut next_border = |node: usize| {
        let (node, weight) = graph[node]
            .iter()
            .find(|(next, _)|graph[*next].len() == 3 && !visited.contains(next))
            .copied()
            .unwrap();
        visited.insert(node);
        (node, weight)
    };

    let weight_between = |node1: usize, node2: usize| {
        graph[node1].iter().find(|(nbor, _)| *nbor == node2).map(|p| p.1)
    };

    for i in 1..5 {
        let (next, weight) = next_border(current);
        horizontal[0][i-1] = weight;
        current = next;
        nodes[0][i] = current;
    }
    nodes[0][5] = nodes[0][4];
    current = next_to_start;
    for i in 1..5 {
        let (next, weight) = next_border(current);
        vertical[i-1][0] = weight;
        current = next;
        nodes[i][0] = current;
    }
    nodes[5][0] = nodes[4][0];

    for x in 1..6 {
        for y in 1..6 {
            let above = nodes[y - 1][x];
            let left = nodes[y][x - 1];
            let (node, hweight, vweight) = graph[left]
                .iter()
                .find_map(|(node, hweight)|
                    if visited.contains(node) {
                        None 
                    } else {
                        weight_between(*node, above).map(|w| (node, *hweight, w))
                    }
                ).unwrap();
            horizontal[y][x-1] = hweight;
            vertical[y-1][x] = vweight;
            nodes[y][x] = *node;
            visited.insert(*node);
        }
    }

    GridGraph { horizontal, vertical, extremities }

}

fn part1(grid: &GridGraph) -> u32 {
    let mut dist = [[0; 6]; 6];
    for x in 0..6 {
        for y in 0..6 {
            if x > 0 {
                dist[y][x] = dist[y][x-1] + grid.horizontal[y][x-1];
            }
            if y > 0 {
                dist[y][x] = dist[y][x].max(dist[y-1][x] + grid.vertical[y-1][x]);
            }
        }
    }
    dist[5][5] + grid.extremities
}

const N: usize = 76;

const STATES: [[u8; 6]; N] = [
    [0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0, 0],
    [1, 0, 0, 0, 0, 0],

    [1, 1, 2, 0, 0, 0],
    [1, 1, 0, 2, 0, 0],
    [1, 1, 0, 0, 2, 0],
    [1, 1, 0, 0, 0, 2],
    [1, 0, 1, 2, 0, 0],
    [1, 0, 1, 0, 2, 0],
    [1, 0, 1, 0, 0, 2],
    [1, 0, 0, 1, 2, 0],
    [1, 0, 0, 1, 0, 2],
    [1, 0, 0, 0, 1, 2],
    [0, 1, 1, 2, 0, 0],
    [0, 1, 1, 0, 2, 0],
    [0, 1, 1, 0, 0, 2],
    [0, 1, 0, 1, 2, 0],
    [0, 1, 0, 1, 0, 2],
    [0, 1, 0, 0, 1, 2],
    [0, 0, 1, 1, 2, 0],
    [0, 0, 1, 1, 0, 2],
    [0, 0, 1, 0, 1, 2],
    [0, 0, 0, 1, 1, 2],

    [2, 1, 1, 0, 0, 0],
    [2, 1, 0, 1, 0, 0],
    [2, 1, 0, 0, 1, 0],
    [2, 1, 0, 0, 0, 1],
    [2, 0, 1, 1, 0, 0],
    [2, 0, 1, 0, 1, 0],
    [2, 0, 1, 0, 0, 1],
    [2, 0, 0, 1, 1, 0],
    [2, 0, 0, 1, 0, 1],
    [2, 0, 0, 0, 1, 1],
    [0, 2, 1, 1, 0, 0],
    [0, 2, 1, 0, 1, 0],
    [0, 2, 1, 0, 0, 1],
    [0, 2, 0, 1, 1, 0],
    [0, 2, 0, 1, 0, 1],
    [0, 2, 0, 0, 1, 1],
    [0, 0, 2, 1, 1, 0],
    [0, 0, 2, 1, 0, 1],
    [0, 0, 2, 0, 1, 1],
    [0, 0, 0, 2, 1, 1],

    [1, 1, 2, 2, 3, 0],
    [1, 1, 2, 2, 0, 3],
    [1, 1, 0, 2, 2, 3],
    [1, 1, 2, 0, 2, 3],
    [0, 1, 1, 2, 2, 3],
    [1, 0, 1, 2, 2, 3],
    [3, 1, 1, 2, 2, 0],
    [3, 1, 1, 2, 0, 2],
    [3, 1, 1, 0, 2, 2],
    [3, 0, 1, 1, 2, 2],
    [3, 1, 0, 1, 2, 2],
    [0, 3, 1, 1, 2, 2],
    [1, 1, 3, 0, 2, 2],
    [1, 1, 0, 3, 2, 2],
    [1, 1, 3, 2, 0, 2],
    [1, 1, 3, 2, 2, 0],
    [0, 1, 1, 3, 2, 2],
    [1, 0, 1, 3, 2, 2],

    [1, 2, 2, 1, 3, 0],
    [1, 2, 2, 1, 0, 3],
    [1, 2, 0, 2, 1, 3],
    [1, 2, 2, 0, 1, 3],
    [1, 0, 2, 2, 1, 3],
    [0, 1, 2, 2, 1, 3],
    [3, 1, 2, 2, 1, 0],
    [3, 0, 1, 2, 2, 1],
    [3, 1, 2, 2, 0, 1],
    [3, 1, 2, 0, 2, 1],
    [3, 1, 0, 2, 2, 1],
    [0, 3, 1, 2, 2, 1],
];

lazy_static! {
    static ref STATE_INDEX: HashMap<[u8; 6], usize> = {
        let perms = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
        let mut m = HashMap::new();
        for (i, state) in STATES.iter().enumerate() {
            for perm in perms {
                let state2 = state.map(|v| if v == 0 {0} else {perm[v as usize -1]+1});
                m.insert(state2, i);
            }
        }
        m
    };
}

fn part2(grid: &GridGraph) -> Option<u32> {
    let h_edges = vec!(
        vec!((0, 1)),
        vec!((0, 2)),
        vec!((0, 3)),
        vec!((0, 4)),
        vec!((0, 5)),
        vec!((1, 2)),
        vec!((1, 3)),
        vec!((1, 4)),
        vec!((1, 5)),
        vec!((2, 3)),
        vec!((2, 4)),
        vec!((2, 5)),
        vec!((3, 4)),
        vec!((3, 5)),
        vec!((4, 5)),

        vec!((0, 1), (2, 3)),
        vec!((0, 1), (2, 4)),
        vec!((0, 1), (2, 5)),
        vec!((0, 1), (3, 4)),
        vec!((0, 1), (3, 5)),
        vec!((0, 1), (4, 5)),
        vec!((0, 2), (3, 4)),
        vec!((0, 2), (3, 5)),
        vec!((0, 2), (4, 5)),
        vec!((1, 2), (3, 4)),
        vec!((1, 2), (3, 5)),
        vec!((1, 2), (4, 5)),
        vec!((0, 3), (4, 5)),
        vec!((1, 3), (4, 5)),
        vec!((2, 3), (4, 5)),
    );

    let mut current = [None; N];
    current[5] = Some(0);
    let mut next = [None; N];
    for i in 0..6 {
        for (state, weight) in STATES.iter().zip(current) {
            if let Some(weight) = weight {
                for edges in &h_edges {
                    if let Some(new_state) = next_state(state, edges) {
                        let mut weight = weight;
                        for (j, &v) in new_state.iter().enumerate() {
                            if v > 0 {
                                weight += grid.vertical[i][j]
                            }
                        }
                        for &(start, end) in edges {
                            for j in start..end {
                                weight += grid.horizontal[i][j]
                            }
                        }
                        let idx = STATE_INDEX[&new_state];
                        if next[idx] < Some(weight) {
                            next[idx] = Some(weight);
                        }
                    }
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
        next = [None; N];
    }
    let v = current[0]?;
    Some(grid.extremities + v) 

}

fn next_state(state: &[u8; 6], h_edges: &[(usize, usize)]) -> Option<[u8; 6]> {
    let mut next = *state;
    for &(start, end) in h_edges {
        if next[start] != 0 && next[start] == next[end] || (start+1..end).any(|i| next[i] != 0) {
            return None
        }
        if next[start] == next[end] { // == 0
            let v = 1 + next.iter().max().unwrap();
            next[start] = v;
            next[end] = v;
        } else if next[start] == 0 {
            next[start] = next[end];
            next[end] = 0;
        } else if next[end] == 0 {
            next[end] = next[start];
            next[start] = 0;
        } else {
            let min = next[start].min(next[end]);
            let max = next[start].max(next[end]);
            for c in next.iter_mut() {
                if *c == max {
                    *c = min
                }
            }
            next[start] = 0;
            next[end] = 0;
        }
    }

    Some(next)
}


#[test]
fn next_state_test() {
    assert_eq!(next_state(&[0, 1, 0, 0, 0, 0], &[(1, 3)]), Some([0, 0, 0, 1, 0, 0]));
    //assert_eq!(next_state(&[1, 0, 0, 0, 0, 0], &[(1, 3)]), None);
    assert_eq!(next_state(&[0, 1, 1, 0, 0, 2], &[(0, 1), (2, 3)]), Some([1, 0, 0, 1, 0, 2]));
    assert_eq!(next_state(&[2, 1, 1, 2, 0, 3], &[(0, 1), (3, 4)]), Some([0, 0, 1, 0, 1, 3]));
    assert_eq!(next_state(&[2, 1, 1, 2, 0, 3], &[(0, 1), (2, 3)]), None);
    assert_eq!(next_state(&[1, 0, 0, 0, 0, 0], &[(2, 3)]), Some([1, 0, 2, 2, 0, 0]));
}
