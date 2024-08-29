use crate::util::grid::Grid;
use crate::util::many_times_on;

// north_cubes[next_north_cubre[p]] is the position of the next cube rock at the north of the position p
pub struct Input {
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

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let parsed = Grid::parse(input);
    let mut grid = Grid::new(parsed.width+2, parsed.height+2, b'#');
    let mut rounded = vec!();
    let mut north_cubes = vec!();
    let mut west_cubes = vec!();
    let mut south_cubes = vec!();
    let mut east_cubes = vec!();
    let mut next_cube_north = Grid::new(grid.width, grid.height, 0);
    let mut next_cube_west = Grid::new(grid.width, grid.height, 0);
    let mut next_cube_south = Grid::new(grid.width, grid.height, 0);
    let mut next_cube_east = Grid::new(grid.width, grid.height, 0);

    for y in 0..parsed.height {
        for x in 0..parsed.width {
            grid[(x+1, y+1)] = parsed[(x, y)];
        }
    }
    
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
    Some((p1, p2))

}

fn tilt(rounded: &mut [i16], cubes: &[i16], next_cube: &[i16], direction: i16) -> Vec<i16> {
    let mut state = cubes.to_vec();

    for rock in rounded {
        let index = next_cube[*rock as usize] as usize;
        state[index] += direction;
        *rock = state[index];
    }

    state
}


pub fn part1(input: &Input) -> usize {
    let mut rounded = input.rounded.clone();
    tilt(&mut rounded, &input.north_cubes, &input.next_cube_north, input.width as i16);
    rounded.iter().map(|&i| input.height - 1 - (i as usize / input.width)).sum()
}

pub fn step(rounded: &[i16], input: &Input) -> (Vec<i16>, Vec<i16>) {
    let mut rounded = rounded.to_vec();
    tilt(&mut rounded, &input.north_cubes, &input.next_cube_north, input.width as i16);
    tilt(&mut rounded, &input.west_cubes, &input.next_cube_west, 1);
    tilt(&mut rounded, &input.south_cubes, &input.next_cube_south, -(input.width as i16));
    let state = tilt(&mut rounded, &input.east_cubes, &input.next_cube_east, -1);
    (rounded, state)
}

pub fn part2(input: &Input) -> usize {
    let (rounded, _) =  many_times_on(1_000_000_000, (input.rounded.clone(), vec!()), |p| p.1.clone(),
                                |p| step(&p.0, input));
    rounded.iter().map(|&i| input.height - 1 - (i as usize / input.width)).sum()
}