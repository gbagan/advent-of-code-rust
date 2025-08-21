// perfect matching
// assume that there are 20 fields and 20 values per ticket
// assume that all values are between 0 and 999

use crate::util::{iter::*, parser::*, range::*};

struct Field {
    departure: bool,
    range1: Range<usize>,
    range2: Range<usize>,
}

impl Field {
    fn parse(line: &str) -> Self {
        let (name, suffix) = line.split_once(':').unwrap();
        let departure = name.starts_with("departure");
        let (a, b, c, d) = suffix.iter_unsigned().next_tuple().unwrap();
        let range1 = Range::new(a, b);
        let range2 = Range::new(c, d);
        Self { departure, range1, range2 }
    }
}

pub fn solve(input: &str) -> (usize, u64) {
    solve_for::<20>(input)
}

fn solve_for<const N: usize>(input: &str) -> (usize, u64) {
    let mut lines = input.lines();
    
    let fields: [Field; N] = std::array::from_fn(|_| Field::parse(lines.next().unwrap()));
    
    // the i-th bit of field_masks[val] is 1 iff val is a valid value for the i-th field.
    let mut field_masks = [0; 1000];
    for (i, field) in fields.iter().enumerate() {
        field_masks[field.range1.start] += 1 << i;
        field_masks[field.range1.end+1] -= 1 << i;
        field_masks[field.range2.start] += 1 << i;
        field_masks[field.range2.end+1] -= 1 << i;
    }
    // partial sum
    let mut sum = 0;
    for mask in &mut field_masks {
        sum += *mask;
        *mask = sum;
    }


    lines.next();
    lines.next();
    let line = lines.next().unwrap();
    let mut it = line.iter_unsigned();
    let my_ticket: [u64; N] = std::array::from_fn(|_| it.next().unwrap());
    lines.next();
    lines.next();

    let mut p1 = 0;
    let mut ticket_masks = [(1u32 << N) - 1; 20];

    for line in lines {
        let mut iter = line.iter_unsigned();
        let values: [usize; 20] = std::array::from_fn(|_| iter.next().unwrap());
        
        let mut valid_ticket = true;

        for &val in &values {
            if field_masks[val] == 0 {
                valid_ticket = false;
                p1 += val;
            }
        }

        if valid_ticket {
            for (i, val) in values.into_iter().enumerate() {
                ticket_masks[i] &= field_masks[val];
            }
        }
    }

    // perfect matching

    let mut matching = [0; N];
    for _ in 0..N {
        let i = ticket_masks.iter().position(|mask| mask.count_ones() == 1).unwrap();
        let mask = ticket_masks[i];
        matching[mask.trailing_zeros() as usize] = i;
        for mask2 in &mut ticket_masks {
            *mask2 &= !mask;
        }
    }

    let p2 = (0..N)
        .filter(|&i| fields[i].departure)
        .map(|i| my_ticket[matching[i]])
        .product();

    (p1, p2)
}