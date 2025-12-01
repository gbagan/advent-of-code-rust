pub fn solve(input: &str) -> (u32, u32) {
    let mut dial = const { (u32::MAX/2).next_multiple_of(100) + 50 };
    let mut p1 = 0;
    let mut p2 = 0;

    let mut input = input.as_bytes();
    while !input.is_empty() {
        let (delta, next) =
            if input[2] == b'\n' {
                (input[1] as u32 - 48, 3)
            } else if input[3] == b'\n' {
                (10 * input[1] as u32  + input[2] as u32 - 11 * 48, 4)
            } else {
               (100 * input[1] as u32 + 10 * input[2] as u32 + input[3] as u32 - 111 * 48, 5) 
            };
        if input[0] == b'R' {
            let next = dial + delta;
            p2 += next / 100 - dial / 100;
            dial = next;           
        } else {
            let next = dial - delta;
            p2 += (dial - 1) / 100 - (next - 1) / 100;
            dial = next;
        };
        p1 += (dial % 100 == 0) as u32;
        input = &input[next..];
    }
    (p1, p2)
}