use std::time::Instant;

fn main() {
    let input = include_str!("../../inputs/2015/01");
    let start = Instant::now();

    let count_parens = input.chars().fold(0, |acc, chr|
        match chr {
            '(' => acc + 1,
            ')' => acc - 1,
            _ => acc
        }
    );

    let first_time_in_basement = input.chars().scan(0, |acc, chr| {
        match chr {
            '(' => *acc = *acc + 1,
            ')' => *acc = *acc - 1,
            _ => ()
        };
        Some(*acc)
    }).position(|r| r < 0).map(|x| x + 1);

    let end = start.elapsed().as_micros();

    println!("Part 1: {}", count_parens);
    println!("Part 2: {:?}", first_time_in_basement);
    println!("Time: {} μs", end);
}
