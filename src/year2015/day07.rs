use ahash::{HashMap, HashMapExt};
use itertools::Itertools;

#[derive(Clone, Copy)]
enum Op {
    And, Or, LShift, RShift
}

enum Wire<'a> {
    Signal(u16), Wire(&'a str)
}

enum Gate<'a> {
    Const(Wire<'a>), Gate2(Wire<'a>, Op, Wire<'a>), Not(Wire<'a>)
}

type Circuit<'a> = HashMap<&'a str,Gate<'a>>;

pub fn solve(input: &str) -> (u16, u16) {
    let mut circuit: Circuit = input.lines().map(parse_line).collect();
    let p1 = eval_circuit(&circuit);
    circuit.insert("b", Gate::Const(Wire::Signal(p1)));
    let p2 = eval_circuit(&circuit);
    (p1, p2)
}

fn parse_wire(s: &str) -> Wire<'_> {
    s.parse::<u16>()
    .map(Wire::Signal)
    .unwrap_or_else(|_| Wire::Wire(s))
}

fn parse_gate(s: &str) -> Gate<'_> {
    let mut words = s.split_ascii_whitespace();
    let first = words.next().unwrap();
    if first == "NOT" {
        let second = words.next().unwrap();
        let wire = parse_wire(second);
        Gate::Not(wire)
    } else {
        match words.next_tuple() {
            None => Gate::Const(parse_wire(first)),
            Some ((second, third)) => {
                let wire1 = parse_wire(first);
                let wire2 = parse_wire(third);
                match second {
                    "AND" => Gate::Gate2(wire1, Op::And, wire2),
                    "OR" => Gate::Gate2(wire1, Op::Or, wire2),
                    "LSHIFT" => Gate::Gate2(wire1, Op::LShift, wire2),
                    "RSHIFT" => Gate::Gate2(wire1, Op::RShift, wire2),
                    _ => panic!("Unexpected '{second}', expecting AND, OR, LSHIFT, RSHIFT")
                }
            }
        }
    }
}

fn parse_line(line: &str) -> (&str, Gate<'_>) {
    let (s1, s2) = line.split_once(" -> ").unwrap();
    let gate = parse_gate(s1);
    (s2, gate)
}

fn eval_circuit(circuit: &Circuit) -> u16 {
    let mut vals = HashMap::new();
    
    fn get_val<'a>(circuit: &'a Circuit, vals: &mut HashMap<&'a str,u16>, label: &'a str) -> u16 {
        match vals.get(label) {
            Some(val) => *val,
            None => {
                let val = eval_gate(circuit, vals, &circuit[&label]);
                vals.insert(label, val);
                val
            }
        }
    }

    fn eval_gate<'a>(circuit: &'a Circuit, vals: &mut HashMap<&'a str,u16>, gate: &'a Gate) -> u16 {
        match gate {
            Gate::Const(wire) => eval_wire(circuit, vals, wire),
            Gate::Gate2(wire1, op, wire2) => eval_op (circuit, vals, wire1, *op, wire2),
            Gate::Not(wire) => ! eval_wire(circuit, vals, wire),
        }
    }

    fn eval_wire<'a>(circuit: &'a Circuit, vals: &mut HashMap<&'a str,u16>, wire: &'a Wire) -> u16 {
        match wire {
            Wire::Signal(n) => *n,
            Wire::Wire(label) => get_val(circuit, vals, label)
        }
    }

    fn eval_op<'a>(circuit: &'a Circuit, vals: &mut HashMap<&'a str,u16>, wire1: &'a Wire, op: Op, wire2: &'a Wire) -> u16 {
        let val1 = eval_wire(circuit, vals, wire1);
        let val2 = eval_wire(circuit, vals, wire2);
        match op {
            Op::And => val1 & val2,
            Op::Or => val1 | val2,
            Op::LShift => val1 << val2,
            Op::RShift => val1 >> val2,
        }
    }
    
    get_val(circuit, &mut vals, "a")
}