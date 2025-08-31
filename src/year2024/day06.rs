use crate::util::parallel::*;
use arrayvec::ArrayVec;

const SOUTH: usize = 0;
const NORTH: usize = 1;
const WEST: usize = 2;
const EAST: usize = 3;

const N: usize = 130;
const SIZE: usize = N * (N+1);

struct Input {
    rocks_x: [u32; 1024],
    rocks_y: [u32; 1024],
    x_ids: [ArrayVec<u32, 16>; N],
    y_ids: [ArrayVec<u32, 16>; N],
    rocks_len: usize,
}

struct Save {
    north: Option<(usize, usize, u32)>,
    south: Option<(usize, usize, u32)>,
    west: Option<(usize, usize, u32)>,
    east: Option<(usize, usize, u32)>,
}

pub fn solve(input: &str) -> (usize, usize) {
    let input = input.as_bytes();
    let mut rocks_len = 0;
    let mut rocks_x = [0; 1024];
    let mut rocks_y = [0; 1024];
    let mut x_ids = std::array::from_fn(|_| ArrayVec::new());
    let mut y_ids = std::array::from_fn(|_| ArrayVec::new());
    let mut start = 0;

    for y in 0..N {
        for x in 0..N {
            let index = y * (N+1) + x;
            match input[index] {
                b'^' => start = index,
                b'#' => {
                    rocks_x[rocks_len] = x as u32;
                    rocks_y[rocks_len] = y as u32;
                    x_ids[x].push(rocks_len as u32);
                    y_ids[y].push(rocks_len as u32);
                    rocks_len += 1;
                }
                _ => {},
            }
        }
    }

    let seen = part1(input, start);
    let p1 = seen.len();

    let input = Input {rocks_x, rocks_y, x_ids, y_ids, rocks_len };

    let moves = compute_moves(&input);

    let p2 = (&seen[1..])
        .par_iter()
        .chunks(100)
        .map(|rocks| {
            let mut moves = moves.clone();
            rocks
                .iter()
                .filter(|&&rock| {
                    let dir = rock as usize & 3;
                    let rock = rock as usize >> 2;
                    let save = add_rock(&input, &mut moves, rock);
                    let rock_idx = rocks_len;
                    let res = has_loop(&moves, rock_idx, dir);
                    remove_rock(&input, &mut moves, rock, &save);
                    res
                }).count()
        })
        .sum();

    (p1, p2)
}

fn part1(input: &[u8], start: usize) -> Vec<u32> {
    let mut total= Vec::with_capacity(6000);
    let mut seen = [false; SIZE];
    let mut current = start;

    macro_rules! move_and_check {
        ($next:expr, $check:expr, $dir:expr) => {{
            loop {
                if !seen[current] {
                    total.push(mov(current, $dir) as u32);
                    seen[current] = true;
                }
    
                let next = $next;
                if $check(next) {
                    return total;
                }
                if input[next] == b'#' {
                    break;
                }
                current = next;            
            }
        }};
    }

    loop {
        move_and_check!(current.wrapping_sub(N+1), |p| p >= SIZE, SOUTH);
        move_and_check!(current + 1, |p| input[p] == b'\n', WEST);
        move_and_check!(current + (N+1), |p| p >= SIZE, NORTH);
        move_and_check!(current.wrapping_sub(1), |p| input[p] == b'\n', EAST);
    }
}

fn compute_moves(input: &Input) -> Vec<u32> {
    let rocks_x = &input.rocks_x;
    let rocks_y = &input.rocks_y;
    let rocksx_idx = &input.x_ids;
    let rocksy_idx = &input.y_ids;

    let outside = input.rocks_len + 1;
    let mut moves = vec![4 * outside as u32; 4 * (input.rocks_len + 2)];

    for y in 0..N - 1 {
        let row1 = &rocksy_idx[y];
        let row2 = &rocksy_idx[y+1];
        let mut next1 = 0;
        let mut next2 = 0;
        let mut current = outside;

        loop {
            if next2 >= row2.len() {
                for &pos in &row1[next1..] {
                    moves[mov(pos as usize, SOUTH)] = mov(outside, WEST) as u32;
                }
                break;
            }

            while next1 < row1.len() && rocks_x[row1[next1] as usize] <= rocks_x[row2[next2] as usize] {
                moves[mov(row1[next1] as usize, SOUTH)] = mov(row2[next2] as usize, WEST) as u32;
                current = row1[next1] as usize;
                next1 += 1;
            }

            if next1 >= row1.len() {
                for &pos in &row2[next2..] {
                    moves[mov(pos as usize, NORTH)] = mov(current, EAST) as u32;
                }
                break;
            }

            while next2 < row2.len() && rocks_x[row2[next2] as usize] <= rocks_x[row1[next1] as usize] {
                moves[mov(row2[next2] as usize, NORTH)] = mov(current, EAST) as u32;
                next2 += 1;
            }
        }
    }

    for x in 0..N - 1 {
        let col1 = &rocksx_idx[x];
        let col2 = &rocksx_idx[x+1];

        let mut next1 = 0;
        let mut next2 = 0;
        let mut current = outside;

        loop {
            if next2 >= col2.len() {
                for &pos in &col1[next1..] {
                    moves[mov(pos as usize, EAST)] = mov(current, SOUTH) as u32;
                }
                break;
            }

            while next1 < col1.len() && rocks_y[col1[next1] as usize] <= rocks_y[col2[next2] as usize] {
                moves[mov(col1[next1] as usize, EAST)] = mov(current, SOUTH) as u32;
                next1 += 1;
            }

            if next1 >= col1.len() {
                for &pos in &col2[next2..] {
                    moves[mov(pos as usize, WEST)] = mov(outside, NORTH) as u32;
                }
                break;
            }

            while next2 < col2.len() && rocks_y[col2[next2] as usize] <= rocks_y[col1[next1] as usize] {
                moves[mov(col2[next2] as usize, WEST)] = mov(col1[next1] as usize, NORTH) as u32;
                current = col2[next2] as usize;
                next2 += 1;
            }
        }
    }

    moves
}

fn add_rock(input: &Input, moves: &mut [u32], rock: usize) -> Save {
    let rocks_x = &input.rocks_x;
    let rocks_y = &input.rocks_y;
    let x_ids = &input.x_ids;
    let y_ids = &input.y_ids;

    let rock_id = input.rocks_len;
    let outside = input.rocks_len + 1;
    let new_x = (rock % (N+1)) as u32;
    let new_y = (rock / (N+1)) as u32;

    let mut old_south = None;
    if new_y != 0 {
        let ids = y_ids[new_y as usize - 1].as_slice();
        if let Some(i) = ids.iter().rposition(|&id| rocks_x[id as usize] < new_x) {
            let id = ids[i] as usize;
            moves[mov(rock_id, NORTH)] = mov(id, EAST) as u32;
            let prev_move = moves[mov(id, SOUTH)];
            if prev_move >> 2 == outside as u32 || rocks_x[prev_move as usize >> 2] > new_x {
                moves[mov(id, SOUTH)] = mov(rock_id, WEST) as u32;
                let mut j = i;
                while j > 0 && moves[mov(ids[j-1] as usize, SOUTH)] == prev_move {
                    j -= 1;
                    moves[mov(ids[j] as usize, SOUTH)] = mov(rock_id, WEST) as u32;
                }
                old_south = Some((j, i, prev_move));
            }
        } else {
            moves[mov(rock_id, NORTH)] = mov(outside, EAST) as u32;
        }
    }

    let mut old_north = None;
    if new_y != N as u32 - 1 {
        let ids = y_ids[new_y as usize + 1].as_slice();
        if let Some(i) = ids.iter().position(|&id| rocks_x[id as usize] > new_x) {
            let id = ids[i] as usize;
            moves[mov(rock_id, SOUTH)] = mov(id, WEST) as u32;
            let prev_move = moves[mov(id, NORTH)];
            if prev_move >> 2 == outside as u32 || rocks_x[prev_move as usize >> 2] < new_x {
                moves[mov(id, NORTH)] = mov(rock_id, EAST) as u32;
                let mut j = i;
                while j != ids.len() - 1 && moves[mov(ids[j+1] as usize, NORTH)] == prev_move {
                    j += 1;
                    moves[mov(ids[j] as usize, NORTH)] = mov(rock_id, EAST) as u32;
                }
                old_north = Some((i, j, prev_move));
            }
        } else {
            moves[mov(rock_id, SOUTH)] = mov(outside, WEST) as u32;
        }
    }

    let mut old_east = None;
    if new_x != 0 {
        let ids = x_ids[new_x as usize - 1].as_slice();
        if let Some(i) = ids.iter().position(|&id| rocks_y[id as usize] > new_y) {
            let id = ids[i] as usize;
            moves[mov(rock_id, WEST)] = mov(id, NORTH) as u32;
            let prev_move = moves[mov(id, EAST)];
            if prev_move >> 2 == outside as u32 || rocks_y[prev_move as usize >> 2] < new_y {
                moves[mov(id, EAST)] = mov(rock_id, SOUTH) as u32;
                let mut j = i;
                while j != ids.len() - 1 && moves[mov(ids[j+1] as usize, EAST)] == prev_move {
                    j += 1;
                    moves[mov(ids[j] as usize, EAST)] = mov(rock_id, SOUTH) as u32;
                }
                old_east = Some((i, j, prev_move));
            }
        } else {
            moves[mov(rock_id, WEST)] = mov(outside, NORTH) as u32;
        }
    }

    let mut old_west = None;
    if new_x != N as u32 - 1 {
        let ids = x_ids[new_x as usize + 1].as_slice();
        if let Some(i) = ids.iter().rposition(|&id| rocks_y[id as usize] < new_y) {
            let id = ids[i] as usize;
            moves[mov(rock_id, EAST)] = mov(id, SOUTH) as u32;
            let prev_move = moves[mov(id, WEST)];
            if prev_move >> 2 == outside as u32 || rocks_y[prev_move as usize >> 2] > new_y {
                moves[mov(id, WEST)] = mov(rock_id, NORTH) as u32;
                let mut j = i;
                while j > 0 && moves[mov(ids[j-1] as usize, WEST)] == prev_move {
                    j -= 1;
                    moves[mov(ids[j] as usize, WEST)] = mov(rock_id, NORTH) as u32;
                }
                old_west = Some((j, i, prev_move));
            }
        } else {
            moves[mov(rock_id, EAST)] = mov(outside, SOUTH) as u32;
        }
    }

    Save { south: old_south, north: old_north, west: old_west, east: old_east }
}

fn remove_rock(input: &Input, moves: &mut [u32], rock: usize, save: &Save) {
    let rock_x = rock % (N+1);
    let rock_y = rock / (N+1);

    if let Some((i, j, old)) = save.south {
        for &id in &input.y_ids[rock_y-1][i..=j] {
            moves[mov(id as usize, SOUTH)] = old;
        }
    }

    if let Some((i, j, old)) = save.north {
        for &id in &input.y_ids[rock_y+1][i..=j] {
            moves[mov(id as usize, NORTH)] = old;
        }
    }

    if let Some((i, j, old)) = save.west {
        for &id in &input.x_ids[rock_x+1][i..=j] {
            moves[mov(id as usize, WEST)] = old;
        }
    }

    if let Some((i, j, old)) = save.east {
        for &id in &input.x_ids[rock_x-1][i..=j] {
            moves[mov(id as usize, EAST)] = old;
        }
    }
}

fn has_loop(moves: &[u32], rock_idx: usize, dir: usize) -> bool {
    let mut pos = mov(rock_idx, dir);
    let mut seen: BitSet<64> = BitSet::new();
    let outside = (moves.len() / 4) - 1;

    while seen.insert(pos) {
        pos = moves[pos] as usize;
        pos = moves[pos] as usize;
        pos = moves[pos] as usize;
        pos = moves[pos] as usize;
    }
    pos >> 2 != outside
}    


fn mov(pos: usize, dir: usize) -> usize {
    (pos << 2) | dir
}

struct BitSet<const N: usize> {
    data: [u64; N],
}

impl<const N: usize> BitSet<N> {
    fn new() -> Self {
        Self { data: [0; N] }
    }

    #[inline]
    fn insert(&mut self, n: usize) -> bool {
        let elem = &mut self.data[n >> 6];
        let mask = 1 << (n & 63);
        let res= *elem & mask == 0;
        *elem |= mask;
        res
    }
}