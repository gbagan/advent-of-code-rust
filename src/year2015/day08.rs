pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += diff1(line);
        p2 += diff2(line);
    }
    (p1, p2)
}

fn diff1(line: &str) -> u32 {
    let mut bytes = line.bytes();
    let mut count = 0;
    while let Some(c) = bytes.next() {
        if c==b'\\' {
            match bytes.next() {
                Some(b'x') => {
                    bytes.next();
                    bytes.next();
                    count += 3;
                }
                Some(_) => count += 1,
                None => panic!(),
            }
        }
    }
    count + 2
}

fn diff2(s: &str) -> u32 {
    2 + s.bytes().filter(|&c| c == b'\\' || c == b'"').count() as u32
}