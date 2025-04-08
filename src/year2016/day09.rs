pub fn solve(input: &str) -> (usize, usize) {
    let input = input.trim().as_bytes();
    let p1 = count(input, false);
    let p2 = count(input, true);
    (p1, p2)
}

fn parse_number(input: &[u8], i: usize, delim: u8) -> (usize, usize) {
    let mut j = i;
    let mut x = 0;
    loop {
        let c = input[j];
        if c == delim {
            break;
        }
        x = x * 10 + (c - b'0') as usize;
        j += 1;
    }
    (x, j+1)
}

fn count(input: &[u8], recursively: bool) -> usize {
    let mut counter = 0;
    let mut i = 0;
    while i  < input.len() {
        if input[i] == b'(' {
            let (x, next) = parse_number(input, i+1, b'x');
            let (y, next) = parse_number(input, next, b')');
            counter += y * if recursively {
                count(&input[next..next+x], recursively)
            } else { 
                x
            };
            i = next + x;
        } else {
            counter += 1;
            i += 1;
        }
    }
    counter
}