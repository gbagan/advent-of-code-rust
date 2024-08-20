pub fn reverse(lengths: &Vec<u8>, nb_rounds: u32) -> Vec<u8> {
    let mut knot: Vec<u8> = (0..=255).collect();
    let mut pos: u8 = 0;
    let mut skip = 0;
    for _ in 0..nb_rounds {
        for i in lengths {
            for j in 0..i/2 {
                knot.swap((pos+j) as usize,(pos+i-1-j) as usize);
            }
            pos += skip + i;
            skip += 1;
        }
    }
    knot
}

pub fn knothash(input: &str) -> Vec<u8> {
    let mut lengths: Vec<_> = input.bytes().collect(); 
    lengths.extend([17, 31, 73, 47, 23]);
    let sparse_hash = reverse(&lengths, 64);
    sparse_hash.chunks(16)
        .map(|chunk| chunk.iter().fold(0, |x, y| x^y))
        .collect()
}