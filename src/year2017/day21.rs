use anyhow::*;
use crate::util::{iter::*, parser::*};

struct Pattern {
    three: u32,
    four: u32,
    six: u32,
    nine: [usize; 9],
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut block_three_index = [0usize; 16]; 
    let mut two_to_three = [[false; 9]; 16]; 
    let mut three_to_four = [[false; 16]; 512]; 
    
    let start= encode(&b".#...####".map(|c| c == b'#'));
    let mut blocks_three = vec!(start);
    let mut i = 1;
    for line in input.lines() {
        let (left, right) = line.try_split_once(" => ")?;
        let left: Vec<_> = left.bytes().filter(|&c| c != b'/').map(|c| c == b'#').collect();
        let right = right.as_bytes();
        match left.len() {
            4 => {
                let right = [0, 1, 2, 4, 5, 6, 8, 9, 10].map(|i| right[i] == b'#');
                blocks_three.push(encode(&right));
                
                for j in orientations_two(&left) {
                    two_to_three[j] = right;
                    block_three_index[j] = i;
                }
                
                i += 1;
            }
            9 => {
                let right = [0, 1, 2, 3, 5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18]
                                        .map(|i| right[i] == b'#');
                for j in orientations_three(&left) {
                    three_to_four[j] = right;
                }
            }
            _ => bail!("invalid input")
        }
    }

    let patterns: Vec<_> = blocks_three.iter().map(|&three| {
        let four = three_to_four[three];
        let mut six = [false; 36];
        for (from, to) in [(0, 0), (2, 3), (8, 18), (10, 21)] {
            let block_two = [four[from], four[from+1], four[from+4], four[from+5]];
            let block_three = two_to_three[encode(&block_two)];
            for i in 0..3 {
                for j in 0..3 {
                    six[to + i * 6 + j] = block_three[i * 3 + j];
                }
            }
        }
        let nine = [0, 2, 4, 12, 14, 16, 24, 26, 28].map(|i| {
            let encoded = encode(&[six[i], six[i + 1], six[i + 6], six[i + 7]]);
            block_three_index[encoded]
        });
        Pattern {
            three: three.count_ones(),
            four: four.iter().count_if(|&&b| b) as u32,
            six: six.iter().count_if(|&&b| b) as u32,
            nine,
        }
    }).collect();

    let mut freqs = vec![0; patterns.len()];
    freqs[0] = 1;

    let mut count = vec!();

    for _ in (0..21).step_by(3) {
        let mut three = 0;
        let mut four = 0;
        let mut six = 0;

        for (freq, pattern) in freqs.iter().zip(&patterns) {
            three += freq * pattern.three;
            four += freq * pattern.four;
            six += freq * pattern.six;
        }

        count.push(three);
        count.push(four);
        count.push(six);

        let mut next_freqs = vec![0; freqs.len()];
        for(freq, pattern) in freqs.iter().zip(&patterns) {
            for i in pattern.nine {
                next_freqs[i] += *freq;
            }
        }
        freqs = next_freqs;
    }
    let p1 = count[5];
    let p2 = count[18];
    Ok((p1, p2))
}

fn orientations_two(block: &[bool]) -> [usize; 8] {
    let mut block: [bool; 4] = block.try_into().unwrap();
    let mut output = [0; 8];
    for (i, o) in output.iter_mut().enumerate() {
        *o = encode(&block);
        // rotate
        block = [block[2], block[0], block[3], block[1]];
        if i == 3 {
            // flip
            block = [block[1], block[0], block[3], block[2]];
        }
    }
    output
}

fn orientations_three(block: &[bool]) -> [usize; 8] {
    let mut block: [bool; 9] = block.try_into().unwrap();
    let mut output = [0; 8];
    for (i, o) in output.iter_mut().enumerate() {
        *o = encode(&block);

        if i == 4 {
            // flip
            block = [block[2], block[1], block[0],
                     block[5], block[4], block[3],
                     block[8], block[7], block[6]];
        }

        // rotate
        block = [block[6], block[3], block[0],
                 block[7], block[4], block[1],
                 block[8], block[5], block[2]
                ];
    }
    output
}

pub fn encode(block: &[bool]) -> usize {
    block.iter().fold(0, |acc, &b| acc * 2 + (if b {1} else {0}))
}