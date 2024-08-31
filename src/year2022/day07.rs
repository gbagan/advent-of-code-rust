use crate::util::parser::*;

pub fn solve(input: &str) -> Option<(u32, u32)> {    
    let mut stack = vec!();
    let mut sizes = vec!();

    for line in input.lines() {
        if line.ends_with("..") {
            let size = stack.pop().unwrap();
            sizes.push(size);
            let index = stack.len()-1;
            stack[index] += size;

        } else if line.starts_with("$ cd") {
            stack.push(0);

        } else if let Some(size) = line.iter_unsigned::<u32>().next() {
            let index = stack.len() - 1;
            stack[index] += size;
        }
    }
    let mut total = 0;
    while let Some(size) = stack.pop() {
        total += size;
        sizes.push(total);
    }

    let p1 = sizes.iter().filter(|&&size| size <= 100_000).sum();
    let total_size = sizes.last().unwrap();
    let min_size = total_size - 40_000_000;
    let p2 = sizes.iter().filter(|&&size| size >= min_size).min().copied()?;

    Some((p1, p2))
}
