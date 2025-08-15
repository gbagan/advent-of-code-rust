use std::array::from_fn;
use crate::util::{iter::*, parser::*};

struct Transition {
    write: bool,
    move_right: bool,
    next: usize,
}

struct Skip {
    next_state: usize,
    next_tape: usize,
    steps: u32,
    move_right: bool,
}

const N: usize = 5;
const MASK: usize = 1 << (N - 1);
const MASK_LIMIT: usize = 1 << (2 * N - 1);
const LEFT_MASK: usize = (1 << (2 * N)) - (1 << N); // 992
const RIGHT_MASK: usize = (1 << N) - 1;
const TABLE_SIZE: usize = 1 << (2 * N);

pub fn solve(input: &str) -> (u32, u32) {
    let mut lines = input.lines();
    let (_, line2) = lines.next_tuple().unwrap();
    let steps = line2.try_unsigned().unwrap();

    let transitions: Vec<_> = lines.tuples().map(|(_, _, _, write0, move0, next0, _, write1, move1, next1)| {
        let write0 = write0.ends_with("1.");
        let move0 = move0.ends_with("right.");
        let next0 = next0.as_bytes();
        let next0 = (next0[next0.len()-2] - b'A') as usize;

        let write1 = write1.ends_with("1.");
        let move1 = move1.ends_with("right.");
        let next1 = next1.as_bytes();
        let next1 = (next1[next1.len()-2] - b'A') as usize;
        ( Transition {write: write0, move_right: move0, next: next0},
          Transition {write: write1, move_right: move1, next: next1})
    }).collect();

    let skip_table: Vec<[Skip; TABLE_SIZE]> = (0..transitions.len())
        .map(|state| from_fn(|tape| skip(&transitions, tape, state, u32::MAX)))
        .collect();

    let mut left = Vec::with_capacity(4000);
    let mut center = 0;
    let mut right =  Vec::with_capacity(4000);
    let mut state = 0;
    let mut remaining = steps;

    loop {
        let Skip { next_state, next_tape, steps, move_right } = skip_table[state][center];        
        if steps > remaining {
            let Skip { next_tape, .. } = skip(&transitions, center, state, remaining);
            center = next_tape;
            break;
        }

        remaining -= steps;
        center = next_tape;
        if move_right {
            left.push(center & LEFT_MASK);
            center = ((center & RIGHT_MASK) << N) | right.pop().unwrap_or(0);
        } else {
            right.push(center & RIGHT_MASK);
            center = (center >> N) | left.pop().unwrap_or(0);
        }
        state = next_state;
    }
    
    let p1 = left.iter().map(|x| x.count_ones()).sum::<u32>() 
            + right.iter().map(|x| x.count_ones()).sum::<u32>()
            + center.count_ones();
    (p1, 0)
}

fn skip(trs: &[(Transition, Transition)], mut tape: usize, mut state: usize, limit: u32) -> Skip {
    let mut mask = MASK;
    let mut steps = 0;

    while 0 < mask && mask < MASK_LIMIT && steps < limit {
        let tr = if (tape & mask) != 0 {&trs[state].1} else {&trs[state].0};
        if tr.write {
            tape |= mask;
        } else {
            tape &= !mask;
        }
        if tr.move_right {
            mask >>= 1;
        } else {
            mask <<= 1;
        }
        state = tr.next;
        steps += 1;
    }

    Skip { next_state: state, next_tape: tape, move_right: mask == 0, steps }
} 
