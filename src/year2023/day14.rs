use crate::util::grid::Grid;
use ahash::{HashMap, HashMapExt};

// north_cubes[next_north_cubre[p]] is the position of the next cube rock at the north of the position p
struct Input {
    width: usize,
    height: usize,
    rounded: Vec::<i16>,
    north_cubes: Vec::<i16>,
    south_cubes: Vec::<i16>,
    west_cubes: Vec::<i16>,
    east_cubes: Vec::<i16>,
    next_cube_north: Vec::<i16>,
    next_cube_south: Vec::<i16>,
    next_cube_west: Vec::<i16>,
    next_cube_east: Vec::<i16>,
}

pub fn solve(input: &str) -> (usize, i32) {
    let grid = Grid::parse_with_padding(input, b'#').unwrap();
    let mut rounded = vec!();
    let mut north_cubes = vec!();
    let mut west_cubes = vec!();
    let mut south_cubes = vec!();
    let mut east_cubes = vec!();
    let mut next_cube_north = Grid::new(grid.width, grid.height, 0);
    let mut next_cube_west = Grid::new(grid.width, grid.height, 0);
    let mut next_cube_south = Grid::new(grid.width, grid.height, 0);
    let mut next_cube_east = Grid::new(grid.width, grid.height, 0);
    
    for (i, &tile) in grid.vec.iter().enumerate() {
        if tile == b'O' {
            rounded.push(i as i16);
        }
    }

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid[(x, y)] == b'#' {
                north_cubes.push((grid.width * y + x) as i16);
            }
            next_cube_north[(x, y)] = (north_cubes.len() - 1) as i16;
        }

        for y in (0..grid.height).rev() {
            if grid[(x, y)] == b'#' {
                south_cubes.push((grid.width * y + x) as i16);
            }
            next_cube_south[(x, y)] = (south_cubes.len() - 1) as i16;
        }
    }

    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid[(x, y)] == b'#' {
                west_cubes.push((grid.width * y + x) as i16);
            }
            next_cube_west[(x, y)] = (west_cubes.len() - 1) as i16;
        }

        for x in (0..grid.width).rev() {
            if grid[(x, y)] == b'#' {
                east_cubes.push((grid.width * y + x) as i16);
            }
            next_cube_east[(x, y)] = (east_cubes.len() - 1) as i16;
        }
    }
    let next_cube_north = next_cube_north.vec;
    let next_cube_south = next_cube_south.vec;
    let next_cube_west = next_cube_west.vec;
    let next_cube_east = next_cube_east.vec;

    let input = Input {rounded, north_cubes, south_cubes, west_cubes, east_cubes,
        next_cube_north, next_cube_south, next_cube_west,  next_cube_east,
        width: grid.width, height: grid.height,
    };

    let p1 = part1(&input);
    let p2 = part2(&input);
    (p1, p2)

}

fn part1(input: &Input) -> usize {
    let mut rounded = input.rounded.clone();
    let mut state = Vec::with_capacity(input.north_cubes.len());
    tilt(&mut rounded, &mut state, &input.north_cubes, &input.next_cube_north, input.width as i16);
    rounded.iter().map(|&i| input.height - 1 - (i as usize / input.width)).sum()
}

fn part2(input: &Input) -> i32 {
    let width = input.width as i32;
    let height = input.height as i32;
    let mut seen = HashMap::with_capacity(100);
    let rounded = &mut input.rounded.clone();
    let (i, j) = loop {
        let state = step(rounded, &input);
        if let Some(prev) = seen.insert(state, seen.len()) {
            break (prev, seen.len());
        }
    };

    let k = i + (1_000_000_000 - 1 - j) % (j - i);
    let (state, _) = seen.iter().find(|pair| *pair.1 == k).unwrap();

    input.east_cubes.iter().zip(state.iter()).map(|(&cube, &n)| {
        let y = height - 1 - (cube as i32) / width;
        (cube - n) as i32 * y
    }).sum()
}



fn tilt(rounded: &mut [i16], state: &mut Vec<i16>, cubes: &[i16], next_cube: &[i16], direction: i16) {
    state.clear();
    state.extend_from_slice(&cubes);

    for rock in rounded {
        let index = next_cube[*rock as usize] as usize;
        state[index] += direction;
        *rock = state[index];
    }
}

#[inline]
fn step(rounded: &mut [i16], input: &Input) -> Vec<i16> {
    let mut state = Vec::with_capacity(input.north_cubes.len());
    tilt(rounded, &mut state, &input.north_cubes, &input.next_cube_north, input.width as i16);
    tilt(rounded, &mut state, &input.west_cubes, &input.next_cube_west, 1);
    tilt(rounded, &mut state, &input.south_cubes, &input.next_cube_south, -(input.width as i16));
    tilt(rounded, &mut state, &input.east_cubes, &input.next_cube_east, -1);
    state
}