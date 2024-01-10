use std::ops::Mul;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permutation { pub indices: Vec<usize> }

impl Permutation {
    pub fn one(n: usize) -> Self {
        Permutation { indices: (0..n).collect() }
    }

    pub fn apply<A: Clone>(&self, arr: &[A]) -> Vec<A> {
        self.indices.iter().map(|&idx| arr[idx].clone()).collect()
    }
}

impl Mul<&Permutation> for Permutation {
    type Output = Self;

    fn mul(self, other: &Permutation) -> Self::Output {
        Permutation { indices: other.apply(&self.indices) }
    }
}

#[test]
fn apply_test() {
    let arr = ['a', 'b', 'c'];
    let perm = Permutation { indices:  vec!(0, 2, 1) };
    assert_eq!(perm.apply(&arr), vec!('a', 'c', 'b'));
}

#[test]
fn mul_test() {
    let perm1 = Permutation { indices:  vec!(0, 2, 1) };
    let perm2 = Permutation { indices:  vec!(1, 2, 0) };
    assert_eq!(perm2 * &perm1, Permutation { indices:  vec!(1, 0, 2) });
}