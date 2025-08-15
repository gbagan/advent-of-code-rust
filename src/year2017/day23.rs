use crate::util::{iter::*, parser::*};

pub fn solve(input: &str) -> (i32, usize) {
    let (x, _, _, _, y, z, t) = input.iter_signed::<i32>().next_tuple().unwrap();
    let p1 = (x-2) * (x-2);
    
    let b = x * y - z;
    let c = b - t;
    let p2 = (b..c+1).step_by(17).filter(|&x| !is_prime(x)).count();
    (p1, p2)
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }

    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}