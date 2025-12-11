const YOU: u16 = 676 * (b'y' - b'a') as u16 + 26 * (b'o' - b'a') as u16 + (b'u' - b'a') as u16;
const OUT: u16 = 676 * (b'o' - b'a') as u16 + 26 * (b'u' - b'a') as u16 + (b't' - b'a') as u16;
const SVR: u16 = 676 * (b's' - b'a') as u16 + 26 * (b'v' - b'a') as u16 + (b'r' - b'a') as u16;
const DAC: u16 = 676 * (b'd' - b'a') as u16 + 26 * (b'a' - b'a') as u16 + (b'c' - b'a') as u16;
const FFT: u16 = 676 * (b'f' - b'a') as u16 + 26 * (b'f' - b'a') as u16 + (b't' - b'a') as u16;

pub fn solve(input: &str) -> (u64, u64) {
    let lines: Vec<_> = input.lines().collect();
    let mut graph: Vec<Vec<u16>> = vec![vec!(); 600];
    let mut index_table = [u16::MAX; 26 * 26 * 26]; 
    index_table[YOU as usize] = 0;
    index_table[OUT as usize] = 1;
    index_table[SVR as usize] = 2;
    index_table[DAC as usize] = 3;
    index_table[FFT as usize] = 4;
    let mut index = 4;

    let mut to_index = |s: &[u8]| {
        let i = s[0] as usize * 676 + s[1] as usize * 26 + s[2] as usize - 97 * (676 + 26 + 1);
        let idx = index_table[i];
        if idx == u16::MAX {
            index += 1;
            index_table[i] = index;
            index
        } else {
            idx
        }
    };

    for line in lines {
        let line = line.as_bytes();
        let id1 = to_index(line);
        for token in line[5..].chunks(4) {
            let id2 = to_index(token);
            graph[id1 as usize].push(id2);
        }
    }

    let order = topological_ordering(&graph);

    let p1 = compute_paths(&graph, &order, 0)[1];

    let from_svr = compute_paths(&graph, &order, 2);
    let svr_dac = from_svr[3];
    let svr_fft = from_svr[4];
    let from_dac = compute_paths(&graph, &order, 3);
    let dac_out = from_dac[1];
    let dac_fft = from_dac[4];
    let from_fft = compute_paths(&graph, &order, 4);
    let fft_out = from_fft[1];
    let fft_dac = from_fft[3];

    let p2 = svr_dac * dac_fft * fft_out + svr_fft * fft_dac * dac_out;

    (p1, p2)
}

fn compute_paths(graph: &[Vec<u16>], order: &[usize], start: usize) -> Vec<u64> {
    let mut paths = vec![0; graph.len()];
    paths[start] = 1;

    for &u in order {
        for &v in &graph[u] {
            paths[v as usize] += paths[u];
        }
    }
    paths
}

fn topological_ordering(graph: &[Vec<u16>]) -> Vec<usize> {
    let mut seen = vec![false; graph.len()];
    let mut order = Vec::new();
    
    fn dfs(graph: &[Vec<u16>], seen: &mut [bool], order: &mut Vec<usize>, i: usize) {
        seen[i] = true;
        for &nbor in &graph[i] {
            if !seen[nbor as usize] {
                dfs(graph, seen, order, nbor as usize);
            }
        }
        order.push(i);
    }
    
    for i in 0..graph.len() {
        if !seen[i] {
            dfs(graph, &mut seen, &mut order, i);
        }
    }
    order.reverse();
    order
}