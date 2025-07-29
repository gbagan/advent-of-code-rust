use ahash::HashMap;
use arrayvec::ArrayVec;

fn parse_line(line: &str) -> (&str, (bool, ArrayVec<&str, 6>)) {
    let (key, values) = line.split_once(" -> ").unwrap();
    let first_char = key.chars().next().unwrap();
    let key = key.trim_start_matches(['%', '&']);
    let values = values.split(", ").collect();
    (key, (first_char == '&', values))
}

pub fn solve(input: &str) -> (u64, u64) {
    let network: HashMap<_, _> = input.lines().map(parse_line).collect();

    let numbers: Vec<u32> = network["broadcaster"].1.iter().map(|&node| {
        let mut number = 0;
        let mut offset = 1;
        let mut current = node;
        let conj = network[current].1.iter().find(|&&node| network[node].0).copied().unwrap(); 
        loop {
            if network[current].1.iter().any(|&node| node == conj) {
                number += offset;
            }
            offset *= 2;
            if let Some(next) = network[current].1.iter().find(|&&node| node != conj) {
                current = next;
            } else {
                return number;
            }
        }
    }).collect();
    let p1 = part1(&numbers);
    let p2 = part2(&numbers);
    (p1, p2)
}

fn part1(numbers: &[u32]) -> u64 {
    let n = numbers.len() as u32;
    let mut nb_low = 1000 * (1 + n); // received and sent by the broadcaster
    let mut nb_high = 1000 * n; // (unique) bit changed to one in each component

    for i in 0..1000u32 {
        let changed_to_one = !i & (i+1);
        let changed_to_zero = !(i+1) & i;
        nb_low += n * changed_to_zero.count_ones(); // bits changed to zero

        for &number in numbers {
            let nb_conj_succs = 13 - number.count_ones();
            
            let high_sent_to_conj = (changed_to_one & number).count_ones();
            nb_high += high_sent_to_conj * (nb_conj_succs + 3);
            nb_low += high_sent_to_conj;
            
            let low_sent_to_conj = (changed_to_zero & number).count_ones();
            nb_high += low_sent_to_conj * (nb_conj_succs + 2);
            nb_low += 2 * low_sent_to_conj;
        }
    }
    nb_low as u64 * nb_high as u64
}

fn part2(numbers: &[u32]) -> u64 {
    numbers.iter().map(|&v| v as u64).product()
}