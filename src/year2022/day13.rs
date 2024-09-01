use itertools::Itertools;

pub fn solve(input: &str) -> Option<(usize, usize)> {
    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();  
    
    let p1 = lines
        .iter()
        .tuples()
        .enumerate()
        .map(|(i, (line1, line2))| if less_than(line1, line2) {i+1} else {0})
        .sum();

    let mut position1 = 1;
    let mut position2  = 2;
    for line in lines.iter() {
        if less_than(line, "[[2]]") {
            position1 += 1;
            position2 += 1;
        } if less_than(line, "[[6]]") {
            position2 += 1;
        }
    }
    let p2 = position1 * position2;

    Some((p1, p2))

}

fn less_than(line1: &str, line2: &str) -> bool {
    let mut it1 = IterWithBuffer::new(line1);
    let mut it2 = IterWithBuffer::new(line2);
    while let (Some (c1), Some(c2)) = (it1.next(), it2.next()) {
        match (c1, c2) {
            _ if c1 == c2 => (),
            (b']', _) => return true,
            (_, b']') => return false,
            (b'[', _) => { it2.push(b']'); it2.push(c2) }
            (_, b'[') => { it1.push(b']'); it1.push(c1) }
            _ => return c1 < c2
        }
    }
    unreachable!()
}



struct IterWithBuffer<'a>{
    bytes: &'a [u8],
    index: usize,
    buffer: Vec<u8>,
}

impl<'a> IterWithBuffer<'a> {
    fn new(s: &'a str) -> Self {
        IterWithBuffer {bytes: s.as_bytes(), index: 0, buffer: vec!()}
    }

    fn push(&mut self, c: u8) {
        self.buffer.push(c);
    }

    fn next(&mut self) -> Option<u8> {
        self.buffer.pop().or_else(|| {
            let index = self.index;
            let bytes = self.bytes;
            if bytes[index] == b'1' && bytes[index+1] == b'0' {
                self.index += 2;
                Some(b'A')
            }  else {
                self.index += 1;
                Some(bytes[index])
            }
        })
    }
}

#[test]
fn less_than_test() {
    assert_eq!(less_than("[[1],[2,3,4]]", "[[1],4]"), true);
    assert_eq!(less_than("[9]", "[[8,7,6]]"), false);
}