use arrayvec::ArrayVec;
use crate::util::{coord::*, grid::*};

pub fn solve(input: &str) -> (usize, usize) {
    let mut grid = Grid::parse(input);
    let width = grid.width as i32;
    let height = grid.height as i32;

    let mut antennas: Vec<ArrayVec<Coord<i32>, 8>> = vec![ArrayVec::new(); 128];

    for y in 0..height {
        for x in 0..width {
            let c = grid[(x, y)];
            if c != b'.' {
                antennas[c as usize].push(Coord::new(x, y))
            }
        }
    }

    for freq in &antennas {
        for &antenna1 in freq {
            for &antenna2 in freq {
                if antenna1 != antenna2 {
                    let diff = antenna2 - antenna1;
                    let pos = antenna1 + diff;
                    if grid.contains(pos) {
                        grid[pos] = b'#';
                    }
                }
            }
        }
    }

    let p1 = grid.vec.iter().filter(|&&c| c == b'#').count();

    for freq in &antennas {        
        for &antenna1 in freq {
            grid[antenna1] = b'#';
            for &antenna2 in freq {
                if antenna1 != antenna2 {
                    let diff = antenna2 - antenna1;
                    let mut pos = antenna1 + diff + diff;
                    while grid.contains(pos) {
                        grid[pos] = b'#';
                        pos += diff;
                    }
                }
            }
        }
    }

    let p2 = grid.vec.iter().filter(|&&c| c == b'#').count();

    (p1, p2)
}