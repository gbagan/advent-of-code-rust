use itertools::Itertools;

pub fn solve(input: &str) -> Option<(usize, u32)> {
    let mut p1 = 0;
    let mut p2 = 0;
    for (i, line) in input.lines().enumerate() {
        let mut valid = true;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for (n, color) in line.split(' ').skip(2).tuples() {
            let n = n.parse().unwrap_or(0); 
            let color = color.trim_end_matches([',', ';']);
            match color {
                "red" => {
                    valid = valid && n <= 12;
                    r = r.max(n);
                }
                "green" => {
                    valid = valid && n <= 13;
                    g = g.max(n);
                }
                "blue" => {
                    valid = valid && n <= 14;
                    b = b.max(n);
                }
                _ => ()
            }
        }

        if valid {
            p1 += i+1;
        }

        p2 += r * g * b;

    }
    Some((p1, p2))
}
