// perfect matching

use crate::util::{iter::*, parser::*, range::*};
use arrayvec::ArrayVec;

const N: usize = 20;

struct Field {
    departure: bool,
    range1: Range<u32>,
    range2: Range<u32>,
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

struct Input {
    fields: ArrayVec<Field, N>,
    my_ticket: ArrayVec<u32, N>,
    nearby_tickets: Vec<ArrayVec<u32, N>>,
    table: [bool; 1000]
}

pub fn solve(input: &str) -> (u32, u64) {
    let input = parse(input);
    let p1 = part1(&input);
    let p2 = part2(&input);

    (p1, p2)
}

fn parse(input: &str) -> Input {
    let mut lines = input.lines();
    //let mut fields = ArrayVec::new();
    //while let Some(line) = lines.next() && !line.is_empty() {
    //    fields.push(Field::parse(line));
    //}
    let fields: ArrayVec<Field, _> = lines.by_ref().take(N).map(Field::parse).collect();
    lines.next();
    lines.next();
    let my_ticket = lines.next().unwrap().iter_unsigned().collect();
    lines.next();
    lines.next();
    let nearby_tickets = lines.map(|line| line.iter_unsigned().collect()).collect();

    let mut table = [false; 1000];
    
    for field in &fields {
        for i in field.range1.lower..field.range1.upper {
            table[i as usize] = true;
        }
        for i in field.range2.lower..field.range2.upper {
            table[i as usize] = true;
        }
    }

    Input { fields, my_ticket, nearby_tickets, table }
}

fn part1(input: &Input) -> u32 {
    let mut p1 = 0;

    for ticket in &input.nearby_tickets {
        for &value in ticket {
            if !input.table[value as usize] {
                p1 += value;
            }
        }
    }

    p1
}

fn part2(input: &Input) -> u64 {
    let good_tickets: Vec<_> = input.nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|&v| input.table[v as usize]))
        .collect();

    let matched = |i: usize, j: usize| {
        let field = &input.fields[i];
        good_tickets.iter().all(|&ticket| {
            field.range1.contains(ticket[j]) || field.range2.contains(ticket[j])
        })
    };

    let mut graph = [0u32; N];
    for i in 0..N {
        for j in 0..N {
            graph[i] |= (matched(i, j) as u32) << j;
        }
    }

    // perfect matching

    let mut matching = [0; N];
    for _ in 0..N {
        let i = graph.iter().position(|mask| mask.count_ones() == 1).unwrap();
        let mask = graph[i];
        matching[i] = mask.trailing_zeros() as usize;
        for mask2 in &mut graph {
            *mask2 &= !mask;
        }
    }

    let p2 = (0..N)
        .filter(|&i| input.fields[i].departure)
        .map(|i| input.my_ticket[matching[i]] as u64)
        .product();

    p2
}