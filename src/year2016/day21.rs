pub fn solve(input: &str) -> (String, String) {
    let to_i = |c: u8| (c - b'0') as usize;
    let mut operations = Vec::with_capacity(100);
    let mut input = input.as_bytes();
    while !input.is_empty() {
        match input[4] {
            b' ' => { // swap or move
                if input[0] == b'm' { // move
                    operations.push(Operation::Move(to_i(input[14]), to_i(input[28])));
                    input = &input[30..];
                } else if input[5] == b'p' { // swap position
                    operations.push(Operation::SwapPosition(to_i(input[14]), to_i(input[30])));
                    input = &input[32..];
                } else { // swap letter
                    operations.push(Operation::SwapLetter(input[12], input[26]));
                    input = &input[28..];
                }
            },
            b't' => { // rotate 
                match input[7] {
                    b'l' => { // rotate left
                        let d = to_i(input[12]);
                        operations.push(Operation::RotateLeft(d));
                        input = &input[(if d==1 {19} else {20})..];
                    },
                    b'r' => { // rotate right
                        let d = to_i(input[13]);
                        operations.push(Operation::RotateRight(d));
                        input = &input[(if d==1 {20} else {21})..];
                    },
                    _ => { // rotate based on position
                        operations.push(Operation::RotatePosition(input[35]));
                        input = &input[37..];
                    }
                }
            },
            _ => { // reverse
                operations.push(Operation::Reverse(to_i(input[18]), to_i(input[28])));
                input = &input[30..];
            }
        }
    }
    
    let p1 =  part1(&operations);
    let p2 = part2(&operations);
    (p1, p2)
}

fn part1(operations: &[Operation]) -> String {
    let mut puzzle = Vec::from(b"abcdefgh");
    for op in operations {
        op.perform(&mut puzzle);
    }
    
    String::from_utf8(puzzle).unwrap()
}

fn part2(operations: &[Operation]) -> String {
    let mut puzzle = Vec::from(b"fbgdceah");
    for op in operations.iter().rev() {
        op.inverse().perform(&mut puzzle);
    }

    String::from_utf8(puzzle).unwrap()
}

#[derive(Clone)]
enum Operation {
    SwapPosition(usize, usize),
    SwapLetter(u8, u8),
    RotateLeft(usize),
    RotateRight(usize),
    RotatePosition(u8),
    Reverse(usize, usize),
    Move(usize, usize),
    InvRotatePosition(u8),
}

impl Operation {
    pub fn inverse(self: &Self) -> Self {
        match self {
            Operation::RotateLeft(d) => Operation::RotateRight(*d),
            Operation::RotateRight(d) => Operation::RotateLeft(*d),
            Operation::RotatePosition(c) => Operation::InvRotatePosition(*c),
            Operation::Move(i, j) => Operation::Move(*j, *i),
            _ => self.clone()
        }
    }
    
    pub fn perform(self: &Self, puzzle: & mut Vec<u8>) {
        match self {
            Operation::SwapPosition(i, j) => puzzle.swap(*i, *j),
            Operation::SwapLetter(c1, c2) => {
                let i = puzzle.iter().position(|&c| c == *c1).unwrap();
                let j = puzzle.iter().position(|&c| c == *c2).unwrap();
                puzzle.swap(i, j);
            },
            Operation::RotateLeft(d) => puzzle.rotate_left(*d),
            Operation::RotateRight(d) => puzzle.rotate_right(*d),
            Operation::RotatePosition(c) => {
                let idx = puzzle.iter().position(|&c2| c2==*c).unwrap();
                puzzle.rotate_right(idx + if idx < 4 {1} else {2});
            },
            Operation::Reverse(i, j) => puzzle[*i..=*j].reverse(),
            Operation::Move(i, j) => {
                let c = puzzle.remove(*i);
                puzzle.insert(*j, c);
            },
            Operation::InvRotatePosition(c) => {
                let idx = puzzle.iter().position(|&c2| c2==*c).unwrap();
                let n = idx / 2 + if idx & 1 == 1 || idx == 0 {1} else {5};
                puzzle.rotate_left(n);
            }
        }
    }
}

