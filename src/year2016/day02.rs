pub fn solve(input: &str) -> (String, String) {
    let lines: Vec<_> = input.lines().collect();
    let p1 = simulate::<5>(&lines, b"######123##456##789######");
    let p2 = simulate::<7>(&lines, b"##########1#####234###56789###ABC#####D##########");
    (p1, p2)
}

#[inline]
fn next_index<const W: usize>(index: usize, c: u8) -> usize {
    match c {
        b'U' => index - W,
        b'L' => index - 1,
        b'R' => index + 1,
        b'D' => index + W,
        _ => panic!("unexpected character {c}"),
    }
}

fn simulate<const W: usize>(input: &[&str], grid: &[u8]) -> String {
    let mut index = grid.iter().position(|&c| c == b'5').unwrap();
    let mut output = String::new();
    for line in input {
        for c in line.bytes() {
            let next = next_index::<W>(index, c);
            if grid[next] != b'#' {
                index = next;
            }
        }
        output.push(grid[index] as char);
    }
    output
}