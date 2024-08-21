use crate::util::iter::AOCIter;

pub fn parse(input: &str) -> Option<Vec<&[u8]>> {
    Some(input.lines().map(str::as_bytes).collect())
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

pub fn part1(input: &Vec<&[u8]>) -> Option<u32> {
    Some(input.iter().count_by(|&line| is_nice_string(line)) as u32)
}

pub fn part2(input: &Vec<&[u8]>) -> Option<usize> {
    let mut pairs = [0; 729];
    Some(input.iter().enumerate().count_by(|(idx, line)| is_nice_string2(line, idx, &mut pairs)))
}