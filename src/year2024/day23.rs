// counting triangles and finding maximum clique

use anyhow::*;
use arrayvec::ArrayVec;

struct Graph {
    adj: Vec<ArrayVec<usize, 13>>,
    labels: Vec<(u8, u8)>,
    matrix: Vec<bool>,
}

pub fn solve(input: &str) -> Result<(u32, String)> {
    let n = input.len() / (3 * 13);

    let mut adj = Vec::with_capacity(n);
    let mut table = [usize::MAX; 26*26];
    let mut labels = Vec::with_capacity(n);
    let mut matrix = vec![false; n*n];

    for &[l11, l12, _, l21, l22, _] in input.as_bytes().array_chunks() {
        let index = l11 as usize * 26 + l12 as usize - 2619;
        let mut i = table[index];
        if i == usize::MAX {
            i = adj.len();
            table[index] = i;
            adj.push(ArrayVec::new());
            labels.push((l11, l12));
        }

        let index = l21 as usize * 26 + l22 as usize - 2619;
        let mut j = table[index];
        if j == usize::MAX {
            j = adj.len();
            table[index] = j;
            adj.push(ArrayVec::new());
            labels.push((l21, l22));
        }
        adj[i].push(j);
        adj[j].push(i);
        matrix[n * i + j] = true;
        matrix[n * j + i] = true;
    }

    let graph = Graph { adj, matrix, labels };
    let p1 = part1(&graph);
    let p2 = part2(&graph).context("Part 2: No solution found")?;

    Ok((p1, p2))
}

fn part1(input: &Graph) -> u32 {
    let Graph {adj, matrix, labels} = input;
    let n = adj.len();
    let mut seen = [false; 676];
    let mut count = 0;
    for (u, nbor) in adj.iter().enumerate() {
        if labels[u].0 != b't' {
            continue;
        }
        seen[u] = true;
        for (j, &v) in nbor.iter().enumerate() {
            if seen[v] {
                continue;
            }
            for &w in &nbor[j+1..] {
                if matrix[v * n + w] && !seen[w] {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &Graph) -> Option<String> {
    let Graph {adj, matrix, labels} = input;
    let n = adj.len();
    let mut found = None;

    'outer: for u in 0..n {
        let mut excluded = None;
        let neighborhood = &matrix[u*n..];
        for &v in &adj[u] {
            let mut c = 0;
            for &w in &adj[v] {
                c += neighborhood[w] as u32;
            }
            if c < 11 {
                match excluded {
                    None => excluded = Some(v),
                    Some(_) => continue 'outer,
                }
            }
        }
        match excluded {
            None => unreachable!(),
            Some(e) => found = Some((u, e)),
        }
        
    };
    if let Some((u, excluded)) = found {
        let mut clique_labels = Vec::with_capacity(13);
        clique_labels.push(labels[u]);
        for &v in &adj[u] {
            if v != excluded {
                clique_labels.push(labels[v]);
            }
        }
        clique_labels.sort_unstable();
        let mut string = String::with_capacity(27);
        for (i, &(u, v)) in clique_labels.iter().enumerate() {
            if i > 0 {
                string.push(',');
            }
            string.push(u as char);
            string.push(v as char);
        }
        Some(string)
    } else {
        None
    }
}