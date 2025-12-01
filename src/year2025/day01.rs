pub fn solve(input: &str) -> (u32, u32) {
    let mut dir = false;
    let mut delta = 0;
    let mut dial = (u32::MAX/2).next_multiple_of(100) + 50;
    let mut p1 = 0;
    let mut p2 = 0;

    for c in input.bytes() {
        match c {
            _ if c.is_ascii_digit() => delta = delta * 10 + (c - b'0') as u32,
            b'\n' => {
                if dir {
                    let next = dial + delta;
                    p2 += next / 100 - dial / 100;
                    dial = next;           
                } else {
                    let next = dial - delta;
                    p2 += (dial - 1) / 100 - (next - 1) / 100;
                    dial = next;
                };
                p1 += (dial % 100 == 0) as u32;
                delta = 0;
            }
            b'L' => dir = false,
            _ => dir = true,
        }
    }
    (p1, p2)
}