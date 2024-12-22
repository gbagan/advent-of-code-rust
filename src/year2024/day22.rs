use anyhow::*;
use crate::util::parser::*;
use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub struct Shared {
    start: AtomicUsize,
    p1: AtomicU64,
}

pub fn solve(input: &str) -> Result<(u64, u32)> {
    let numbers: Vec<_> = input.iter_unsigned::<u64>().collect();

    let shared = Shared { p1: AtomicU64::new(0), start: AtomicUsize::new(0) };
    let (tx, rx) = channel();

    let nb_threads = thread::available_parallelism().unwrap().get() - 1;
    let chunks_size = numbers.len().div_ceil(nb_threads);

    let (p1, p2) =  thread::scope(|scope| {
        for _ in 0..nb_threads {
            scope.spawn(|| sender(&numbers, chunks_size, &shared, &tx));
        }
        receiver(numbers.len(), chunks_size, &shared, &rx)
    });
    Ok((p1, p2))
}

fn sender(numbers: &[u64], chunks_size: usize, shared: &Shared, tx: &Sender<Vec<u32>>) {
    let mut seen = vec![0u16; 130321];
    let mut diff = Vec::with_capacity(1000);
    let mut iter = 1;

    loop {
        let start = shared.start.fetch_add(chunks_size, Ordering::Relaxed);
        let mut p1 = 0;
        if start >= numbers.len() {
            break;
        }
        let mut prices = vec![0; 130321];
        for &n in &numbers[start..(start+chunks_size).min(numbers.len())] {
            diff.clear();
            let mut m = n;
            let mut x = (m % 10) as u32;
            for _ in 0..2000 {
                m = next_secret(m);
                let y = (m % 10) as u32;
                diff.push((9 + y - x, y));
                x = y;
            }
            for &[(d1, _), (d2, _), (d3, _), (d4, p)] in diff.array_windows() {
                let index = (d1 * 19 * 19 * 19 + d2 * 19 * 19 + d3 * 19 + d4) as usize;
                if seen[index] != iter {
                    prices[index] += p;
                    seen[index] = iter;
                }
            }
            p1 += m;
            iter += 1;    
        }
        shared.p1.fetch_add(p1, Ordering::Relaxed);
        _ = tx.send(prices);
    }
}


fn receiver(size: usize, chunks_size: usize, shared: &Shared, rx: &Receiver<Vec<u32>>) -> (u64, u32) {
    let mut made = 0;
    let mut prices = vec![0; 130321];
    while made < size {
        let new_prices = rx.recv().unwrap();
        for (u, v) in prices.iter_mut().zip(new_prices) {
            *u += v;
        }
        made += chunks_size;
    }
    let p1 = shared.p1.load(Ordering::Relaxed);
    let p2 = *prices.iter().max().unwrap();
    (p1, p2)
}

#[inline]
fn next_secret(mut n: u64) -> u64 {
    n ^= n << 6;
    n &= 16777215;
    n ^= n >> 5;
    n &= 16777215;
    n ^= n << 11;
    n & 16777215
}