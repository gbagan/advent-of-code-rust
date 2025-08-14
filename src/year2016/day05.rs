use std::sync::Mutex;
use std::thread;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::simd::{LaneCount, SupportedLaneCount};
use crate::util::md5::simd_hash;

pub struct Atomics {
    done: AtomicBool,
    counter: AtomicU32,
}

struct Exclusive {
    hashes: Vec<(u32, u8, u8)>,
    mask: u8,
}

// todo: manage the first 1000

pub fn solve(input: &str) -> (String, String) {
    let input = input.trim();
    let atomics = Atomics {
        done: AtomicBool::new(false),
        counter: AtomicU32::new(1000),
    };

    let mutex = Mutex::new(Exclusive { hashes: vec![], mask: 0 });

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(input, &atomics, &mutex));
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

    (p1, p2)
}

fn worker(input: &str, atomics: &Atomics, mutex: &Mutex<Exclusive>) {
    while !atomics.done.load(Ordering::Relaxed) {        
        let mut counter = atomics.counter.fetch_add(1000, Ordering::Relaxed);
        let string = format!("{input}{counter}");
        let len = string.len();

        let mut buffer = [0; 64];
        buffer[0..len].copy_from_slice(string.as_bytes());
        let mut buffers = [buffer; 16];

        let mut digits = (b'0', b'0', b'0');

        for _ in 0..992/16 {
            for buffer in buffers.iter_mut() {
                buffer[len - 3] = digits.0;
                buffer[len - 2] = digits.1;
                buffer[len - 1] = digits.2;
                increment(&mut digits);
            }

            check::<16>(&mut buffers, len, counter, atomics, mutex);
            counter += 16;
        }
        for buffer in buffers.iter_mut().take(8) {
            buffer[len - 3] = digits.0;
            buffer[len - 2] = digits.1;
            buffer[len - 1] = digits.2;
            increment(&mut digits);
        }
        check::<8>(&mut buffers, len, counter, atomics, mutex);
    }
}

fn check<const N: usize>(buffers: &mut [[u8; 64]], len: usize, counter: u32, shared: &Atomics, mutex: &Mutex<Exclusive>)
where LaneCount<N>: SupportedLaneCount {
    let result = simd_hash::<N>(buffers, len).0;

    for (i, res) in result.iter().enumerate() {
        if res & 0xfffff000 == 0 {
            let mut exclusive = mutex.lock().unwrap();
            let sixth = ((res >> 8) & 0xf) as u8;
            let seventh = ((res >> 4) & 0xf) as u8;
            exclusive.hashes.push((counter + i as u32, sixth, seventh));
            if sixth <= 7 {
                exclusive.mask |= 1 << sixth;

                if exclusive.mask == 0xff {
                    shared.done.store(true, Ordering::Relaxed);
                }
            }
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