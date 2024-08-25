use crate::util::coord::Coord;
use crate::util::grid::Grid;
use rayon::prelude::*;
use std::collections::{HashSet, HashMap, VecDeque};

pub struct GridGraph {
    extremities: u32,
    horizontal: [[u32; 5]; 6],
    vertical: [[u32; 6]; 5],
}

fn neighbors2 (grid: &Grid<u8>, c: Coord) -> Vec<Coord> {
    if grid[c] == b'#' {
        vec!()
    } else {
        c.adjacent()
        .iter()
        .filter(|&n| grid.contains(*n) && grid[*n] != b'#')
        .copied()
        .collect()
    }
}

fn follow_path(grid: &Grid<u8>, pos: Coord, pred: Coord, goal: Coord) -> Option<(Coord, u32)>
{
    let mut pred = pred;
    let mut pos = pos;
    let mut len = 1;
    loop {
        let nbors = neighbors2(&grid, pos);
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
    let h = grid.height as i32;
    let w = grid.width as i32;
    let start = Coord::new(1, 0);
    let goal = Coord::new(w-2, h-1);
    let mut junctions = vec!();
    let mut n = 0;
    let grid2 = grid.map_with_indices(|c, _| {
        let nbors = neighbors2(&grid, c);
        if c == start || c == goal || nbors.len() > 2 {
            let m = n;
            n += 1;
            let nbors2 = nbors.iter().map(|&next| follow_path(&grid, next, c, goal)).flatten().collect();
            junctions.push(c);
            (m, nbors2)
        } else {
            (n, vec!())
        }
    });
    junctions.iter().map(|&c| {
        let nbors = &grid2[c];
        nbors.1.iter().map(|&(c2, len)| (grid2[c2].0, len)).collect()
    }).collect()
}

fn graph_to_grid(graph: &Vec<Vec<(usize, u32)>>) -> GridGraph {
    let start = 0;
    let next_to_start = graph[start][0].0;
    let goal = graph.len()-1;
    let extremities = graph[0][0].1 + graph[goal][0].1;

    let mut nodes = [[0; 6]; 6];
    let mut horizontal = [[0; 5]; 6];
    let mut vertical = [[0; 6]; 5];
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

pub fn longest_path_heuristic(graph: &Vec<Vec<(usize, u32)>>, prev_border: &Vec<usize>) -> Option<u32> {
    let mut todo = VecDeque::new();
    todo.push_front((0, 1 as usize, 0));

    while todo.len() < 64 {
        if let Some((current, visited, len)) = todo.pop_front() {
            for (nbor, len2) in graph[current].iter() {
                let mask = 1 << nbor;
                if visited & mask == 0 && visited & (1 << prev_border[*nbor]) != 0 {
                    todo.push_back((*nbor, visited | mask, len+len2));
                }
            }
        }
    }
    todo
    .into_par_iter()
    .map(|tuple| longest_path_heuristic_aux(graph, prev_border, tuple))
    .max()
}

pub fn longest_path_heuristic_aux(graph: &Vec<Vec<(usize, u32)>>, prev_border: &Vec<usize>, tuple: (usize, usize, u32)) -> u32 {
    let goal = graph.len()-1;
    let mut todo = Vec::new();
    todo.push(tuple);
    let mut best_score = 0;

    while let Some((current, visited, len)) = todo.pop() {
        if current == goal {
            best_score = best_score.max(len);
        } else {
            for (nbor, len2) in graph[current].iter() {
                let mask = 1 << nbor;
                if visited & mask == 0 && visited & (1 << prev_border[*nbor]) != 0 {
                    todo.push((*nbor, visited | mask, len+len2));
                }
            }
        }
    };
    best_score
}

pub fn parse(input: &str) -> Option<GridGraph> {
    let grid = Grid::parse(input);
    let graph = compress_grid(&grid);
    Some(graph_to_grid(&graph))
}

pub fn part1(grid: &GridGraph) -> Option<u32> {
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
    Some(dist[5][5] + grid.extremities)
    //Some(longest_path(&graph))
}


pub fn part2(grid: &GridGraph) -> Option<u32> {
    let mut state = HashMap::new();
    state.insert(vec!(0), 0);
    for y in 0..6 {
        for x in 0..6 {
            if x == 0 && y == 0 {
                continue
            }
            let v = y * 6 + x;
            state = add_vertex(&state, v);
            if x > 0 {
                state = add_edge(&state, v-1, v, grid.horizontal[y][x-1]);
            }
            if y > 0 {
                state = add_edge(&state, v-6, v, grid.vertical[y-1][x]);
                state = delete_vertex(&state, v-6);
            }
            state = filter_isolated_paths(&state);
        }
    }
    Some(state[&vec!(35)] + grid.extremities)
    //let grid = graph_to_grid(&graph);
    //let prev_border = previous_border(&graph);
    //longest_path_heuristic(&graph, &prev_border)
}


type State = HashMap<Vec<usize>, u32>;

fn remove_isolated_path(paths: &[usize]) -> Vec<usize> {
    if paths.len() == 0 {
        return vec!();
    }
    let mut i = 0;
    let n = paths.len()-1;
    let mut res = vec!();
    while i < n {
        if paths[i] == paths[i+1]{
            i += 2;
        } else {
            res.push(paths[i]);
            i += 1;
        }
    }
    res.push(paths[n]);
    res
}

fn filter_isolated_paths(state: &State) -> State {
    let mut res = HashMap::new();
    for (paths, &weight) in state {
        let paths = remove_isolated_path(&paths);
        if res.get(&paths).copied() < Some(weight) {
            res.insert(paths, weight);
        }
    }
    res
}

fn add_vertex(state: &State, v: usize) -> State {
    let mut res = HashMap::new();
    for (paths, &value) in state {
        let mut paths = paths.clone();
        paths.push(v);
        paths.push(v);
        res.insert(paths, value);
    }
    res
}

fn delete_vertex(state: &State, v: usize) -> State {
    let mut res = HashMap::new();
    for (paths, &weight) in state {
        if !paths.contains(&v) {
            res.insert(paths.clone(), weight);
        }
    }
    res
}

fn add_edge(state: &State, u: usize, v: usize, weight: u32) -> State {
    let mut res = HashMap::new();
    for (paths, &value) in state {
        res.insert(paths.clone(), value);
        if paths.contains(&u) {
            if let Some(i) = paths.iter().position(|&a| a == v) {
                let mut paths = paths.clone();
                paths.remove(i);
                let i = paths.iter().position(|&a| a == u).unwrap();
                paths.remove(i);
                if res.get(&paths).copied() < Some(value + weight) {
                    res.insert(paths, value + weight);
                }
            }
        }
    }
    res
}