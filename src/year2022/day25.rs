pub fn solve(input: &str) -> (String, u32) {
    let mut sum = 0;
    let mut acc = 0;
    for c in input.bytes() {
        if c == b'\n' {
            sum += acc;
            acc = 0;
        } else {
            acc = 5 * acc + DECIPHER[c as usize];
        }
    }
    
    let p1 = cipher(sum);

    (p1, 0)
}

const DECIPHER: [i64; 128] = {
    let mut table = [0; 128];
    table[b'1' as usize] = 1;
    table[b'2' as usize] = 2;
    table[b'-' as usize] = -1;
    table[b'=' as usize] = -2;
    table
};

const CIPHER: [u8; 5] = [b'0', b'1', b'2', b'=', b'-'];

fn cipher(mut n: i64) -> String {
    let mut res = Vec::new();
    while n != 0 {
        let r = n % 5;
        res.push(CIPHER[r as usize]);
        n /= 5;
        if r > 2 {
            n += 1;
        }
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}
