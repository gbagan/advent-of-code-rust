// discrete logarithm
// https://en.wikipedia.org/wiki/Baby-step_giant-step
// https://en.wikipedia.org/wiki/Pohlig%E2%80%93Hellman_algorithm

use crate::util::{iter::*, parser::*};

const MODULO: u64 = 20_201_227;
// phi(MODULO) = MODULO-1 = 2 * 3 * 29 * 116099
const BASE: u64 = 7;

pub fn solve(input: &str) -> (u64, u32) {
    let (card_key, door_key) = input.iter_unsigned().next_tuple().unwrap();
    let exponent = discrete_logarithm(card_key);
    let p1 = mod_pow(door_key, exponent);

    (p1, 0)
}

fn discrete_logarithm(n: u64) -> u64 {
	(CHINESE[0] * log_mod_2(n)
    + CHINESE[1] * log_mod_3(n)
	+ CHINESE[2] * log_mod_29(n)
    + CHINESE[3] * log_mod_116099(n)
    ) % (MODULO - 1)
}

fn log_mod_2(n: u64) -> u64 {
    const EXP: u64 = (MODULO - 1) / 2;
    (mod_pow(n, EXP) != 1) as u64
}

fn log_mod_3(n: u64) -> u64 {
    const EXP: u64 = (MODULO - 1) / 3;
	mod_pow(n, EXP) >> 5 & 3
}

fn log_mod_29(n: u64) -> u64 {
    const EXP: u64 = (MODULO - 1) / 29;

    const fn hash(x: u64) -> usize {
        (((x >> 8) ^ (x >> 18)) % 53) as usize
    } 

    const TABLE: [u8; 53] = {
        const B: u64 = mod_pow(BASE, EXP);
        let mut table = [u8::MAX; 53];
        let mut i = 0;
        let mut x = 1;
        while i < 29 {
            table[hash(x)] = i;
            x = x * B % MODULO;
            i += 1;
        }
        table
    };

	TABLE[hash(mod_pow(n, EXP))] as u64
}

fn log_mod_116099(n: u64) -> u64 {
    const N: u64 = 116_099;
    const EXP: u64 = (MODULO - 1) / N;
    const B: u64 = mod_pow(BASE, EXP);
	const R: u64 = N.isqrt() + 1;
    const C: u64 = mod_pow(modular_inverse(B, MODULO), R);

	const fn hash(n: u64) -> u64 {
        ((19*n) ^ (n>>3) ^ (n>>10)) % 2762
    }

    const EXP_TABLE: [u16; 2762] = {
        let mut table = [0; 2762];
        let mut i = 0;
        let mut z = 1;
	    while i < R {
		    let h = hash(z) as usize;
            table[h] = i as u16;
            i += 1;
		    z = z * B % MODULO;
        }
        table
    };

    const POW_TABLE: [u32; 2762] = {
        let mut table = [0; 2762];
        let mut i = 0;
        let mut z = 1;
	    while i < R {
		    let h = hash(z) as usize;
            table[h] = z as u32;
            i += 1;
		    z = z * B % MODULO;
        }
        table
    };

	let mut y = mod_pow(n, EXP);
	for i in 0..R {
		let h = hash(y) as usize;
		if POW_TABLE[h] as u64 == y {
			return i * R + EXP_TABLE[h] as u64;
		}
		y = y * C % MODULO;
	}

	unreachable!();
}

const CHINESE: [u64; 4] = {
    let n1 = 2;
    let n2 = 3;
    let n3 = 29;
    let n4 = 116_099;
    let q1 = (MODULO - 1) / n1;
    let q2 = (MODULO - 1) / n2;
    let q3 = (MODULO - 1) / n3;
    let q4 = (MODULO - 1) / n4;
    let m1 = modular_inverse(q1, n1);
    let m2 = modular_inverse(q2, n2);
    let m3 = modular_inverse(q3, n3);
    let m4 = modular_inverse(q4, n4);

    [q1 * m1, q2 * m2, q3 * m3, q4 * m4]
};

const fn mod_pow(mut x: u64, mut n: u64) -> u64 {
    let mut p = 1;
    while n > 0 {
        if n & 1 == 1 {
            p = (p * x) % MODULO;
        }
        x = (x * x) % MODULO;
        n >>= 1;
    }
    p
}

const fn modular_inverse(n: u64, m: u64) -> u64 {
    let (mut t, mut t1) = (0, 1);
    let (mut r, mut r1) = (m as i64, n as i64);

    while r1 != 0 {
        let q = r / r1;
        (t, t1) = (t1, t - q * t1);
        (r, r1) = (r1, r - q * r1);
    }

    if t < 0 {
        t += m as i64;
    }
    t as u64
}