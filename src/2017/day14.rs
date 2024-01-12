use std::collections::HashMap;
use aoc::knothash::knothash;
use aoc::aoc;
use petgraph::Graph;
use petgraph::algo::connected_components;

fn get_hashes(input: &str) -> Vec<Vec<u8>> {
    (0..128)
    .map(|i| knothash(&format!("{}-{}", input, i)))
    .collect()
}

fn count_ones(n: u8) -> u32 {
    let mut count = 0;
    let mut n = n;

    while n != 0 {
        count += 1;
        n &= n - 1;
    }

    count
}

fn part1(hashes: &Vec<Vec<u8>>) -> u32 {
    hashes
    .iter()
    .map(|h| h.iter()
              .map(|&n| count_ones(n))
              .sum::<u32>()
        )
    .sum()
}

fn is_used(hashes: &Vec<Vec<u8>>, (i, j) : (usize, usize)) -> bool {
    hashes[i][j/8] >> (7 - j%8) & 1 == 1
}

fn part2(hashes: &Vec<Vec<u8>>) -> usize {
    let mut graph = Graph::new_undirected();
    let mut node_map = HashMap::new();
    for i in 0..128 {
        for j in 0..128 {
            if is_used(hashes, (i, j)) {
                let node = graph.add_node(());
                node_map.insert((i, j), node);
            }
        }
    }
    for i in 0..128 {
        for j in 0..127 {
            if is_used(hashes, (i, j)) && is_used(hashes, (i, j+1)) {
                graph.add_edge(node_map[&(i, j)], node_map[&(i, j+1)], ());
            }
            if is_used(hashes, (j, i)) && is_used(hashes, (j+1, i)) {
                graph.add_edge(node_map[&(j, i)], node_map[&(j+1, i)], ());
            }
        }
    }
    connected_components(&graph)
}

fn main() {
    let input = include_str!("../../inputs/2017/14");
    aoc(|| {
        let hashes = get_hashes(&input);
        (part1(&hashes), part2(&hashes))
    })
}