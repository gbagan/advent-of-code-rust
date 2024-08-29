// connected components

fn parse_node(line: &str) -> Option<Vec<usize>> {
    let (_, nbors) = line.split_once("<-> ")?;
    Some(nbors.split(", ").filter_map(|v| v.parse().ok()).collect())
}

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let graph: Vec<_> = input.lines().filter_map(parse_node).collect();
    let n = graph.len();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut seen = vec![false; n];
    
    for start in 0..n {
        if seen[start] {
            continue;
        }
        p2 += 1;
        let mut stack = vec!(start);
        while let Some(v) = stack.pop() {
            if seen[v] {
                continue;
            }
            seen[v] = true;
            if start == 0 {
                p1 += 1;
            }
            for u in &graph[v] {
                stack.push(*u);
            }
        }

    }
    Some((p1, p2))
}