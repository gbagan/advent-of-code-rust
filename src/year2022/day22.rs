const EAST: usize = 0;
const SOUTH: usize = 1;
const WEST: usize = 2;
const NORTH: usize = 3;

#[derive(Clone, Copy)]
enum Instruction {
    Forward(u32),
    Left,
    Right,
}
use Instruction::*;

struct Input {
    board: Vec<u8>,
    width: usize,
    square_size: usize,
    grid_offsets: [usize; 6],
    supergrid: [[usize; 4]; 4],
    start: usize,
    path: Vec<Instruction>,
}

type Borders = [[(usize, usize); 4]; 6];

pub fn solve(input: &str) -> (usize, usize) {
    let mut lines = input.lines().map(str::as_bytes);
    let board: Vec<_> = lines.by_ref().take_while(|line| !line.is_empty()).collect();
    
    let line = lines.next().unwrap();
    let mut path = Vec::new();
    let mut number = 0;
    for &c in line {
        match c {
            b'L' => {
                if number != 0 {
                    path.push(Forward(number));
                    number = 0;
                }
                path.push(Left);
                
            },
            b'R' => {
                if number != 0 {
                    path.push(Forward(number));
                    number = 0;
                }
                path.push(Right);
            },
            _ => {
                number = number * 10 + (c - b'0') as u32;
            }
        }
    }
    if number != 0 {
        path.push(Forward(number))
    }

    let input = prepare(&board, path);

    let p1 = get_password(&input, &part1_borders(&input.supergrid));
    let p2 = get_password(&input, &part2_borders(&input.supergrid));

    (p1, p2)
}

fn get_password(input: &Input, borders: &Borders) -> usize {
    let board = &input.board;
    let square_size = input.square_size;
    let supergrid = input.supergrid;
    let grid_offsets = input.grid_offsets;
    let width = input.width;

    let get_dir = |dir: usize| {
        match dir {
            NORTH => 0usize.wrapping_sub(width),
            SOUTH => width,
            WEST => usize::MAX,
            _ => 1
        }
    };


    let mut pos = input.start;
    let mut dir = EAST;
    let mut vdir = 1;

    let change_square = |pos: usize, dir: usize| {
        let mut xpos = (pos % width) - 1;
        let mut ypos = (pos / width) - 1;
        let size = square_size - 1;
        let grid_id = supergrid[ypos / square_size][xpos / square_size];
        xpos %= square_size;
        ypos %= square_size;
        let (new_id, rot) = borders[grid_id][dir as usize];
        let offset = grid_offsets[new_id];

        let (mut xnext, mut ynext) = match dir {
            NORTH => (xpos, size),
            SOUTH => (xpos, 0),
            WEST => (size, ypos),
            _ => (0, ypos)
        };

        let next_dir = (4 + dir - rot) & 3;
        
        (xnext, ynext) = match rot {
            0 => (xnext, ynext),
            3 => (size - ynext, xnext),
            2 => (size - xnext, size - ynext),
            _ => (ynext, size - xnext),
        };
        (offset + ynext * width + xnext, next_dir)
    };

    for &instr in &input.path {
        match instr {
            Left => {
                dir = (dir + 3) & 3;
                vdir = get_dir(dir);
            },
            Right => {
                dir = (dir + 1) & 3;
                vdir = get_dir(dir);
            }
            Forward(mut n) => {
                loop {
                    while n > 0 && board[pos + vdir] == b'.' {
                        pos += vdir;
                        n -= 1;
                    }
                    if n == 0 {
                        break;
                    }
                    match board[pos + vdir] {
                        b'#' => break,
                        _ => {
                            let (next, next_dir) = change_square(pos, dir);
                            if board[next] == b'#' {
                                break;
                            }
                            n -= 1;
                            pos = next;
                            dir = next_dir;
                            vdir = get_dir(dir);
                        }
                    }
                }
            }
        }
    }

    let xpos = pos % width;
    let ypos = pos / width;

    1000 * ypos + 4 * xpos + dir

}


fn prepare(board: &[&[u8]], path: Vec<Instruction>) -> Input {
    let height = board.len();
    let width = board.iter().map(|row| row.len()).max().unwrap();
    let square_size = if height > width { height / 4 } else { width / 4 };
    let mut padded_board = vec![b' '; (height+2) * (width+2)];
    for i in 0..height {
        let index = (width+2)*(i+1)+1;
        let size = board[i].len();
        padded_board[index..index+size].copy_from_slice(board[i]);
    }
    let board = padded_board;
    let width = width + 2;

    let mut supergrid = [[usize::MAX; 4]; 4];
    let mut grid_offsets = [0; 6];
    let mut i = 0;
    for y in 0..4 {
        for x in 0..4 {
            let offset = (y * square_size + 1) * width + x * square_size + 1;
            if board[offset] != b' ' {
                grid_offsets[i] = offset;
                supergrid[y][x] = i;
                i += 1;
            }
        }
    }
    let start = board.iter().position(|&c| c == b'.').unwrap();

    Input { board, square_size, width, grid_offsets, supergrid, start, path }
}

fn part1_borders(supergrid: &[[usize; 4]; 4]) -> Borders {
    let mut borders = [[(0, 0); 4]; 6];
    for y in 0..4 {
        for x in 0..4 {
            let i = supergrid[y][x];
            if i != usize::MAX {
                // north
                let j = (0..4).rev().map(|y| supergrid[y][x]).find(|&j| j != usize::MAX).unwrap();
                borders[i][NORTH as usize] = (j, 0);
                // south
                let j = (0..4).map(|y| supergrid[y][x]).find(|&j| j != usize::MAX).unwrap();
                borders[i][SOUTH as usize] = (j, 0);
                // west
                let j = (0..4).rev().map(|x| supergrid[y][x]).find(|&j| j != usize::MAX).unwrap();
                borders[i][WEST as usize] = (j, 0);
                // east
                let j = (0..4).map(|x| supergrid[y][x]).find(|&j| j != usize::MAX).unwrap();
                borders[i][EAST as usize] = (j, 0);
            }
        }
    }

    borders
}

const CUBE_EDGES: [[(usize, usize); 4]; 6] = [
    [(1, 3), (3, 2), (4, 3), (2, 0)], // face 0
    [(2, 3), (5, 2), (3, 3), (0, 0)], // face 1
    [(0, 3), (4, 2), (5, 3), (1, 0)], // face 2
    [(5, 1), (4, 0), (0, 1), (1, 2)], // face 3
    [(3, 1), (5, 0), (2, 1), (0, 2)], // face 4
    [(4, 1), (3, 0), (1, 1), (2, 2)]  // face 5
];

fn part2_borders(supergrid: &[[usize; 4]; 4]) -> Borders {
    let mut mapping = [(usize::MAX, usize::MAX); 6];
    mapping[0] = (0, 0);

    let mut start = (usize::MAX, usize::MAX);
    'outer: for y in 0..4 {
        for x in 0..4 {
            if supergrid[y][x] == 0 {
                start = (x, y);
                break 'outer;
            }
        }
    }

    let mut queue = vec![start];
    while let Some((x, y)) = queue.pop() {
        let (face, rot) = mapping[supergrid[y][x]];
        for (i, &(xnext, ynext)) in [(x+1, y), (x, y+1), (x.wrapping_sub(1), y), (x, y.wrapping_sub(1))]
            .iter()
            .enumerate()
        {
            if xnext >= 4 || ynext >= 4 {
                continue;
            }
            let square = supergrid[ynext][xnext];
            if square == usize::MAX || mapping[square].0 != usize::MAX {
                continue
            }
            let (new_face, new_rot) = CUBE_EDGES[face][(rot + i) & 3];
            mapping[square] = (new_face, (6 + new_rot - i) & 3);
            queue.push((xnext, ynext));
        }
    }

    let mut inv_mapping = [0; 6];
    for (i, &(j, _)) in mapping.iter().enumerate() {
        inv_mapping[j] = i;
    }

    mapping.map(|(face, rot)| {
        let mut row = [(0, 0); 4];
        for (i, &(next_face, rot2)) in CUBE_EDGES[face].iter().enumerate() {
            let next_square = inv_mapping[next_face];
            row[(i+4-rot) & 3] = (next_square, (8 + mapping[next_square].1 + rot2 - rot - i) & 3);
        }
        row
    })
}