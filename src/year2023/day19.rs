use ahash::HashMap;
use itertools::Itertools;
use memchr::memmem;
use crate::util::parser::*;

type Workflows<'a> = HashMap<&'a str, Vec<Step<'a>>>;

enum Instr<'a> { Accept, Reject, Goto(&'a str) }
enum Test { LT(usize, u16), GT(usize, u16), Otherwise }
struct Step<'a> {
    test: Test,
    instr: Instr<'a>,
}

pub fn solve(input: &str) -> (u32, u64) {
    // todo
    let sep = memmem::find(input.as_bytes(), b"\n\n").unwrap();
    let input1 = &input[..sep];
    let input2 = &input[sep+2..];
    let workflows = input1.lines().map(parse_workflow).collect();
    let ratings: Vec<_> = input2.iter_unsigned::<u16>().array_chunks::<4>().collect();
    let p1 = part1(&workflows, &ratings);
    let p2 = part2(&workflows);
    (p1, p2)
}


fn parse_instr(s: &str) -> Instr<'_> {
    match s {
        "A" => Instr::Accept,
        "R" => Instr::Reject,
        _ => Instr::Goto(s),
    }
}

fn parse_workflow(line: &str) -> (&str, Vec<Step<'_>>) {
    let (name, line) = line.split_once('{').unwrap();
    let workflow = line.split([',', ':', '}']).tuples().map(|(first, second)| {
        if second.is_empty() {
            Step{test: Test::Otherwise, instr: parse_instr(first)}
        } else {
            let (c, rel) = first.chars().next_tuple().unwrap();
            let c = match c {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!()
            };
            let val = (&first[2..]).try_unsigned().unwrap();
            let test = match rel {
                '<' => Test::LT(c, val),
                _ => Test::GT(c, val),
            };
            Step{test, instr: parse_instr(second)}
        }
    }).collect();
    (name, workflow)
}

fn accepts(rating: &[u16; 4], workflows: &HashMap<&str, Vec<Step>>) -> bool {
    let mut current = "in";
    loop {
        let workflow = &workflows[current];
        for step in workflow {
            let res = match step.test {
                Test::LT(idx, val) => rating[idx] < val,
                Test::GT(idx, val) => rating[idx] > val,
                Test::Otherwise => true
            };
            if res {
                match step.instr {
                    Instr::Accept => return true,
                    Instr::Reject => return false,
                    Instr::Goto(next) => { current = next; break }
                }
            }
        }
    }
}

fn part1(workflows: &Workflows, ratings: &[[u16;4]]) -> u32 {
    ratings.iter().map(|rating|
        if accepts(rating, workflows) {
            rating.iter().sum::<u16>() as u32
        } else {
            0
        }
    ).sum()
}

type Box = [(u16, u16); 4];

fn partition(bx: &Box, test: &Test) -> (Option<Box>, Option<Box>) {
    match test {
        Test::LT(idx, val) => {
            let val = *val;
            let r = bx[*idx];
            if val < r.0 {
                (None, Some(*bx))
            } else if val >= r.1 {
                (Some(*bx), None)
            } else {
                let mut box1 = *bx;
                let mut box2 = *bx;
                box1[*idx] = (r.0, val);
                box2[*idx] = (val, r.1);
                (Some(box1), Some(box2))
            }
        }
        Test::GT(idx, val) => {
            let val = val + 1;
            let r = bx[*idx];
            if val >= r.1 {
                (None, Some(*bx))
            } else if val < r.0 {
                (Some(*bx), None)
            } else {
                let mut b1 = *bx;
                let mut b2 = *bx;
                b1[*idx] = (val, r.1);
                b2[*idx] = (r.0, val);
                (Some(b1), Some(b2))
            }
        }
        Test::Otherwise => {
            (Some(*bx), None)
        }
    }
}

fn box_size(bx: &Box)  -> u64 {
    bx.iter().map(|r| (r.1 - r.0) as u64).product()
}

fn part2(workflows: &Workflows) -> u64 {
    let mut total = 0;
    let init = [(1, 4001); 4];
    let mut stack = vec!(("in", init));
    while let Some((name, mut bx)) = stack.pop() {
        let workflow = &workflows[name];
        for step in workflow {
            let (accepted, rejected) = partition(&bx, &step.test);
            if let Some(bx) = accepted {
                match step.instr {
                    Instr::Accept => total += box_size(&bx),
                    Instr::Reject => (),
                    Instr::Goto(next) => stack.push((next, bx)),
                }
            }
            if let Some(bx2) = rejected {
                bx = bx2
            } else {
                break
            }
        }
    }
    total
}