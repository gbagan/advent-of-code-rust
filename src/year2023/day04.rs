use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut table = [0; 100];

    let scores: Vec<_> =
        input
        .lines()
        .enumerate()
        .map(|(i, line)| card_score(&mut table, (i+1) as u16, line))
        .collect();
    
    let p1 = part1(&scores);
    let p2 = part2(&scores);
    (p1, p2)
}

fn card_score(table: &mut [u16; 100], i: u16, line: &str) -> u32 {
    let mut score = 0;
    for v in line.iter_unsigned::<usize>().skip(1) {
        if table[v] == i {
            score += 1;
        } else {
            table[v] = i;
        }
    }
    score
}

pub fn part1(scores: &[u32]) -> u32 {
    scores.iter()
        .map(|&s| if s == 0 {0} else {2_u32.pow(s-1)} )
        .sum()
}

pub fn part2(scores: &[u32]) -> u32 {
    let mut vec: Vec<_> = scores.iter().map(|&s| (s, 1)).collect();
    let mut total = 0;
    for i in 0..vec.len() {
        let (score, freq) = vec[i];
        total += freq;
        for pair in vec.iter_mut().skip(i+1).take(score as usize) {
            pair.1 += freq;
        }
    }
    total
}