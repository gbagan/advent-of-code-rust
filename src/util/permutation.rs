#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Permutation<const N: usize>{ pub indices: [usize; N] }

impl<const N: usize> Permutation<N> {
    pub fn one() -> Self {
        Permutation { indices: std::array::from_fn(|i| i) }
    }

    pub fn apply<A: Clone>(&self, arr: &[A]) -> [A; N] {
        self.indices.map(|idx| arr[idx].clone())
    }

    pub fn swap(i: usize, j: usize) -> Self {
        let mut perm = Self::one();
        perm.indices[i] = j;
        perm.indices[j] = i;
        perm
    }

    pub fn inv(&self) -> Self {
        let mut indices = [0; N];
        for (i, j) in self.indices.iter().enumerate() {
            indices[*j] = i;
        }
        Permutation { indices }
    }

    pub fn right_compose(&self, other: &Self) -> Self {
        Self { indices: other.apply(&self.indices) }
    }
}

#[test]
fn apply_test() {
    let arr = ['a', 'b', 'c'];
    let perm = Permutation { indices:  [0, 2, 1] };
    assert_eq!(perm.apply(&arr), ['a', 'c', 'b']);
}

#[test]
fn shr_test() {
    let perm1 = Permutation { indices:  [0, 2, 1] };
    let perm2 = Permutation { indices:  [1, 2, 0] };
    assert_eq!(perm1.right_compose(&perm2), Permutation { indices:  [2, 1, 0] });
}