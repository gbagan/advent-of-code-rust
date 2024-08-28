    use crate::util::iter::AOCIter;

type Hand = (Vec<char>, usize);

fn card_score(c: char) -> u64 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap_or(0) as u64,
    }
}

fn parse_hand(line: &str) -> Option<Hand> {
    let (cards, bid) = line.split_once(' ')?;
    let bid = bid.parse().ok()?;
    let cards = cards.chars().collect();
    Some ((cards, bid))
}


pub fn parse(input: &str) -> Option<Vec<Hand>> {
    Some(input.lines().filter_map(parse_hand).collect())
}

fn card_freq(cards: &[char]) -> Vec<u64> {
    let mut cards = cards.to_vec();
    cards.sort_unstable();
    let mut freqs = vec!();
    let mut counter = 0;
    let mut previous = cards[0];
    for c in cards {
        if c == previous {
            counter += 1;
        } else {
            if previous != '1' {
                freqs.push(counter);
            }
            counter = 1;
            previous = c;
        }
    }
    if previous != '1' {
        freqs.push(counter);
    }
    freqs.sort_unstable_by(|a, b| b.cmp(a));
    freqs
}

fn encode_score(cards: &[char], freq: &[u64]) -> u64 {
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

fn hand_score(cards: &[char]) -> u64 {
    let freq = card_freq(cards);
    encode_score(cards, &freq)
}

fn hand_score2(cards: &[char]) -> u64 {
    let mut cards = cards.to_vec();
    let nb_jokers = cards.iter().count_by(|&c| c == 'J');
    for card in cards.iter_mut() {
        if *card == 'J' {
            *card = '1';
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



pub fn part1(hands: &[Hand]) -> Option<usize> {
    let mut hands: Vec<_> = hands.iter().map(|(hand, bid)| (hand_score(hand), bid)).collect();
    hands.sort_unstable();
    Some(hands.iter().enumerate().map(|(i, c)| (i+1) * c.1).sum())
}

pub fn part2(hands: &[Hand]) -> Option<usize> {
    let mut hands: Vec<_> = hands.iter().map(|(hand, bid)| (hand_score2(hand), bid)).collect();
    hands.sort_unstable();
    Some(hands.iter().enumerate().map(|(i, c)| (i+1) * c.1).sum())
}


#[test]
fn power_test() {
    assert_eq!(card_freq(&vec!('K', 'A', 'A', 'K', 'A')), vec!(3, 2));
}