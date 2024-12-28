use memchr::{memchr, memmem};

pub fn solve(input: &str) -> (usize, usize) {
    let mut input = input.as_bytes();
    let mut p1 = 0;
    let mut p2 = 0;
    let mut rows = Vec::with_capacity(16);
    let mut columns = Vec::with_capacity(16);
    let nn_finder = memmem::Finder::new(b"\n\n");

    while let Some(sep) = nn_finder.find(input) {
        let grid = &input[..sep];
        input = &input[sep+2..];
        let (a, b) = parse_grid(grid, &mut rows, &mut columns);
        p1 += a;
        p2 += b;
    };
    let (a, b) = parse_grid(input, &mut rows, &mut columns);
    p1 += a;
    p2 += b;

    (p1, p2)
}

fn parse_grid(grid: &[u8], rows: &mut Vec<u32>, columns: &mut Vec<u32>) -> (usize, usize) {
    let width = memchr(b'\n', grid).unwrap();
    let height = (grid.len() + 1) / (width + 1);

    columns.clear();
    rows.clear();
    columns.resize(width, 0);
    rows.resize(height, 0);

    let mut x= 0;
    let mut y = 0;
    for &c in grid {
        match c {
            b'.' => {
                columns[x] <<= 1;
                rows[y] <<= 1;
                x += 1;
            }
            b'#' => {
                columns[x] = columns[x] << 1 | 1;
                rows[y] = rows[y] << 1 | 1;
                x += 1;
            }
            _ => { y += 1; x = 0; }
        }
    }

    let p1 = if let Some (v) = reflect(&columns) {
        v
    } else if let Some(v) = reflect(&rows) {
        v * 100
    } else {
        0
    };
    let p2 = if let Some (v) = reflect2(&columns) {
        v
    } else if let Some(v) = reflect2(&rows) {
        v * 100
    } else {
        0
    };


    (p1, p2)
}

fn reflect(encoding: &[u32]) -> Option<usize> {
    let n = encoding.len();
    (1..n).find(|&i| (0..i.min(n - i)).all(|j|
        encoding[i - j - 1] == encoding[i + j])
    )
}

fn reflect2(encoding: &[u32]) -> Option<usize> {
    let n = encoding.len();
    (1..n).find(|&i| {
        let mut one_diff = false;
        for j in 0..i.min(n - i) {
            let diff = encoding[i - j - 1] ^ encoding[i + j];
            if diff != 0 {
                if diff & (diff-1) != 0 || one_diff {
                    return false;
                } else {
                    one_diff = true;
                }

            }
        }
        one_diff
    })
}