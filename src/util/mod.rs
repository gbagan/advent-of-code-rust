pub mod bits;
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
        let _ = std::mem::replace(&mut y, z);
    }
    y
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
        if !i.is_multiple_of(2) {
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

pub fn foreach_permutation<T>(arr: &mut[T], mut callback: impl FnMut(&[T])) {
    let n = arr.len();
    let mut c = vec![0; n];
    let mut i = 1;

    callback(arr);

    while i < n {
        if c[i] < i {
            if i & 1 == 0 {
                arr.swap(0, i);
            } else {
                arr.swap(c[i], i);
            }
            callback(arr);
            c[i] += 1;
            i = 1;
        } else {
            c[i] = 0;
            i += 1
        }
    }
}