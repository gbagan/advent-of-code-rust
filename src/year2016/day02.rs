use crate::util::grid::Grid;

pub fn solve(input: &str) -> Option<(String, String)> {
    let lines: Vec<_> = input.lines().collect();
    let p1 = simulate(&lines, "#####\n#123#\n#456#\n#789#\n#####");
    let p2 = simulate(&lines, "#######\n###1###\n##234##\n#56789#\n##ABC##\n###D###\n#######");
    Some((p1, p2))
}

#[inline]
fn next_index(index: usize, width: usize, c: u8) -> usize {
    match c {
        b'U' => index - width,
        b'L' => index - 1,
        b'R' => index + 1,
        b'D' => index + width,
        _ => panic!("unexpected character {c}"),
    }
}

fn simulate(input: &[&str], grid: &str) -> String {
    let grid = Grid::parse(grid);
    let width = grid.width;
    let grid = grid.vec;
    let mut index = grid.iter().position(|&c| c == b'5').unwrap();
    let mut output = String::new();
    for line in input {
        for c in line.bytes() {
            let next = next_index(index, width, c);
            if grid[next] != b'#' {
                index = next;
            }
        }
        output.push(grid[index] as char);
    }
    output
}