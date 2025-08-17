use crate::util::iter::*;

pub struct Input {
    width: usize,
    height: usize,
    horizontal: Vec<u128>,
    vertical: Vec<u128>,
}

pub fn solve(input: &str) -> (usize, usize) {
    let input: Vec<_> = input.lines().map(|line| &line.as_bytes()[1..line.len()-1]).collect();
    let width = input[0].len();
    let height = input.len() - 2;

    let mk_masks = |wind| -> Vec<u128> {
        input[1..height+1].iter().map(|&line|
            line.iter().map(|&c| c != wind).to_bitmask()
        ).collect()
    };

    let left = mk_masks(b'<');
    let right = mk_masks(b'>');
    let up = mk_masks(b'^');
    let down = mk_masks(b'v');

    let mut horizontal = Vec::with_capacity(width * height);
    for time in 0..width {
        for i in 0..height {
            let left = (left[i] << time) | (left[i] >> (width - time));
            let right = (right[i] >> time) | (right[i] << (width - time));
            horizontal.push(left & right);
        }
    }

    let mut vertical = Vec::with_capacity(height * height);
    for time in 0..height {
        for i in 0..height {
            let up = up[(i + time) % height];
            let down = down[(height + i - time) % height];
            vertical.push(up & down);
        }
    }

    let input = Input {height, width, vertical, horizontal};

    let p1 = bfs(&input, 0, true);
    let time1 = bfs(&input, p1, false);
    let p2 = bfs(&input, time1, true);
    (p1, p2)
}

fn bfs(input: &Input, mut time: usize, forward: bool) -> usize {
    let width = input.width;
    let height = input.height;
    let vertical = &input.vertical;
    let horizontal = &input.horizontal;

    let mut reachable = vec![0; height + 2];
    let mut next = vec![0; height + 2];
    
    loop {
        time += 1;
        
        let h = &horizontal[height * (time % width)..];
        let v = &vertical[height * (time % height)..];

        for i in 0..height {
            let current = reachable[i+1];
            let above = reachable[i];
            let below = reachable[i+2];
            next[i+1] = (current | (current >> 1) | (current << 1) | above | below) & h[i] & v[i];
        }

        std::mem::swap(&mut reachable, &mut next);
        if forward {
            reachable[1] |= 1 << (width - 1) & h[0] & v[0];
            if reachable[height] & 1 != 0 {
                return time + 1;
            }
        } else {
            reachable[height] |= 1 & h[height-1] & v[height-1];
            if reachable[1] & (1 << (width - 1)) != 0 {
                return time + 1;
            }
        }
    }

}