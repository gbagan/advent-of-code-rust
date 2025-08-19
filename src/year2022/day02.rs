pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    let score1 = [4, 8, 3, 1, 5, 9, 7, 2, 6];
    let score2 = [3, 4, 8, 1, 5, 9, 2, 6, 7];

    for &[c1, _, c2,_] in input.as_bytes().as_chunks::<4>().0 {
        let index = ((c1 - b'A') * 3 + (c2 - b'X')) as usize;
        p1 += score1[index];
        p2 += score2[index];
    }

    (p1, p2)
}