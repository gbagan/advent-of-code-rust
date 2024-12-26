use anyhow::*;
use crate::util::parser::*;

pub fn solve(input: &str) -> Result<(usize, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut valid = true;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for (color, n) in line.iter_lowercase().zip(line.iter_unsigned()).skip(1) {
            match color {
                "red" => {
                    valid = valid && n <= 12;
                    r = r.max(n);
                }
                "green" => {
                    valid = valid && n <= 13;
                    g = g.max(n);
                }
                _ => {
                    valid = valid && n <= 14;
                    b = b.max(n);
                }
            }
        }

        if valid {
            p1 += i+1;
        }

        p2 += r * g * b;

    }
    Ok((p1, p2))
}
