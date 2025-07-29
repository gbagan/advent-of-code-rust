use crate::util::grid::Grid;

pub fn solve(input: &str) -> (usize, usize) {
    let grid = Grid::parse_with_padding(input, b'*');
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    (p1, p2)
}

fn step1(grid: &Grid<u8>, next: &mut Grid<u8>) {
    let w = grid.width;
    let directions = [1, w, w-1, w+1, usize::MAX, usize::MAX-w
                                , 0usize.wrapping_sub(w), 1usize.wrapping_sub(w)];
    for (i, &v) in grid.vec.iter().enumerate() {
        match v {
            b'L' =>
                if directions.iter().all(|d| grid[i+d] != b'#') {
                    next[i] = b'#';
                } else {
                    next[i] = b'L';
                }
            b'#' =>
                if directions.iter().filter(|&&d| grid[i+d] == b'#').count() >= 4 {
                    next[i] = b'L';
                } else {
                    next[i] = b'#';
                }
            _ => {},
        }
    }
}

fn part1(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut next = grid.clone();
    loop {
        step1(&grid, &mut next);
        if next.vec[grid.width..] == grid.vec[grid.width..] {
            return grid.vec.iter().filter(|&&c| c == b'#').count()
        }
        std::mem::swap(&mut grid,  &mut next);
    }
}

fn first_seat(grid: &Grid<u8>, mut position: usize, direction: usize) -> bool {
    loop {
        position += direction;
        match grid[position] {
            b'#' => return true,
            b'.' => {},
            _ => return false
        }
    }
}

fn step2(grid: &Grid<u8>, next: &mut Grid<u8>) {
    let w = grid.width;
    let directions = [1, w, w-1, w+1, usize::MAX, usize::MAX-w
                                , 0usize.wrapping_sub(w), 1usize.wrapping_sub(w)];
    for (i, &v) in grid.vec.iter().enumerate() {
        match v {
            b'L' =>
                if directions.iter().all(|&d| !first_seat(grid, i, d)) {
                    next[i] = b'#';
                } else {
                    next[i] = b'L';
                }
            b'#' =>
                if directions.iter().filter(|&&d| first_seat(grid, i, d)).count() >= 5 {
                    next[i] = b'L';
                } else {
                    next[i] = b'#';
                }
            _ => {},
        }
    }
}

fn part2(grid: &Grid<u8>) -> usize {
    let mut grid = grid.clone();
    let mut next = grid.clone();
    loop {
        step2(&grid, &mut next);
        if next.vec[grid.width..] == grid.vec[grid.width..] {
            return grid.vec.iter().filter(|&&c| c == b'#').count()
        }
        std::mem::swap(&mut grid,  &mut next);
    }
}