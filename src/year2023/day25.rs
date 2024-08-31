// Ford-Fulkerson algorithm

use crate::util::iter::AOCIter;
use std::collections::{HashMap, HashSet, VecDeque};

type Graph = Vec<Vec<usize>>;

fn parse_line(line: &str) -> Option<(&str, Vec<&str>)> {
    let (node, neighbors) = line.split_once(": ")?;
    let neighbors = neighbors.split(' ').collect();
    Some((node, neighbors))
}

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let nodes: Vec<_> = input.lines().filter_map(parse_line).collect();
    let mut graph: Vec<Vec<_>> = Vec::with_capacity(nodes.len());
    let mut i = 0;
    let mut dict = HashMap::new();
    for (node, nbors) in &nodes {
        if !dict.contains_key(node) {
            dict.insert(node, i);
            i += 1;
        }
        for nbor in nbors {
            if !dict.contains_key(nbor) {
                dict.insert(nbor, i);
                i += 1;
            }
        }

    }
    for _ in 0..i {
        graph.push(vec!());
    }
    for (node, nbors) in &nodes {
        let i = dict[node];
        for nbor in nbors {
            let j = dict[nbor];
            graph[i].push(j);
            graph[j].push(i);
        }
    }

    let p1 = part1(&graph);
    Some((p1, 0))
}

fn part1(graph: &Graph) -> usize {
    let n = graph.len();
    let mut saturated = HashSet::new(); //Grid::new(n, n, false);
    let mut visited: Vec<_> = graph.iter().map(|_| 0).collect();
    let mut parent: Vec<_> = graph.iter().map(|_| 0).collect();
    let source = 0;
    let mut sink = 0;
    let mut i = 1;
    // choose the sink as the farest vertex from the source
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
                if !saturated.contains(&(node, nbor)) {
                    queue.push_back((nbor, node));
                }
            }
        }

        if sink_reached {
            let mut current = sink;
            while parent[current] != current {
                saturated.insert ((parent[current], current));
                current = parent[current];
            }
        } else {
            let m = visited.iter().count_by(|&node| node == i);
            return m * (n - m);
        }
    }
}