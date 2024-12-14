use anyhow::*;
use std::sync::Mutex;
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::simd::{LaneCount, SupportedLaneCount};
use crate::util::md5::multiple_hash;

pub struct Shared {
    done: AtomicBool,
    counter: AtomicU32,
}

struct Exclusive {
    hashes: Vec<(u32, u8, u8)>,
    mask: u8,
}

pub fn solve(input: &str) -> Result<(String, String)> {
    let input = input.trim();
    let shared = Shared {
        done: AtomicBool::new(false),
        counter: AtomicU32::new(100),
    };

    let mutex = Mutex::new(Exclusive { hashes: vec![], mask: 0 });

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(input, &shared, &mutex));
        }
    });

    let mut hashes = mutex.into_inner().unwrap().hashes;
    hashes.sort_unstable_by_key(|x| x.0);

    let p1 = hashes.iter().take(8).fold(0, |acc, &(_, sixth, _)| acc << 4 | sixth as u32);
    let p1 = format!("{p1:08x}");

    let mut p2 = 0u32;
    let mut mask = 0u32;
    for &(_, sixth, seventh) in &hashes {
        if sixth < 8 && mask & (1 << sixth) == 0 {
            mask |= 1 << sixth;
            p2 |= (seventh as u32) << (4 * (7 - sixth));
        }
    }
    let p2 = format!("{p2:08x}");

    Ok((p1, p2))
}

fn worker(input: &str, shared: &Shared, mutex: &Mutex<Exclusive>) {
    while !shared.done.load(Ordering::Relaxed) {
        let counter = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let string = format!("{input}{counter}");
        let len = string.len();
        let mut buffer = [0; 64];
        buffer[0..len].copy_from_slice(string.as_bytes());
        let mut buffers = [buffer; 16];

        for i in (0..992).step_by(16) {
            check::<16>(&mut buffers, len, counter, i, shared, mutex);
        }
        check::<8>(&mut buffers, len, counter, 992, shared, mutex);
    }
}

fn check<const N: usize>(buffers: &mut [[u8; 64]], len: usize, counter: u32, offset: u32, shared: &Shared, mutex: &Mutex<Exclusive>)
where LaneCount<N>: SupportedLaneCount {
    for i in 0..N {
        let n = offset + i as u32;
        buffers[i][len - 3] = b'0' + (n / 100) as u8;
        buffers[i][len - 2] = b'0' + ((n / 10) % 10) as u8;
        buffers[i][len - 1] = b'0' + (n % 10) as u8;
    }
    let result = multiple_hash::<N>(buffers, len).0;

    for i in 0..N {
        if result[i] & 0xfffff000 == 0 {
            let mut exclusive = mutex.lock().unwrap();
            let sixth = ((result[i] >> 8) & 0xf) as u8;
            let seventh = ((result[i] >> 4) & 0xf) as u8;
            exclusive.hashes.push((counter + offset + i as u32, sixth, seventh));
            if sixth <= 7 {
                exclusive.mask |= 1 << sixth;

                if exclusive.mask == 0xff {
                    shared.done.store(true, Ordering::Relaxed);
                }
            }
        }
    }
}