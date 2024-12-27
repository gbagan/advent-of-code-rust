use crate::util::parser::*;

type Hand<'a> = ([u8; 5], u32);

pub fn solve(input: &str) -> (u32, u32) {
    let mut scores1 = Vec::with_capacity(1000);
    let mut scores2 = Vec::with_capacity(1000);
    
    for hand in input.lines().map(parse_hand) {
        let mut card_counts = [0; 14];
        let mut score1a = 0;
        let mut score2a = 0;
        let mut distinct_cards = 0;
        let mut max_count = 0;

        for card in hand.0 {
            let mut value = card_score(card);
            let count= &mut card_counts[value as usize]; 
            *count += 1;
            if *count == 1 {
                distinct_cards += 1;
            }
            score1a = score1a << 4 | value;

            if card == b'J' {
                value = 0;
            } else if *count > max_count {
                max_count = *count;
            }

            score2a = score2a << 4 | value;
        }
        let j_count = card_counts[10];
        let score1b = 5 * max_count.max(j_count) - distinct_cards;
        let score2b = 5 * (max_count + j_count) - distinct_cards + (j_count > 0 && j_count < 5) as u32;

        scores1.push((score1b << 24 | score1a, hand.1));
        scores2.push((score2b << 24 | score2a, hand.1));
    }

    scores1.sort_unstable();
    scores2.sort_unstable();

    let p1 = scores1.iter().enumerate().map(|(i, c)| (i as u32 + 1) * c.1).sum();
    let p2 = scores2.iter().enumerate().map(|(i, c)| (i as u32 + 1) * c.1).sum();
    (p1, p2)
}

fn parse_hand(line: &str) -> Hand {
    let bid = (&line[6..]).try_unsigned().unwrap();
    let cards = line.as_bytes()[0..5].try_into().unwrap();
    (cards, bid)
}

#[inline]
fn card_score(c: u8) -> u32 {
    match c {
        b'T' => 9,
        b'J' => 10,
        b'Q' => 11,
        b'K' => 12,
        b'A' => 13,
        _ => (c - b'1') as u32,
    }
}
