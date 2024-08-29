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

pub fn solve(input: &str) -> Option<(u32, u32)> {
    let boxes: Vec<_> =
        input
        .lines()
        .filter_map(parse_box)
        .collect();
    let p1 = boxes.iter().map(paper).sum();
    let p2 = boxes.iter().map(ribbon).sum();
    Some((p1, p2))
}

fn paper (Box {l, h, w}: &Box) -> u32 {
    let areas = [l*w, l*h, w*h];
    let sum_areas: u32 = areas.into_iter().sum();
    2 * sum_areas + areas.into_iter().min().unwrap()
}

fn ribbon (Box {l, h, w}: &Box) -> u32 {
    l * h * w + 2 * (l+w).min(l+h).min(w+h)
}