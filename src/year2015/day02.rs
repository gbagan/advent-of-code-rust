use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    for [l, h, w] in input.iter_unsigned::<u32>().array_chunks() {
        let (x, y, z) = (l*w, l*h, w*h);
        p1 += 2 * (x + y  + z) + x.min(y).min(z);
        p2 += l * h * w + 2 * (l+w).min(l+h).min(w+h);
    }
    (p1, p2)
}