use anyhow::*;
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::simd::{LaneCount, SupportedLaneCount};
use crate::util::md5::multiple_hash;

pub struct Shared {
    done: AtomicBool,
    counter: AtomicU32,
    p1: AtomicU32,
    p2: AtomicU32,
}

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let input = input.trim();
    let shared = Shared {
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
    Ok((p1, p2))
}

fn worker(input: &str, shared: &Shared) {
    while !shared.done.load(Ordering::Relaxed) {
        let counter = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let string = format!("{input}{counter}");
        let len = string.len();
        let mut buffer = [0; 64];
        buffer[0..len].copy_from_slice(string.as_bytes());
        let mut buffers = [buffer; 16];

        for i in (0..992).step_by(16) {
            check_hash::<16>(&mut buffers, len, counter, i, shared);
        }
        check_hash::<8>(&mut buffers, len, counter, 992, shared);
    }
}

fn check_hash<const N: usize>(
    buffers: &mut [[u8; 64]],
    len: usize,
    counter: u32,
    offset: u32,
    shared: &Shared
) where LaneCount<N>: SupportedLaneCount {
    for i in 0..N {
        let n = offset + i as u32;
        buffers[i][len - 3] = b'0' + (n / 100) as u8;
        buffers[i][len - 2] = b'0' + ((n / 10) % 10) as u8;
        buffers[i][len - 1] = b'0' + (n % 10) as u8;
    }
    let result = multiple_hash::<N>(buffers, len).0;

    for i in 0..N {
        if result[i] & 0xffffff00 == 0 {
            shared.p2.fetch_min(counter + offset + i as u32, Ordering::Relaxed);
            shared.done.store(true, Ordering::Relaxed);
        } else if result[i] & 0xfffff000 == 0 {
            shared.p1.fetch_min(counter + offset + i as u32, Ordering::Relaxed);
        }       
    }
}