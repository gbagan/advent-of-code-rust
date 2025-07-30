use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u64) {
    let [mut position1, _, mut position2] = (&input[28..]).iter_unsigned::<u32>().next_chunk().unwrap();
    position1 -= 1;
    position2 -= 1;
    let p1 = part1(position1, position2);
    let p2 = part2(position1 as usize, position2 as usize);

    (p1, p2)
}


fn part1(mut position1: u32, mut position2: u32) -> u32 {
    let mut dice_sum = 6;
    let mut score1 = 0;
    let mut score2 = 0;
    let mut rolled = 3;

    loop {
        position1 = (position1 + dice_sum) % 10;
        score1 += position1 + 1;
        if score1 >= 1000 {
            return score2 * rolled;
        }
        dice_sum += 9;
        rolled += 3;
        position2 = (position2 + dice_sum) % 10;
        score2 += position2 + 1;
        if score2 >= 1000 {
            return score1 * rolled;
        }
        dice_sum += 9;
        rolled += 3;
    }
}

const DICE_FREQ: [u64; 10] = {
    let mut table = [0; 10];
    let mut i = 1;
    while i <= 3 {
        let mut j = 1;
        while j <= 3 {
            let mut k = 1;
            while k <= 3 {
                table[i+j+k] += 1;
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
    table
};

fn part2(position1: usize, position2: usize) -> u64 { 
    let mut table = [(0u64, 0u64); 21 * 31 * 10 * 10];
    
    for s1 in 0..21 {
        for s2 in 21..31 {
            let start = to_index(s1, s2, 0, 0);
            for v in &mut table[start..start+100] {
                *v = (0, 1);
            }
        }
    }

    for total in (0usize..41).rev() {
        for score1 in total.saturating_sub(20usize)..21.min(total+1) {
            let score2 = total - score1;
            let mut idx = to_index(score1, score2, 0, 0);
            for position1 in 0..10 {
                for position2 in 0..10 {
                    let mut win = 0;
                    let mut lose = 0;
                    for dice in 3..10 {
                        let freq = DICE_FREQ[dice];
                        let next_position = (position1 + dice) % 10;
                        let next_score = score1 + next_position + 1;
                        let next_index = to_index(score2, next_score, position2, next_position);
                        let (next_lose, next_win) = table[next_index];
                        win += freq * next_win;
                        lose += freq * next_lose;
                    }
                    table[idx] = (win, lose);
                    idx += 1;
                }
            }
        }
    }

    let (win, lose) = table[position1 * 10 + position2];
    win.max(lose)
}

#[inline]
fn to_index(score1: usize, score2: usize, position1: usize, position2: usize) -> usize {
    3100 * score1 + 100 * score2 + 10 * position1 + position2
}