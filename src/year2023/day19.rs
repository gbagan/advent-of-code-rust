use std::collections::HashMap;
use itertools::Itertools;

pub struct Input<'a> {
    pub workflows: HashMap<&'a str, Vec<Step<'a>>>,
    pub ratings: Vec<[u16; 4]>
}

pub enum Instr<'a> { Accept, Reject, Goto(&'a str) }
pub enum Test { LT(usize, u16), GT(usize, u16), Otherwise }
pub struct Step<'a> {
    pub test: Test,
    pub instr: Instr<'a>,
}

fn parse_instr<'a>(s: &'a str) -> Instr<'a> {
    match s {
        "A" => Instr::Accept,
        "R" => Instr::Reject,
        _ => Instr::Goto(&s),
    }
}

fn parse_workflow(line: &str) -> Option<(&str, Vec<Step>)> {
    let (name, line) = line.split_once('{')?;
    let workflow = line.split([',', ':', '}']).tuples().filter_map(|(first, second)| {
        if second.is_empty() {
            Some(Step{test: Test::Otherwise, instr: parse_instr(first)})
        } else {
            let (c, rel) = first.chars().next_tuple()?;
            let c = match c {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => panic!("unexcepted character {c}")
            };
            let val = first[2..].parse().ok()?;
            let test = match rel {
                '<' => Test::LT(c, val),
                '>' => Test::GT(c, val),
                _ => panic!("unexcepted character {rel}")
            };
            Some(Step{test, instr: parse_instr(second)})
        }
    }).collect();
    Some((name, workflow))
}

fn parse_rating(line: &str) -> Option<[u16; 4]> {
    let (x, m, a, s) = line.split([',', '}']).filter_map(|s| {
        let (_, v) = s.split_once('=')?;
        v.parse().ok()
    }).next_tuple()?;
    Some([x, m, a, s])
}

pub fn parse(input: &str) -> Option<Input> {
    let mut workflows = HashMap::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break
        } else if let Some((name, workflow)) = parse_workflow(line) {
            workflows.insert(name, workflow);
        }
    }
    let ratings = lines.filter_map(parse_rating).collect();
    Some(Input {workflows, ratings})
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

pub fn part1(input: &Input) -> Option<u32> {
    Some(input.ratings.iter().map(|rating|
        if accepts(rating, &input.workflows) {
            rating.iter().sum::<u16>() as u32
        } else {
            0
        }
    ).sum())
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

pub fn part2(input: &Input) -> Option<u64> {
    let mut total = 0;
    let init = [(1, 4001); 4];
    let mut stack = vec!(("in", init));
    while let Some((name, mut bx)) = stack.pop() {
        let workflow = &input.workflows[name];
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
    Some(total)
}