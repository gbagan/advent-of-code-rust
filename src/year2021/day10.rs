const WEIGHT: [u32; 128] = {
    let mut table = [0; 128];
    table[b')' as usize] = 3;
    table[b']' as usize] = 57;
    table[b'}' as usize] = 1197;
    table[b'>' as usize] = 25137;
    table
};

const WEIGHT2: [u64; 128] = {
    let mut table = [0; 128];
    table[b'(' as usize] = 1;
    table[b'[' as usize] = 2;
    table[b'{' as usize] = 3;
    table[b'<' as usize] = 4;
    table
};

pub fn solve(input: &str) -> (u32, u64) {
    let mut stack = Vec::new();
    
    let mut p1 = 0;
    let mut p2 = Vec::with_capacity(60);

    'outer: for line in input.lines() {
        stack.clear();
        for c in line.bytes() {
            let weight = WEIGHT[c as usize];
            if weight == 0 {
                stack.push(c);
            } else if let Some(c2) = stack.pop() && c - c2 <= 2 {
            } else {
                p1 += weight;
                continue 'outer;
            }
        }
        if !stack.is_empty() {
            p2.push(stack_weight(&stack));
        }
    }


    let len = p2.len();
    let p2 = *p2.select_nth_unstable(len/2).1;

    (p1, p2)
}

fn stack_weight(stack: &[u8]) -> u64 {
    stack.iter().rev().fold(0, |acc, &c| acc * 5 + WEIGHT2[c as usize])
}