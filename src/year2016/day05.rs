use md5::{Md5, Digest};
use std::sync::Mutex;
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub struct Shared {
    done: AtomicBool,
    counter: AtomicU32,
}

struct Exclusive {
    hashes: Vec<(u32, u8, u8)>,
    mask: u8,
}

pub fn solve(input: &str) -> Option<(String, String)> {
    let input = input.trim();
    let shared = Shared {
        done: AtomicBool::new(false),
        counter: AtomicU32::new(100),
    };

    let mutex = Mutex::new(Exclusive { hashes: vec![], mask: 0 });

    for i in 1..100 {
        let mut buffer= format_string(input, i);
        check(&mut buffer, i, &shared, &mutex);
    }

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(input, &shared, &mutex));
        }
    });

    let mut hashes = mutex.into_inner().unwrap().hashes;
    hashes.sort_unstable();

    let p1 = hashes.iter().take(8).fold(0, |acc, (_, n, _)| (acc << 4) | *n as u32);
    let p1 = format!("{p1:08x}");

    let mut p2 = 0u32;
    let mut mask = 0u32;
    for &(_, b1, b2) in &hashes {
        if b1 < 8 && mask & (1 << b1) == 0 {
            mask |= 1 << b1;
            p2 |= (b2 as u32) << (4 * (7 - b1));
        }
    }
    let p2 = format!("{p2:08x}");

    Some((p1, p2))
}

fn format_string(input: &str, n: u32) -> Vec<u8> {
    format!("{input}{n}").as_bytes().to_vec()
}


fn worker(input: &str, shared: &Shared, mutex: &Mutex<Exclusive>) {
    while !shared.done.load(Ordering::Relaxed) {
        let offset = shared.counter.fetch_add(1000, Ordering::Relaxed);
        let mut buffer = format_string(input, offset);
        let len = buffer.len();

        for i in 0..1000 {
            buffer[len - 3] = b'0' + (i / 100) as u8;
            buffer[len - 2] = b'0' + ((i / 10) % 10) as u8;
            buffer[len - 1] = b'0' + (i % 10) as u8;
            check(&mut buffer, offset + i, shared, mutex);
        }
    }
}

fn check(buffer: &mut [u8], i: u32, shared: &Shared, mutex: &Mutex<Exclusive>) {
    let mut hasher = Md5::new();
    hasher.update(buffer);
    let hash = hasher.finalize(); 
    
    if hash[0] | hash[1] | (hash[2] & 240) == 0 {
        let mut exclusive = mutex.lock().unwrap();
        exclusive.hashes.push((i, hash[2], hash[3] >> 4));
        if hash[2] <= 7 {
            exclusive.mask |= 1 << hash[2];

            if exclusive.mask == 0xff {
                shared.done.store(true, Ordering::Relaxed);
            }
        }
    }
}