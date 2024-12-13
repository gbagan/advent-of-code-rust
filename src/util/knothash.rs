use itertools::Itertools;
pub fn reverse(lengths: &Vec<usize>, nb_rounds: u32) -> Vec<u8> {
    let mut knot: Vec<u8> = (0..=255).collect();
    let mut pos = 0;
    let mut skip = 0;
    for _ in 0..nb_rounds {
        for &length in lengths {
            let next = length + skip;
            knot[0..length].reverse();
            knot.rotate_left(next % 256);
            pos += next;
            skip += 1;
        }
    }
    knot.rotate_right(pos % 256);
    knot
}

pub fn knothash(input: &str) -> [u8;16] {
    let mut lengths: Vec<_> = input.bytes().map(|l| l as usize).collect(); 
    lengths.extend([17, 31, 73, 47, 23]);
    let sparse_hash = reverse(&lengths, 64);
    sparse_hash.chunks_exact(16)
        .map(|chunk| chunk.iter().fold(0, |x, y| x^y))
        .collect_vec()
        .try_into()
        .unwrap()
}