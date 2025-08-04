use crate::util::parser::*;

pub fn solve(input: &str) -> (usize, usize) {
    let mut iter = input.iter_unsigned::<usize>();
    let mut stack = Vec::new();
    parse(&mut iter, &mut stack)
}

fn parse(iter: &mut impl Iterator<Item = usize>, stack: &mut Vec<usize>) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let node_len =  iter.next().unwrap();
    let metadata_len = iter.next().unwrap();
    for _ in 0..node_len {
        let (sub_p1, sub_p2) = parse(iter, stack);
        p1 += sub_p1;
        stack.push(sub_p2);
    }
    for _ in 0..metadata_len {
        let n = iter.next().unwrap();
        p1 += n;
        if node_len == 0 {
            p2 += n;
        } else if n > 0 && n <= node_len {
            p2 += stack[stack.len() - node_len + (n - 1)];
        }
    }
    stack.truncate(stack.len() - node_len);

    (p1, p2)
}