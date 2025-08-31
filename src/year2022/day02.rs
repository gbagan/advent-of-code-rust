pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    let score1 = [4, 8, 3, 1, 5, 9, 7, 2, 6];
    let score2 = [3, 4, 8, 1, 5, 9, 2, 6, 7];

    let mut input = input.as_bytes();

    while input.len() >= 4 {
        let index = ((input[0] - b'A') * 3 + (input[2] - b'X')) as usize;
        p1 += score1[index];
        p2 += score2[index];
        input = &input[4..];
    }

    (p1, p2)
}