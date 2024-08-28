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

pub fn parse(input: &str) -> Option<(usize, usize)> {
    let mut p1 = 0;
    let mut boxes: Vec<Vec<Item>> = (0..256).map(|_| vec!()).collect();

    for instr in input.trim().split(',') {
        p1 += hash(instr.as_bytes());
        if let Some(label) = instr.strip_suffix('-') {
            let label = label.as_bytes();
            let hash = hash(label);
            let box_ = &mut boxes[hash];
            if let Some(i) = box_.iter().position(|item| item.label == label) {
                box_.remove(i);
            }
        } else {
            let (label, lens) = instr.split_once('=')?;
            let label = label.as_bytes();
            let lens = lens.parse().ok()?;
            let hash = hash(label);
            let box_ = &mut boxes[hash];
            if let Some(i) = box_.iter().position(|item| item.label == label) {
                box_[i].lens = lens;
            } else {
                box_.push(Item { label, lens });
            }
        }
    }

    let p2 = focusing_power(&boxes);

    Some((p1, p2))
}

pub fn part1(input: &(usize, usize)) -> Option<usize> {
    Some(input.0)
}

pub fn part2(input: &(usize, usize)) -> Option<usize> {
    Some(input.1)
}