// discrete logarithm
// https://en.wikipedia.org/wiki/Baby-step_giant-step

use ahash::{HashMap, HashMapExt};

use crate::util::{iter::*, math::*, parser::*};


pub fn solve(input: &str) -> (u64, u32) {
    let (card_key, door_key) = input.iter_unsigned().next_tuple().unwrap();
    const MODULO: u64 = 20_201_227;
    const BASE: u64 = 7;
    let exponent = discrete_logarithm(BASE, MODULO, card_key);
    let p1 = door_key.mod_pow(exponent as usize, MODULO);

    (p1, 0)
}

#[inline(always)]
fn discrete_logarithm(base: u64, modulo: u64, beta: u64) -> u64 {
    let m = modulo.isqrt() + 1;
    let mut table = HashMap::with_capacity(m as usize);
    let mut b_i = 1;
    for i in 0..m {
        table.insert(b_i, i);
        b_i = (base * b_i) % modulo;
    }
    let c = base.mod_pow((m * (modulo - 2)) as usize, modulo);

    let mut y = beta;
    for i in 0..m {
        if let Some(&j) = table.get(&y) {
            return i * m + j;
        }
       y = (c * y) % modulo;
    }


    unreachable!();
}

#[test]
fn discrete_logarithm_test() {
    assert_eq!(discrete_logarithm(2, 5, 3), 3);
    assert_eq!(discrete_logarithm(3, 7, 4), 4);
}