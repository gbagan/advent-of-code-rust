pub fn solve(input: &str) -> (u32, u32) {
    let input = input.as_bytes();
    let min: [usize; 6] = std::array::from_fn(|i| (input[i] - b'0') as usize);
    let max: [usize; 6] = std::array::from_fn(|i| (input[7+i] - b'0') as usize);
    let p1 = count_passwords::<false>(min, max);
    let p2 = count_passwords::<true>(min, max);
    (p1, p2)
}

pub fn count_passwords<const PART2: bool>(min: [usize; 6], max: [usize; 6]) -> u32 {
    let mut table = [0u32; 7*512];

    for i in 6*512..7*512 {
        table[i] = (is_ok(i) || repetition(i) == 2) as u32;
    }

    for i in (0..6*512).rev() {
        let previous_digit = previous_digit(i);
        if previous_digit > 9 {
            continue;
        }
        let idx = idx(i);
        let repetition = repetition(i);
        let mut lo = previous_digit;
        let mut hi = 9;
        if is_min(i) {
            lo = lo.max(min[idx]);
        }
		if is_max(i) {
            hi = hi.min(max[idx]);
        }
        for digit in lo..hi+1 {
            let next_is_ok = is_ok(i) || (
                repetition == 2 && (!PART2 || digit != previous_digit)
            );
            let next_is_min = is_min(i) && digit == min[idx];
            let next_is_max = is_max(i) && digit == max[idx];
            let next_idx = idx + 1;
            let next_prev_digit = digit;
            let next_repetition = if digit == previous_digit { (repetition + 1).min(3) } else { 1 };
            let j = pack(
                next_is_ok, next_is_min, next_is_max, next_prev_digit, next_repetition, next_idx
            );
            table[i] += table[j];
        }
    }
    let i = pack(false,true, true,0,0, 0);
    table[i]

}

fn pack(
    is_ok: bool,
    is_min: bool,
    is_max: bool, 
    previous_digit: usize,
    repetition: usize,
    idx: usize) -> usize
{
    (is_ok as usize)
    | (is_min as usize) << 1
    | (is_max as usize) << 2
    | previous_digit << 3
    | repetition << 7
    | idx << 9
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
fn repetition(state: usize) -> usize {
    (state >> 7) & 3
}

#[inline]
fn idx(state: usize) -> usize {
    state >> 9
}