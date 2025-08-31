use std::sync::Mutex;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::util::constants::THREADS;
use crate::util::{coord::Coord, grid::Grid, knothash::knothash};

pub fn solve(input: &str) -> (u32, usize) {
    let mutex = Mutex::new([[0u8; 16]; 128]);
    let counter = AtomicUsize::new(0);
    thread::scope(|scope| {
        for _ in 0..THREADS {
            scope.spawn(|| worker(input, &counter, &mutex));
        }
    });    
    let hashes = mutex.into_inner().unwrap();
    let p1 = part1(&hashes);
    let p2 = part2(&hashes);
    (p1, p2)
}

fn worker(input: &str, counter: &AtomicUsize, mutex: &Mutex<[[u8; 16]; 128]>) {
    while let i = counter.fetch_add(1, Ordering::Relaxed)  && i <  128 {
        let hash = knothash(&format!("{input}-{i}"));
        let mut hashes = mutex.lock().unwrap();
        hashes[i] = hash;
    }
}

fn part1(hashes: &[[u8; 16]]) -> u32 {
    hashes
    .iter()
    .map(|h| h.iter()
              .map(|&n| n.count_ones())
              .sum::<u32>()
        )
    .sum()
}

fn is_used(hashes: &[[u8;16]], (i, j) : (usize, usize)) -> bool {
    hashes[i][j/8] >> (7 - j%8) & 1 == 1
}

fn part2(hashes: &[[u8;16]]) -> usize {
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
                for next in current.adjacent4() {
                    if grid.contains(next) && grid[next] {
                        stack.push(next);
                    }
                }
            }
        }
    }
    nb_components
}   
