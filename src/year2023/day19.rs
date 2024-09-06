use anyhow::*;
use std::collections::HashMap;
use itertools::Itertools;

use crate::util::parser::UnsignedIter;

type Workflows<'a> = HashMap<&'a str, Vec<Step<'a>>>;

enum Instr<'a> { Accept, Reject, Goto(&'a str) }
enum Test { LT(usize, u16), GT(usize, u16), Otherwise }
struct Step<'a> {
    test: Test,
    instr: Instr<'a>,
}

pub fn solve(input: &str) -> Result<(u32, u64)> {
    let mut workflows = HashMap::new();
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break
        } else {
            let (name, workflow) = parse_workflow(line)?;
            workflows.insert(name, workflow);
        }
    }
    let ratings: Vec<_> = lines.filter_map(parse_rating).collect();
    let p1 = part1(&workflows, &ratings);
    let p2 = part2(&workflows);
    Ok((p1, p2))
}


fn parse_instr(s: &str) -> Instr {
    match s {
        "A" => Instr::Accept,
        "R" => Instr::Reject,
        _ => Instr::Goto(s),
    }
}

fn parse_workflow(line: &str) -> Result<(&str, Vec<Step>)> {
    let error = || anyhow!("Parse error on line: {line}");

    let (name, line) = line.split_once('{').ok_or_else(error)?;
    let workflow = line.split([',', ':', '}']).tuples().map(|(first, second)| {
        if second.is_empty() {
            Ok(Step{test: Test::Otherwise, instr: parse_instr(first)})
        } else {
            let (c, rel) = first.chars().next_tuple().ok_or_else(error)?;
            let c = match c {
                'x' => 0,
                'm' => 1,
                'a' => 2,
                's' => 3,
                _ => bail!("Parse error: unexpected '{c}, expecting 'x', 'm', 'a', s'")
            };
            let val = first[2..].parse()?;
            let test = match rel {
                '<' => Test::LT(c, val),
                '>' => Test::GT(c, val),
                _ => bail!("Parse error: unexpected '{c}, expecting '<', '>'")
            };
            Ok(Step{test, instr: parse_instr(second)})
        }
    }).try_collect()?;
    Ok((name, workflow))
}

fn parse_rating(line: &str) -> Option<[u16; 4]> {
    let (x, m, a, s) = line.iter_unsigned().collect_tuple()?;
    Some([x, m, a, s])
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