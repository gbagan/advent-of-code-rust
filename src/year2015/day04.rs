use anyhow::*;
use md5::{Md5, Digest};
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

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
        let offset = shared.counter.fetch_add(1000, Ordering::Relaxed);
        for i in offset..offset+1000 {
            let mut hasher = Md5::new();
            hasher.update(format!("{input}{i}"));
            let hash = hasher.finalize(); 
            if hash[0] | hash[1] | (hash[2] & 240) == 0 {
                shared.p1.fetch_min(i, Ordering::Relaxed);
                if hash[2] == 0 {
                    shared.p2.fetch_min(i, Ordering::Relaxed);
                    shared.done.store(true, Ordering::Relaxed);
                }
            }
        }
    }
}