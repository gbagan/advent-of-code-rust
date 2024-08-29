use crate::util::iter::AOCIter;

fn diff1(s: &str) -> u32 {
    let mut chars = s.bytes();
    let mut count = 0;
    while let Some(c) = chars.next() {
        if c==b'\\' {
            match chars.next() {
                Some(b'x') => {
                    chars.next();
                    chars.next();
                    count += 3;
                }
                Some(_) => count += 1,
                None => panic!("unexpected end of input"),
            }
        }
    }
    count + 2
}

fn diff2(s: &str) -> u32 {
    2 + s.bytes().count_by(|c| c == b'\\' || c == b'"') as u32
}

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += diff1(line);
        p2 += diff2(line);
    }
    Some((p1, p2))
}