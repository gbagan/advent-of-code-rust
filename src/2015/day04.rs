use aoc::aoc;

fn solve_for(input: &str, pattern: &str) -> isize {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{input}{i}"));
        if format!("{digest:x}").starts_with(pattern) {
            return i;
        }
        i += 1;
    }
}

fn main() {
    let input = include_str!("../../inputs/2015/04");
    aoc(|| {
        let p1 = solve_for(input, "00000");
        let p2 = solve_for(input, "000000");
        (p1, p2)
    })
}