use ahash::{HashMap, HashMapExt};
use crate::util::{iter::*, parser::*};

enum Instruction {
    Mask { ones: u64, xs: u64 },
    Mem(u64, u64) }

use Instruction::*;

pub fn solve(input: &str) -> (u64, u64) {
    let instructions: Vec<_> = input.lines().map(|line| {
        if let Some(mask) = line.strip_prefix("mask = ") {
            parse_mask(mask.as_bytes())
        } else {
            let (mem, addr) = line.iter_unsigned().next_tuple().unwrap();
            Mem(mem, addr)
        }
    }).collect();

    let p1 = part1(&instructions);
    let p2 = part2(&instructions);

    (p1, p2)
}


fn parse_mask(slice: &[u8]) -> Instruction {
    let mut ones = 0;
    let mut xs = 0;
    let mut bit = 1 << 35;
    for &c in slice {
        match c {
            b'1' => ones |= bit,
            b'X' => xs |= bit,
            _ => {},
        };
        bit >>= 1;
    }
    Mask { ones, xs }
}


fn part1(instructions: &[Instruction]) -> u64 {
    let mut memory = HashMap::with_capacity(500);
    let mut m_ones = 0;
    let mut m_xs = 0;
    for instr in instructions {
        match instr {
            Mask {ones, xs } => { m_ones = *ones; m_xs = *xs },
            Mem(addr, val) => { memory.insert(*addr, (*val & m_xs) | m_ones); }
        }
    }
    memory.values().sum()
}


struct Set {
    ones: u64,
    floating: u64,
    value: u64,
}

impl Set {
    fn from(ones: u64, xs: u64, addr: u64, value: u64) -> Self {
        Self {
            ones: (ones | addr) & !xs,
            floating: xs,
            value
        }
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        ((self.ones ^ other.ones) & !(self.floating | other.floating) == 0).then_some(
            Self {
                ones: self.ones | other.ones,
                floating: self.floating & other.floating,
                value: 0,
            }
        )
    }

    fn size(&self) -> u64 {
        1 << self.floating.count_ones()
    }
}

fn part2(instructions: &[Instruction]) -> u64 {
    let mut sets = Vec::new();
    let mut m_ones = 0;
    let mut m_xs = 0;
    for instr in instructions {
        match instr {
            Mask {ones, xs } => { m_ones = *ones; m_xs = *xs },
            Mem(addr, value) => sets.push(Set::from(m_ones, m_xs, *addr, *value)),
        }
    }

    let mut intersected = Vec::new();
    let mut total = 0;

    for (i, set) in sets.iter().enumerate() {
        intersected.extend(sets[i+1..]
            .iter()
            .filter_map(|set2| set2.intersect(set))
        );

        total += set.value * include_exclude(set, &intersected);
        intersected.clear();
    }

    total
}

fn include_exclude(set: &Set, others: &[Set]) -> u64 {
    let mut total = set.size();

    for (i, other) in others.iter().enumerate() {
        if let Some(set2) = set.intersect(other) {
            total -= include_exclude(&set2, &others[(i + 1)..]);
        }
    }

    total
}