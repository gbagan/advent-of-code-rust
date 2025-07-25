pub fn solve(input: &str) -> (u32, u32) {
    let mut input = input.as_bytes();
    let mut position = 0;
    let mut depth1 = 0;
    let mut depth2 = 0;
    let mut aim = 0;

    while !input.is_empty() {
        match input[0] {
            b'f' => { // forward
                let n = (input[8] - b'0') as u32;
                position += n;
                depth2 += aim * n;
                input = &input[10..];
            }
            b'd' => { // down
                let n = (input[5] - b'0') as u32;
                depth1 += n;
                aim += n;
                input = &input[7..];
            }
            _ => { // up
                let n = (input[3] - b'0') as u32;
                depth1 -= n;
                aim -= n;
                input = &input[5..];
            }
        }
    }

    (position * depth1, position * depth2)
}