use std::ops::{BitAnd, BitOr, Not};

const WIDTH: usize = 139;

pub fn solve(input: &str) -> (u32, u32) {
    let width = input.lines().next().unwrap().len();
    let height = input.len() / (width + 1);
    
    assert!(width == WIDTH);
    
    let mut east = Vec::with_capacity(height);
    let mut south = Vec::with_capacity(height);

    for line in input.lines() {
        let mut east_row: Row<WIDTH> = Row::default();
        let mut south_row: Row<WIDTH> = Row::default();
        for (i, c) in line.bytes().enumerate() {
            match c {
                b'>' => east_row.set_bit(i),
                b'v' => south_row.set_bit(i),
                _ => {},
            }
        }
        east.push(east_row);
        south.push(south_row);
    }

    let mut changed = true;
    let mut p1 = 0;

    while changed {
        changed = false;
        p1 += 1;

        for i in 0..height {
            let moved = east[i].rotate_right() & !(east[i] | south[i]);
            let stay = east[i] & !moved.rotate_left();
            east[i] = moved | stay;

            changed |= !moved.is_zero();
        }

        let first_blocker = east[0] | south[0];
        let mut moved = south[height-1] & !first_blocker;

        for i in 0..height-1 {
            let blocker = east[i+1] | south[i+1];
            let stay = south[i] & blocker;
            let next_moved = south[i] & !blocker;
            south[i] = moved | stay;
            changed |= !moved.is_zero();
            moved = next_moved;
        }

        let stay = south[height-1] & first_blocker;
        south[height-1] = moved | stay;
        changed |= !moved.is_zero();
    }

    (p1, 0)
}

#[derive(Copy, Clone)]
struct Row<const N: usize> {
    left: u64,
    middle: u64,
    right: u64,
}

impl<const N: usize> Row<N> {
    #[inline]
    fn default() -> Self {
        Self { left: 0, middle: 0, right: 0 }
    }

    fn set_bit(&mut self, i: usize) {
        let i = N - i - 1;
        if i < 64 {
            self.right |= 1 << i;
        } else if i < 128 {
            self.middle |= 1 << (i - 64);
        } else {
            self.left |= 1 << (i - 128);
        }
    }

    #[inline]
    fn is_zero(&self) -> bool {
        self.left == 0 && self.middle == 0 && self.right == 0 
    }

    fn rotate_left(&self) -> Self {
        let mask = !(1 << (N - 128));
        let left = self.left << 1 & mask | self.middle >> 63;
        let middle = self.middle << 1 | self.right >> 63;
        let right = self.right << 1 | self.left >> (N - 129);
        Self { left, middle, right }
    }

    fn rotate_right(&self) -> Self {
        let left = self.left >> 1 | (self.right & 1) << (N - 129);
        let middle = self.middle >> 1 | (self.left & 1) << 63;
        let right = self.right >> 1 | (self.middle & 1) << 63;
        Self { left, middle, right }
    }

}

impl<const N: usize> BitAnd for Row<N> {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Row {
            left: self.left & other.left,
            middle: self.middle & other.middle,
            right: self.right & other.right
        }
    }
}

impl<const N: usize> BitOr for Row<N> {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            left: self.left | other.left,
            middle: self.middle | other.middle,
            right: self.right | other.right
        }
    }
}

impl<const N: usize> Not for Row<N> {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            left: !self.left,
            middle: !self.middle,
            right: !self.right
        }
    }
}