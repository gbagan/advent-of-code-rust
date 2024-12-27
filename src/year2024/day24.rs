use memchr::memmem;
use ahash::{HashMap, HashMapExt};

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Op {
    And, Or, Xor
}

enum Wire {
    Input(bool),
    Wire(usize, Op, usize),
}

struct LWire {
    label: (u8, u8, u8),
    wire: Wire,
}

pub fn solve(input: &str) -> (u64, String) {
    let wires = parse_wires(input);
    let p1 = part1(&wires);
    let p2 = part2(&wires);
    (p1, p2)
}

fn parse_wires(input: &str) -> Vec<LWire> {
    let mut table = HashMap::with_capacity(350);
    let mut wires = Vec::with_capacity(350);
    let limit = memmem::find(input.as_bytes(),  b"\n\n").unwrap();
    let section1 = input[..limit+1].as_bytes();
    let section2 = input[limit+2..].as_bytes();
    for &[l1, l2, l3, _, _, b, _] in section1.array_chunks() {
        table.insert((l1, l2, l3), wires.len());
        wires.push(LWire {label: (l1, l2, l3), wire: Wire::Input(b == b'1')});
    }
    let mut line = section2;
    
    while !line.is_empty() {
        assert!(line.len() >= 18);
        if line[4] == b'O' { //  OR
            let label1 = (line[0], line[1], line[2]);
            let label2 = (line[7], line[8], line[9]);
            let label3 = (line[14], line[15], line[16]);
            
            let index1 = *table.entry(label1).or_insert_with(|| {
                wires.push(LWire {label: label1, wire: Wire::Input(false)});
                wires.len() - 1
            });
            
            let index2 = *table.entry(label2).or_insert_with(|| {
                wires.push(LWire {label: label2, wire: Wire::Input(false)});
                wires.len() - 1
            });

            let index3 = *table.entry(label3).or_insert_with(|| {
                wires.push(LWire {label: label3, wire: Wire::Input(false)});
                wires.len() - 1
            });
            
            wires[index3].wire = Wire::Wire(index1, Op::Or, index2);
            line = &line[18..];
        } else { // AND and XOR
            let label1 = (line[0], line[1], line[2]);
            let label2 = (line[8], line[9], line[10]);
            let label3 = (line[15], line[16], line[17]);

            let index1 = *table.entry(label1).or_insert_with(|| {
                wires.push(LWire {label: label1, wire: Wire::Input(false)});
                wires.len() - 1
            });
            
            let index2 = *table.entry(label2).or_insert_with(|| {
                wires.push(LWire {label: label2, wire: Wire::Input(false)});
                wires.len() - 1
            });

            let index3 = *table.entry(label3).or_insert_with(|| {
                wires.push(LWire {label: label3, wire: Wire::Input(false)});
                wires.len() - 1
            });
            
            wires[index3].wire = Wire::Wire(index1, if line[4] == b'A' {Op::And} else {Op::Xor}, index2);
            line = &line[19..];
        }
    }
    wires
}

fn part1(wires: &[LWire]) -> u64 {
    let mut cache = vec![None; wires.len()];
    let mut p1 = 0;

    for (i, wire) in wires.iter().enumerate() {
        if wire.label.0 == b'z' {
            let v = calc(wires, &mut cache, i) as u64;
            let d = wire.label.1 as u32 * 10 + wire.label.2 as u32 - 528;
            p1 |= v << d;
        }
    }

    p1
}

fn calc(wires: &[LWire], cache: &mut [Option<bool>], i: usize) -> bool {
    match cache[i] {
        Some(v) => v,
        None => {
            let v= match wires[i].wire {
                Wire::Input(v) => v,
                Wire::Wire(j, Op::And, k) => calc(wires, cache, j) & calc(wires, cache, k),
                Wire::Wire(j, Op::Or, k) => calc(wires, cache, j) | calc(wires, cache, k),
                Wire::Wire(j, Op::Xor, k) => calc(wires, cache, j) ^ calc(wires, cache, k),
            };
            cache[i] = Some(v);
            v
        }
    }
}

fn part2(wires: &[LWire]) -> String {
    let mut outputs = HashMap::with_capacity(300);
    let mut swapped = Vec::with_capacity(8);
    let mut carry_in = None;

    for (i, wire) in wires.iter().enumerate() {
        if let Wire::Wire(mut index1, op, mut index2) = wire.wire {
            if index1 > index2 {
                (index1, index2) = (index2, index1);
            }
            outputs.insert((index1, op, index2), i);
        }
    }

    for x in 0..45 {
        let y = 45 + x;
        let (sum, mut carry_out) = full_adder(x, y, carry_in, wires, &outputs, &mut swapped);
        let (l1, l2, l3) = wires[carry_out].label;
        if l1 == b'z' && !(l2 == b'4' && l3 == b'5') {
            swapped.push(wires[sum].label);
            swapped.push(wires[carry_out].label);
            carry_out = sum;
        }
        carry_in = Some(carry_out);
    }

    swapped.sort_unstable();
    let mut p2 = String::with_capacity(31);
    for (i, &(l1, l2, l3)) in swapped.iter().enumerate() {
        if i > 0 {
            p2.push(',');
        }
        p2.push(l1 as char);
        p2.push(l2 as char);
        p2.push(l3 as char);
    }
    p2
}

fn output_wire(mut index1: usize, mut index2: usize, op: Op, output: &HashMap<(usize, Op, usize), usize>) -> Option<usize> {
    if index1 > index2 {
        (index1, index2) = (index2, index1);
    }
    output.get(&(index1, op, index2)).copied()
}

fn full_adder(
    x: usize,
    y: usize,
    carry_in: Option<usize>,
    wires: &[LWire],
    outputs: &HashMap<(usize, Op, usize), usize>,
    swapped: &mut Vec<(u8, u8, u8)>)
        -> (usize, usize) {
    let mut a = output_wire(x, y, Op::Xor, outputs).unwrap();
    let mut b = output_wire(x, y, Op::And, outputs).unwrap();

    match carry_in {
        None => (a, b),
        Some(carry_in) => {
            let mut c = output_wire(carry_in, a, Op::And, outputs).unwrap_or_else(|| {
                swapped.push(wires[a].label);
                swapped.push(wires[b].label);
                (b, a) = (a, b);
                output_wire(carry_in, a, Op::And, outputs).unwrap()
            });

            let mut sum = output_wire(carry_in, a, Op::Xor, outputs).unwrap();

            if wires[a].label.0 == b'z' {
                swapped.push(wires[a].label);
                swapped.push(wires[sum].label);
                sum = a;
            } else if wires[b].label.0 == b'z' {
                swapped.push(wires[b].label);
                swapped.push(wires[sum].label);
                (b, sum) = (sum, b);
            } else if wires[c].label.0 == b'z' {
                swapped.push(wires[c].label);
                swapped.push(wires[sum].label);
                (c, sum) = (sum, c);
            }

            let carry_out = output_wire(c, b, Op::Or, outputs).unwrap();
            (sum, carry_out)
        }
    }
}