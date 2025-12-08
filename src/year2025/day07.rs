pub fn solve(input: &str) -> (u32, u64) {
    let input = input.as_bytes();
    let width = input.iter().position(|&b| b == b'\n').unwrap() + 1;
    let mid = input.iter().position(|&b| b == b'S').unwrap();
    let mut p1 = 0;
    let mut timelines = vec![0; width];
    timelines[mid] = 1;
    for (delta, line_start) in (2*width..input.len()).step_by(2*width).enumerate() {
        let start = mid - delta;
        let end = mid + delta + 1;
        for i in start..end {
            if input[line_start+i] == b'^' {
                p1 += (timelines[i] != 0) as u32;
                timelines[i-1] += timelines[i];
                timelines[i+1] += timelines[i];
                timelines[i] = 0;
            }
        }
    }


    let p2 = timelines.iter().sum();

    (p1, p2)
}

/* 
use std::{ops::{BitAnd, BitOr, Not}, simd::prelude::*};
const WIDTH: usize = 142;
const MID: usize = 70;

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut beams = U192::new(0, 0, 0);
    beams.second |= 1 << (MID - 64);
    
    (2*WIDTH..input.len()).step_by(2*WIDTH).map(|line_start| {
        let splits = U192::from_slice(&input[line_start..line_start+WIDTH-1]);
        let beam_on_split = beams & splits;
        beams = (beams | beam_on_split.shift_left() | beam_on_split.shift_right()) & !splits;
        beam_on_split.count_ones()
    }).sum()
}

#[derive(Clone, Copy)]
struct U192 {
    first: u64,
    second: u64,
    third: u64,
}

impl U192 {
    #[inline]
    fn new(first: u64, second: u64, third: u64) -> Self {
        Self { first, second, third }
    }

    fn from_slice(slice: &[u8]) -> Self {
        let v = u8x64::splat(b'^');
        let first = u8x64::from_slice(&slice[..64]).simd_eq(v).to_bitmask();
        let second = u8x64::from_slice(&slice[64..128]).simd_eq(v).to_bitmask();
        const REM: usize = 192 - (WIDTH - 1);
        let third = u8x64::from_slice(&slice[128-REM..]).simd_eq(v).to_bitmask() >> REM;
        Self { first, second, third}
    }


    #[inline]
    fn shift_left(&self) -> Self {
        Self::new(
            self.first >> 1 | self.second << 63,
            self.second >> 1 | self.third << 63,
            self.third >> 1
        )
    }

    #[inline]
    fn shift_right(&self) -> Self {
        Self::new(
            self.first << 1,
            self.second << 1 | self.first >> 63,
            self.third << 1 | self.second >> 63,
        )
    }

    #[inline]
    fn count_ones(&self) -> u32 {
        self.first.count_ones() + self.second.count_ones() + self.third.count_ones()
    }
}

impl BitAnd for U192 {
    type Output = Self;

    #[inline]
    fn bitand(self, other: Self) -> Self {
        Self {
            first: self.first & other.first,
            second: self.second & other.second,
            third: self.third & other.third
        }
    }
}

impl BitOr for U192 {
    type Output = Self;

    #[inline]
    fn bitor(self, other: Self) -> Self {
        Self {
            first: self.first | other.first,
            second: self.second | other.second,
            third: self.third | other.third
        }
    }
}

impl Not for U192 {
    type Output = Self;

    fn not(self) -> Self {
        Self {
            first: !self.first,
            second: !self.second,
            third: !self.third
        }
    }
}
*/