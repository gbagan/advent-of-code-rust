use crate::util::{coord::Coord,grid::Grid};

type Point = Coord<i32>;

pub fn solve(input: &str) -> Option<(String, u32)> {
    let grid = Grid::parse(input);
    let start = (0..grid.height).find_map(|i| {
        let c = Point::new(i as i32, 0);
        if grid[c] != b' ' { Some(c) } else { None }
    }).expect("no start found");
    let mut pos = start;
    let mut dir = Point::SOUTH;
    let mut letter_path = vec!();
    let mut len = 0;
    loop {
        if grid[pos] == b' ' {
            break;
        }
        match grid[pos] {
            b'|' | b'-' => (),
            b'+' => {
                let dir1 = dir.turn_left();
                let dir2 = dir.turn_right();
                let pos1 = pos + dir1;
                dir = if grid[pos1] != b' ' { dir1 } else { dir2 }
            }
            c => letter_path.push(c),
        }
        pos += dir;
        len += 1;
    }
    Some((String::from_utf8(letter_path).unwrap(), len))
}