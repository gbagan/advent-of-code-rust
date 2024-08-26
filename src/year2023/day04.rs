fn card_score(table: &mut [u16; 100], i: usize, line: &str) -> Option<u32> {
    let i = i as u16;
    let mut score = 0;
    let (left, right) = line.split_once('|')?;
    for v in left.split_ascii_whitespace().skip(2) {
        if let Ok(v) = v.parse::<usize>() {
            table[v] = i;
        }
    }
    for v in right.split_ascii_whitespace() {
        if let Ok(v2) = v.parse::<usize>() {
            if table[v2] == i {
                score += 1;
            }
        }
    }
    Some(score)
}

pub fn parse(input: &str) -> Option<Vec<u32>> {
    let mut table = [0; 100];

    Some(input
        .lines()
        .enumerate()
        .filter_map(|(i, line)| card_score(&mut table, i+1, line))
        .collect()
    )
}

pub fn part1(scores: &[u32]) -> Option<u32> {
    Some(scores.iter()
        .map(|&s| if s == 0 {0} else {2_u32.pow(s-1)} )
        .sum()
    )
}

pub fn part2(scores: &[u32]) -> Option<u32> {
    let mut vec: Vec<_> = scores.iter().map(|&s| (s, 1)).collect();
    let mut total = 0;
    let mut i = 0;
    while i < vec.len() {
        let (score, freq) = vec[i];
        total += freq;
        for j in i+1..=i+score as usize {
            let (score2, freq2) = vec[j];
            vec[j] = (score2, freq2+freq);
        }
        i+=1;
    }
    Some(total)
}