use crate::util::parser::*;

pub fn solve(input: &str) -> (usize, usize) {
    let starting: Vec<_> = input.iter_unsigned::<usize>().collect();

    let p1 = simulate(&starting, 2020);
    let p2 = simulate(&starting, 30_000_000);

    (p1, p2)
}

fn simulate(starting: &[usize], nth: usize) -> usize {
    let mut seen = vec![0u32; nth];
    let mut bitset = BitSet::with_capacity(nth);

    for (i, &s) in starting[..starting.len()-1].iter().enumerate() {
        seen[s] = i as u32 + 1;
    }

    let mut last = starting[starting.len()-1];

    for round in starting.len()..nth {
        if last < 10000 {
            let prev_round = std::mem::replace(&mut seen[last], round as u32);
            last = if prev_round == 0 { 0 } else { round - prev_round as usize };
        } else if bitset.contains(last) {
            last = round - std::mem::replace(&mut seen[last], round as u32) as usize;
        } else {
            bitset.insert(last);
            seen[last] = round as u32;
            last = 0;
        }
    }

    last
}


struct BitSet {
    data: Vec<u64>,
}

impl BitSet {
    fn with_capacity(n: usize) -> Self {
        Self { data: vec![0; (n + 63) >> 6] }
    }

    fn insert(&mut self, n: usize) {
        self.data[n >> 6] |= 1 << (n & 63);
    }

    fn contains(&self, n: usize) -> bool {
        self.data[n >> 6] & 1 << (n & 63) != 0
    }

}