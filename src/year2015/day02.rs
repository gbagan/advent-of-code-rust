use itertools::Itertools;

pub struct Box {
    l: u32,
    h: u32,
    w: u32,
}

fn parse_box(s: &str) -> Option<Box> {
    let mut words = s.split('x');
    let (str1, str2, str3) = words.next_tuple()?;
    let l = str1.parse().ok()?;
    let h = str2.parse().ok()?;
    let w = str3.parse().ok()?;
    Some(Box { l, h, w })
}

pub fn parse(input: &str) -> Vec<Box> {
    input
    .lines()
    .filter_map(parse_box)
    .collect()
}

fn paper (Box {l, h, w}: &Box) -> u32 {
    let areas = [l*w, l*h, w*h];
    let sum_areas: u32 = areas.into_iter().sum();
    2 * sum_areas + areas.into_iter().min().unwrap()
}

fn ribbon (Box {l, h, w}: &Box) -> u32 {
    l * h * w + 2 * (l+w).min(l+h).min(w+h)
}

pub fn part1(boxes: &Vec<Box>) -> Option<u32> {
    Some(boxes.iter().map(paper).sum())
}

pub fn part2(boxes: &Vec<Box>) -> Option<u32> {
    Some(boxes.iter().map(ribbon).sum())
}