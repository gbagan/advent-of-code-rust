use anyhow::*;
use crate::util::{coord::*, grid::*};

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut grid = Grid::parse(input)?;
    let width = grid.width as i32;
    let height = grid.height as i32;

    let mut antennas: Vec<Vec<Coord<i32>>> = vec![Vec::new(); 128];

    for y in 0..height {
        for x in 0..width {
            let c = grid[(x, y)];
            if c != b'.' {
                antennas[c as usize].push(Coord::new(x, y))
            }
        }
    }

    for freq in &antennas {
        if freq.len() < 2 {
            continue;
        }
        for (i, &antenna1) in freq[0..freq.len()-1].iter().enumerate() {
            for &antenna2 in freq[i + 1 ..].iter() {
                let diff = antenna2 - antenna1;
                let pos1 = antenna1 - diff;
                if grid.contains(pos1) {
                    grid[pos1] = b'#';
                }
                let pos2 = antenna2 + diff;
                if grid.contains(pos2) {
                    grid[pos2] = b'#';
                }
            }
        }
    }

    let p1 = grid.vec.iter().filter(|&&c| c == b'#').count();

    for freq in &antennas {
        if freq.len() < 2 {
            continue;
        }
        for &antenna in freq {
            grid[antenna] = b'#';
        }
        
        for (i, &antenna1) in freq[0..freq.len()-1].iter().enumerate() {
            for &antenna2 in freq[i + 1 ..].iter() {
                let diff = antenna2 - antenna1;
                
                let mut pos1 = antenna1 - diff - diff;
                let mut pos2 = antenna2 + diff + diff;
                while grid.contains(pos1) {
                    grid[pos1] = b'#';
                    pos1 = pos1 - diff;
                }
                while grid.contains(pos2) {
                    grid[pos2] = b'#';
                    pos2 = pos2 + diff;
                }
            }
        }
    }

    let p2 = grid.vec.iter().filter(|&&c| c == b'#').count();

    Ok((p1, p2))

}