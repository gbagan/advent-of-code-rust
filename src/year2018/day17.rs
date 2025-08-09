use crate::util::parser::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile { Flowing, Empty, Set } 
use Tile::*;

pub fn solve(input: &str) -> (u32, u32) {
    let veins: Vec<_> = input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            let [a, b, c] = line.iter_unsigned::<usize>().next_chunk().unwrap();
            if line[0] == b'x' { [a, a, b, c] } else { [b, c, a, a] }
        }).collect();
    
    let mut xmin = usize::MAX;
    let mut xmax = 0;
    let mut ymin = usize::MAX;
    let mut ymax = 0;
    for &[x1, x2, y1, y2] in &veins {
        xmin = xmin.min(x1);
        xmax = xmax.max(x2);
        ymin = ymin.min(y1);
        ymax = ymax.max(y2);
    }
    xmin -= 1;
    xmax += 1;
    let width = xmax - xmin + 1;

    let mut grid = vec![Empty; width * (ymax+1)];
    for &[x1, x2, y1, y2] in &veins {
        if y1 == y2 {
            for x in x1..x2+1 {
                grid[y1 * width + x - xmin] = Set;
            }
        } else {
            for y in y1..y2+1 {
                grid[y * width + x1 - xmin] = Set;
            }
        }
    }

    let mut scan = Scan { grid, width, flowing: 0, set: 0, top: width * ymin };
    scan.flow(500 - xmin);

    let p1 = scan.flowing + scan.set;
    let p2 = scan.set;

    (p1, p2)

}

struct Scan {
    grid: Vec<Tile>,
    width: usize,
    top: usize,
    flowing: u32,
    set: u32
}

impl Scan {
    fn flow(&mut self, index: usize) -> Tile {
        if index >= self.grid.len() {
            Flowing
        } else if self.grid[index] != Empty {
            self.grid[index]
        } else if self.flow(index + self.width) == Flowing {
            self.grid[index] = Flowing;
            if index >= self.top {
                self.flowing += 1;
            }
            Flowing
        } else {
            let mut left = index;
            let mut right = index;

            while self.grid[left - 1] == Empty && self.flow(left + self.width) == Set {
                left -= 1;
            }

            while self.grid[right + 1] == Empty && self.flow(right + self.width) == Set {
                right += 1;
            }
            
            if self.grid[left - 1] == Set && self.grid[right + 1] == Set {
                for index in left..right + 1 {
                    self.grid[index] = Set;
                }
                if index >= self.top {
                    self.set += (right - left + 1) as u32;
                }
                Set
            } else {
                for index in left..right + 1 {
                    self.grid[index] = Flowing;
                }
                if index >= self.top {
                    self.flowing += (right - left + 1) as u32;
                }
                Flowing
            }
        }

    }
}