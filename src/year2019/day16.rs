use std::simd::prelude::*;

pub fn solve(input: &str) -> (String, String) {
    let input = input.trim().as_bytes();

    let p1 = part1(input);
    let p2 = part2(input);

    for i in 0..50 {
        c_k99_k(i);
    }

    (p1, p2)
}

fn part1(input: &[u8]) -> String {
    let n = input.len();
    let mut signal = Vec::with_capacity(n+1);
    signal.push(0);
    signal.extend(input.iter().map(|&c| (c - b'0') as i32));

    for _ in 0..100 {
        let mut sum = 0;
        for v in &mut signal {
            sum += *v;
            *v = sum;
        }

        let mut sav = 0;
        for i in 0..n {
			(sav, signal[i]) = (signal[i], sav);
		    let mut sum = 0;
			let mut k = i;
            while k < n {
				let k2 = (k + i + 1).min(n);
				sum = signal[k2] - signal[k] - sum;
				k = k2 + (i + 1);
			}

			signal[i] = sav;
            sav = signal[i + 1];
			signal[i + 1] = sum.abs() % 10;
		}
    }
    
    signal[1..9].iter().map(|&c| (c as u8 + b'0') as char).collect()

}

fn part2(input: &[u8]) -> String {
    let len = input.len();
    let signal: Vec<_> = input.iter().map(|&c| (c - b'0') as usize).collect();
    let start: usize = signal[..7].iter().fold(0, |acc, &c| acc * 10 + c);
    let mid = len * 5_000;
    let end = len * 10_000;
    assert!(mid <= start && start < end);

    let mask = mask32x8::from_bitmask(1);
    let mut binomials = u32x8::splat(0);
    let mut result = u32x8::splat(0);

    let mut index = start % len;
    for k in 0..end-start {
        binomials = mask.select(
            u32x8::splat(c_k99_k(k as u32)),
            binomials.rotate_elements_right::<1>(),
        );
        result += binomials * u32x8::splat(signal[index] as u32);
        index = if index == len-1 { 0 } else {index + 1};
    }

    result.to_array().iter().map(|&c| ((c % 10) as u8 + b'0') as char).collect()
}

fn c_k99_k(k: u32) -> u32 {
    let mod5 = match k % 125 {
        0 => 1,
        25 => 4,
        _ => 0,
    };
    let mod2 = (k & !(k+99) == 0) as u32;
    5 * mod2 + 6 * mod5
}