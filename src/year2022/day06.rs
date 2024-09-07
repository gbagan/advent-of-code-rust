use anyhow::*;

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let p1 = marker(input, 4).context("Part 1: No solution found")?;
    let p2 = marker(input, 14).context("Part 2: No solution found")?;
    Ok((p1, p2))
}

pub fn marker(input: &str, size: usize) -> Option<usize> {
    let mut seen = [0; 26];
    let mut last_duplicate = 0;

    for (i, c) in input.bytes().enumerate() {
        let i = i + 1;
        let index = (c - b'a') as usize;
        let prev = seen[index];
        seen[index] = i;

        last_duplicate += 1;

        if i - prev < last_duplicate {
            last_duplicate = i - prev;
        }

        if last_duplicate == size {
            return Some(i);
        }
    }
    None
}

#[test]
fn marker_test() {
    assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), Some(5));
    assert_eq!(marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), Some(19));
    assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), Some(23));
}