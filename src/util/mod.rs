use anyhow::*;
use std::fmt::Debug;
use std::mem;
use std::str::pattern::Pattern;
use core::hash::Hash;
use itertools::{Itertools, iterate};
use crate::util::iter::AOCIter;

pub mod boxes;
pub mod coord;
pub mod graph;
pub mod grid;
pub mod heap;
pub mod iter;
pub mod knothash;
pub mod math;
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
    let n = power(|&x, &y| x* y, 2, 6);
    assert_eq!(n, 64);
}

pub trait TryParseLines {
    fn try_parse_lines_and_collect<A, F>(self, f: F) -> Result<Vec<A>> 
        where Self: Sized, F: Fn(Self) -> Result<A>;

    fn try_split_once<P>(self, delim: P) -> Result<(Self, Self)>
        where Self: Sized, P: Pattern + Debug + Copy;
}

impl TryParseLines for &str {
    #[inline]
    fn try_parse_lines_and_collect<A, F>(self, f: F) -> Result<Vec<A>>
        where Self: Sized, F: Fn(Self) -> Result<A> {
        self
            .lines()
            .map(|line| f(line).with_context(|| format!("Parse error on line: '{line}'")))
            .try_collect()
    }

    fn try_split_once<P>(self, delim: P) -> Result<(Self, Self)>
        where Self: Sized, P: Pattern + Debug + Copy
    {
        self.split_once(delim).with_context(|| format!("No delimiter '{delim:?}' found in string '{self}'"))
    }
}   
