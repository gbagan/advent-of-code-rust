use std::time::Instant;
use std::collections::HashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, u16},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded,tuple},
    IResult,
};

pub enum Op {
    And, Or, LShift, RShift
}

pub enum Wire {
    Signal(u16), Wire(String)
}

pub enum Gate {
    Const(Wire), Gate2(Wire, Op, Wire), Not(Wire)
}

type Circuit = HashMap<String,Gate>;

fn input_parser(input: &str) -> IResult<&str, Circuit> {
    fn wire(input: &str) -> IResult<&str, Wire> {
        alt((map(u16,|x| Wire::Signal(x)),
            map(alpha1, |x:&str| Wire::Wire(x.to_string()))
        ))(input)
    }

    fn operator(input: &str) -> IResult<&str, Op> {
        alt((map(tag(" AND "), |_| Op::And),
            map(tag(" OR "), |_| Op::Or),
            map(tag(" LSHIFT "), |_| Op::LShift),
            map(tag(" RSHIFT "), |_| Op::RShift)
            ))(input)
        }

    fn gate2(input: &str) -> IResult<&str, Gate> {
        let (input, (wire1, op, wire2)) = tuple((wire, operator, wire))(input)?;
        Ok((input, Gate::Gate2(wire1, op, wire2)))
    }

    fn gate(input: &str) -> IResult<&str, Gate> {
        alt((map(preceded(tag("NOT "), wire), Gate::Not),
            gate2,
            map(wire, Gate::Const)
            ))(input)
    }
    
    fn instr(input: &str) -> IResult<&str, (String, Gate)> {
        let (input, g) = gate(input)?;
        let (input, _) = tag(" -> ")(input)?;
        let (input, label) = alpha1(input)?;
        Ok((input, (label.to_string(), g)))
    }

    let (input, vec) = separated_list1(char('\n'), instr)(input)?;
    let mut circuit = HashMap::new();
    for (label, g) in vec {
        circuit.insert(label.to_string(), g);
    }
    Ok((input, circuit))
}

fn eval_circuit(circuit: &Circuit) -> u16 {
    let mut vals = HashMap::new();
    
    fn get_val(circuit: &Circuit, vals: &mut HashMap<String,u16>, label: String) -> u16 {
        match vals.get(&label) {
            Some(val) => *val,
            None => {
                let val = eval_gate(circuit, vals, circuit.get(&label).unwrap());
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
        println!("{} {}", val1, val2);
        match op {
            Op::And => val1 & val2,
            Op::Or => val1 | val2,
            Op::LShift => val1 << val2,
            Op::RShift => val1 >> val2,
        }
    }
    
    get_val(&circuit, &mut vals, "a".to_string())

}


fn main() {
    let input = include_str!("../../inputs/2015/07");

    match input_parser(input) {
        Err(_) => println!("parsing error"),
        Ok ((_, mut circuit)) => {
            let start = Instant::now();
            let p1 = eval_circuit(&circuit);
            circuit.insert("b".to_string(), Gate::Const(Wire::Signal(p1)));
            let p2 = eval_circuit(&circuit);
            let end = start.elapsed().as_micros();
        
            println!("Part 1: {}", p1);
            println!("Part 2: {}", p2);
            println!("Time: {} μs", end);
        }
    }
}