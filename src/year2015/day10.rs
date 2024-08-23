use crate::util::times;

pub fn parse(input: &str) -> Option<(usize, usize)> {
    let integers = input
        .trim_end()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|v| v as u8)
        .collect();

    let integers = times(40, integers, next);
    let p1 = integers.len();
    let integers = times(10, integers, next);
    let p2 = integers.len();
    Some((p1, p2))
}

fn next(v: &Vec<u8>) -> Vec<u8> {
    let mut it = v.iter();
    
    match it.next() {
        None => vec!(),
        Some(n) => {
            let mut output = Vec::with_capacity(2*v.len());
            let mut cur = *n;
            let mut i = 1;
            for c in it {
                if *c == cur {
                    i += 1
                } else {
                    output.push(i);
                    output.push(cur);
                    cur = *c;
                    i = 1;
                }
            }
            output.push(i);
            output.push(cur);
            output
        }
    }
}

pub fn part1(input: &(usize, usize)) -> Option<usize> {
    Some(input.0)
}

pub fn part2(input: &(usize, usize)) -> Option<usize> {
    Some(input.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = vec!(1,1,1,2, 2, 1);
        let result = vec!(3, 1, 2, 2, 1, 1);
        assert_eq!(result, next(&input));
    }
}