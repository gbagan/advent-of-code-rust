pub fn solve(input: &str) -> (u64, u64) {
    let input = input.trim_ascii_end().as_bytes();
    let p1 = part1(input);
    let p2 = part2(input);
    (p1, p2)
}

fn part1(input: &[u8]) -> u64 {
    let mut checksum = 0;
    let mut left = 0;
    let mut right = input.len() - 1;
    let mut left_pos = 0;
    let mut available = 0;
    let mut to_copy = input.last().unwrap() - b'0';
    while left < right {
        if left.is_multiple_of(2) {
            let id = (left / 2) as u64;
            let n = (input[left] - b'0') as u64;
            checksum += id * n * (2 * left_pos + (n-1)) / 2;
            left_pos += n;
            left += 1;
            available = input[left] - b'0';
        } else if to_copy <= available {
            let id = (right / 2) as u64;
            let n = to_copy as u64;
            checksum += id * n * (2 * left_pos + (n-1)) / 2;
            left_pos += n;
            available -= to_copy;
            right -= 2;
            to_copy = input[right] - b'0';
            if available == 0 {
                left += 1;
            }
        } else { // to_copy > available
            let id = (right / 2) as u64;
            for _ in 0..available {
                checksum += id * left_pos;
                left_pos += 1;
            }
            to_copy -= available;
            left += 1;
        }
    }

    checksum
}

struct Block {
    start: u32,
    size: u32,
}

fn part2(input: &[u8]) -> u64 {
    let length = input.len();
    let mut checksum = 0;
    let mut blocks = Vec::with_capacity(input.len());
    let mut start = 0;

    for n in input {
        let size = (n - b'0') as u32;
        blocks.push(Block { start, size });
        start += size;
    }

    let mut indices = [1; 10];

    for pos in (0..blocks.len()).rev().step_by(2) {
        let block = &blocks[pos];
        let size = block.size;
        let mut index = indices[size as usize];
        while index < length && blocks[index].size < size {
            index += 2;
        }
        indices[size as usize] = index;
        let id = pos as u64 / 2;
        if index >= pos {
            for p in block.start..block.start + size {
                checksum += id * (p as u64);
            }
        } else {
            let start = blocks[index].start;
            for p in start..start+size {
                checksum += id * (p as u64);
            }
            blocks[index].start += size;
            blocks[index].size -= size;
        }
    }

    checksum
}