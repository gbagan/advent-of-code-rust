use aoc::aoc;
use aoc::coord::Coord;
use aoc::grid::Grid;

fn part1(grid: &Grid<u8>) -> (String, u32) {
    let start = (0..grid.height).find_map(|i| {
        let c = Coord::new(i as i64, 0);
        if grid[c] != b' ' { Some(c) } else { None }
    }).unwrap();
    let mut pos = start;
    let mut dir = Coord::south();
    let mut letter_path = vec!();
    let mut len = 0;
    loop {
        if !grid.contains(pos) || grid[pos] == b' ' {
            return (String::from_utf8(letter_path).unwrap(), len)
        }
        match grid[pos] {
            b'|' | b'-' => (),
            b'+' => {
                let dir1 = dir.turn_left();
                let dir2 = dir.turn_right();
                let pos1 = pos + dir1;
                dir = if grid.contains(pos1) && grid[pos1] != b' ' { dir1 } else { dir2 }
            }
            c => letter_path.push(c),
        }
        pos += dir;
        len += 1;
    }
}

fn main() {
    let input = include_str!("../../inputs/2017/19");
    let grid = Grid::parse(input);
    aoc(|| {
        (part1(&grid), 0)
    })
}