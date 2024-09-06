use anyhow::*;
use std::collections::HashMap;
use itertools::Itertools;

enum Op {
    And, Or, LShift, RShift
}

enum Wire {
    Signal(u16), Wire(String)
}

enum Gate {
    Const(Wire), Gate2(Wire, Op, Wire), Not(Wire)
}

type Circuit = HashMap<String,Gate>;

fn parse_wire(s: &str) -> Wire {
    s.parse::<u16>()
    .map(Wire::Signal)
    .unwrap_or_else(|_| Wire::Wire(s.to_string()))
}

fn parse_gate(s: &str) -> Option<Gate> {
    let mut words = s.split_ascii_whitespace();
    let first = words.next()?;
    if first == "NOT" {
        let second = words.next()?;
        let wire = parse_wire(second);
        Some(Gate::Not(wire))
    } else {
        match words.next_tuple() {
            None => Some(Gate::Const(parse_wire(first))),
            Some ((second, third)) => {
                let wire1 = parse_wire(first);
                let wire2 = parse_wire(third);
                match second {
                    "AND" => Some(Gate::Gate2(wire1, Op::And, wire2)),
                    "OR" => Some(Gate::Gate2(wire1, Op::Or, wire2)),
                    "LSHIFT" => Some(Gate::Gate2(wire1, Op::LShift, wire2)),
                    "RSHIFT" => Some(Gate::Gate2(wire1, Op::RShift, wire2)),
                    _ => None
                }
            }
        }
    }
}

fn parse_line(line: &str) -> Option<(String, Gate)> {
    let (s1, s2) = line.split_once(" -> ")?;
    let gate = parse_gate(s1)?;
    Some((s2.to_string(), gate))
}

fn eval_circuit(circuit: &Circuit) -> u16 {
    let mut vals = HashMap::new();
    
    fn get_val(circuit: &Circuit, vals: &mut HashMap<String,u16>, label: String) -> u16 {
        match vals.get(&label) {
            Some(val) => *val,
            None => {
                let val = eval_gate(circuit, vals, &circuit[&label]);
                vals.insert(label, val);
                val
            }
        }
    }

    fn eval_gate(circuit: &Circuit, vals: &mut HashMap<String,u16>, gate: &Gate) -> u16 {
        match gate {
            Gate::Const(wire) => eval_wire(circuit, vals, wire),
            Gate::Gate2(wire1, op, wire2) => eval_op (circuit, vals, wire1, op, wire2),
            Gate::Not(wire) => ! eval_wire(circuit, vals, wire),
        }
    }

    fn eval_wire(circuit: &Circuit, vals: &mut HashMap<String,u16>, wire: &Wire) -> u16 {
        match wire {
            Wire::Signal(n) => *n,
            Wire::Wire(label) => get_val(circuit, vals, label.clone())
        }
    }

    fn eval_op(circuit: &Circuit, vals: &mut HashMap<String,u16>, wire1: &Wire, op: &Op, wire2: &Wire) -> u16 {
        let val1 = eval_wire(circuit, vals, wire1);
        let val2 = eval_wire(circuit, vals, wire2);
        match op {
            Op::And => val1 & val2,
            Op::Or => val1 | val2,
            Op::LShift => val1 << val2,
            Op::RShift => val1 >> val2,
        }
    }
    
    get_val(circuit, &mut vals, "a".to_string())
}

pub fn solve(input: &str) -> Result<(u16, u16)> {
    let mut circuit: HashMap<String, Gate> = input
        .lines()
        .map(|line| parse_line(line).ok_or_else(|| anyhow!("Parsing error: ")))
        .try_collect()?;
    let p1 = eval_circuit(&circuit);
    circuit.insert("b".to_string(), Gate::Const(Wire::Signal(p1)));
    let p2 = eval_circuit(&circuit);
    Ok((p1, p2))
}