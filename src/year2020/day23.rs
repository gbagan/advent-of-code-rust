pub fn solve(input: &str) -> (String, u64) {
    let input: Vec<_> = input.trim().bytes().map(|c| (c - b'1') as u32).collect();

    let p1 = part1(&input);
    let p2 = part2(&input);

    (p1, p2)
}

fn part1(input: &[u32]) -> String {
    let mut circle = make_circle(input.len(), input);
    simulate(&mut circle, 100, input[0]);

    let mut result = Vec::new();
    let mut cup = 0;
    for _ in 0..8 {
        cup = circle[cup as usize];
        result.push(cup as u8 + b'1');
    }

    String::from_utf8(result).unwrap()
}

fn part2(input: &[u32]) -> u64 {
    let mut circle = make_circle(1_000_000, input);
    simulate(&mut circle, 10_000_000, input[0]);

    let cup1 = circle[0];
    let cup2 = circle[cup1 as usize];
    (cup1 + 1) as u64 * (cup2 + 1) as u64
}



fn make_circle(n: usize, input: &[u32]) -> Vec<u32> {
    let m = input.len();
    let mut circle: Vec<_> = (1..(n+1) as u32).collect();
    
    for &[n, m] in input.array_windows() {
        circle[n as usize] = m;
    }
    
    if m == n {
        circle[input[n-1] as usize] = input[0];
    } else {
        circle[input[m-1] as usize] = m as u32;
        circle[n-1] = input[0];
    }
    circle
}

#[inline]
fn simulate(circle: &mut [u32], steps: usize, mut current: u32) {
    for _ in 0..steps {
        let top = circle.len() as u32 - 1;

        let cup1 = circle[current as usize];
        let cup2 = circle[cup1 as usize];
        let cup3 = circle[cup2 as usize];

        let mut dest = if current == 0 { top } else { current - 1 };
        while dest == cup1 || dest == cup2 || dest == cup3 {
            dest = if dest == 0 { top } else { dest - 1 }
        }

        let next = circle[cup3 as usize];
        circle[current as usize] = next;
        circle[cup3 as usize] = circle[dest as usize];
        circle[dest as usize] = cup1;
        current = next;
    }
}