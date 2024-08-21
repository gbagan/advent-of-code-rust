pub fn parse(input: &str) -> Option<&str> {
    Some(input)
}

pub fn part1(input: &str) -> Option<i32> {
    let mut total = 0;
    let mut first_position = None;
    for (i,c) in input.chars().enumerate() {
        if c.is_ascii_digit() || c == '-' {
            if first_position.is_none() {
                first_position = Some(i);
            }
        } else if let Some(j) = first_position {
            total += input[j..i].parse::<i32>().unwrap();
            first_position = None;
        }
    }
    Some(total)
}

struct Parsed {
    value: i32,
    end: usize,
    ignore: bool
}

const RED: &[u8;3] = b"red";

fn parse_json(text: &[u8], i: usize) -> Parsed {
    match text[i] {
        b'[' => parse_array(text, i),
        b'{' => parse_object(text, i),
        b'"' => parse_string(text, i),
        _ => parse_number(text, i),
    }
}

fn parse_number(text: &[u8], i: usize) -> Parsed {
    let mut j = i;
    let mut value = 0;
    let mut sign = 1;
    if text[j] == b'-' {
        sign = -1;
        j += 1;
    }
    while text[j].is_ascii_digit() {
        value = value * 10 + (text[j] - b'0') as i32;
        j+=1;
    }
    value *= sign;
    Parsed { value, end: j, ignore: false } 
}

fn parse_string(text: &[u8], i: usize) -> Parsed {
    let mut j = i+1;
    while text[j] != b'"' {
        j+=1;
    }
    Parsed {value: 0, end:j+1, ignore: &text[i+1..j] == RED}
}


fn parse_array(text: &[u8], i: usize) -> Parsed {
    let mut j = i;
    let mut value = 0;
    while text[j] != b']' {
        let parsed = parse_json(text, j+1);
        value += parsed.value;
        j = parsed.end;
    }
    Parsed {value, end: j+1, ignore: false}
}

fn parse_object(text: &[u8], i: usize) -> Parsed {
    let mut j = i;
    let mut value = 0;
    let mut ignored = false;
    while text[j] != b'}' {
        while text[j] != b':' {
            j += 1;
        }
        let parsed = parse_json(text, j+1);
        value += parsed.value;
        ignored = ignored || parsed.ignore;
        j = parsed.end;
    }
    if ignored {
        value = 0;
    }
    Parsed {value, end: j+1, ignore: false}
}


pub fn part2(input: &str) -> Option<i32> {
    Some(parse_json(input.as_bytes(), 0).value)
}
