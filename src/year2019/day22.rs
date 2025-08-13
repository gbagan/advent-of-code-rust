use crate::util::{math::*, parser::*, power};

pub fn solve(input: &str) -> (i128, i128) {
    let mut shuffle1 = Affine::<10_007>::one();
    let mut shuffle2 = Affine::<119_315_717_514_047>::one();

    for (a, b) in input.lines().map(parse_line) {
        shuffle1 = Affine::new(a, b).compose(shuffle1);
        shuffle2 = Affine::new(a, b).compose(shuffle2);
    }

    let p1 = shuffle1.apply(2019);
    let p2 = shuffle2.power(101_741_582_076_661).inverse().apply(2020);

    (p1, p2)
}

fn parse_line(line: &str) -> (i128, i128) {
    let line = line.as_bytes();
    match line[5] {
        b'i'  => (-1, -1), // new stack
        b'w' => { // increment
            let a = (&line[20..]).to_unsigned();
            (a, 0)
        },
        _ => { // cut
            let b = -(&line[3..]).try_signed::<i128>().unwrap();
            (1, b)
        }
    }
}


#[derive (Clone, Copy)]
struct Affine<const N: i128> {
    a: i128,
    b: i128,
}

impl<const N: i128> Affine<N> {
    fn new(a: i128, b: i128) -> Self {
        Self {a, b}
    }

    fn one() -> Self {
        Self { a: 1, b: 0 }
    }

    fn compose(self, other: Self) -> Self { 
        Self {
            a: (self.a * other.a).rem_euclid(N),
            b: (self.a * other.b + self.b).rem_euclid(N)
        }
    }

    fn inverse(self) -> Self {
        let a1 = modular_inverse(self.a, N);
        Self { a: a1, b: -a1 * self.b }
    }

    fn apply(self, x: i128) -> i128 {
        (self.a * x + self.b).rem_euclid(N)
    }

    fn power(self, n: usize) -> Self {
        power(|&a, &b| b.compose(a), self, n)
    }
}