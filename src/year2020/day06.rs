pub fn solve(input: &str) -> (u32, u32) {
    let masks: Vec<u32> = input
        .lines()
        .map(|line|
            line.bytes().fold(0, |acc, c| acc | 1 << (c - b'a'))
        ).collect();
 
    let mut p1 = 0;
    let mut p2 = 0;
    let mut anyone = 0u32;
    let mut everyone = u32::MAX;

    for mask in masks {
        if mask == 0 {
            p1 += anyone.count_ones();
            p2 += everyone.count_ones();
            anyone = 0;
            everyone = u32::MAX;
        } else {
            anyone |= mask;
            everyone &= mask;
        }
    }

    p1 += anyone.count_ones();
    p2 += everyone.count_ones();

    (p1, p2)
}
