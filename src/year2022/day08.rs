use crate::util::grid::Grid;

pub fn solve(input: &str) -> (usize, u32) {
    let grid = Grid::parse(input).unwrap();
    let grid = grid.map(|v| (8 * (v - b'0')) as i8);
    let p1 = part1(&grid);
    let p2 = part2(&grid);
    (p1, p2)

}

fn part1(grid: &Grid<i8>) -> usize {
    let mut visible = Grid::new(grid.width, grid.height, false);
    for i in 0..grid.width {
        let mut last = -1;
        for j in 0..grid.height {
            let h = grid[(i, j)];
            if h > last {
                last = h;
                visible[(i, j)] = true;
            }
        }

        last = -1;
        for j in (0..grid.height).rev() {
            let h = grid[(i, j)];
            if h > last {
                last = h;
                visible[(i, j)] = true;
            }
        }
    }

    for j in 0..grid.height {
        let mut last = -1;
        for i in 0..grid.width {
            let h = grid[(i, j)];
            if h > last {
                last = h;
                visible[(i, j)] = true;
            }
        }

        last = -1;
        for i in (0..grid.width).rev() {
            let h = grid[(i, j)];
            if h > last {
            last = h;
                visible[(i, j)] = true;
            }
        }
    }
    visible.vec.iter().filter(|&&v| v).count()
}

const ONES: u128 = 0x0101_0101_0101_0101_0101;
const MASK: u128 = 0xffff_ffff_ffff_ffff_ff00;

fn part2(grid: &Grid<i8>) -> u32 {
    let mut score = Grid::new(grid.width, grid.height, 1u32);

    for i in 0..grid.width {
        let mut visible = 0;
        for j in 0..grid.height {
            let v = grid[(i, j)];
            score[(i, j)] *= ((visible >> v) & 0xff) as u32;
            visible = (visible & (MASK << v)) + ONES;
        }

        visible = 0;
        for j in (0..grid.height).rev() {
            let v = grid[(i, j)];
            score[(i, j)] *= ((visible >> v) & 0xff) as u32;
            visible = (visible & (MASK << v)) + ONES;
        }
    }
    for j in 0..grid.height {
        let mut visible = 0;
        for i in 0..grid.width {
            let v = grid[(i, j)];
            score[(i, j)] *= ((visible >> v) & 0xff) as u32;
            visible = (visible & (MASK << v)) + ONES;
        }

        visible = 0;
        for i in (0..grid.width).rev() {
            let v = grid[(i, j)];
            score[(i, j)] *= ((visible >> v) & 0xff) as u32;
            visible = (visible & (MASK << v)) + ONES;
        }
    }
    *score.vec.iter().max().unwrap()
}