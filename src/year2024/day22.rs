use anyhow::*;
use crate::util::parser::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::simd::u16x32;
pub struct Shared {
    p1: u64,
    prices: Vec<u16>,
}

pub fn solve(input: &str) -> Result<(u64, u16)> {
    let numbers: Vec<_> = input.iter_unsigned::<u32>().collect();
    let start = AtomicUsize::new(0);
    let mutex = Mutex::new(Shared {p1: 0, prices: vec![0; 130336]});
    let nb_threads = thread::available_parallelism().unwrap().get();
    let chunks_size = numbers.len().div_ceil(nb_threads);

    thread::scope(|scope| {
        for _ in 0..nb_threads {
            scope.spawn(|| worker(&numbers, chunks_size, &start, &mutex));
        }
    });
    let shared = mutex.lock().unwrap();
    let p2 = *shared.prices.iter().max().unwrap();
    Ok((shared.p1, p2))
}

fn worker(numbers: &[u32], chunks_size: usize, start: &AtomicUsize, mutex: &Mutex<Shared>) {
    let mut seen = vec![0u16; 130321];
    let mut prices = vec![0; 130336];
    let mut iter = 1;
    let start = start.fetch_add(chunks_size, Ordering::Relaxed);
    let mut p1 = 0;
    
    for &secret0 in &numbers[start..(start+chunks_size).min(numbers.len())] {
        let secret1 = next_secret(secret0);
        let secret2 = next_secret(secret1);
        let secret3 = next_secret(secret2);
        let price0 = (secret0 % 10) as u16;
        let price1 = (secret1 % 10) as u16;
        let price2 = (secret2 % 10) as u16;
        let price3 = (secret3 % 10) as u16;
        let mut diff1;
        let mut diff2 = 9 + price1 - price0;
        let mut diff3 = 9 + price2 - price1;
        let mut diff4 = 9 + price3 - price2;
        let mut secret = secret3;
        let mut prev_price = price3;

        for _ in 0..1997 {
            secret = next_secret(secret);
            let price = (secret % 10) as u16;
            (diff1, diff2, diff3, diff4) = (diff2, diff3, diff4, 9 + price - prev_price);
            let index = (diff1 * 19 * 19 * 19 + diff2 * 19 * 19 + diff3 * 19 + diff4) as usize;
            if seen[index] != iter {
                prices[index] += price;
                seen[index] = iter;
            }
            prev_price = price;
        }
        p1 += secret as u64;
        iter += 1;
    }
    let mut shared = mutex.lock().unwrap();
    shared.p1 += p1;
    add_vector(&mut shared.prices, &prices);
}

#[inline]
fn next_secret(mut n: u32) -> u32 {
    n ^= n << 6;
    n &= 16777215;
    n ^= n >> 5;
    n &= 16777215;
    n ^= n << 11;
    n & 16777215
}

fn add_vector(v1: &mut [u16], v2: &[u16]) {
    let n = v1.len() / 32;
    for i in 0..n {
        let s1 = u16x32::from_slice(&v1[32*i..]);
        let s2 = u16x32::from_slice(&v2[32*i..]);
        (s1 + s2).copy_to_slice(&mut v1[32*i..])
    }
}