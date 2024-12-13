use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(i64, i64)> {
    let mut p1 = 0;
    let mut p2 = 0;

    for [ax, ay, bx, by, px, py] in input.iter_unsigned().array_chunks() {
        p1 += tokens(ax, ay, bx, by, px, py);
        p2 += tokens(ax, ay, bx, by, px+10_000_000_000_000, py+10_000_000_000_000);
    }
    Ok((p1, p2))
}

fn tokens(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> i64 {
    let det1 = bx * py - by * px;
    let det2 = bx * ay - ax * by;
    if det1 % det2 != 0 {
        return 0;
    }
    let d1 = det1 / det2;
    let p2 = px - d1 * ax;
    if p2 % bx != 0 {
        return 0;
    }
    let d2 = p2 / bx;
    if d1 < 0 || d2 < 0 {
        return 0;
    }
    3 * d1 + d2
}