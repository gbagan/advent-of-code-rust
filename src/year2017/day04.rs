use ahash::{HashSet, HashSetExt};

pub fn solve(input: &str) -> (u32, u32) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut words = Vec::new();
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();

    for line in input.lines() {
        words.clear();
        words.extend(line.split_ascii_whitespace().map(|w| w.as_bytes()));
        set1.clear();
        set2.clear();
        p1 += no_duplicate(&words, &mut set1) as u32;
        p2 += no_duplicate2(&words, &mut set2) as u32;
    }
    (p1, p2)
}

fn no_duplicate<'a>(words: &[&'a [u8]], set: &mut HashSet<&'a [u8]>) -> bool {
    for word in words {
        if !set.insert(word) {
            return false;
        }
    }
    true
}

fn no_duplicate2(words: &[&[u8]], set: &mut HashSet<[u8; 26]>) -> bool {
    for &word in words {
        let mut table = [0u8; 26];
        for &c in word {
            table[(c - b'a') as usize] += 1;
        }
        if !set.insert(table) {
            return false;
        }
    }
    true
}