use crate::util::parser::*;

pub fn solve(input: &str) -> (i64, i64) {
    let numbers: Vec<i16> = input.iter_signed().collect();
    let p1 = decrypt(&numbers, 1, 1);
    let p2 = decrypt(&numbers, 811_589_153, 10);

    (p1, p2)
}

fn decrypt(numbers: &[i16], key: i64, rounds: usize) -> i64 {   
    let len = numbers.len();
    let modulo = numbers.len() - 1;
    let zero = numbers.iter().position(|&v| v == 0).unwrap();
    let offsets: Vec<_> = numbers.iter().map(|&i| (i as i64 * key).rem_euclid(modulo as i64) as u16).collect();
    let mut indices: Vec<_> = (0..len as u16).collect();
    let mut search = indices.clone();
    
    let mut search_cost = 0;

    for _ in 0..rounds {
        for (i, &offset) in offsets.iter().enumerate() {
            if search_cost > len/2 {
				search_cost = 0;
				for j in 0..len {
					search[indices[j] as usize] = j as u16;
				}
			}

            let mut from = search[i] as usize;
			if indices[from as usize] != i as u16 {
				let mut left = from;
                let mut right = from;
				from = loop {
					search_cost += 1;
					left -= 1;
                    if left < len && indices[left] as usize == i {
						break left;
					}
                    right += 1;
					if right < len && indices[right] as usize == i {
						break right;
					}
				}
			}
            let mut to = from + offset as usize;
			if to >= modulo {
                to -= modulo
            }

			if from < to {
                rotate_left1(&mut indices[from..=to]);
				
			} else if to < from {
                rotate_right1(&mut indices[to..=from]);
			}

			search[i] = to as u16;
        }
    }

	let pzero = indices.iter().position(|&i| i == zero as u16).unwrap();

	let answer: i16 = [1000, 2000, 3000]
        .iter()
        .map(|i| numbers[indices[(pzero + i) % len] as usize])
        .sum();
	answer as i64 * key
}


fn rotate_left1(slice: &mut [u16]) {
    let first = slice[0];
    let len = slice.len();
    let mut ptr = slice.as_mut_ptr();
    unsafe {
        let end = ptr.add(len-1);
        while ptr != end {
            *ptr = *ptr.add(1);
            ptr = ptr.add(1);
        }
        *ptr = first;
    }
}

fn rotate_right1(slice: &mut [u16]) {
    let len = slice.len();
    let first = slice[len-1];
    
    unsafe {
        let mut ptr = slice.as_mut_ptr();
        let end = ptr;
        ptr = ptr.add(len-1);
        while ptr != end {
            *ptr = *ptr.sub(1);
            ptr = ptr.sub(1);
        }
        *ptr = first;
    }
}