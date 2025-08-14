use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::simd::{LaneCount, SupportedLaneCount};
use crate::util::md5::simd_hash;

pub struct Atomics {
    done: AtomicBool,
    counter: AtomicU32,
    p1: AtomicU32,
    p2: AtomicU32,
}

pub fn solve(input: &str) -> (u32, u32) {
    let input = input.trim();
    let shared = Atomics {
        done: AtomicBool::new(false),
        counter: AtomicU32::new(1000),
        p1: AtomicU32::new(u32::MAX),
        p2: AtomicU32::new(u32::MAX),
    };

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(input, &shared));
        }
    });

    let p1 = shared.p1.load(Ordering::Relaxed);
    let p2 = shared.p2.load(Ordering::Relaxed);
    (p1, p2)
}

fn worker(input: &str, shared: &Atomics) {
    while !shared.done.load(Ordering::Relaxed) {
        let mut counter = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let string = format!("{input}{counter}");
        let len = string.len();
        let mut buffer = [0; 64];
        buffer[0..len].copy_from_slice(string.as_bytes());
        let mut buffers = [buffer; 16];
        
        let mut digits = (b'0', b'0', b'0');

        for _ in 0..992/16 {
            check_hash::<16>(&mut buffers, len, counter, &mut digits, shared);
            counter += 16;
        }
        check_hash::<8>(&mut buffers, len, counter, &mut digits, shared);
    }
}

fn check_hash<const N: usize>(
    buffers: &mut [[u8; 64]],
    len: usize,
    counter: u32,
    digits: &mut (u8, u8, u8),
    shared: &Atomics
) where LaneCount<N>: SupportedLaneCount {
    for buffer in buffers.iter_mut().take(N) {
        buffer[len - 3] = digits.0;
        buffer[len - 2] = digits.1;
        buffer[len - 1] = digits.2;
        increment(digits);
    }
    let result = simd_hash::<N>(buffers, len).0;

    for (i, res) in result.iter().enumerate() {
        if res & 0xffffff00 == 0 {
            shared.p2.fetch_min(counter + i as u32, Ordering::Relaxed);
            shared.done.store(true, Ordering::Relaxed);
        } else if result[i] & 0xfffff000 == 0 {
            shared.p1.fetch_min(counter + i as u32, Ordering::Relaxed);
        }       
    }
}

fn increment((a, b, c): &mut (u8, u8, u8)) {
    *c += 1;
    if * c > b'9' {
        *c = b'0';
        *b += 1;
        if *b > b'9' {
            *b = b'0';
            *a += 1;
        }
    }

}