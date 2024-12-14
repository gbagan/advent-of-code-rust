use std::mem;
use core::hash::Hash;
use itertools::iterate;
use crate::util::iter::*;

pub mod boxes;
pub mod coord;
pub mod graph;
pub mod grid;
pub mod heap;
pub mod iter;
pub mod knothash;
pub mod math;
pub mod md5;
pub mod parallel;
pub mod parser;
pub mod permutation;
pub mod range;

pub fn times<A, F>(n: usize, x: A, f: F) -> A
    where
    F: Fn(&A) -> A
{
    let mut y = x;
    for _ in 0..n {
        let z = f(&y);
        let _ = mem::replace(&mut y, z);
    }
    y
}

pub fn many_times<A, F>(n: usize, x: A, f: F) -> A
    where 
    A: Eq + Hash,
    F: Fn(&A) -> A
{
    let (i, j, y) = iterate(x, &f).find_duplicate().unwrap();
    let period = j - i;
    let remaining = n - j;
    times(remaining % period, y, f)
}

pub fn many_times_on<A, B, F, G>(n: usize, x: A, f :F, g: G) -> A
    where 
    B: Eq + Hash,
    F: Fn(&A) -> B,
    G: Fn(&A) -> A
{
    let (i, j, y) = iterate(x, &g).find_duplicate_on(f).unwrap();
    let period = j - i;
    let remaining = n - j;
    times(remaining % period, y, g)
}

pub fn power<A,F>(mul: F, x: A, n: usize) -> A
    where
        A: Clone,
        F: Fn(&A, &A) -> A,
{
    let mut i = n-1;
    let mut p = x.clone();
    let mut x = x;
    while i > 0 {
        if i % 2 > 0 {
            p = mul(&p, &x);
        }
        x = mul(&x, &x);
        i /= 2;
    }
    p
}

#[test]
fn power_test() {
    let n = power(|&x, &y| x * y, 2, 6);
    assert_eq!(n, 64);
}