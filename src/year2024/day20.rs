use anyhow::*;
use crate::util::{grid::*, parallel::*};

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let grid = Grid::parse_with_padding(input, b'#')?;
    let width = grid.width;
    let grid = grid.vec;
    let start = grid.iter().position(|&c| c == b'S').context("No start symbol found")?;
    let mut distances = vec![i16::MAX; grid.len()];
    
    let mut path = Vec::with_capacity(10000);
    
    let bottom = grid.len() - 2 * width;

    let mut current = start;
    let mut previous = start;
    for dist in 0.. {
        distances[current] = dist;
        path.push(current);
        let nexts = [current+1, current-1, current+width, current-width];
        let next = nexts
            .iter()
            .find(|&&p| p != previous && grid[p] != b'#');
        match next {
            None => break,
            Some(&next) => {
                previous = current;
                current = next;
            }
        }
    }

    let res = path
        .into_par_iter()
        .chunks_with_index(128)
        .map(|(index, chunk)| {
            let mut acc1 = 0u32;
            let mut acc2 = 0u32;
            for (index2, &pos) in chunk.iter().enumerate() {
                let dist = (index + index2) as i16;
        
                for j in [pos - width - 1, pos - width + 1, pos + width - 1, pos + width + 1, pos - 2, pos + 2, pos - 2 * width, pos + 2 * width] {
                    if distances[j] < dist - 100 {
                        acc1 += 1;
                    }
                }
                
                let px = pos % width;
                let mut nexty = pos;
                for i in 0..21 {
                    let min = nexty - (20-i).min(px - 2);
                    let max = nexty + (20-i).min(width - px - 2);
                    for next in min..max+1 {
                        let j = next.abs_diff(nexty) as i16;
                        if distances[next] <= dist - 100 - i as i16 - j {
                            acc2 += 1;
                        }
                    }
                    nexty += width;
                    if nexty >= bottom {
                        break
                    }
                }
                nexty = pos-width;
                for i in 1..21 {
                    let min = nexty - (20-i).min(px - 2);
                    let max = nexty + (20-i).min(width - px - 2);
                    for next in min..max+1 {
                        let j = next.abs_diff(nexty) as i16;
                        if distances[next] <= dist - 100 - i as i16 - j {
                            acc2 += 1;
                        }
                    }
                    nexty -= width;
                    if nexty < 2 * width {
                        break;
                    }
                }
            }
            acc2 << 11 | acc1
        }).sum();

    Ok((res & 2047, res >> 11))
}