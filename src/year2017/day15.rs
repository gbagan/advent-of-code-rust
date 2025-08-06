use itertools::iterate;
use crate::util::{parser::*, power};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

const NB_BLOCKS: usize = 512;
const BLOCK_SIZE: usize = 40_000_000 / NB_BLOCKS;
const EXTRA_BLOCKS: usize = 2;

struct Block {
    part1: usize,
    div_by_4: Vec<u16>,
    div_by_8: Vec<u16>,
}

pub fn solve(input: &str) -> (usize, usize) {
    let [a, b] = input.iter_unsigned().next_chunk().unwrap();

    let mutex = Mutex::new(Vec::with_capacity(NB_BLOCKS));
    let counter = AtomicUsize::new(0);
    
    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(a, b, &counter, &mutex));
        }
    });    
    let blocks = mutex.into_inner().unwrap();
    let p1 = blocks[0..NB_BLOCKS].iter().map(|b| b.as_ref().unwrap().part1).sum();

    let mut p2 = 0;
    let mut todo = 5_000_000;
    let mut a_idx1 = 0;
    let mut a_idx2 = 0;
    let mut b_idx1 = 0;
    let mut b_idx2 = 0;

    while todo > 0 {
        let block1 = &blocks[a_idx1].as_ref().unwrap().div_by_4;
        let block2 = &blocks[b_idx1].as_ref().unwrap().div_by_8;
        let n = todo.min(block1.len() - a_idx2).min(block2.len() - b_idx2);
        for (x, y) in block1[a_idx2..a_idx2+n].iter().zip(block2[b_idx2..b_idx2+n].iter()) {
            if x == y {
                p2 += 1;
            }
        }

        a_idx2 += n;
        if a_idx2 == block1.len() {
            a_idx1 += 1;
            a_idx2 = 0;
        }

        b_idx2 += n;
        if b_idx2 == block2.len() {
            b_idx1 += 1;
            b_idx2 = 0;
        }
        todo -= n;
    }

    (p1, p2)
}

#[inline]
fn next_a(a: &u64) -> u64 {
    a * 16_807 % 2_147_483_647
}

#[inline]
fn next_b(a: &u64) -> u64 {
    a * 48_271 % 2_147_483_647
}

fn nth_a(n: usize, x: u64) -> u64 {
    if n == 0 {
        x
    } else {
        (x * power(|a, b| (a * b) % 2_147_483_647, 16_807, n)) % 2_147_483_647
    }
}

fn nth_b(n: usize, x: u64) -> u64 {
    if n == 0 {
        x
    } else {
        (x * power(|a, b| (a * b) % 2_147_483_647, 48271, n)) % 2_147_483_647
    }
}

fn worker(a: u64, b: u64, counter: &AtomicUsize, mutex: &Mutex<Vec<Option<Block>>>) {
    while let idx = counter.fetch_add(1, Ordering::Relaxed) && idx < NB_BLOCKS+EXTRA_BLOCKS {
        let mut p1 = 0;
        let mut div_by_4 = Vec::with_capacity(BLOCK_SIZE * 28 / 100);
        let mut div_by_8 = Vec::with_capacity(BLOCK_SIZE * 14 / 100);
        let iter_a = iterate(nth_a(idx*BLOCK_SIZE, a), next_a);
        let iter_b = iterate(nth_b(idx*BLOCK_SIZE, b), next_b);
        for (x, y) in iter_a.zip(iter_b).take(BLOCK_SIZE) {
            let x = x as u16;
            let y = y as u16;
            if x == y {
                p1 += 1;
            }
            if x & 3 == 0 {
                div_by_4.push(x);
            }
            if y & 7 == 0 {
                div_by_8.push(y);
            }
        }
        let block = Block { part1: p1, div_by_4, div_by_8 };

        let mut blocks = mutex.lock().unwrap();
        if idx < blocks.len() {
            blocks[idx] = Some(block);
        } else {
            for _ in blocks.len()..idx {
                blocks.push(None);
            }
            blocks.push(Some(block));
        }
    }
}