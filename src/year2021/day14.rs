struct Rule {
    from: usize,
    left: usize,
    right: usize,
    inserted: usize,
}

pub fn solve(input: &str) -> (usize, usize) {
    let to_idx = |c: u8| (c - b'A') as usize;

    let (template, rules_str) = input.split_once("\n\n").unwrap();
    let template = template.as_bytes();

    let mut elements = [0; 26];
    let mut pairs = [0; 26*26];

    for &c in template {
        elements[to_idx(c)] += 1;
    }

    for &[c1, c2] in template.array_windows() {
        pairs[to_idx(c1) * 26 + to_idx(c2)] = 1;
    }

    let rules: Vec<_> = rules_str
        .as_bytes()
        .as_chunks::<8>().0
        .iter()
        .map(|chk| {
            let a = to_idx(chk[0]);
            let b = to_idx(chk[1]);
            let c = to_idx(chk[6]);
            let from = a*26+b;
            let left = a*26+c;
            let right = c*26+b;
            Rule { from, left, right, inserted: c }
        })
        .collect();

    for _ in 0..10 {
        simulate_one_step(&rules, &mut elements, &mut pairs);
    }

    let p1 = score(&elements);

    for _ in 10..40 {
        simulate_one_step(&rules, &mut elements, &mut pairs);
    }

    let p2 = score(&elements);

    (p1, p2)
}

fn simulate_one_step(rules: &[Rule], elements: &mut [usize; 26], pairs: &mut [usize; 676]) {
    let mut pairs2 = [0; 676];
    for rule in rules {
        let freq = pairs[rule.from];
        pairs2[rule.left] += freq;
        pairs2[rule.right] += freq;
        elements[rule.inserted] += freq;
    }
    *pairs = pairs2;
}

fn score(elements: &[usize]) -> usize {
    let max = *elements.iter().max().unwrap();
    let min = *elements.iter().filter(|&&x| x > 0).min().unwrap();
    max - min
}