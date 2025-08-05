use std::collections::VecDeque;
use crate::util::parser::*;

pub fn solve(input: &str) -> (u64, u64) {
    let [nb_players, last_marble] = input.iter_unsigned::<usize>().next_chunk().unwrap();

    let p1 = simulate(nb_players, last_marble);
    let p2 = simulate(nb_players, 100*last_marble);

    (p1, p2)
}

fn simulate(nb_players: usize, last_marble: usize) -> u64 {
    let mut scores = vec![0u64; nb_players];
    let mut circle = VecDeque::with_capacity(last_marble+1);
    circle.push_back(0);

    for marble in 1..last_marble+1 {
        if marble % 23 == 0 {
            circle.rotate_right(7);
            scores[marble % nb_players] += marble as u64 + circle.pop_back().unwrap() as u64;
            circle.rotate_left(1);
        }
        else {
            circle.rotate_left(1);
            circle.push_back(marble as u32);
        }
    }

    *scores.iter().max().unwrap()
}