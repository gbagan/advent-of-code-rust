pub fn solve(input: &str) -> (String, String) {
    let words: Vec<_> = input.lines().map(str::as_bytes).collect();
    let mut p1 = String::new();
    let mut p2 = String::new();
    for i in 0..words[0].len() {
        let (most_common, least_common) = most_and_least_common(&words, i);
        p1.push(most_common as char);
        p2.push(least_common as char);
    }
    (p1, p2)
}

fn most_and_least_common(words: &[&[u8]], i: usize) -> (u8, u8) {
    let mut freqs = [0u32; 26];
    for &word in words {
        freqs[(word[i] - b'a') as usize] += 1;
    }
    let most_common = (b'a'..b'z'+1).max_by_key(|&c| freqs[(c - b'a') as usize]).unwrap();
    let least_common = (b'a'..b'z'+1).min_by_key(|&c| freqs[(c - b'a') as usize]).unwrap();
    (most_common, least_common)
}