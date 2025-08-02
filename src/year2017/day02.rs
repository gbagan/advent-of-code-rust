use crate::util::parser::*;

fn parse_line(line: &str, numbers: &mut Vec<u32>) {
    numbers.clear();
    numbers.extend(line.iter_unsigned::<u32>());
    numbers.sort_unstable();
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;
    let mut numbers = Vec::new();
    for line in input.lines() {
        parse_line(line, &mut numbers);
        p1 += numbers[numbers.len()-1] - numbers[0];
        
        'outer: for (i, &x) in numbers.iter().enumerate() {
            for &y in &numbers[i+1..] {
                if y.is_multiple_of(x) {
                    p2 += y / x;
                    break 'outer;
                }

            }
        }
        
        //p2 += numbers.iter().tuple_combinations().find_map(|(&x, &y)|
        //    if y.is_multiple_of(x) {Some(y / x)} else { None }
        //).unwrap_or(0);
    }
    (p1, p2)
}
