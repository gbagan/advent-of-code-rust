use crate::util::grid::Grid;

const WILL_FLASH: u8 = b'9' + 1;
const FLASHED: u8 = b'9' + 2;
const BORDER: u8 = b'9' + 3;

pub fn solve(input: &str) -> (u32, usize) {
    let mut grid = Grid::parse_with_padding2::<10, 10>(input, BORDER).vec;
    let mut stack = Vec::new();

    let mut p1 = 0;

    for _ in 0..100 {
        p1 += simulate(&mut grid, &mut stack);
    }

    let mut step = 100;
    let p2 = loop {
        step += 1;
        if simulate(&mut grid, &mut stack) == 100 {
            break step;
        }
    };

    (p1, p2)
}

fn simulate(grid: &mut [u8], stack: &mut Vec<usize>) -> u32 {
    stack.clear();
    let mut flashes = 0;
    for (i, octopus) in grid.iter_mut().enumerate() {
        if *octopus == BORDER {
            continue;
        }
        *octopus += 1;
        if *octopus == WILL_FLASH {
            stack.push(i);
        }
    }

    macro_rules! propagate {
        ($i: expr, $j: expr) => {
            let k = $i.wrapping_add($j);
            if grid[k] < FLASHED {
                grid[k] += 1;
                if grid[k] == WILL_FLASH {
                    stack.push(k);
                }
            }
        }
    }

    while let Some(i) = stack.pop() {
        flashes += 1;
        grid[i] = FLASHED;

        propagate!(i, 1);
        propagate!(i, 11);
        propagate!(i, 12);
        propagate!(i, 13);
        propagate!(i, usize::MAX);
        propagate!(i, usize::MAX-10);
        propagate!(i, usize::MAX-11);
        propagate!(i, usize::MAX-12);
    }

    for octopus in grid {
        if *octopus == FLASHED {
            *octopus = b'0';
        }
    }

    flashes
}