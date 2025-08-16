use std::ops::{AddAssign, SubAssign, DivAssign, MulAssign, ShlAssign, ShrAssign};
use num_traits::{Euclid, Num, PrimInt, Signed};

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
pub fn extgcd<A>(a: A, b: A) -> (A, A, A) where A: Num + Signed + Copy {
    assert!(!a.is_zero() || !b.is_zero(), "extgcd(0, 0) is undefined");
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

fn binary_gcd<A>(mut a: A, mut b: A) -> A where  A: PrimInt + SubAssign + ShlAssign<u32> + ShrAssign<u32> {
    if a.is_zero() { return b };
    if b.is_zero() { return a };

    let shift = (a | b).trailing_zeros();
    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }
    a <<= shift;
    a
}


pub trait MathInteger {
    fn gcd(self, other: Self) -> Self;

    fn lcm(self, other: Self) -> Self where Self: PrimInt + ShlAssign<u32> + ShrAssign<u32> {
        let gcd = self.gcd(other);
        self * (other / gcd)
    }
    
    fn modular_inverse(self, m: Self) -> Self where Self: Num + Signed + Copy  {
        let (mut t, mut t1) = (Self::zero(), Self::one());
        let (mut r, mut r1) = (m, self);

        while !r1.is_zero() {
            let q = r / r1;
            (t, t1) = (t1, t - q * t1);
            (r, r1) = (r1, r - q * r1);
        }

        if t.is_negative() {
            t = t + m;
        }
        t
    }
}

macro_rules! unsigned_integer_impl {
    ($($t:ty)*) => ($(
        impl MathInteger for $t {
            fn gcd(self, other: Self) -> Self {
                binary_gcd(self, other)
            }
        }
    )*)
}

macro_rules! signed_integer_impl {
    ($($t:ty)*) => ($(
        impl MathInteger for $t {
            fn gcd(self, other: Self) -> Self {
                let a = self.abs();
                let b = other.abs();
                binary_gcd(a, b)
            }
        }
    )*)
}

unsigned_integer_impl!(u8 u16 u64 u128 usize);
signed_integer_impl!(i8 i16 i32 i64 i128);

#[test]
fn gcd_test() {
    assert_eq!(24.gcd(32), 8);
}


#[test]
fn modular_inverse_test() {
    assert_eq!(3i64.modular_inverse(11), 4);
}



// Given a slice of (ri, mi)
// returns a tuple (q, m) where {q + j m | j in Z} is the set of solutions
// of the equations x = ri (mod mi)
// It is not necessary that all mi are pairwise coprime
// returns Nothing if there is no solution
pub fn chinese_remainder<A>(pairs: &[(A, A)]) -> Option<(A, A)> 
    where A: Num + Euclid + Signed + Copy
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
    Some((a.rem_euclid(&n), n))
}


// same as chinese_remainder but only works if all mi are pairwise coprime
// faster than chinese_remainder if the mi are small
pub fn chinese_remainder2<A>(pairs: &[(A, A)]) -> Option<(A, A)> 
    where A: Num + Euclid + Signed + Copy + AddAssign + MulAssign
{
    if pairs.is_empty() {
        return None
    }
    let (mut a, mut n) = pairs[0];
    a = a.rem_euclid(&n);
    for &(b, m) in &pairs[1..] {
        let b = b.rem_euclid(&m);
        while a % m != b {
            a += n;
        }
        n *= m;
    }

    Some((a, n))
}
