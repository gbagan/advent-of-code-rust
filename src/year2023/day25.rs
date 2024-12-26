// Ford-Fulkerson algorithm

use anyhow::*;
use arrayvec::ArrayVec;
use std::collections::VecDeque;

type Graph = Vec<ArrayVec<usize, 10>>;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut table = vec![usize::MAX; 26*26*26];
    let mut graph: Graph = Vec::with_capacity(1500);
    
    for line in input.lines() {
        let line = line.as_bytes();
        let index1 = line[0] as usize * 676 + line[1] as usize * 26 + line[2] as usize - 68191;
        let mut i = table[index1];
        if i == usize::MAX {
            i = graph.len();
            table[index1] = i;
            graph.push(ArrayVec::new());
        }
        for &[_, l1, l2, l3] in line[4..].array_chunks() {
            let index2 = l1 as usize * 676 + l2 as usize * 26 + l3 as usize - 68191;
            let mut j = table[index2];
            if j == usize::MAX {
                j = graph.len();
                table[index2] = j;
                graph.push(ArrayVec::new());
            }
            graph[i].push(j);
            graph[j].push(i);
        }

    }

    let p1 = part1(&mut graph);
    Ok((p1, 0))
}

fn part1(graph: &mut Graph) -> usize {
    let n = graph.len();
    let mut visited: Vec<_> = vec![0; n];
    let mut parent: Vec<_> = vec![0; n];
    let source = 0;
    let mut sink = 0;
    let mut i = 1;
    // choose the sink as the furthest vertex from the source
    let mut queue = VecDeque::new();
    queue.push_back(source);
    while let Some(node) = queue.pop_front() {
        if visited[node] == i {
            continue;
        }
        visited[node] = i;
        sink = node;
        for nbor in &graph[node] {
            queue.push_back(*nbor);
        }
    }

    loop {
        i += 1;
        let mut queue = VecDeque::new();
        queue.push_back((source, source));
        let mut sink_reached = false;

        while let Some((node, par)) = queue.pop_front() {
            if node == sink {
                sink_reached = true;
                parent[node] = par;
                break;
            }
            
            if visited[node] == i {
                continue;
            }
            visited[node] = i;
            parent[node] = par;
            for &nbor in &graph[node] {
                queue.push_back((nbor, node));
            }
        }

        if sink_reached {
            let mut current = sink;
            while parent[current] != current {
                let par = parent[current];
                let i = graph[par].iter().position(|&r| r == current).unwrap();
                graph[par].remove(i);
                current = parent[current];
            }
        } else {
            let m = visited.iter().filter(|&&node| node == i).count();
            return m * (n - m);
        }
    }
}