use crate::util::{iter::*, parser::*};

const ROWS: [[usize; 5]; 10] = [
    [0, 1, 2, 3, 4],
    [5, 6, 7, 8, 9],
    [10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19],
    [20, 21, 22, 23, 24],
    [0, 5, 10, 15, 20],
    [1, 6, 11, 16, 21],
    [2, 7, 12, 17, 22],
    [3, 8, 13, 18, 23],
    [4, 9, 14, 19, 24]
];


pub fn solve(input: &str) -> (u32, u32) {
    let mut numbers = input.iter_unsigned::<u8>();

    let draw: Vec<_> = numbers.by_ref().take(100).collect();
    let mut inverse_draw = vec![0; 100];
    for (i, &n) in draw.iter().enumerate() {
        inverse_draw[n as usize] = i as u32;
    }

    let boards: Vec<_> = numbers
        .array_chunks::<25>()
        .map(|b| {
            let b2 = b.map(|i| inverse_draw[i as usize]);
            let round = winning_round(&b2);
            let s = score(&b2, round, &draw);
            (s, round)
        })
        .collect();

    let (&(p1, _), &(p2, _)) = boards.iter().minmax_by_key(|b| b.1).unwrap();
    
    (p1, p2)
}


fn winning_round(board: &[u32; 25]) -> u32 {
    ROWS
    .iter()
    .map(|row| row.iter().map(|&i| board[i]).max().unwrap())
    .min()
    .unwrap()
}


fn score(board: &[u32], step: u32, draw: &[u8]) -> u32 {
    board
    .iter()
    .filter(|&&i| i > step) 
    .map(|&i| draw[i as usize] as u32)
    .sum::<u32>()
    * draw[step as usize] as u32
}