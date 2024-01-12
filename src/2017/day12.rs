use aoc::aoc_with_parser;
use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending,u32},
    IResult,
    sequence::separated_pair,
    multi::separated_list1,
};
use petgraph::Graph;
use petgraph::visit::Dfs;
use petgraph::algo::connected_components;

fn input_parser(input: &str) -> IResult<&str,Vec<(u32, Vec<u32>)>> {
    let node = separated_pair(u32, tag(" <-> "), separated_list1(tag(", "), u32));
    separated_list1(line_ending, node)(input)
}

fn main() {
    let input = include_str!("../../inputs/2017/12");
    aoc_with_parser(input, input_parser, |nodes| {
        let mut graph = Graph::new_undirected();
        let mut node_map = HashMap::new();

        for (node, _) in &nodes {
            let node_index = graph.add_node(node);
            node_map.insert(node, node_index);
        }

        for (node, nbors) in &nodes {
            for nbor in nbors {
                if node < nbor {
                    graph.add_edge(node_map[&node], node_map[&nbor], ());
                }
            }
        }

        let mut dfs = Dfs::new(&graph, node_map[&0]);
        let mut p1 = 0;
        while let Some(_) = dfs.next(&graph) { p1 += 1; }

        let p2 = connected_components(&graph);
        (p1, p2)
    })
}