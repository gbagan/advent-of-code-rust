use std::ops::{Shr,ShrAssign};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permutation { pub indices: Vec<usize> }

impl Permutation {
    pub fn one(n: usize) -> Self {
        Permutation { indices: (0..n).collect() }
    }

    pub fn apply<A: Clone>(&self, arr: &[A]) -> Vec<A> {
        self.indices.iter().map(|&idx| arr[idx].clone()).collect()
    }

    pub fn swap(n: usize, i: usize, j: usize) -> Self {
        let mut perm = Self::one(n);
        perm.indices[i] = j;
        perm.indices[j] = i;
        perm
    }

    pub fn inv(&self) -> Self {
        let mut indices = vec![0; self.indices.len()];
        for (i, j) in self.indices.iter().enumerate() {
            indices[*j] = i;
        }
        Permutation { indices }
    }
}

impl Shr<&Permutation> for Permutation {
    type Output = Self;

    fn shr(self, other: &Permutation) -> Self::Output {
        Permutation { indices: other.apply(&self.indices) }
    }
}

impl Shr<&Permutation> for &Permutation {
    type Output = Permutation;

    fn shr(self, other: &Permutation) -> Self::Output {
        Permutation { indices: other.apply(&self.indices) }
    }
}

// todo: implements apply_in_place
impl ShrAssign<&Permutation> for Permutation {
    fn shr_assign(&mut self, other: &Permutation) {
        self.indices = other.apply(&self.indices)
    }
}

#[test]
fn apply_test() {
    let arr = ['a', 'b', 'c'];
    let perm = Permutation { indices:  vec!(0, 2, 1) };
    assert_eq!(perm.apply(&arr), vec!('a', 'c', 'b'));
}

#[test]
fn shr_test() {
    let perm1 = Permutation { indices:  vec!(0, 2, 1) };
    let perm2 = Permutation { indices:  vec!(1, 2, 0) };
    assert_eq!(perm1 >> &perm2, Permutation { indices:  vec!(2, 1, 0) });
}