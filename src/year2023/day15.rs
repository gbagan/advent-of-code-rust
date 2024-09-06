use std::array::from_fn;

use anyhow::*;

struct Item<'a> {
    label: &'a [u8],
    lens: usize,
}

fn hash(string: &[u8]) -> usize {
    string.iter().fold(0, |n, &c| (n + c as usize) * 17 % 256)
}

fn focusing_power(boxes: &[Vec<Item>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| 
            b.iter()
              .enumerate()
              .map(|(j, item)| (i+1) * (j+1) * item.lens)
              .sum::<usize>()
            )
        .sum()
}

pub fn solve(input: &str) -> Result<(usize, usize)> {
    let mut p1 = 0;
    let mut boxes: [Vec<Item>; 256] = from_fn(|_| vec!());

    for instr in input.trim().split(',') {
        p1 += hash(instr.as_bytes());
        if let Some(label) = instr.strip_suffix('-') {
            let label = label.as_bytes();
            let hash = hash(label);
            let bx = &mut boxes[hash];
            if let Some(i) = bx.iter().position(|item| item.label == label) {
                bx.remove(i);
            }
        } else if let Some((label, lens)) = instr.split_once('=') {
            let label = label.as_bytes();
            let lens = lens.parse()?;
            let hash = hash(label);
            let bx = &mut boxes[hash];
            if let Some(i) = bx.iter().position(|item| item.label == label) {
                bx[i].lens = lens;
            } else {
                bx.push(Item { label, lens });
            }
        } else {
            bail!("Parse error: {instr}");
        }
    }

    let p2 = focusing_power(&boxes);

    Ok((p1, p2))
}