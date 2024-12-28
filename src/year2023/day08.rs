use ahash::{HashMap, HashMapExt};
use num_integer::Integer;

pub fn solve(input: &str) -> (usize, usize) {
    let (directions, rest) = input.split_once("\n\n").unwrap();
    let directions = directions.as_bytes();
    
    let mut nodes = HashMap::with_capacity(750);
    let mut starting_nodes = Vec::new();
    
    for chunk in rest.as_bytes().array_chunks::<17>() {
        let label1 = (chunk[0], chunk[1], chunk[2]);
        let label2 = (chunk[7], chunk[8], chunk[9]);
        let label3 = (chunk[12], chunk[13], chunk[14]);
        nodes.insert(label1, (label2, label3));
        if label1.2 == b'A' {
            starting_nodes.push(label1);
        }
    }

    let n = directions.len();
    let mut p1 = n;
    let mut p2 = n;

    for &start in &starting_nodes {
        let mut counter = 0;
        let mut current1 = start;
        let mut current2 = start;
        while current1.2 != b'Z' && current2.2 != b'Z' {
            (current1, current2) = nodes[&current2];
            counter += 1;
        }
        if current1 == (b'Z', b'Z', b'Z') || current2 == (b'Z', b'Z', b'Z') {
            p1 = p1.lcm(&counter);
        }
        p2 = p2.lcm(&counter);
    }

    (p1, p2)
}