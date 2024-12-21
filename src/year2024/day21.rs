use anyhow::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let row = line.as_bytes();
        let (x, y) = solve_row(row);
        p1 += x;
        p2 += y;
    }
    Ok((p1, p2))
}

pub fn solve_row(row: &[u8]) -> (u64, u64) {
    assert!(row.len() == 4);
    let n = (row[0] - b'0') as u64 * 100
        + (row[1] - b'0') as u64 * 10
        + (row[2] - b'0') as u64;
    let a = numpad_index(row[0]);
    let b = numpad_index(row[1]);
    let c = numpad_index(row[2]);
    let d = numpad_index(row[3]);
    let p1 =
        NUMPAD_TABLE_WEIGHT2[110 + a] 
        + NUMPAD_TABLE_WEIGHT2[a * 11 + b]
        + NUMPAD_TABLE_WEIGHT2[b * 11 + c]
        + NUMPAD_TABLE_WEIGHT2[c * 11 + d];
    let p2 =
        NUMPAD_TABLE_WEIGHT25[110 + a] 
        + NUMPAD_TABLE_WEIGHT25[a * 11 + b]
        + NUMPAD_TABLE_WEIGHT25[b * 11 + c]
        + NUMPAD_TABLE_WEIGHT25[c * 11 + d];

    (n * p1, n * p2)
}

const DIRPAD_TABLE: [[u64; 25]; 25] = compute_dirpad_table();
const DIRPAD_TABLE_WEIGHT2: [u64; 25] = dirpad_pair_weights(2);
const DIRPAD_TABLE_WEIGHT25: [u64; 25] = dirpad_pair_weights(25);
const NUMPAD_TABLE_WEIGHT2: [u64; 121] = numpad_pair_weights(&DIRPAD_TABLE_WEIGHT2);
const NUMPAD_TABLE_WEIGHT25: [u64; 121] = numpad_pair_weights(  &DIRPAD_TABLE_WEIGHT25);

const fn numpad_index(c: u8) -> usize {
    if c == b'A' { 10 } else { (c - b'0') as usize }
}

const fn mk_numpad() -> [(i32, i32); 128] {
    let mut table = [(0, 0); 128];
    table[b'0' as usize] = (1, 0);
    table[b'A' as usize] = (2, 0);
    table[b'1' as usize] = (0, 1);
    table[b'2' as usize] = (1, 1);
    table[b'3' as usize] = (2, 1);
    table[b'4' as usize] = (0, 2);
    table[b'5' as usize] = (1, 2);
    table[b'6' as usize] = (2, 2);
    table[b'7' as usize] = (0, 3);
    table[b'8' as usize] = (1, 3);
    table[b'9' as usize] = (2, 3);
    table
}

const NUMPAD: [(i32, i32); 128] = mk_numpad();

const DIRPAD: [(i32, i32); 5] = [(2, 0), (0, 0), (1, 1), (1, 0), (2, 1)];

const fn numpad_step(from: u8, to: u8) -> (u8, i32, u8, i32) {
    let (fx,fy) = NUMPAD[from as usize];
    let (tx,ty) = NUMPAD[to as usize];
    let dx = tx - fx;
    let dy = ty - fy;
    let horiz = if dx > 0 {(0, dx)} else {(1, -dx)};
    let vert = if dy > 0 {(2, dy)} else {(3, -dy)};
    if (dx > 0 && !(fx == 0 && ty == 0)) || tx == 0 && fy == 0  {
        (vert.0, vert.1, horiz.0, horiz.1)
    } else {
        (horiz.0, horiz.1, vert.0, vert.1)
    }
}

const fn dirpad_step(from: u8, to: u8) -> (u8, i32, u8, i32) {
    let (fx,fy) = DIRPAD[from as usize];
    let (tx,ty) = DIRPAD[to as usize];
    let dx = tx - fx;
    let dy = ty - fy;
    let horiz = if dx > 0 {(0, dx)} else {(1, -dx)};
    let vert = if dy > 0 {(2, dy)} else {(3, -dy)};
    if (dx > 0 && !(fx == 0 && ty == 1)) || tx == 0 && fy == 1  {
        (vert.0, vert.1, horiz.0, horiz.1)
    } else {
        (horiz.0, horiz.1, vert.0, vert.1)
    }
}

const fn compute_dirpad_table() -> [[u64; 25]; 25] {
    let mut table = [[0; 25]; 25];
    let mut from = 0;
    while from < 5 {
        let mut to = 0;
        while to < 5 {
            let index = ((from * 5) + to) as usize;
            let (d1, f1, d2, f2) = dirpad_step(from, to);
            if f1 == 0 && f2 == 0 {
                table[index][4 * 5 + 4] = 1;
            } else if f1 == 0 {
                table[index][4 * 5 + d2 as usize] += 1;
                let mut i = 0;
                while i < f2 - 1 {
                    table[index][d2 as usize * 5 + d2 as usize] += 1;
                    i += 1;
                }
                table[index][d2 as usize * 5 + 4] += 1;
            }  else if f2 == 0 {
                table[index][4 * 5 + d1 as usize] += 1;
                let mut i = 0;
                while i < f1 - 1 {
                    table[index][d1 as usize * 5 + d1 as usize] += 1;
                    i += 1;
                }
                table[index][d1 as usize * 5 + 4] += 1;
            } else {
                table[index][4 * 5 + d1 as usize] += 1;
                let mut i = 0;
                while i < f1 - 1 {
                    table[index][d1 as usize * 5 + d1 as usize] += 1;
                    i += 1;
                }
                table[index][d1 as usize * 5 + d2 as usize] += 1;
                i = 0;
                while i < f2 - 1 {
                    table[index][d2 as usize * 5 + d2 as usize] += 1;
                    i += 1;
                }
                table[index][d2 as usize * 5 + 4] += 1;
            }
            to += 1;

        }
        from += 1;
    }
    table
}

const fn dirpad_pair_weight(pair: usize, depth: u32) -> u64 {
    let mut counter = [0; 25];
    let mut next_counter = [0; 25];
    counter[pair] = 1;

    let mut i = 0;
    while i < depth {
        let mut j = 0;
        while j < 25 {
            let freq = counter[j];
            let mut k = 0;
            while k < 25 {
                next_counter[k] += freq * DIRPAD_TABLE[j][k];
                k += 1;
            }
            j += 1;
        }
        counter = next_counter;
        next_counter = [0; 25];
        i += 1;
    }
    let mut res = 0;
    let mut i = 0; 
    while i < 25 {
        res += counter[i];
        i += 1;
    }
    res
}

const fn dirpad_pair_weights(depth: u32) -> [u64; 25] {
    let mut res = [0; 25];
    let mut i = 0;
    while i < 25 {
        res[i] += dirpad_pair_weight(i, depth);
        i += 1;
    }
    res
}

const fn numpad_pair_weight(from: u8, to: u8, dirpad_weights: &[u64]) -> u64 {
    let mut table = [0; 25];
    let (d1, f1, d2, f2) = numpad_step(from, to);
    if f1 == 0 && f2 == 0 {
        table[4 * 5 + 4] = 1;
    } else if f1 == 0 {
        table[4 * 5 + d2 as usize] += 1;
        let mut i = 0;
        while i < f2 - 1 {
            table[d2 as usize * 5 + d2 as usize] += 1;
            i += 1;
        }
        table[d2 as usize * 5 + 4] += 1;
    }  else if f2 == 0 {
        table[4 * 5 + d1 as usize] += 1;
        let mut i = 0;
        while i < f1 - 1 {
            table[d1 as usize * 5 + d1 as usize] += 1;
            i += 1;
        }
        table[d1 as usize * 5 + 4] += 1;
    } else {
        table[4 * 5 + d1 as usize] += 1;
        let mut i = 0;
        while i < f1 - 1 {
            table[d1 as usize * 5 + d1 as usize] += 1;
            i += 1;
        }
        table[d1 as usize * 5 + d2 as usize] += 1;
        i = 0;
        while i < f2 - 1 {
            table[d2 as usize * 5 + d2 as usize] += 1;
            i += 1;
        }
        table[d2 as usize * 5 + 4] += 1;
    }
    let mut res = 0;
    let mut i = 0;
    while i < 25 {
        res += table[i] * dirpad_weights[i];
        i += 1;
    }
    res
}

const fn numpad_pair_weights(dirpad_weights: &[u64]) -> [u64; 121] {
    const T: [u8; 11] = [b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A'];
    let mut res = [0; 121];
    let mut i = 0;
    while i < 11 {
        let mut j = 0;
        while j < 11 {
            res[i * 11 + j] += numpad_pair_weight(T[i], T[j], dirpad_weights);
            j += 1;
        }
        i += 1;
    }
    res
}