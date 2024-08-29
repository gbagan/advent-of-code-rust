struct Result {
    value: u64,
    next: usize,
}

fn skip_spaces(text: &[u8], i: usize) -> usize {
    let n = text.len();
    let mut j = i;
    while j < n && text[j] == b' ' {
        j += 1;
    }
    j
}

fn parse_number(text: &[u8], i: usize) -> Result {
    let n = text.len();
    let mut j = i;
    let mut value = 0;
    while j < n && text[j].is_ascii_digit() {
        value = value * 10 + (text[j] - b'0') as u64;
        j+=1;
    }
    Result { value, next: j }
}

fn parse_term(text: &[u8], i: usize) -> Result {
    let i = skip_spaces(text, i);
    if text[i] == b'(' {
        let res = parse_sequence(text, i+1);
        assert_eq!(text[res.next], b')');
        Result { value: res.value, next: res.next + 1 }
    } else if text[i].is_ascii_digit() {
        parse_number(text, i)
    } else {
        panic!("unexcepted character: {}", text[i])
    }
}

fn parse_sequence(text: &[u8], i: usize) -> Result {
    let n = text.len();
    let res = parse_term(text, i);
    let mut j = skip_spaces(text, res.next);
    let mut value = res.value;
    while j < n && text[j] != b')' {
        let res = parse_term(text, j+1);
        match text[j] {
            b'*' => value *= res.value,
            b'+' => value += res.value,
            _ => panic!("unexcepted character: '{}'", text[i])
        }
        j = skip_spaces(text, res.next);
    }
    Result {value, next: j}
}

fn parse_term2(text: &[u8], i: usize) -> Result {
    let i = skip_spaces(text, i);
    if text[i] == b'(' {
        let res = parse_sequence2(text, i+1);
        assert_eq!(text[res.next], b')');
        Result { value: res.value, next: res.next + 1 }
    } else if text[i].is_ascii_digit() {
        parse_number(text, i)
    } else {
        panic!("unexcepted character: {}", text[i])
    }
}

fn parse_sequence2(text: &[u8], i: usize) -> Result {
    let n = text.len();
    let mut value = 1;
    let res = parse_term2(text, i);
    let mut j = skip_spaces(text, res.next);
    let mut value2 = res.value;
    while j < n && text[j] != b')' {
        let res = parse_term2(text, j+1);
        match text[j] {
            b'*' => {
                value *= value2;
                value2 = res.value;
            }
            b'+' => {
                value2 += res.value;
            }
            _ => panic!("unexcepted character: '{}'", text[i])
        }
        j = skip_spaces(text, res.next);
    }
    value *= value2;
    Result {value, next: j}
}

pub fn solve(input: &str) -> Option<(u64, u64)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let line = line.as_bytes();
        p1 += parse_sequence(line, 0).value;
        p2 += parse_sequence2(line, 0).value;
    }
    Some((p1, p2))
}

#[test]
fn part1_test() {
    let text = b"5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(parse_sequence(text, 0).value, 437);
}

#[test]
fn part2_test() {
    let text = b"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(parse_sequence2(text, 0).value, 23340);
}