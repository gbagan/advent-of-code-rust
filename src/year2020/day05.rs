pub fn solve(input: &str) -> (u32, u32) {
    let mut ids: Vec<_> = input.lines().map(parse_id).collect();
    ids.sort_unstable();
    let p1 = *ids.last().unwrap();
    let p2 = ids
            .array_windows()
            .find_map(|&[x, y]| (y-x == 2).then_some(x+1))
            .unwrap();
    (p1, p2)
}

fn parse_id(line: &str) -> u32 {
    line.bytes().fold(0, |acc, c|
        match c {
            b'F' | b'L' => 2*acc,
            _ => 2*acc+1,
        }
    )
}