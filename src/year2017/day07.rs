use std::collections::HashMap;
use itertools::{Itertools, MinMaxResult};

pub struct Node<'a> {
    id: &'a str,
    weight:u32,
    total_weight: u32,
    children: Vec<usize>,
}

fn parse_line(line: &str) -> Option<(&str, u32, Vec<&str>)> {
    let (node, children) =
        if let Some((left, right)) = line.split_once(" -> ") {
            (left, right.split(", ").collect())
        } else {
            (line, vec!())
        };
    let (id, weight) = node.split_once(" (")?;
    let weight = weight.trim_end_matches(')').parse().ok()?;
    Some((id, weight, children))
}

pub fn parse(input: &str) -> Option<(String, u32)> {
    let raw_nodes: Vec<_> = input.lines().filter_map(parse_line).collect();
    let mut tree = Vec::with_capacity(raw_nodes.len());
    let mut has_parent = vec![false; raw_nodes.len()];
    let mut  id_to_index = HashMap::new();
    for (i, (id, ..)) in raw_nodes.iter().enumerate() {
        id_to_index.insert(*id, i);
    }
    for (id, weight, children) in &raw_nodes {
        tree.push(Node {id, weight: *weight, total_weight: *weight, 
                        children: children.iter().map(|id| id_to_index[id]).collect()
                    });
        for child in &tree[tree.len()-1].children {
            has_parent[*child] = true;
        }
    }
    
    let root = has_parent.iter().position(|b| !b)?;
    let p1 = tree[root].id.to_string();
    let mut p2 = 0;

    let ordering: Vec<usize> = dfs(&tree, root);
    for node in ordering.iter().rev() {
        match tree[*node].children.iter().map(|&n| tree[n].total_weight).minmax() {
            MinMaxResult::MinMax(min, max) if min < max => {
                let (mins, maxs): (Vec<usize>, Vec<usize>) =
                    tree[*node].children.iter().partition(|&n| tree[*n].total_weight == min);
                p2 = if mins.len() == 1 {
                    tree[mins[0]].weight + max - min
                } else {
                    tree[maxs[0]].weight + min - max
                };
                break;
            }
            _ => {
                tree[*node].total_weight +=
                    tree[*node].children.iter().map(|&n| tree[n].total_weight).sum::<u32>();
            }
        }
    }
    Some((p1, p2))
}

pub fn part1(solution: &(String, u32)) -> Option<String> {
    Some(solution.0.clone())
}

pub fn part2(solution: &(String, u32)) -> Option<u32> {
    Some(solution.1)
}

fn dfs(tree: &[Node], root: usize) -> Vec<usize> {
    let mut output = vec!();
    let mut queue = vec!(root);
    while let Some(node) = queue.pop() {
        output.push(node);
        for child in &tree[node].children {
            queue.push(*child);
        }
    }
    output
}