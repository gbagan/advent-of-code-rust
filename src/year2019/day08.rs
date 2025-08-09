use std::simd::prelude::*;

pub fn solve(input: &str) -> (usize, String) {
    let input = input.trim().as_bytes();
    let chunks = input.as_chunks::<150>().0;

    let p1 = part1(chunks);
    let p2 = part2(input);

    (p1, p2)
}

fn part1(chunks: &[[u8; 150]]) -> usize {
    let mut min_chunk = &chunks[0];
    let mut min_zeros = usize::MAX;
    for chunk in chunks {
        let zeros = count(chunk, b'0');
        if zeros < min_zeros {
            min_chunk = chunk;
            min_zeros = zeros;
        }
    }

    count(min_chunk, b'1') * count(min_chunk, b'2')
}

fn count(chunk: &[u8; 150], pat: u8) -> usize {
    let mut ptr = chunk.as_ptr();
    let mut count = i8x32::splat(0);
    let mask = u8x32::splat(pat);
    unsafe {
        for _ in 0..4 {
            count += ptr.cast::<u8x32>().read_unaligned().simd_eq(mask).to_int();
            ptr = ptr.add(32);
        }
        ptr = ptr.sub(10);
        count += ptr.cast::<u8x32>().read_unaligned().simd_eq(mask).to_int();
    }
    let c = -count.reduce_sum() as u8 as usize;
    c - chunk[118..128].iter().filter(|&&c| c == pat).count()
}

fn part2(input: &[u8]) -> String {
    let mut drawing = String::with_capacity(26*6);
    let mut i = 0;
    for _ in 0..6 {
        drawing.push('\n');
        for _ in 0..25 {
            drawing.push(if pixel(input, i) { '#'} else { '.' });
            i += 1;
        }
    }
    drawing
}

fn pixel(input: &[u8], i: usize) -> bool {
    for &c in input[i..].iter().step_by(150) {
        match c {
            b'2' => {}, 
            b'1' => return true,
            _ => return false,
        }
    }
    unreachable!();
}