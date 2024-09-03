pub fn solve(input: &str) -> Option<(u32, u32)> {
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
                        '!' => { it.next(); },
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

#[test]
fn part1_test() {
    let input = "{{{},{},{{}}}}";
    assert_eq!(solve(input), Some((16, 0))); 
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    assert_eq!(solve(input), Some((3, 17))); 
}