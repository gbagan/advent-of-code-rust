use anyhow::*;

pub fn solve(input: &str) -> Result<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut it = input.bytes();
    let mut level = 0;
    while let Some(c) = it.next() {
        match c {
            b'<' => {
                while let Some(c) = it.next() {
                    match c {
                        b'>' => break,
                        b'!' => { it.next(); },
                        _ => p2 += 1,
                    }
                }
            }
            b'{' => level += 1,
            b'}' => {
                p1 += level;
                level -= 1;
            }
            _ => (),
        }
    }
    Ok((p1, p2))
}


#[test]
fn part1_test() {
    let input = "{{{},{},{{}}}}";
    assert_eq!(solve(input).ok(), Some((16, 0))); 
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(solve(input).ok(), Some((3, 17))); 
}