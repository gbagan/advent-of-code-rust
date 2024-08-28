pub fn parse(input: &str) -> Option<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut it = input.chars();
    let mut level = 0;
    while let Some(c) = it.next() {
        match c {
            '<' => {
                while let Some(c) = it.next() {
                    match c {
                        '>' => break,
                        '!' => {let _ = it.next(); },
                        _ => p2 += 1,
                    }
                }
            }
            '{' => level += 1,
            '}' => {
                p1 += level;
                level -= 1;
            }
            _ => (),
        }
    }
    Some((p1, p2))
}

pub fn part1(solutions: &(u32, u32)) -> Option<u32> {
    Some(solutions.0)
}

pub fn part2(solutions: &(u32, u32)) -> Option<u32> {
    Some(solutions.1)
}


#[test]
fn part1_test() {
    let input = "{{{},{},{{}}}}";
    assert_eq!(parse(input), Some((16, 0))); 
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(parse(input), Some((3, 17))); 
}