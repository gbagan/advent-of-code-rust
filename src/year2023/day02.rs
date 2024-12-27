pub fn solve(input: &str) -> (u32, u32) {
    let input = input.as_bytes();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut i = 0;
    let mut id = 1;

    while i < input.len() - 1 {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        i += 5 + (if id < 10 {1} else if id < 100 {2} else {3});

        while input[i] != b'\n' {
            i += 2;
            let mut n = 0;

            while input[i] != b' ' {
                n = 10 * n + (input[i] - b'0') as u32;
                i += 1;
            }

            i += 1;

            match input[i] {
                b'r' => {
                    r = r.max(n);
                    i += 3;
                },
                b'g' => {
                    g = g.max(n);
                    i += 5;    
                },
                _ => {
                    b = b.max(n);
                    i += 4;
                }
            }
        }

        if r <= 12 && g <= 13 && b <= 14 {
            p1 += id;
        }

        p2 += r * g * b;
        id += 1;
        i += 1;
    }
    (p1, p2)
}