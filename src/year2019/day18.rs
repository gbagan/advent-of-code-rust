use ahash::{HashMap, HashMapExt};
use crate::util::{bits::*, heap::*};

#[derive(Clone, Copy)]
struct Edge {
    distance: u32,
    doors: u32,
}

pub fn solve(input: &str) -> (u32, u32) {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    let height = input.len() / width;
    let mut quartiles = [usize::MAX; 26];
    
    let xmid = (width-1) / 2;
    let ymid = height / 2; 
    
    let mut keys_and_robots = [0; 30];
    

    let mut missing_keys = 0u32;

    for y in 1..height-1 {
        for x in 1..width-2 {
            let index = y * width + x;
            let tile = input[index];
            if tile.is_ascii_lowercase() {
                let key = (tile - b'a') as usize;
                let quartile = 2 * (y < ymid) as usize + (x < xmid) as usize;
                quartiles[key] = quartile;
                keys_and_robots[key] = index;
                missing_keys |= 1 << key;
            }
        }
    }
    // robot
    let start = ymid * width + xmid;
    assert!(input[start] == b'@');
    keys_and_robots[26] = start;

    let directions = [1, 0usize.wrapping_sub(1), width, 0usize.wrapping_sub(width)];

    let mut matrix = [[Edge {distance: u32::MAX, doors: 0}; 30]; 30];
    let mut seen = vec![u8::MAX; input.len()];
    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();

    for (from, &start) in keys_and_robots[..27].iter().enumerate() {
        let dfs_iteration = from as u8;
        let mut distance = 0;
        queue1.push((start, 0));
        seen[start] = dfs_iteration;
        while !queue1.is_empty() {
            for &(position, mut doors) in &queue1 {
                if let Some(door) = is_door(input[position]) {
                    doors |= 1 << door;
                }
                if let Some(to) = is_key(input[position]) && distance > 0 {
                    matrix[from][to] = Edge {distance, doors};
                    matrix[to][from] = Edge {distance, doors};
                    continue;
                }
                for direction in directions {
                    let next = position + direction;
                    if input[next] != b'#' && seen[next] != dfs_iteration {
                        queue2.push((next, doors));
                        seen[next] = dfs_iteration;
                    }
                }
            }
            distance += 1;
            std::mem::swap(&mut queue1, &mut queue2);
            queue2.clear();
        }
    }

    for k in 0..30 {
        for i in 0..30 {
            for j in 0..30 {
                let dist = matrix[i][k].distance.saturating_add(matrix[k][j].distance);
                if dist < matrix[i][j].distance {
                    matrix[i][j].distance = dist;
                    matrix[i][j].doors = matrix[i][k].doors | matrix[k][j].doors | 1 << k;
                }
            }
        }
    }

    let robots = 1 << 26;
    let p1 = dijkstra(&matrix, robots, missing_keys);


    for i in 0..25 {
        for j in i+1..26 {
            if quartiles[i] != quartiles[j] {
                matrix[i][j].distance = u32::MAX;
                matrix[j][i].distance = u32::MAX;
            }
        }
        let mut edge = matrix[26][i];
        if edge.distance != u32::MAX {
            matrix[26][i].distance = u32::MAX;
            edge.distance -= 2;
            matrix[26+quartiles[i]][i] = edge;
        }
    }

    let robots = 15 << 26;
    let p2 = dijkstra(&matrix, robots, missing_keys);

    (p1, p2)
}


fn dijkstra(matrix: &[[Edge; 30]; 30], robots: u32, missing_keys: u32) -> u32 {
    let mut heap = MinHeap::with_capacity(10_000);
    let mut cache = HashMap::with_capacity(15_000);
    
    heap.push(0, (robots, missing_keys));

    while let Some((total, (positions, missing_keys))) = heap.pop() {
        if missing_keys == 0 {
            return total;
        }

        for from in positions.bit_iterator() {
            for to in missing_keys.bit_iterator() {
                let Edge { distance, doors } = matrix[from][to];

                if distance != u32::MAX && missing_keys & doors == 0 {
                    let next_total = total + distance;
                    let from = 1 << from;
                    let to = 1 << to;
                    let next_state = (
                        positions ^ from ^ to,
                        missing_keys ^ to,
                    );

                    cache
                        .entry(next_state)
                        .and_modify(|e| {
                            if next_total < *e {
                                heap.push(next_total, next_state);
                                *e = next_total;
                            }
                        })
                        .or_insert_with(|| {
                            heap.push(next_total, next_state);
                            next_total
                        });
                }
            }
        }
    }

    unreachable!()
}


fn is_key(c: u8) -> Option<usize> {
    c.is_ascii_lowercase().then(|| (c - b'a') as usize)
}

fn is_door(c: u8) -> Option<usize> {
    c.is_ascii_uppercase().then(|| (c - b'A') as usize)
}