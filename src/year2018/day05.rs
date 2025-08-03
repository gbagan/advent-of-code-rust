pub fn solve(input: &str) -> (usize, usize) {
    let mut output = Vec::with_capacity(input.len() / 4);
    let p1 = react(input.trim().bytes(), &mut output);

    let mut output2 = Vec::with_capacity(p1);
    let p2 = (b'a'..=b'z')
        .map(|c| react(output.iter().copied().filter(|&c2| c2 | 32 != c), &mut output2))
        .min()
        .unwrap();

    (p1, p2)
}

fn react(input: impl Iterator<Item = u8>, stack: &mut Vec<u8>) -> usize {
    stack.clear();
    for c in input {
        if let Some(&c2) = stack.last() && c ^ c2 == 32 {
            stack.pop();
        } else {
            stack.push(c);
        }
    }
    stack.len()
}