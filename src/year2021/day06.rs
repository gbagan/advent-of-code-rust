// assume that all numbers of the input are between 0 and 8

pub fn solve(input: &str) -> (u64, u64) {
    let mut v = [0u64; 9];
    for c in input.bytes().step_by(2) {
        let i = (c - b'0') as usize;
        v[i] += 1;
    }

    let mut i = 0;
    
    simulate(80, &mut v, &mut i);
    let p1 = v.iter().sum();

    simulate(256-80, &mut v, &mut i);
    let p2 = v.iter().sum();

    (p1, p2)
}

#[inline]
fn simulate(steps: usize, v: &mut[u64; 9], i: &mut usize) {
    for _ in 0..steps {
        let mut i7 = *i + 7;
        if i7 >= 9 {
            i7 -= 9;
        }
        v[i7] += v[*i];
        *i = (*i + 1) * (*i != 8) as usize;
    }
}