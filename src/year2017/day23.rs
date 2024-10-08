use anyhow::*;
use itertools::Itertools;
use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> Result<(i32, usize)> {
    let (x, _, _, _, y, z, t) = input.iter_signed::<i32>().next_tuple().context("Not enough integers")?;
    let p1 = (x-2) * (x-2);
    
    let b = x * y - z;
    let c = b - t;
    let p2 = (b..c+1).step_by(17).count_if(|&x| !is_prime(x));
    Ok((p1, p2))
}

fn is_prime(n: i32) -> bool {
    n >= 2 && (2..(n as f64).sqrt() as i32).all(|i| n % i != 0)
}
