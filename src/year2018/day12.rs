struct State {
    pots: Vec<bool>,
    start: i64,
    sum: i64,
}

pub fn solve(input: &str) -> (i64, i64) {
    let input = input.as_bytes();
    let (rules, mut state) = parse(input);
    let mut next = State { pots: Vec::with_capacity(300), start: 0, sum: 0 };

    for _ in 0..20 {
        simulate(&rules, &state, &mut next);
        std::mem::swap(&mut state, &mut next);
    }

    let p1 = state.sum;

    let mut index = 20;
    let (index, sum1, sum2) = loop {
        simulate(&rules, &state, &mut next);
        let (pots1, _) = trim(&state.pots);
        let (pots2, _) = trim(&next.pots);
        if pots1 == pots2 {
            break (index, state.sum, next.sum);
        }
        std::mem::swap(&mut state, &mut next);
        index += 1;
    };

    let p2 = state.sum + (sum2 - sum1) * (50_000_000_000 - index);

    (p1, p2)
}

fn parse(input: &[u8]) -> ([bool; 32], State) {
    let (input1, input2) = input.split_once(|&c| c == b'\n').unwrap();
    let mut state = State { pots: Vec::with_capacity(300), start: 0, sum: 0};
    for (i, &c) in input1[15..].iter().enumerate() {
        let b = c == b'#';
        state.pots.push(b);
        state.sum += i as i64 * b as i64;
    }
    
    let mut rules = [false; 32];
    for line in input2[1..].array_chunks::<11>() {
        let mut index = 0;
        for &c in &line[..5] {
            index = index << 1 | (c == b'#') as usize;
        }
        rules[index] = line[9] == b'#';
    }

    (rules, state)
}

fn simulate(rules: &[bool; 32], state: &State, next: &mut State) {
    let (pots, i) = trim(&state.pots);
    let next_pots = &mut next.pots;
    next_pots.clear();
    next.start = state.start + i - 2;
    let mut index = 0;
    let mut sum = 0;
    for (i, &pot) in pots.iter().enumerate() {
        index = (index & 0xf) << 1 | pot as usize;
        sum += (i as i64 + next.start) * rules[index] as i64;
        next_pots.push(rules[index]);
    }
    for i in pots.len()..pots.len()+4 {
        index = (index & 0xf) << 1;
        sum += (i as i64 + next.start) * rules[index] as i64;
        next_pots.push(rules[index]);
    }
    next.sum = sum;
}

fn trim(state: &[bool]) -> (&[bool], i64) {
    let mut i = 0;
    while !state[i]{
        i += 1;
    }
    let mut j = state.len() - 1;
    while !state[j]{
        j -= 1;
    }
    (&state[i..j+1], i as i64)
}