// perfect matching

use crate::util::parser::*;

pub fn solve(input: &str) -> (u32, usize) {
    let mut lines = input.lines();

    let mut p1 = 0;
    let mut masks: [u32; 16] = [0xffff; 16];

    while let Some(line1) = lines.next() && !line1.is_empty() {
        let line2 = lines.next().unwrap();
        let line3 = lines.next().unwrap();
        lines.next();
        let before = line1.iter_unsigned::<usize>().next_chunk::<4>().unwrap();
        let [name, a, b, c] = line2.iter_unsigned::<usize>().next_chunk().unwrap();
        let after = line3.iter_unsigned::<usize>().next_chunk::<4>().unwrap();
        let ra = before[a];
        let rb = before[b];
        let rc = after[c];
        let mut mask = 0u32;
        mask |= (rc == (ra + rb)) as u32;
		mask |= ((rc == (ra + b)) as u32) << 1;
		mask |= ((rc == (ra * rb)) as u32) << 2;
		mask |= ((rc == (ra * b)) as u32) << 3;
		mask |= ((rc == (ra & rb)) as u32) << 4;
		mask |= ((rc == (ra &  b)) as u32) <<  5;
		mask |= ((rc == (ra | rb)) as u32) <<  6;
		mask |= ((rc == (ra | b)) as u32) <<  7;
		mask |= ((rc == ra) as u32) <<  8;
		mask |= ((rc == a) as u32) << 9;
		mask |= ((rc == (a > rb) as usize) as u32) << 10;
		mask |= ((rc == (ra > b) as usize) as u32) << 11;
		mask |= ((rc == (ra > rb) as usize) as u32) << 12;
		mask |= ((rc == (a == rb) as usize) as u32) << 13;
		mask |= ((rc == (ra == b) as usize) as u32) << 14;
		mask |= ((rc == (ra == rb) as usize) as u32) << 15;
        masks[name] &= mask;
        if mask.count_ones() >= 3 {
            p1 += 1;
        }
    }

    // find a perfect matching
    let mut name_to_code = [0; 16];
    for _ in 0..16 {
        let i = masks.iter().position(|mask| mask.count_ones() == 1).unwrap();
        let mask = masks[i];
        name_to_code[i] = mask.trailing_zeros() as usize;
        for mask2 in &mut masks {
            *mask2 &= !mask;
        }
    }

    let input  = lines.remainder().unwrap();
    
    let mut registers = [0, 0, 0, 0];

    for [name, a, b, c] in input.iter_unsigned().array_chunks() {
        registers[c] = do_instr(name_to_code[name], &registers, a, b);
    }

    let p2 = registers[0];

    (p1, p2)

}

fn do_instr(opcode: usize, registers: &[usize; 4], a: usize, b: usize) -> usize {
    match opcode {
        0 => registers[a] + registers[b],             // addr
        1 => registers[a] + b,                        // addi
        2 => registers[a] * registers[b],             // mulr
        3 => registers[a] * b,                        // muli
        4 => registers[a] & registers[b],             // banr
        5 => registers[a] & b,                        // bani
        6 => registers[a] | registers[b],             // borr
        7 => registers[a] | b,                        // bori
        8 => registers[a],                            // setr
        9 => a,                                       // seti
        10 => (a > registers[b]) as usize,            // gtir
        11 => (registers[a] > b) as usize,            // gtri
        12 => (registers[a] > registers[b]) as usize, // gtri
        13 => (a == registers[b]) as usize,           // eqir
        14 => (registers[a] == b) as usize,           // eqri
        _ => (registers[a] == registers[b]) as usize, // eqri
    }
}