use itertools::Itertools;

fn no_duplicate(words: &mut [Vec<u8>]) -> bool {
    words.sort_unstable();
    words.iter().tuple_windows().all(|(w1, w2)| w1 != w2)
}

pub fn parse(input: &str) -> Option<(u32, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let mut words: Vec<_> = line.split_ascii_whitespace().map(|w| w.as_bytes().to_vec()).collect();
        if no_duplicate(&mut words) {
            p1 += 1;
        }
        for word in words.iter_mut() {
            (*word).sort_unstable();
        }
        if no_duplicate(&mut words) {
            p2 += 2;
        }
    }
    Some((p1, p2))
}

pub fn part1(solutions: &(u32, u32)) -> Option<u32> {
    Some(solutions.0)
}

pub fn part2(solutions: &(u32, u32)) -> Option<u32> {
    Some(solutions.1)
}
