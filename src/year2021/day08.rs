const DIGIT_FROM_FREQ: [u32; 50] = {
    let mut table = [0; 50];
    table[42] = 0;
    table[17] = 1;
    table[34] = 2;
    table[39] = 3;
    table[30] = 4;
    table[37] = 5;
    table[41] = 6;
    table[25] = 7;
    table[49] = 8;
    table[45] = 9;
    table
};

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let digits = decrypt(line);
        for digit in digits {
            if matches!(digit, 1 | 4 | 7 | 8) {
                p1 += 1;
            }
        }
        p2 += digits[0] * 1000 + digits[1] * 100 + digits[2] * 10 + digits[3];
    }
    (p1, p2)
}

fn decrypt(line: &str) -> [u32; 4] {
    let line = line.as_bytes();
    let mut freq = [0usize; 104];
    for &c in &line[0..58] {
        freq[c as usize] += 1;
    }
    let mut it = line[61..]
        .split(|&c| c == b' ')
        .map(|word| DIGIT_FROM_FREQ[word.iter().map(|&c| freq[c as usize]).sum::<usize>()]);
    std::array::from_fn(|_| it.next().unwrap())
}