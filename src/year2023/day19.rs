use ahash::{HashMap, HashMapExt};
use crate::util::{iter::*, parser::*};

type Workflows<'a> = HashMap<&'a [u8], Vec<Step<'a>>>;

enum Instr<'a> { Accept, Reject, Goto(&'a [u8]) }
enum Test { LT(usize, u16), GT(usize, u16), Otherwise }
struct Step<'a> {
    test: Test,
    instr: Instr<'a>,
}

pub fn solve(input: &str) -> (u32, u64) {
    let mut lines = input.lines();
    let mut workflows = HashMap::new();
    while let Some(line) = lines.next() && !line.is_empty() {
        let (k, v) = parse_workflow(line);
        workflows.insert(k, v);
    }
    let ratings: Vec<_> = lines.remainder().unwrap().iter_unsigned::<u16>().array_chunks::<4>().collect();
    let p1 = part1(&workflows, &ratings);
    let p2 = part2(&workflows);
    (p1, p2)
}


fn parse_instr(s: &[u8]) -> Instr<'_> {
    match s {
        b"A" => Instr::Accept,
        b"R" => Instr::Reject,
        _ => Instr::Goto(s),
    }
}

fn parse_workflow(line: &str) -> (&[u8], Vec<Step<'_>>) {
    let line = line.as_bytes();
    let (name, line) = line.split_once(|&c| c == b'{').unwrap();
    let workflow = line.split(|&c| matches!(c, b',' | b':' | b'}')).tuples().map(|(first, second)| {
        if second.is_empty() {
            Step{test: Test::Otherwise, instr: parse_instr(first)}
        } else {
            let c = match first[0] {
                b'x' => 0,
                b'm' => 1,
                b'a' => 2,
                b's' => 3,
                _ => panic!()
            };
            let val = (&first[2..]).try_unsigned().unwrap();
            let test = match first[1] {
                b'<' => Test::LT(c, val),
                _ => Test::GT(c, val),
            };
            Step{test, instr: parse_instr(second)}
        }
    }).collect();
    (name, workflow)
}

fn accepts(rating: &[u16; 4], workflows: &HashMap<&[u8], Vec<Step>>) -> bool {
    let mut current = b"in".as_slice();
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
    let mut stack = vec!((b"in".as_slice(), init));
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