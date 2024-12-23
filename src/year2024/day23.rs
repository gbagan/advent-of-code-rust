use anyhow::*;

struct Input {
    graph: Vec<Vec<usize>>,
    reverse_table: Vec<(u8, u8)>,
    matrix: Vec<bool>
}

pub fn solve(input: &str) -> Result<(u32, String)> {
    let mut graph: Vec<Vec<usize>> = Vec::new();
    let mut table = [usize::MAX; 26*26];
    let mut reverse_table = Vec::new();
    
    for &[l11, l12, _, l21, l22, _] in input.as_bytes().array_chunks() {
        let n = graph.len();
        let index = l11 as usize * 26 + l12 as usize - 2619;
        let mut i = table[index];
        if i == usize::MAX {
            i = n;
            table[index] = n;
            graph.push(Vec::with_capacity(13));
            reverse_table.push((l11, l12));
        }

        let n = graph.len();
        let index = l21 as usize * 26 + l22 as usize - 2619;
        let mut j = table[index];
        if j == usize::MAX {
            j = n;
            table[index] = n;
            graph.push(Vec::with_capacity(13));
            reverse_table.push((l21, l22));
        }
        graph[i].push(j);
        graph[j].push(i);
    }

    let n = graph.len();
    let mut matrix = vec![false; n*n];
    for (i, nbor) in graph.iter().enumerate() {
        for j in nbor {
            matrix[n * i + j] = true;
        }
    }

    let input = Input { graph, matrix, reverse_table };

    let p1 = part1(&input);
    let p2 = part2(&input).context("Part 2: No solution found")?;

    Ok((p1, p2))
}

fn part1(input: &Input) -> u32 {
    let Input {graph, matrix, reverse_table} = input;
    let n = graph.len();
    let mut count = 0;
    for (u, nbor) in graph.iter().enumerate() {
        if reverse_table[u].0 != b't' {
            continue;
        }
        for (j, &v) in nbor.iter().enumerate() {
            if v < u && reverse_table[v].0 == b't' {
                continue;
            }
            for &w in &nbor[j+1..] {
                if matrix[v * n + w] && !(w < u && reverse_table[w].0 == b't') {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part2(input: &Input) -> Option<String> {
    let Input {graph, matrix: _, reverse_table} = input;
    let n = graph.len();
    let mut neighborhood = vec![false; n];

    let mut found = None;


    'outer: for u in 0..n {
        let mut excluded = None;
        for x in &mut neighborhood {
            *x = false;
        }
        neighborhood[u] = true;
        for &v in &graph[u] {
            neighborhood[v] = true;
        }

        let nbor = &graph[u];
        for &v in nbor {
            let mut c = 0;
            for &w in &graph[v] {
                if !neighborhood[w] {
                    c += 1;
                }
            }
            if c > 1 {
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
        let mut labels = Vec::new();
        labels.push(reverse_table[u]);
        for &v in &graph[u] {
            if v != excluded {
                labels.push(reverse_table[v]);
            }
        }
        labels.sort_unstable();
        let mut string = String::with_capacity(2*labels.len() + 1);
        for (i, &(u, v)) in labels.iter().enumerate() {
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