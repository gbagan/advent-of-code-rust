use std::array::from_fn;
use arrayvec::ArrayVec;

use crate::util::parser::*;

const CAP: usize = 8;

struct Item<'a> {
    label: &'a [u8],
    lens: u32,
}

fn hash(string: &[u8]) -> u32 {
    string.iter().fold(0, |n, &c| (n + c as u32) * 17 & 255)
}

fn focusing_power(boxes: &[ArrayVec<Item, CAP>]) -> u32 {
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| 
            b.iter()
              .enumerate()
              .map(|(j, item)| ((i+1) * (j+1)) as u32 * item.lens)
              .sum::<u32>()
            )
        .sum()
}

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut boxes: [ArrayVec<Item, CAP>; 256] = from_fn(|_| ArrayVec::new());

    for instr in input.trim().split(',') {
        p1 += hash(instr.as_bytes());
        if let Some(label) = instr.strip_suffix('-') {
            let label = label.as_bytes();
            let hash = hash(label);
            let bx = &mut boxes[hash as usize];
            if let Some(i) = bx.iter().position(|item| item.label == label) {
                bx.remove(i);
            }
        } else {
            let (label, lens) = instr.split_once('=').unwrap();
            let label = label.as_bytes();
            let lens = lens.try_unsigned().unwrap();
            let hash = hash(label);
            let bx = &mut boxes[hash as usize];
            if let Some(item) = bx.iter_mut().find(|item| item.label == label) {
                item.lens = lens;
            } else {
                bx.push(Item { label, lens });
            }
        }
    }

    let p2 = focusing_power(&boxes);

    (p1, p2)
}