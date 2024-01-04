use std::time::Instant;

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
            
    let start = Instant::now();
    let p1 = solve_for(input, "00000");
    let p2 = solve_for(input, "000000");
    let end = start.elapsed().as_micros();
        
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Time: {} μs", end);
}