use crate::util::foreach_permutation;

const N: usize = 8; 

pub fn solve(input: &str) -> (u32, u32) {
    let input = input.as_bytes();
    let mut positions = [0; N];
    let width = input.iter().position(|&c| c == b'\n').unwrap() + 1;
    for (i, &c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            positions[(c - b'0') as usize] = i;
        }
    }

    let mut distance = [0; N*N];

    // BFS
    let mut seen = vec![u8::MAX; input.len()];
    let directions = [1, usize::MAX, width, 0usize.wrapping_sub(width)];

    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();

    for (i, &start) in positions.iter().enumerate() {
        queue1.push(start);
        seen[start] = i as u8;
        let mut dist = 0;
        while !queue1.is_empty() {
            for &position in &queue1 {
                if input[position].is_ascii_digit() {
                    distance[i * N + (input[position] - b'0') as usize] = dist;
                }
                for &dir in &directions {
                    let next = position.wrapping_add(dir);
                    if input[next] != b'#' && seen[next] != i as u8 {
                        seen[next] = i as u8;
                        queue2.push(next);
                    }
                }
            }
            dist += 1;
            std::mem::swap(&mut queue1, &mut queue2);
            queue2.clear();
        }
    }

    let mut p1 = u32::MAX;
    let mut p2 = u32::MAX;
    let mut indices: [usize; N-1] = std::array::from_fn(|i| i+1);

    foreach_permutation(&mut indices, |perm| {
        let path = perm.array_windows().map(|&[a, b]| distance[a*N+b]).sum::<u32>();
        let start = distance[perm[0]];
        let end = distance[perm[perm.len()-1]];
        p1 = p1.min(path+start);
        p2 = p2.min(path+start+end);
    });

    (p1, p2)
}