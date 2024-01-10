use aoc::aoc;

fn main() {
    let input = include_str!("../../inputs/2015/01");
    aoc(|| {
        let p1 = input.chars().fold(0, |acc, chr|
            match chr {
                '(' => acc + 1,
                ')' => acc - 1,
                _ => acc
            }
        );

        let p2 = input.chars().scan(0, |acc, chr| {
            match chr {
                '(' => *acc = *acc + 1,
                ')' => *acc = *acc - 1,
                _ => ()
            };
            Some(*acc)
        }).position(|r| r < 0).map(|x| x + 1);
        
        (p1, p2)
    })
}
