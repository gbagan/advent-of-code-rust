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