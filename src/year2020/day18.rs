pub fn solve(input: &str) -> (u64, u64) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += part1(line);
        p2 += part2(line);
    }
    (p1, p2)
}

fn part1(line: &str) -> u64 {
    eval_expr(&mut line.bytes().filter(|&c| c != b' '))
}

fn eval_expr(bytes: &mut impl Iterator::<Item=u8>) -> u64 {
    let mut total = next_value(bytes);
    while let Some(c) = bytes.next() {
        match c {
            b'+' => total += next_value(bytes),
            b'*' => total *= next_value(bytes),
            b')' => break,
            _ => panic!("Unexpected character: {}", c as char) 
        }
    }
    total
}

fn next_value(bytes: &mut impl Iterator::<Item=u8>) -> u64 {
    let c = bytes.next().unwrap();
    match c {
        b'0'..=b'9' => (c - b'0') as u64,
        b'(' => eval_expr(bytes),
        _ => panic!("Unexpected character {}", c as char)
    }
}

fn part2(line: &str) -> u64 {
    eval_expr2(&mut line.bytes().filter(|&c| c != b' '))
}

fn eval_expr2(bytes: &mut impl Iterator::<Item=u8>) -> u64 {
    let mut total = 1;
    let mut sum = next_value2(bytes);
    while let Some(c) = bytes.next() {
        match c {
            b'+' => sum += next_value2(bytes),
            b'*' => { total *= sum; sum = next_value2(bytes) },
            b')' => break,
            _ => panic!("Unexpected character: {}", c as char) 
        }
    }
    total *= sum;
    total
}

fn next_value2(bytes: &mut impl Iterator::<Item=u8>) -> u64 {
    let c = bytes.next().unwrap();
    match c {
        b'0'..=b'9' => (c - b'0') as u64,
        b'(' => eval_expr2(bytes),
        _ => panic!("Unexpected character {}", c as char)
    }
}


#[test]
fn part1_test() {
    let text = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(part1(text), 437);
}

#[test]
fn part2_test() {
    let text = "3 + 2 * 3 + 3 * 2 ";
    assert_eq!(part2(text), 60);


    let text = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(part2(text), 23340);
}