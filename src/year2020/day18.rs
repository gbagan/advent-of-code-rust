use anyhow::*;

pub fn solve(input: &str) -> Result<(u64, u64)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        p1 += part1(line).with_context(|| format!("Part 1: Parsing line: {line}"))?;
        p2 += part2(line)?;
    }
    Ok((p1, p2))
}

fn part1(line: &str) -> Result<u64> {
    eval_expr(&mut line.bytes().filter(|&c| c != b' '))
}

fn eval_expr<I>(bytes: &mut I) -> Result<u64> where I: Iterator::<Item=u8> {
    let mut total = next_value(bytes)?;
    while let Some(c) = bytes.next() {
        match c {
            b'+' => total += next_value(bytes)?,
            b'*' => total *= next_value(bytes)?,
            b')' => break,
            _ => bail!("Unexpected character: {}", c as char) 
        }
    }
    Ok(total)
}

fn next_value<I>(bytes: &mut I) -> Result<u64> where I: Iterator::<Item=u8> {
    let c = bytes.next().context("Unexpected end of line")?;
    match c {
        b'0'..=b'9' => Ok((c - b'0') as u64),
        b'(' => eval_expr(bytes),
        _ => bail!("Unexpected character {}", c as char)
    }
}

fn part2(line: &str) -> Result<u64> {
    eval_expr2(&mut line.bytes().filter(|&c| c != b' '))
}

fn eval_expr2<I>(bytes: &mut I) -> Result<u64> where I: Iterator::<Item=u8> {
    let mut total = 1;
    let mut sum = next_value2(bytes)?;
    while let Some(c) = bytes.next() {
        match c {
            b'+' => sum += next_value2(bytes)?,
            b'*' => { total *= sum; sum = next_value2(bytes)? },
            b')' => break,
            _ => bail!("Unexpected character: {}", c as char) 
        }
    }
    total *= sum;
    Ok(total)
}

fn next_value2<I>(bytes: &mut I) -> Result<u64> where I: Iterator::<Item=u8> {
    let c = bytes.next().context("Unexpected end of line")?;
    match c {
        b'0'..=b'9' => Ok((c - b'0') as u64),
        b'(' => eval_expr2(bytes),
        _ => bail!("Unexpected character {}", c as char)
    }
}


#[test]
fn part1_test() {
    let text = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
    assert_eq!(part1(text).unwrap(), 437);
}

#[test]
fn part2_test() {
    let text = "3 + 2 * 3 + 3 * 2 ";
    assert_eq!(part2(text).unwrap(), 60);


    let text = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(part2(text).unwrap(), 23340);
}