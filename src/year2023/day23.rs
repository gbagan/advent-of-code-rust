use arrayvec::ArrayVec;
use crate::util::grid::Grid;
use ahash::{HashMap, HashMapExt};

struct GridGraph {
    extremities: i32,
    horizontal: [[i32; 5]; 6],
    vertical: [[i32; 6]; 6],
}

pub fn solve(input: &str) -> (i32, i32) {
    let mut grid = Grid::parse_with_padding(input, b'#');
    let graph = compress_grid(&mut grid);
    let grid = graph_to_grid(&graph);
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    (p1, p2)
}

fn neighbors2 (grid: &Grid<u8>, idx: usize) -> ArrayVec<usize, 4> {
    if grid[idx] == b'#' {
        ArrayVec::new()
    } else {
        [idx-1, idx+1, idx-grid.width, idx+grid.width]
        .iter()
        .filter(|&&n| grid[n] != b'#')
        .copied()
        .collect()
    }
}

fn follow_path(grid: &mut Grid<u8>, mut pos: usize, mut pred: usize, goal: usize) -> Option<(usize, i32)> {  
    let mut len = 1;
    if grid[pos] == b'?' {
        return None
    }
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
                    grid[pred] = b'?';
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
            _ => {
                grid[pred] = b'?';
                return Some((pos, len))
            }
        }
    }
}

fn compress_grid(grid: &mut Grid<u8>) -> Vec<ArrayVec<(usize, i32), 4>> {
    let h = grid.height;
    let w = grid.width;
    let start = w+2;
    let goal = w * (h-1) - 3;
    let mut junctions: ArrayVec<(usize, ArrayVec<usize, 4>), 36> = ArrayVec::new();
    
    let junction_index: Vec<_> = (0..h*w).map(|idx| {
        let nbors = neighbors2(grid, idx);
        if idx == start || idx == goal || nbors.len() > 2 {
            junctions.push((idx, nbors));
            junctions.len() - 1
        } else {
            0
        }
    }).collect();

    let mut compressed = vec![ArrayVec::new(); 36];
    
    for (i, (idx, nbors)) in junctions.iter().enumerate() {
        for &nbor in nbors {
            if let Some((idx2, len)) = follow_path(grid, nbor, *idx, goal) {
                let j = junction_index[idx2];
                compressed[i].push((j, len));
                compressed[j].push((i, len));
            }
        }
    }

    compressed

}

fn graph_to_grid(graph: &[ArrayVec<(usize, i32), 4>]) -> GridGraph {
    let start = 0;
    let next_to_start = graph[start][0].0;
    let goal = graph.len()-1;
    let extremities = graph[0][0].1 + graph[goal][0].1;

    let mut nodes = [[0; 6]; 6];
    let mut horizontal = [[0; 5]; 6];
    let mut vertical = [[0; 6]; 6];
    let mut seen = [false; 36];
    nodes[0][0] = next_to_start;
    let mut current = next_to_start;
    seen[current] = true;

    let mut next_border = |node: usize| {
        let (node, weight) = *graph[node]
            .iter()
            .find(|(next, _)|graph[*next].len() == 3 && !seen[*next])
            .unwrap();
        seen[node] = true;
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
                .find_map(|&(node, hweight)|
                    if seen[node] {
                        None 
                    } else {
                        weight_between(node, above).map(|w| (node, hweight, w))
                    }
                ).unwrap();
            horizontal[y][x-1] = hweight;
            vertical[y-1][x] = vweight;
            nodes[y][x] = node;
            seen[node] = true;
        }
    }

    GridGraph { horizontal, vertical, extremities }

}

fn part1(grid: &GridGraph) -> i32 {
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

/*
const fn to_state_index(c: [u8; 6]) -> usize {
    ((c[0] as usize) << 10)
    + ((c[1] as usize) << 8)
    + ((c[2] as usize) << 6)
    + ((c[3] as usize) << 4)
    + ((c[4] as usize) << 2)
    + c[5] as usize
}

const STATE_INDEX: [usize; 4096] = {
    let mut table = [0; 4096];
    let perms = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let mut i = 0; 
    
    while i < STATES.len() {
        let mut j = 0;
        let st = STATES[i];
        while j < 6 {
            let mut st2 = [0; 6];
            let mut k = 0;
            let perm = perms[j];
            while k < 6 {
                if st[k] > 0 {
                    st2[k] = perm[st[k] as usize - 1] + 1;
                }
                k += 1;
            }
            table[to_state_index(st2)] = i;

            j += 1;
        }
        i += 1;
    }

    table
};
*/

fn part2(grid: &GridGraph) -> i32 {
    let state_index: HashMap<[u8; 6], usize> = {
        let perms = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
        let mut m = HashMap::new(); // HashMap::with_capacity(438);
        for (i, state) in STATES.iter().enumerate() {
            for perm in perms {
                let state2 = state.map(|v| if v == 0 {0} else {perm[v as usize -1]+1});
                m.insert(state2, i);
            }
        }
        m
    };

    let mut h_edges: [ArrayVec<(usize, usize), 2>; 30] = std::array::from_fn(|_| ArrayVec::new());
        
    h_edges[0].push((0, 1));
    h_edges[1].push((0, 2));
    h_edges[2].push((0, 3));
    h_edges[3].push((0, 4));
    h_edges[4].push((0, 5));
    h_edges[5].push((1, 2));
    h_edges[6].push((1, 3));
    h_edges[7].push((1, 4));
    h_edges[8].push((1, 5));
    h_edges[9].push((2, 3));
    h_edges[10].push((2, 4));
    h_edges[11].push((2, 5));
    h_edges[12].push((3, 4));
    h_edges[13].push((3, 5));
    h_edges[14].push((4, 5));

    h_edges[15].push((0, 1));
    h_edges[15].push((2, 3));
    
    h_edges[16].push((0, 1));
    h_edges[16].push((2, 4));
    
    h_edges[17].push((0, 1));
    h_edges[17].push((2, 5));
    
    h_edges[18].push((0, 1));
    h_edges[18].push((3, 4));
    
    h_edges[19].push((0, 1));
    h_edges[19].push((3, 5));
    
    h_edges[20].push((0, 1));
    h_edges[20].push((4, 5));
    
    h_edges[21].push((0, 2));
    h_edges[21].push((3, 4));
    
    h_edges[22].push((0, 2));
    h_edges[22].push((3, 5));
    
    h_edges[23].push((0, 2));
    h_edges[23].push((4, 5));
    
    h_edges[24].push((1, 2));
    h_edges[24].push((3, 4));
    
    h_edges[25].push((1, 2));
    h_edges[25].push((3, 5));
    
    h_edges[26].push((1, 2));
    h_edges[26].push((4, 5));

    h_edges[27].push((0, 3));
    h_edges[27].push((4, 5));

    h_edges[28].push((1, 3));
    h_edges[28].push((4, 5));

    h_edges[29].push((2, 3));
    h_edges[29].push((4, 5));

    let mut current = [i32::MIN; N];
    current[5] = 0;
    let mut next = [i32::MIN; N];
    for i in 0..6 {
        for (state, weight) in STATES.iter().zip(current) {
            if weight != i32::MIN {
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
                        let idx = state_index[&new_state];
                        if next[idx] < weight {
                            next[idx] = weight;
                        }
                    }
                }
            }
        }
        current = next;
        next = [i32::MIN; N];
    }
    let v = current[0];
    grid.extremities + v

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
            for c in &mut next {
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
