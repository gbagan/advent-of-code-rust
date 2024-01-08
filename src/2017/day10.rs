use std::time::Instant;
use nom::{
    character::complete::{char,u8},
    IResult,
    multi::separated_list1,
};

fn input_parser(input: &str) -> IResult<&str,Vec<u8>> {
    separated_list1(char(','), u8)(input)
}

fn reverse(lengths: &Vec<u8>, nb_rounds: u32) -> Vec<u8> {
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

fn part2(input: &str) -> String {
    let mut lengths2: Vec<_> = input.bytes().collect();
    lengths2.extend([17, 31, 73, 47, 23]);
    let sparse_hash = reverse(&lengths2, 64);
    let dense_hash: Vec<u8> = sparse_hash.chunks(16)
                                .map(|chunk| chunk
                                        .iter()
                                        .fold(0, |x, y| x^y)
                                ).collect();
    let p2: Vec<String> = dense_hash.iter().map(|i| format!("{:02x}", i)).collect();
    p2.join("")
}

fn main() {
    let input = include_str!("../../inputs/2017/10");
    //let input = "AoC 2017";
    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, lengths)) => {
            let start = Instant::now();
            
            let knot = reverse(&lengths, 1);
            let p1 = knot[0] as usize * knot[1] as usize;
            let p2 = part2(&input);
            
            let end = start.elapsed().as_micros();

            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
            println!("Time: {} μs", end);
        }
    }
}
