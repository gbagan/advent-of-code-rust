use core::mem::transmute;
use std::simd::prelude::*;
use crate::util::{bits::*, parser::*};

const LEN: usize = 1280;

#[repr(align(64))]
struct Input {
    data: [i8; LEN*4]
}

pub fn solve(input: &str) -> (usize, u32) {
    let mut points = Input { data: [0; _]};
    
    for (i, v) in input.iter_signed::<i16>().enumerate() {
        points.data[i] = v as i8;
    }

    let mut uf = UnionFind::new(LEN);
    let mut ptr = points.data.as_ptr().cast::<u32>();
    let four = u32x16::splat(4);
    let ff = u32x16::splat(0xff);
    for i in 0..LEN {
        let point: i8x64 = unsafe { transmute(u32x16::splat(*ptr)) };
        let i2 = i & !15;
        let mut ptr2 = points.data[4*i2..].as_ptr().cast::<i8x64>();
        for j in (i2..LEN).step_by(16) {
            let dist = (point - unsafe { *ptr2 }).abs();
            let mut dist: u32x16 = unsafe { transmute(dist) };
            dist = dist + (dist >> 8);
            dist = (dist + (dist >> 16)) & ff;
            let mask = dist.simd_lt(four).to_bitmask();
            if mask != 0 {
                for b in mask.bits() {
                    let k = j + b;
                    if i < k {
                        uf.union(i, k);
                    }
                }
            }
            ptr2 = unsafe { ptr2.add(1) };
        }
        ptr = unsafe { ptr.add(1) };
    }

    let p1 = uf.count;

    (p1, 0)
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<u32>,
    count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        Self { parent, size, count: n }
    }

    pub fn find(&mut self, mut x: usize) -> usize {
        let mut root = x;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = root;
            x = p;
        }
        root
    }

    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.count -= 1;
        true
    }

    pub fn connected(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}