use anyhow::*;
use crate::util::parser::*;

type Hand<'a> = ([u8; 5], usize);

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let hands: Vec<_> = input.lines().map(parse_hand).collect();
    let p1 = solve_with(&hands, hand_score1);
    let p2 = solve_with(&hands, hand_score2);
    Ok((p1, p2))
}

fn parse_hand(line: &str) -> Hand {
    let bid = (&line[6..]).try_unsigned().unwrap();
    let cards = line.as_bytes()[0..5].try_into().unwrap();
    (cards, bid)
}

fn solve_with(hands: &[Hand], hand_score: fn([u8; 5], &mut Vec<u64>) -> u64) -> usize {
    let mut freqs = Vec::new();
    let mut hands: Vec<_> = hands.iter().map(|(hand, bid)| (hand_score(*hand, &mut freqs), bid)).collect();
    hands.sort_unstable();
    hands.iter().enumerate().map(|(i, c)| (i+1) * c.1).sum()
}

fn card_score(c: u8) -> u64 {
    match c {
        b'T' => 9,
        b'J' => 10,
        b'Q' => 11,
        b'K' => 12,
        b'A' => 13,
        _ => (c - b'1') as u64,
    }
}

fn card_freq(mut cards: [u8; 5], freqs: &mut Vec<u64>) {
    cards.sort_unstable();
    freqs.clear();
    let mut counter = 0;
    let mut previous = cards[0];
    for c in cards {
        if c == previous {
            counter += 1;
        } else {
            if previous != b'1' {
                freqs.push(counter);
            }
            counter = 1;
            previous = c;
        }
    }
    if previous != b'1' {
        freqs.push(counter);
    }
    freqs.sort_unstable();
    freqs.reverse()
}

fn encode_score(cards: [u8; 5], freq: &[u64]) -> u64 {
    let mut score = freq[0] << 4;
    if freq.len() >= 2 {
        score |= freq[1];
    }
    for c in cards {
        score = (score << 4) | card_score(c);
    }
    score
}

fn hand_score1(cards: [u8; 5], freqs: &mut Vec<u64>) -> u64 {
    card_freq(cards, freqs);
    encode_score(cards, freqs)
}

fn hand_score2(cards: [u8; 5], freqs: &mut Vec<u64>) -> u64 {
    let mut cards = cards;
    let nb_jokers = cards.iter().filter(|&&c| c == b'J').count();
    for card in &mut cards {
        if *card == b'J' {
            *card = b'1';
        }
    }
    card_freq(cards, freqs);
    if freqs.is_empty() {
        freqs.push(nb_jokers as u64);
    } else {
        freqs[0] += nb_jokers as u64;
    }
    encode_score(cards, &freqs)
}

/* 
#[test]
fn power_test() {
    assert_eq!(card_freq([b'K', b'A', b'A', b'K', b'A']), vec!(3, 2));
}
*/