use anyhow::*;
use arrayvec::ArrayVec;

struct Input {
    matrix: Vec<u32>,
    size: usize,
    start: usize,
    flows: Vec<u32>
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let input = parse_input(input);

    Ok((0, 0))
} 

fn parse_input(input: &str) -> Input {
    let mut graph: Vec<ArrayVec<usize, 12>> = Vec::new();
    let mut mat_to_graph = Vec::new();
    let mut flows = Vec::new();
    let mut table = [usize::MAX; 676];
    let mut start = 0;
    for line in input.lines() {
        let line = line.as_bytes();
        let index = 26 * line[6] as usize + line[7] as usize - 1755;
        let flow = (line[23] - b'0') as u32;
        let (flow, nbor_index) =
            if line[24] == b';' {
                (flow, 49)
            } else {
                (flow * 10 + (line[24] - b'0') as u32, 50)
            };
        let mut i = table[index];
                
        if i == usize::MAX {
            i = graph.len();
            table[index] = i;
            graph.push(ArrayVec::new());
        }
        if flow > 0 {
            flows.push(flow);
            mat_to_graph.push(i);
        }

        if index == 0 {
            start = mat_to_graph.len() - 1;
        }
        for &[c1, c2] in line[nbor_index..].array_chunks().step_by(2) {
            let index2 = 26 * c1 as usize + c2 as usize - 1755;
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

    let size = mat_to_graph.len();
    let mut graph_to_mat = vec![usize::MAX; graph.len()];
    for (i, &j) in mat_to_graph.iter().enumerate() {
        graph_to_mat[j] = i;
    }
    
    let mut matrix = vec![0; size * size];

    Input { matrix, size, start, flows }
}