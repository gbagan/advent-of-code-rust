use arrayvec::ArrayVec;
use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let line1 = &input[..width];
    let line2 = &input[width+1..2*width+1];
    let line3 = &input[2*width+2..3*width+2];
    let line4 = &input[3*width+3..4*width+3];
    let op_line = &input[4*width+4..5*width+4];
    
    // part 1

    let mut numbers1 = line1.iter_unsigned::<u64>();
    let mut numbers2 = line2.iter_unsigned::<u64>();
    let mut numbers3 = line3.iter_unsigned::<u64>();
    let mut numbers4 = line4.iter_unsigned::<u64>();

    let p1 = op_line
        .iter().filter(|&&c| c != b' ')
        .map(|&op| {
            if op == b'+' {
                numbers1.next().unwrap()
                + numbers2.next().unwrap()
                + numbers3.next().unwrap()
                + numbers4.next().unwrap()
            } else  {
                numbers1.next().unwrap()
                * numbers2.next().unwrap()
                * numbers3.next().unwrap()
                * numbers4.next().unwrap()
            }
        }).sum::<u64>();
    
    // part 2

    let mut p2 = 0;
    let mut todo2: ArrayVec<u64, 10> = ArrayVec::new();
    for i in (0..width).rev() {
        let mut n = 0;
        n = reduce(n, line1[i]);
        n = reduce(n, line2[i]);
        n = reduce(n, line3[i]);
        n = reduce(n, line4[i]);
        if n != 0 {
            todo2.push(n);
        }

        match op_line[i] {
            b'+' => {
                p2 += todo2.iter().sum::<u64>();
                todo2.clear();
            }
            b'*' => {
                p2 += todo2.iter().product::<u64>();
                todo2.clear();
            }
            _ => {},
        } 
    }

    (p1, p2)
}

fn reduce(acc: u64, digit: u8) -> u64 {
    if digit == b' ' { acc } else { 10 * acc + (digit - b'0') as u64 }
}