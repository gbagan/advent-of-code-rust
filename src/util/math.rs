use std::ops::{AddAssign, DivAssign};
use num_traits::{Num, Signed};

pub fn solve_linear_system<A>(mat: &Vec<Vec<A>>) -> Option<Vec<A>>
where A: Ord + Num + Signed + Copy + AddAssign + DivAssign
{
    let mut mat = mat.clone();
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
        for k in 0..=n {
            mat[i][k] /= factor;
        }

        for j in (0..n).filter(|&j| j != i) {
            let factor = -mat[j][i];
            if factor.is_zero() {
                continue;
            }
            for k in 0..=n {
                let v= mat[i][k];
                mat[j][k] += factor * v;
            }
        }
    }
    Some(mat.iter().map(|v| v[n]).collect())
}
