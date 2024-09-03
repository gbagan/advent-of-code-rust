use itertools::Itertools;

use crate::util::iter::put_many_back;

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
            println!("{line} < [[2]]");
            position1 += 1;
            position2 += 1;
        } else if less_than(line, "[[6]]") {
            println!("{line} < [[6]]");
            position2 += 1;
        }
    }
    let p2 = position1 * position2;

    Some((p1, p2))

}

fn less_than(line1: &str, line2: &str) -> bool {
    let mut it1 = put_many_back(PacketIterator::new(line1));
    let mut it2 = put_many_back(PacketIterator::new(line2));
    while let (Some (c1), Some(c2)) = (it1.next(), it2.next()) {
        match (c1, c2) {
            _ if c1 == c2 => (),
            (b']', _) => return true,
            (_, b']') => return false,
            (b'[', _) => { it2.put_back(b']'); it2.put_back(c2) }
            (_, b'[') => { it1.put_back(b']'); it1.put_back(c1) }
            _ => return c1 < c2
        }
    }
    unreachable!()
}


struct PacketIterator<'a> {
    bytes: &'a [u8],
    index: usize,
}


impl<'a> PacketIterator<'a> {
    fn new(s: &'a str) -> Self {
        PacketIterator{bytes: s.as_bytes(), index: 0}
    }
}

impl<'a> Iterator for PacketIterator<'a>
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let index = self.index;
        let bytes = self.bytes;
        if bytes[index] == b'1' && bytes[index+1] == b'0' {
            self.index += 2;
            Some(b'A')
        }  else {
            self.index += 1;
            Some(bytes[index])
        }
    }
}

#[test]
fn less_than_test() {
    assert_eq!(less_than("[[1],[2,3,4]]", "[[1],4]"), true);
    assert_eq!(less_than("[9]", "[[8,7,6]]"), false);
    assert_eq!(less_than("[1,[2,[3,[4,[5,6,7]]]]", "[[2]]"), true);
    assert_eq!(less_than("[[8,7,6]]", "[[6]]"), false);
}