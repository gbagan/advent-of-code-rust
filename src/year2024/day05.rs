pub fn solve(input: &str) -> (usize, usize) {
    let mut input = input.as_bytes();
    let mut rules = [0u128; 100];

    while input[0] != b'\n' && input.len() >= 6 {
        let x = input[0] as usize * 10 + input[1] as usize - 528;
        let y = input[3] as usize * 10 + input[4] as usize - 528;
        rules[x] |= 1 << y;
        input = &input[6..];
    }

    input = &input[1..];
    let mut p1 = 0;
    let mut p2 = 0;
    let mut row = [0; 25];
    let mut idx = 0;
    let mut is_sorted = true;
    let mut mask = 0;

    for chunk in input.chunks_exact(3) {
        let x = chunk[0] as usize * 10 + chunk[1] as usize - 528;
        row[idx] = x;
        idx += 1;
        is_sorted &= rules[x] & mask == 0;
        mask |= 1 << x; 

        if chunk[2] == b'\n' {
            if is_sorted {
                p1 += row[idx/2];
            } else {
                for &n in &row[..idx] {
                    if (rules[n] & mask).count_ones() == (idx/2) as u32 {
                        p2 += n;
                    } 
                }
            }
            idx = 0;
            is_sorted = true;
            mask = 0;
        }
    }
    (p1, p2)
}