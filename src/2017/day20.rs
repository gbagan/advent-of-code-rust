use aoc::aoc_with_parser;
use aoc::coord::Coord3;
use nom::{
    bytes::complete::tag,
    character::complete::{char, line_ending, i64},
    multi::separated_list1,
    combinator::map,
    sequence::tuple,
    IResult,
};
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Particle {
    p: Coord3,
    v: Coord3,
    a: Coord3
}

impl Particle {
    fn move_(&mut self) {
        let v = self.v + self.a;
        self.p = self.p+v;
        self.v = v; 
    }
}

fn input_parser(input: &str) -> IResult<&str, Vec<Particle>> {
    fn coord(input: &str) -> IResult<&str, Coord3> {
        map(
            tuple((char('<'), i64, char(','), i64, char(','), i64, char('>'))),
            |(_, x, _, y, _, z, _)| Coord3::new(x, y, z)
        )(input)
    }
    let row = map(
        tuple((tag("p="), coord, tag(", v="), coord, tag(", a="), coord)),
        |(_, p, _, v, _, a)| Particle {p, v, a});

    separated_list1(line_ending, row)(input)
}

fn part1(particles: &Vec<Particle>) -> usize {
    let origin = Coord3::origin();
    particles
    .iter()
    .enumerate()
    .min_by(|(_, a), (_, b)|  a.a.manhattan(&origin).cmp(&b.a.manhattan(&origin)))
    .unwrap()
    .0
}

fn part2(particles: &Vec<Particle>) -> usize {
    let mut particles: Vec<_> = particles.clone();
    for _ in 0..100 {
        particles = particles
                    .iter()
                    .sorted_by_key(|p| p.p)
                    .group_by(|p| p.p)
                    .into_iter()
                    .filter_map(|(_, gr)| {
                        let gr: Vec<_> = gr.collect();
                        if gr.len() == 1 { Some(gr[0].clone()) } else { None }
                    })
                    .collect();
        for p in &mut particles {
            p.move_();
        }
    }
    particles.len()
}

fn main() {
    let input = include_str!("../../inputs/2017/20");
    aoc_with_parser(input, input_parser, |particles| {
        (part1(&particles), part2(&particles))  
    })
}