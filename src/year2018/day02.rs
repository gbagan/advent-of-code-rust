use ahash::{HashSet, HashSetExt};

pub fn solve(input: &str) -> (u32, String) {
    let ids: Vec<_> = input.lines().map(str::as_bytes).collect();

    let p1 = part1(&ids);
    let p2 = part2(&ids);

    (p1, p2)
}

fn part1(ids: &[&[u8]]) -> u32 {
    let mut count2 = 0;
    let mut count3 = 0;

    for &id in ids {
        let mut freq = [0; 26];
        for &c in id.iter() {
            freq[(c - b'a') as usize] += 1;
        }
        count2 += freq.contains(&2) as u32;
        count3 += freq.contains(&3) as u32;

    }


    count2 * count3
}

fn part2(ids: &[&[u8]]) -> String {
    let mut seen = HashSet::with_capacity(ids.len());
    
    for i in 0..ids[0].len() - 1 {
        for &id in ids {
            if !seen.insert((&id[..i], &id[i+1..])) {
                let mut res = String::with_capacity(id.len()-1);
                for &c in &id[..i] {
                    res.push(c as char);
                }
                for &c in &id[i+1..] {
                    res.push(c as char);
                }
                return res;
            }
        }
        seen.clear();
    }

    unreachable!();

}