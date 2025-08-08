// wheel factorization

use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, u32) {
    let numbers: Vec<u32> = input.iter_unsigned().collect();
    let n = 22 * numbers[65] + numbers[71];
    let p1 = divisor_sum(n + 836);
    let p2 = divisor_sum(n + 10551236);
    (p1, p2)
}

fn divisor_sum(mut n: u32) -> u32 {
    let mut sum = 1;

	let mut factor = |n: &mut u32, m: u32| {
		if *n % m == 0 {
			let mut mult = 1;
            let mut mk = 1;
			loop {
				*n /= m;
				mk *= m;
				mult += mk;
                if *n % m != 0 {
                    break
                }
            }
		    sum *= mult;
		}
	};

	factor(&mut n, 2);
    factor(&mut n, 3);
    factor(&mut n, 5);

	let mut m = 7;
    while m * m <= n {
        factor(&mut n, m);  // 7
        factor(&mut n, m+4); // 11
        factor(&mut n, m+6); // 13
        factor(&mut n, m+10); // 17
        factor(&mut n, m+12); // 19
        factor(&mut n, m+16); // 23
        factor(&mut n, m+22); // 29
        factor(&mut n, m+24); // 31
        m += 30;
	}

	if n > 1 { sum * (n + 1) } else { sum }
}