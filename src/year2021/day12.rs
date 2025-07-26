use ahash::{HashMap, HashMapExt};

pub fn solve(input: &str) -> (u32, u32) {
    const N: usize = 0x10000;
    
    let input: Vec<_> = input
        .as_bytes()
        .split(|c| !c.is_ascii_alphabetic())
        .filter(|s| !s.is_empty())
        .collect();
    let mut to_index = HashMap::new();
    to_index.insert(b"start".as_slice(), 0);
    to_index.insert(b"end".as_slice(), 1);
    
    let mut small_index = 1;
    let mut big_index = N-1;

    for &label in &input {
        to_index.entry(label).or_insert_with(|| {
            if label[0].is_ascii_lowercase() {
                small_index += 1;
                small_index
            } else {
                big_index += 1;
                big_index
            }
        });
    }

    let n = small_index + 1;
    let mut matrix = vec![0u32; n*n];
    let mut edges: Vec<_> = vec![Vec::new(); big_index+1-N];
    
    for [&l1, &l2] in input.iter().array_chunks() {
        let u = to_index[l1];
        let v = to_index[l2];
        let min = u.min(v);
        let max = u.max(v);
        if max < n {
            matrix[min*n+max] += 1;
            matrix[max*n+min] += 1;
        } else {
            edges[max-N].push(min);
        }
    }

    for nbors in &edges {
        for nbor in nbors {
            for nbor2 in nbors {
                matrix[nbor*n+nbor2] += 1;
            }
        }
    }

    let m = 1usize << n;
    let mut paths = vec![0; m * n];
    let mut paths_twice = vec![0; m * n];
    // 1 is the state with vertex = start and visited = {start}
    paths[1] = 1;

    // not visited twice
    for visited in 0..m {
        for vertex in 0..n {
            let count = paths[vertex << n | visited];
            if count == 0 {
                continue;
            }
            for (nbor, w) in matrix[vertex*n..(vertex+1)*n].iter().enumerate() {
                if 1 << nbor & !visited != 0 { // not visited
                    paths[nbor << n | visited | 1 << nbor] += w * count;
                } else if nbor > 1 { // visited
                    paths_twice[nbor << n | visited] += w * count;
                }
            }
        }
    }

    // visited twice
    for visited in 0..m {
        for vertex in 0..n {
            let count = paths_twice[vertex << n | visited];
            if count == 0 {
                continue;
            }
            for nbor in 0..n {
                if 1 << nbor & !visited != 0 { // not visited
                    let w = matrix[vertex*n+nbor];
                    paths_twice[nbor << n | visited | 1 << nbor] += w * count;
                }
            }
        }
    }

    let p1 = paths[m..2*m].iter().sum();
    let p2 = p1 + paths_twice[m..2*m].iter().sum::<u32>();

    (p1, p2)
}