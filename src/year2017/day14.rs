use rayon::prelude::*;

use crate::util::{coord::Coord, grid::Grid, knothash::knothash};

pub fn solve(input: &str) -> Option<(u32, usize)> {
    let mut hashes: Vec<_> = vec!(); 
    (0..128)
        .into_par_iter()
        .map(|i| knothash(&format!("{}-{}", input, i)))
        .collect_into_vec(&mut hashes);
    let p1 = part1(&hashes);
    let p2 = part2(&hashes);
    Some((p1, p2))
}

fn part1(hashes: &[Vec<u8>]) -> u32 {
    hashes
    .iter()
    .map(|h| h.iter()
              .map(|&n| n.count_ones())
              .sum::<u32>()
        )
    .sum()
}

fn is_used(hashes: &[Vec<u8>], (i, j) : (usize, usize)) -> bool {
    hashes[i][j/8] >> (7 - j%8) & 1 == 1
}

fn part2(hashes: &[Vec<u8>]) -> usize {
    let grid: Grid<bool> = Grid::generate(128, 128, |i, j| is_used(hashes, (i, j)));
    let mut seen = Grid::new(128, 128, false);
    let mut nb_components = 0;
    for j in 0..128 {
        for i in 0..128 {
            let v: bool = grid[(i, j)];
            if !v || seen[(i, j)] {
                continue;
            }
            nb_components += 1;
            let mut stack = vec!(Coord::new(i,j));
            while let Some(current) = stack.pop() {
                if seen[current] {
                    continue;
                }
                seen[current] = true;
                for next in current.adjacent() {
                    if grid.contains(next) && grid[next] {
                        stack.push(next);
                    }
                }
            }
        }
    }
    nb_components
}   
