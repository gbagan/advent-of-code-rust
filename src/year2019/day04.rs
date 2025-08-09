pub fn solve(input: &str) -> (u32, u32) {
    let input = input.as_bytes();
    let min: [usize; 6] = std::array::from_fn(|i| (input[i] - b'0') as usize);
    let max: [usize; 6] = std::array::from_fn(|i| (input[7+i] - b'0') as usize);
    let p1 = count_passwords::<false>(min, max);
    let p2 = count_passwords::<true>(min, max);
    (p1, p2)
}

pub fn count_passwords<const PART2: bool>(min: [usize; 6], max: [usize; 6]) -> u32 {
    let mut table = [0u32; 7*1024];

    for i in 6*1024..7*1024 {
        table[i] = (is_ok(i) || current_sequence(i) == 2) as u32;
    }

    for i in (0..6*1024).rev() {
        let idx = idx(i);
        let current_sequence = current_sequence(i);
        let previous_digit = previous_digit(i);
        if current_sequence > 6 {
            continue;
        }
        let mut lo = previous_digit;
        let mut hi = 9;
        if is_min(i) {
            lo = lo.max(min[idx]);
        }
		if is_max(i) {
            hi = hi.min(max[idx]);
        }
        for d in lo..hi+1 {
            let next_is_ok = is_ok(i) || (
                current_sequence == 2 && (!PART2 || d != previous_digit)
            );
            let next_is_min = is_min(i) && d == min[idx];
            let next_is_max = is_max(i) && d == max[idx];
            let next_idx = idx + 1;
            let next_prev_digit = d;
            let next_sequence = if d == previous_digit { current_sequence + 1 } else { 1 };
            let j = mk_state(
                next_is_ok, next_is_min, next_is_max, next_prev_digit, next_sequence, next_idx
            );
            table[i] += table[j];
        }
    }
    let i = mk_state(false,true, true,0,0, 0);
    table[i]

}

fn mk_state(
    is_ok: bool,
    is_min: bool,
    is_max: bool, 
    previous_digit: usize,
    current_sequence: usize,
    idx: usize) -> usize
{
    (is_ok as usize)
    | (is_min as usize) << 1
    | (is_max as usize) << 2
    | previous_digit << 3
    | current_sequence << 7
    | idx << 10
}

#[inline]
fn is_ok(state: usize) -> bool {
    state & 1 == 1
}

#[inline]
fn is_min(state: usize) -> bool {
    (state >> 1) & 1 == 1
}

#[inline]
fn is_max(state: usize) -> bool {
    (state >> 2) & 1 == 1
}

#[inline]
fn previous_digit(state: usize) -> usize {
    (state >> 3) & 15
}

#[inline]
fn current_sequence(state: usize) -> usize {
    (state >> 7) & 7
}

#[inline]
fn idx(state: usize) -> usize {
    state >> 10
}