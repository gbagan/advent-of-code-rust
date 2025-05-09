use std::ops::{AddAssign, DivAssign, MulAssign};
use num_integer::Integer;
use num_traits::{Num, Signed};

pub fn solve_linear_system<A, const N: usize>(mat: &[[A; N]]) -> Option<Vec<A>>
where A: Ord + Num + Signed + Copy + AddAssign + DivAssign
{
    let mut mat = mat.to_vec();
    let n = mat.len();
    for i in 0..n {
        let j = (i..n).max_by_key(|&j| mat[j][i].abs()).unwrap();
        if mat[j][i].is_zero() {
            return None;
        }
        if i != j {
            mat.swap(i, j);
        }
        let factor = mat[i][i];
        for k in 0..n+1 {
            mat[i][k] /= factor;
        }

        for j in (0..n).filter(|&j| j != i) {
            let factor = -mat[j][i];
            if factor.is_zero() {
                continue;
            }
            for k in 0..n+1 {
                let v= mat[i][k];
                mat[j][k] += factor * v;
            }
        }
    }
    Some(mat.iter().map(|v| v[n]).collect())
}


// Return (g, x, y) such that g is the gcd of a and b
// and ax + by = gcd(a,b)
pub fn extgcd<A>(a: A, b: A) -> (A, A, A) where A: Integer + Signed + Copy {
    if a.is_zero() && b.is_zero() {
        panic!("extgcd(0, 0) is undefined")
    }
    let mut x = A::one();
    let mut y = A::zero();
    let mut x1 = A::zero();
    let mut y1 = A::one();
    let mut a = a;
    let mut b = b;

    while !b.is_zero() {
        let q = a / b;
        (x, x1) = (x1, x - q * x1);
        (y, y1) = (y1, y - q * y1);
        (a, b) = (b, a - q * b);
    }
    (a, x, y)
}

#[test]
fn extgcd_test() {
    let a = 55;
    let b = 80;
    let (g, x, y) = extgcd(a, b);
    assert_eq!(g, 5);
    assert_eq!(a * x + b * y, g);
}

// Given a slice of (ri, mi)
// returns a tuple (q, m) where {q + j m | j in Z} is the set of solutions
// of the equations x = ri (mod mi)
// It is not necessary that all mi are pairwise coprime
// returns Nothing if there is no solution
pub fn chinese_remainder<A>(pairs: &[(A, A)]) -> Option<(A, A)> 
    where A: Integer + Signed + Copy
{
    let mut a = A::zero();
    let mut n = A::one();
    for &(b, m) in pairs {
        let (g, u, v) = extgcd(m, n);
        if !((a - b) % g).is_zero() {
            return None;
        }
        let x = (a * u * m + b * v * n) / g;
        n = (n * m) / g;
        a = x % n;
    }
    Some((a.mod_floor(&n), n))
}


// same as chinese_remainder but only works if all mi are pairwise coprime
// faster than chinese_remainder if the mi are small
pub fn chinese_remainder2<A>(pairs: &[(A, A)]) -> Option<(A, A)> 
    where A: Integer + Signed + Copy + AddAssign + MulAssign
{
    if pairs.is_empty() {
        return None
    }
    let (mut a, mut n) = pairs[0];
    a = a.mod_floor(&n);
    for &(b, m) in &pairs[1..] {
        let b = b.mod_floor(&m);
        while a % m != b {
            a += n;
        }
        n *= m;
    }

    Some((a, n))
}
