use crate::util::parser::*;

pub fn solve(input: &str) -> (String, String) {
    let lines: Vec<_> = input.lines().collect();
    let n = lines.len() / 18;

    let mut p1 = vec![b'9'; n];
    let mut p2 = vec![b'1'; n];
    
    let mut stack = Vec::new();

    for (i, chunk) in lines.as_chunks::<18>().0.iter().enumerate() {
        let a = chunk[5].try_signed::<i32>().unwrap();
        let b = chunk[15].try_signed::<i32>().unwrap();
        if a > 0 {
            stack.push((i, b));
        } else {
            let (j, c) = stack.pop().unwrap();
            let d = (a+c).unsigned_abs() as u8;
            if a > -c { 
                p1[j] -= d;
                p2[i] += d;
            } else {
                p1[i] -= d;
                p2[j] += d;
            }
        }
    }

    let p1 = String::from_utf8(p1).unwrap();
    let p2 = String::from_utf8(p2).unwrap();

    (p1, p2)
}