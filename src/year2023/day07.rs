use anyhow::*;
use crate::util::{iter::*, parser::*};

type Hand<'a> = (&'a [u8], usize);

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let hands: Vec<_> = input.try_parse_lines_and_collect(parse_hand)?;
    let p1 = solve_with(&hands, hand_score1);
    let p2 = solve_with(&hands, hand_score2);
    Ok((p1, p2))
}

fn parse_hand(line: &str) -> Result<Hand> {
    let (cards, bid) = line.split_once(' ')
                    .with_context(|| format!("Parse error on line: {line}"))?;
    let bid = bid.try_unsigned()?;
    let cards = cards.as_bytes();
    Ok((cards, bid))
}

fn solve_with(hands: &[Hand], hand_score: fn(&[u8]) -> u64) -> usize {
    let mut hands: Vec<_> = hands.iter().map(|(hand, bid)| (hand_score(hand), bid)).collect();
    hands.sort_unstable();
    hands.iter().enumerate().map(|(i, c)| (i+1) * c.1).sum()
}

fn card_score(c: u8) -> u64 {
    match c {
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => (c - b'0') as u64,
    }
}

fn card_freq(cards: &[u8]) -> Vec<u64> {
    let mut cards = cards.to_vec();
    cards.sort_unstable();
    let mut freqs = vec!();
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
    freqs.sort_unstable_by(|a, b| b.cmp(a));
    freqs
}

fn encode_score(cards: &[u8], freq: &[u64]) -> u64 {
    let mut score = freq[0] << 4;
    if freq.len() >= 2 {
        score += freq[1];
    }
    for &c in cards {
        score <<= 4;
        score += card_score(c);
    }
    score
}

fn hand_score1(cards: &[u8]) -> u64 {
    let freq = card_freq(cards);
    encode_score(cards, &freq)
}

fn hand_score2(cards: &[u8]) -> u64 {
    let mut cards = cards.to_vec();
    let nb_jokers = cards.iter().count_if(|&c| c == b'J');
    for card in cards.iter_mut() {
        if *card == b'J' {
            *card = b'1';
        }
    }
    let mut freq = card_freq(&cards);
    if freq.is_empty() {
        freq.push(nb_jokers as u64);
    } else {
        freq[0] += nb_jokers as u64;
    }
    encode_score(&cards, &freq)
}

#[test]
fn power_test() {
    assert_eq!(card_freq(b"KAAKA"), vec!(3, 2));
}