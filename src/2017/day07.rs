use aoc::aoc_with_parser;
use std::collections::HashMap;
use petgraph::algo::toposort;
use petgraph::{Graph,graph::NodeIndex};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, i32},
    multi::separated_list1,
    combinator::opt,
    sequence::{preceded,delimited},
    IResult,
};
use itertools::{Itertools, MinMaxResult};

type TreeList = Vec<(String, i32, Vec<String>)>;

struct Node {
    id: String,
    weight: i32,
    total_weight: i32,
}

fn input_parser(input: &str) -> IResult<&str, TreeList> {
    fn row(input: &str) -> IResult<&str, (String, i32, Vec<String>)> {
        let (input, node) = alpha1(input)?;
        let (input, weight) = delimited(tag(" ("), i32, tag(")"))(input)?;
        let (input, ns) = children(input)?;
        let ns = ns.unwrap_or(Vec::new()).iter().map(|s| s.to_string()).collect();
        Ok((input, (node.to_string(), weight, ns)))
    }

    fn children(input: &str) -> IResult<&str, Option<Vec<&str>>> {
        opt(preceded(tag(" -> "), separated_list1(tag(", "), alpha1)))(input)
    }
    separated_list1(line_ending, row)(input)
}

fn build_graph_and_ordering(treelist: &TreeList) -> (Graph<Node,()>, Vec<NodeIndex>) {
    let mut indices = HashMap::new();
    let mut graph = Graph::<Node, ()>::new();

    for (node_id, weight, _) in treelist {
        let node = Node {
            id: node_id.clone(),
            weight: *weight,
            total_weight: *weight,
        };
        let idx = graph.add_node(node);
        indices.insert(node_id, idx);
    }

    for (node_id, _, children) in treelist {
        for child in children {
            graph.add_edge(indices[node_id], indices[child], ());
        }
    }

    let ordering = toposort(&graph, None).unwrap();
    (graph, ordering)
}

fn part2(graph: &mut Graph<Node,()>, ordering: &Vec<NodeIndex>) -> Option<i32> {
    for &node in ordering.iter().rev() {
        match graph.neighbors(node).map(|n| graph[n].total_weight).minmax() {
            MinMaxResult::MinMax(min, max) if min < max => {
                let (mins, maxs): (Vec<_>, Vec<_>) =
                    graph.neighbors(node).partition(|n| graph[*n].total_weight == min);
                return if mins.len() == 1 {
                    Some(graph[mins[0]].weight + max - min)
                } else {
                    Some(graph[maxs[0]].weight + min - max)
                }
            }
            _ => {
                graph[node].total_weight +=
                    graph.neighbors(node).map(|n| graph[n].total_weight).sum::<i32>();
            }
        }
    }
    None
}

fn main() {
    let input = include_str!("../../inputs/2017/07");
    aoc_with_parser(input, input_parser, |treelist| {
        let (mut graph, ordering) = build_graph_and_ordering(&treelist);            
        let p1 = graph[ordering[0]].id.clone();
        let p2 = part2(&mut graph, &ordering).unwrap();
        (p1, p2)
    })
}