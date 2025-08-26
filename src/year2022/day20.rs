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

/*
fn decrypt(numbers: &[i64], key: i64, rounds: usize) -> i64 {   
    let len = numbers.len();
    let modulo = numbers.len() - 1;
    let zero = numbers.iter().position(|&v| v == 0).unwrap();
    
    let numbers: Vec<_> = numbers.iter().map(|&i| i as i64 * key).collect();
    let offsets: Vec<_> = numbers.iter().map(|&i| i.rem_euclid(modulo as i64) as u16).collect();
    

    let mut ba: BlockArray = BlockArray::from_input(len);
    
    for _ in 0..rounds {
        for (i, &offset) in offsets.iter().enumerate() {
            ba.mov(i, offset as usize);
        }
    }

    let pzero = ba.position(zero);
    let a = ba.nth((pzero + 1000) % len);
    let b = ba.nth((pzero + 2000) % len);
    let c = ba.nth((pzero + 3000) % len);

    numbers[a] + numbers[b] + numbers[c]
}


const B: usize = 512;
const S: usize = 150;

struct BlockArray {
    blocks: [ArrayVec<u16, S>; B],
    positions: Vec<(u16, u16)>,
    search: [u16; B*2-1],
}

impl BlockArray {
    fn from_input(n: usize) -> Self {
        //let len = input.len();
        let mut blocks = std::array::from_fn(|_| ArrayVec::new());
        let mut positions = Vec::with_capacity(n);
        let mut search = [0; 2*B-1];
        let c = n.div_ceil(B);
        for i in 0..B {
            for (j, idx) in (i * c..((i+1) * c).min(n)).enumerate() {
                blocks[i].push(idx as u16);
                positions.push((i as u16, j as u16));
                increment_search(&mut search, i);
            }
        }

        Self { blocks, positions, search }
    }

    fn mov(&mut self, from: usize, offset: usize) {
        let len = self.positions.len();
        let (from_block, from_idx) = self.positions[from];
        
        let val = self.blocks[from_block as usize].remove(from_idx as usize);
        decrement_search(&mut self.search, from_block as usize);
        self.update_block_indices(from_block as usize);
        /*
        if from_block > 0 {
            self.rebalance(from_block as usize -1, from_block as usize);
        }
        if from_block < B as u16 - 1 {
            self.rebalance(from_block as usize, from_block as usize+1);
        }
        */

        let mut pto= self.position(from) + offset;
        if pto >= len - 1 {
            pto -= len - 1;
        }
        //pto = if from < pto { pto } else { pto };

        if pto == len - 1 {
            self.blocks[B-1].push(val);
            self.update_block_indices(B-1 as usize);
            increment_search(&mut self.search, from_block as usize);
        } else {
            let to = self.nth(pto);
            let (to_block, to_idx) = self.positions[to];
            self.blocks[to_block as usize].insert(to_idx as usize, val);
            self.update_block_indices(to_block as usize);
            increment_search(&mut self.search, to_block as usize);
            /*
            if to_block > 0 {
                self.rebalance(to_block as usize -1, to_block as usize);
            }
            if to_block < B as u16 - 1 {
                self.rebalance(to_block as usize, to_block as usize+1);
            }
            */
        }
    }

    /*
    fn find_position(&self, idx: usize) -> (u16, u16) {
        let mut total = 0;
        self.blocks.iter().enumerate().find_map(|(i, block)| {
            total += block.len();
            (idx < total).then_some((i as u16, (idx + block.len() - total) as u16))
        }).unwrap()
    }
    */

    fn update_block_indices(&mut self, i: usize) {
        for (j, &idx) in self.blocks[i].iter().enumerate() {
            self.positions[idx as usize] = (i as u16, j as u16);
        }
    }

    fn nth(&self, n: usize) -> usize {
        let mut n = n as u16;
        let mut idx = 0;
        while idx < B - 1 {
            if n < self.search[2 * idx + 1] {
                idx = 2 * idx + 1;
            } else {
                n -= self.search[2 * idx + 1];
                idx = 2 * idx + 2;
            }
        }

        self.blocks[idx - B + 1][n as usize] as usize
    }

    /*
    fn nth(&self, idx: usize) -> usize {
        let mut total = 0;
        self.blocks.iter().find_map(|block| {
            total += block.len();
            (idx < total).then(|| block[idx + block.len() - total])
        }).unwrap() as usize
    }
    */

    fn position(&self, i: usize) -> usize {
        let (block, idx) = self.positions[i];
        let mut i = B - 1 + block as usize;
        let mut n = 0;
        loop {
            if i & 1 == 0 {
                n += self.search[i - 1];
            }
            i = (i - 1) / 2;
            if i == 0 {
                break;
            }
        }

        //let n: usize = self.blocks[..block as usize].iter().map(|b| b.len()).sum();
        (n + idx) as usize
    }

    fn rebalance(&mut self, mut i: usize, mut j: usize) {
        let bi = self.blocks[i].len();
        let bj = self.blocks[j].len();
        if bi.abs_diff(bj) <= 1 {
            return
        }

        if bi < bj {
            let idx = self.blocks[j].pop().unwrap();
            self.blocks[i].push(idx);
            decrement_search(&mut self.search, j);
            increment_search(&mut self.search, i);
            self.positions[idx as usize] = (i as u16, self.blocks[i].len() as u16 - 1)
        }
    } 


    /*
    fn position(&self, i: usize) -> usize {
        let (block, idx) = self.positions[i];
        let n: usize = self.blocks[..block as usize].iter().map(|b| b.len()).sum();
        n + idx as usize
    }
    */

}


fn increment_search(search: &mut [u16], mut idx: usize) {
    idx = B - 1 + idx;
    while idx != 0 {
        search[idx] += 1;
        idx = (idx - 1) / 2;
    }
}

fn decrement_search(search: &mut [u16], mut idx: usize) {
    idx = B - 1 + idx;
    while idx != 0 {
        search[idx] -= 1;
        idx = (idx - 1) / 2;
    }
}
*/