use std::time::Instant;
use std::collections::HashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use petgraph::Graph;
use petgraph::algo::tarjan_scc;
use aoc::graph::minimumcut::minimum_cut;

fn input_parser(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    fn row(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
        let (input, node) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (input, nbors) = separated_list1(space1, alpha1)(input)?;
        Ok((input, (node, nbors)))
    }
    separated_list1(line_ending, row)(input)
}

fn main() {
    let input = include_str!("../../inputs/2023/25");
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, network_list)) => {
            let start = Instant::now();
            
            let mut node_map = HashMap::new();           
            let mut graph = Graph::new_undirected();
            for (label, _) in &network_list {
                let node = graph.add_node(label);
                node_map.insert(label, node);
            }

            for (label, nbors) in &network_list {
                for nbor in nbors {
                    if ! node_map.contains_key(nbor) {
                        let node = graph.add_node(nbor);
                        node_map.insert(nbor, node);
                    }
                    graph.add_edge(node_map[label], node_map[nbor], 1);
                }
            }

            let (cut, _) = minimum_cut(&graph, |e| *e.weight());
            for edge in cut {
                graph.remove_edge(edge);
            }
            let components = tarjan_scc(&graph);
            let p1: usize = components.iter().map(|c| c.len()).product();

            let end = start.elapsed().as_micros();
        
            println!("Part 1: {}", p1);
            println!("Time: {} μs", end);
        }
    }
}