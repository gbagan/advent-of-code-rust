use crate::util::{parallel::*, grid::*};
use ahash::{HashSet, HashSetExt};

#[derive(Clone, Copy)]
enum Dir { North, South, West, East }

pub fn solve(input: &str) -> (usize, usize) {
    let mut grid = Grid::parse(input);
    
    let mut start = (0, 0);
    'outer: for i in 0..grid.height {
        for j in 0..grid.width {
            if grid[(j, i)] == b'^' {
                start = (j, i);
                break 'outer;
            }
        }
    }

    let (mut currentx, mut currenty) = start;

    grid[(currentx, currenty)] = b'$';

    let mut seen = Vec::new();

    'outer: loop {
        loop {
            let nexty = currenty.wrapping_sub(1);
            if nexty >= grid.height {
                break 'outer;
            }
            match grid[(currentx, nexty)] {
                b'#' => break,
                b'$' => currenty = nexty,
                _ => {
                    currenty = nexty;
                    grid[(currentx, currenty)] = b'$';
                    seen.push((currentx, currenty, Dir::North))
                }
            }
        }

        loop {
            let nextx = currentx + 1;
            if nextx >= grid.width {
                break 'outer;
            }
            match grid[(nextx, currenty)] {
                b'#' => break,
                b'$' => currentx = nextx,
                _ => {
                    currentx = nextx;
                    grid[(currentx, currenty)] = b'$';
                    seen.push((currentx, currenty, Dir::East))
                }
            }
        }
        loop {
            let nexty = currenty + 1;
            if nexty >= grid.height {
                break 'outer;
            }
            match grid[(currentx, nexty)] {
                b'#' => break,
                b'$' => currenty = nexty,
                _ => {
                    currenty = nexty;
                    grid[(currentx, currenty)] = b'$';
                    seen.push((currentx, currenty, Dir::South))
                }
            }
        }

        loop {
            let nextx = currentx.wrapping_sub(1);
            if nextx >= grid.width {
                break 'outer;
            }
            match grid[(nextx, currenty)] {
                b'#' => break,
                b'$' => currentx = nextx,
                _ => {
                    currentx = nextx;
                    grid[(currentx, currenty)] = b'$';
                    seen.push((currentx, currenty, Dir::West))
                }
            }
        }
    }
    
    let p1 = seen.len() + 1;

    let slide = Slide::new(&grid);

    let p2 = seen
        .into_par_iter()
        .chunks(100)
        .map(|obstacles| {
            let mut seen: HashSet<(i32, i32)> = HashSet::new();
            obstacles
                .iter()
                .filter(|&&(obsx, obsy, dir)|
                    has_cycle(&grid, &mut seen, &slide,obsx as i32, obsy as i32, dir)
                ).count()
        })
        .sum();

    (p1, p2)
}

fn has_cycle(grid: &Grid<u8>, seen: &mut HashSet<(i32, i32)>, slide: &Slide, obsx: i32, obsy: i32, mut dir: Dir) -> bool {
    let width = grid.width as i32;
    let height = grid.height as i32;
    let (mut currentx, mut currenty) =
        match dir {
            Dir::North => (obsx, obsy+1),
            Dir::East => (obsx-1, obsy),
            Dir::South => (obsx, obsy-1),
            Dir::West => (obsx+1, obsy),
        };
    seen.clear();
    
    loop {
        match dir {
            Dir::North => {
                let nexty = slide.up[(currentx, currenty)];
                currenty =
                    if currentx == obsx && currenty > obsy && obsy >= nexty {
                        obsy + 1
                    } else {
                        nexty
                    };
                if currenty < 0 {
                    return false;
                }
                dir = Dir::East
            },
            Dir::East => {
                let nextx = slide.right[(currentx, currenty)];
                currentx =
                    if currenty == obsy && currentx < obsx && obsx <= nextx {
                        obsx - 1
                    } else {
                        nextx
                    };
                if currentx >= width {
                    return false;
                }
                dir = Dir::South
            },
            Dir::South => {
                let nexty = slide.down[(currentx, currenty)];
                currenty =
                    if currentx == obsx && currenty < obsy && obsy <= nexty {
                        obsy - 1
                    } else {
                        nexty
                    };
                if currenty >= height {
                    return false;
                }
                dir = Dir::West
            }
            Dir::West => {
                let nextx = slide.left[(currentx, currenty)];
                currentx =
                    if currenty == obsy && nextx <= obsx && obsx < currentx {
                        obsx + 1
                    } else {
                        nextx
                    };
                if currentx < 0 {
                    return false;
                }
                if !seen.insert((currentx, currenty)) {
                    return true;
                }
                dir = Dir::North
            }
        }
    }
}

struct Slide {
    up: Grid<i32>,
    down: Grid<i32>,
    left: Grid<i32>,
    right: Grid<i32>,
}

impl Slide {
    fn new(grid: &Grid<u8>) -> Self {
        let width = grid.width as i32;
        let height = grid.height as i32;
        let mut up = Grid::new(grid.width, grid.height, 0i32);
        let mut down = up.clone();
        let mut left = up.clone();
        let mut right = up.clone();

        for x in 0..width {
            let mut last = -1;

            for y in 0..height {
                if grid[(x, y)] == b'#' {
                    last = y + 1;
                }
                up[(x, y)] = last;
            }

            last = i32::MAX;

            for y in (0..height).rev() {
                if grid[(x, y)] == b'#' {
                    last = y - 1;
                }
                down[(x, y)] = last;
            }
        }

        for y in 0..height {
            let mut last = -1;

            for x in 0..width {
                if grid[(x, y)] == b'#' {
                    last = x + 1;
                }
                left[(x, y)] = last;
            }

            last = i32::MAX;

            for x in (0..width).rev() {
                if grid[(x, y)] == b'#' {
                    last = x - 1;
                }
                right[(x, y)] = last;
            }
        }

        Self { up, down, left, right}
    }
}