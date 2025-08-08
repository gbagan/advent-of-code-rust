// Ford-Fulkerson algorithm

use arrayvec::ArrayVec;

type Graph = Vec<ArrayVec<usize, 10>>;

pub fn solve(input: &str) -> (usize, usize) {
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
        for &[_, l1, l2, l3] in line[4..].as_chunks().0 {
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
    (p1, 0)
}

fn part1(graph: &mut Graph) -> usize {
    let n = graph.len();
    let mut visited: Vec<_> = vec![0; n];
    let mut parent: Vec<_> = vec![0; n];
    let source = 0;
    let mut sink = 0;
    // choose the sink as the furthest vertex from the source
    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();
    queue1.push(source);
    visited[source] = 1;
    while !queue1.is_empty() {
        for &node in &queue1 {
            sink = node;
            for &next in &graph[node] {
                if visited[next] != 1 {
                    visited[next] = 1;
                    queue2.push(next);
                }
            }
        }
        std::mem::swap(&mut queue1, &mut queue2);
        queue2.clear();
    }
    let mut i = 1;

    loop {
        i += 1;
        queue1.clear();
        queue2.clear();
        queue1.push(source);
        visited[source] = i;
        parent[source] = source;

        let mut sink_reached = false;

        while !queue1.is_empty() {
            for &node in &queue1 {
                if node == sink {
                    sink_reached = true;
                    break;
                }
                for &next in &graph[node] {
                    if visited[next] != i {
                        visited[next] = i;
                        parent[next] = node;
                        queue2.push(next);
                    }
                }
            }
            std::mem::swap(&mut queue1, &mut queue2);
            queue2.clear();
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