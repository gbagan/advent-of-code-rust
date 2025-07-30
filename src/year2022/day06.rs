pub fn solve(input: &str) -> (usize, usize) {
    let p1 = marker(input, 4);
    let p2 = marker(input, 14);
    (p1, p2)
}

pub fn marker(input: &str, size: usize) -> usize {
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
            return i;
        }
    }
    unreachable!();
}

#[test]
fn marker_test() {
    assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
}