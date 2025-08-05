use crate::util::parser::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::thread;

pub fn solve(input: &str) -> (String, String) {
    let serial_number = input.try_unsigned().unwrap();
    
    let mut table = vec![0; 301*301];
    for y in 1..301 {
        for x in 1..301 {
            let power_level = power_level(serial_number, x, y);
            let index = (301 * y + x) as usize;
            table[index] = power_level + table[index-1] + table[index-301] - table[index-302];
        }
    }

    let mutex = Mutex::new(((0, 0), (0, 0, 0, i32::MIN)));
    let counter = AtomicUsize::new(1);

    thread::scope(|scope| {
        for _ in 0..thread::available_parallelism().unwrap().get() {
            scope.spawn(|| worker(&table, &counter, &mutex));
        }
    });

    let (p1, p2) = mutex.into_inner().unwrap();
    let p1 = format!("{},{}", p1.0, p1.1);
    let p2 = format!("{},{},{}", p2.0, p2.1, p2.2);

    (p1, p2)
}

#[inline]
fn power_level(serial_number: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let level = (rack_id * y + serial_number) * rack_id;
    (level / 100) % 10 - 5
}

#[inline]
fn square_power(table: &[i32], x: usize, y: usize, size: usize) -> i32 {
    let index = 301 * y + x;
    table[index + size * 302] - table[index + size] - table[index + 301 * size] + table[index]
}

fn largest_square(table: &[i32], size: usize) -> (usize, usize, i32) {
    let mut max_power = i32::MIN;
    let mut xmax = 0;
    let mut ymax = 0;

    for y in 0..301-size {
        for x in 0..301-size {
            let power = square_power(table, x, y, size);
            if power > max_power {
                max_power = power;
                xmax = x+1;
                ymax = y+1;
            }
        }
    }

    (xmax, ymax, max_power)
}

fn worker(table: &[i32], counter: &AtomicUsize, mutex: &Mutex<((usize, usize), (usize, usize, usize, i32))>) {
    loop {
        let size = counter.fetch_add(1, Ordering::Relaxed);
        if size > 300 {
            break;
        }

        let (x, y, power) = largest_square(table, size);

        let mut solution = mutex.lock().unwrap();
        if size == 3 {
            solution.0 = (x, y);
        }
        if power > solution.1.3 {
            solution.1 = (x, y, size, power);
        }
    }
}

/*
&[
    -2984,
    -2998,
    -3003,
    -2999,
    -3004,
    -3000,
    -3025,
    -3019,
    -3026,
    -3026,
    -3023,
    -3017,
    -3022,
    -3028,
    -3028,
    -3034,
    -3249,
    -3233,
    -3230,
    -3380,
]
    */