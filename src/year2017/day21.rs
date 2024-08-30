pub fn solve(input: &str) -> Option<(u32, u32)> {
    let mut two_to_three: [usize; 16] = [0; 16]; 
    let mut three_to_two: [[usize; 4]; 512] = [[0; 4]; 512]; 
    for line in input.lines() {
        let (left, right) = line.split_once(" => ")?;
        let left: Vec<_> = left.bytes().filter(|&c| c != b'/').collect();
        let right: Vec<_> = right.bytes().filter(|&c| c != b'/').collect();
        if left.len() == 4 {
            let right = encode(&right);
            for i in orientations2(&left) {
                two_to_three[i] = right;
            }
        }
    }
    None
}

fn orientations2(block: &[u8]) -> [usize; 8] {
    let mut block = block.to_vec();
    let output = [0; 8];
    for i in 0..8 {

    }
}


pub fn encode(block: &[u8]) -> usize {
    block.iter().fold(0, |acc, &n| acc * 2 + (if n == b'#' {1} else {0}))
}