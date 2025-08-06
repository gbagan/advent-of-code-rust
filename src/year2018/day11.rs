use crate::util::parser::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub fn solve(input: &str) -> (String, String) {
    let serial_number = input.try_unsigned().unwrap();
    let table = build_table(serial_number);
    let counter = AtomicUsize::new(1);
    let n = thread::available_parallelism().unwrap().get();
    let mut results = vec![(None, (0, 0, 0, i32::MIN)); n];

    thread::scope(|scope| {
        for res in &mut results {
            scope.spawn(|| worker(&table, &counter, res));
        }
    });

    let (p1, p2) = results
        .iter()
        .fold((None, (0, 0, 0, i32::MIN)), |(a1, a2) , &(b1, b2)| {
            (a1.or(b1), if a2.3 > b2.3 { a2 } else { b2 })
        });

    let p1 = p1.unwrap();
    let p1 = format!("{},{}", p1.0, p1.1);
    let p2 = format!("{},{},{}", p2.0, p2.1, p2.2);

    (p1, p2)
}

fn worker(table: &[i32], counter: &AtomicUsize, result: &mut (Option<(usize, usize)>, (usize, usize, usize, i32))) { 
    let mut p1 = None;
    let mut p2 = (0, 0, 0, i32::MIN);
    while let size = counter.fetch_add(1, Ordering::Relaxed) && size <= 300 {  
        let (x, y, power) = largest_square(&table, size);
        if size == 3 {
            p1 = Some((x, y));
        }
        if power > p2.3 {
            p2 = (x, y, size, power)
        }
    }
    *result = (p1, p2);
}

#[inline]
fn power_level(serial_number: i32, x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let level = (rack_id * y + serial_number) * rack_id;
    (level / 100) % 10 - 5
}

fn build_table(serial_number: i32) -> Vec<i32> {
    let mut table = vec![0; 301*301];
    for y in 1..301 {
        for x in 1..301 {
            let power_level = power_level(serial_number, x, y);
            let index = (301 * y + x) as usize;
            table[index] = power_level + table[index-1] + table[index-301] - table[index-302];
        }
    }
    table
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

    /*
    // util::parallel::*
    let (p1, p2) = (1..301)
        .into_par_iter()
        .map(|size| {
            let (x, y, power) = largest_square(&table, size);
            ((size == 3).then_some((x, y)), (x, y, size, power))
        }).reduce2(|| (None, (0, 0, 0, i32::MIN)), |&(a1, a2) , &(b1, b2)| {
            (a1.or(b1), if a2.3 > b2.3 { a2 } else { b2 })
        });
    */