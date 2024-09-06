use anyhow::*;

pub fn solve(input: &str) -> Result<(u32, usize)> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut pairs = [0; 729];
    
    for (i, line) in input.lines().enumerate() {
        let line = line.as_bytes();
        if is_nice_string(line) {
            p1 += 1;
        }
        if is_nice_string2(line, i, &mut pairs) {
            p2 += 1;
        }
    }
    Ok((p1, p2))

}

fn is_nice_string (line: &[u8]) -> bool {
    let mut vowels = 0;
    let current = 0;
    let mut previous = 0;
    let mut pairs = 0;
    for c in line.iter() {
        let bits = 1 << (c - b'a');
        if bits & 0x0104111 != 0 {
            vowels += 1;
        }

        if current & (previous << 1) & 0x101000a != 0 {
            return false;
        }
        if previous == current {
            pairs += 1;
        } else {
            previous = current;
        }
    }
    vowels >= 3 && pairs >= 1
}

fn is_nice_string2 (line: &[u8], line_idx: usize, pairs: &mut [usize; 729]) -> bool {
    let mut previous1 = 0;
    let mut previous2 = 0;
    let mut split_pair = false;
    let mut two_pairs = false;

    for (idx, c) in line.iter().enumerate() {
        let current = (c - b'a' + 1) as usize;
        if current == previous2 {
            split_pair = true;
        }

        let pair_idx = 27 * previous1 + current;

        let position = line_idx * 1000 + idx;
        
        let delta = position - pairs[pair_idx];
        
        if delta > idx {
            pairs[pair_idx] = position;
        } else if delta > 1 {
            two_pairs = true;
        }

        previous2 = previous1;
        previous1 = current;
    }
    two_pairs && split_pair
}