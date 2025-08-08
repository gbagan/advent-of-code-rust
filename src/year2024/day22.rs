use crate::util::parser::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;
use std::simd::prelude::*;

const LANES: usize = 16;

pub struct Shared {
    p1: u64,
    prices: Vec<u16>,
}

pub fn solve(input: &str) -> (u64, u16) {
    let mut numbers: Vec<_> = input.iter_unsigned::<u32>().collect();
    numbers.resize(numbers.len().next_multiple_of(LANES), 0);
    let start = AtomicUsize::new(0);
    let mutex = Mutex::new(Shared {p1: 0, prices: vec![0; 130336]});
    let nb_threads = thread::available_parallelism().unwrap().get();
    let chunks_size = (numbers.len() / LANES).div_ceil(nb_threads);

    thread::scope(|scope| {
        for _ in 0..nb_threads {
            scope.spawn(|| worker(&numbers, chunks_size, &start, &mutex));
        }
    });
    let shared = mutex.lock().unwrap();
    let p2 = *shared.prices.iter().max().unwrap();
    (shared.p1, p2)
}

fn worker(numbers: &[u32], chunks_size: usize, start: &AtomicUsize, mutex: &Mutex<Shared>) {
    let mut prices = vec![0; 130336];
    let mut seen = vec![0; 130321];
    let start = start.fetch_add(chunks_size * LANES, Ordering::Relaxed);
    let s10 = Simd::splat(10);
    let s9 = Simd::splat(9);
    let s19 = Simd::splat(19);
    let s361 = Simd::splat(361);
    let s6859 = Simd::splat(6859);
    let mut p1 = 0;

    for secret0 in numbers[start..(start+LANES*chunks_size).min(numbers.len())].as_chunks::<LANES>().0 {
        seen.fill(0);
        let secret0 = Simd::from_slice(secret0);
        let secret1 = next_secret(secret0);
        let secret2 = next_secret(secret1);
        let secret3 = next_secret(secret2);
        let price0 = secret0 % s10;
        let price1 = secret1 % s10;
        let price2 = secret2 % s10;
        let price3 = secret3 % s10;
        let mut diff1;
        let mut diff2 = s9 + price1 - price0;
        let mut diff3 = s9 + price2 - price1;
        let mut diff4 = s9 + price3 - price2;
        let mut secret = secret3;
        let mut prev_price = price3;

        for _ in 0..1997 {
            secret = next_secret(secret);
            let price = secret % s10;
            (diff1, diff2, diff3, diff4) = (diff2, diff3, diff4, s9 + price - prev_price);
            let index_arr = (diff1 * s6859 + diff2 * s361 + diff3 * s19 + diff4).to_array();
            let price_arr = price.to_array();
            for i in 0..LANES {
                let index = index_arr[i] as usize;
                let b = (seen[index] >> i) & 1;
                seen[index] |= 1 << i;
                prices[index] += price_arr[i] as u16 * (1 - b);
            }
            prev_price = price;
        }
        p1 += secret.reduce_sum() as u64;
    }
    let mut shared = mutex.lock().unwrap();
    shared.p1 += p1;
    unsafe {
        add_vector(&mut shared.prices, &prices)
    }
}

#[inline]
fn next_secret(mut n: Simd<u32, LANES>) -> Simd<u32, LANES> {
    let mask = Simd::splat(16777215);
    n ^= n << 6;
    n &= mask;
    n ^= n >> 5;
    n &= mask;
    n ^= n << 11;
    n & mask
}

unsafe fn add_vector(v1: &mut [u16], v2: &[u16]) {
    let n = v1.len() / 32;
    for i in 0..n {
        let s1 = u16x32::from_slice( unsafe { v1.get_unchecked(32*i..) });
        let s2 = u16x32::from_slice( unsafe { v2.get_unchecked(32*i..) });
        (s1 + s2).copy_to_slice( unsafe { v1.get_unchecked_mut(32*i..) })
    }
}