use crate::util::bits::*;

pub fn solve(input: &str) -> (u32, u32) {
    let input = parse_input(input);

    let state = State { 
        position: 0,
        minutes: 30,
        pressure: 0,
        remaining_valves: (1 << input.size) - 2,
    };
    let mut p1 = 0;
    let mut update = |pressure, _| {
        p1 = p1.max(pressure);
        p1
    };
    branch_and_bound(&input, state, &mut update);

    let state = State { 
        position: 0,
        minutes: 26,
        pressure: 0,
        remaining_valves: (1 << input.size) - 2,
    };
    let mut you = 0;
    let mut remaining = 0;
    let mut update = |pressure, rem| {
        if pressure > you {
            you = pressure;
            remaining = rem;
        }
        you    
    };
    branch_and_bound(&input, state, &mut update); 
    
    let state = State { 
        position: 0,
        minutes: 26,
        pressure: 0,
        remaining_valves: remaining,
    };
    let mut elephant = 0;
    let mut update = |pressure, _| {
        elephant = elephant.max(pressure);
        elephant
    };
    branch_and_bound(&input, state, &mut update); 

    /*
    let mut scores = vec![0; 1 << (input.size - 1)];
    let state = State { 
        position: 0,
        minutes: 26,
        pressure: 0,
        remaining_valves: remaining,
    };  
    let mut update = |pressure, remaining| {    
        let rem = (remaining >> 1) as usize;
        scores[rem] = scores[rem].max(pressure);
        elephant
    };
    branch_and_bound(&input, state, &mut update);
    */

    let p2 = you + elephant;

    (p1, p2)
} 


struct Valve {
    id: usize,
    rate: u32,
    neighbors: Vec<usize>,
}

impl Valve {
    fn parse(line: &str) -> Self {
        let line = line.as_bytes();
        let id = 26 * line[6] as usize + line[7] as usize - 1755;
        let rate = (line[23] - b'0') as u32;
        let rate = if line[24] == b';' { rate } else { rate * 10 + (line[24] - b'0') as u32 };

        let neighbors = line[48..]
            .split(|c| !c.is_ascii_uppercase())
            .filter(|w| !w.is_empty())
            .map(|w| 26 * w[0] as usize + w[1] as usize - 1755)
            .collect();

        Self { id, rate, neighbors }

    }
}


struct Input {
    valves: Vec<Valve>,
    distance: Vec<u32>,
    size: usize,
}

struct State {
    remaining_valves: u64,
    position: usize,
    minutes: u32,
    pressure: u32,
}


fn parse_input(input: &str) -> Input {
    let mut table = [usize::MAX; 676];

    let mut valves: Vec<_> = input.lines().map(Valve::parse).collect();
    valves.sort_unstable_by_key(|v|  if v.id == 0 {0} else { u32::MAX - v.rate });
    for (i, valve) in valves.iter().enumerate() {
        table[valve.id] = i;
    }

    for valve in valves.iter_mut() {
        valve.id = table[valve.id];
        for nbor in valve.neighbors.iter_mut() {
            *nbor = table[*nbor];
        }
    }

    let size = valves.iter().filter(|v| v.rate > 0).count() + 1;
    let mut distance = vec![u32::MAX; size * size];

    // bfs on all non null valves and the starting valve AA
    let mut seen = vec![u8::MAX; valves.len()];
    let mut queue1 = Vec::new();
    let mut queue2 = Vec::new();

    for from in 0..size {
        let mut dist = 0;
        queue1.push(from);
        seen[from] = from as u8;
        while !queue1.is_empty() {
            for &valve in &queue1 {
                if valve < size {
                    distance[from * size + valve] = dist + 1;
                }
                for &next in &valves[valve].neighbors {
                    if seen[next] != from as u8 {
                        seen[next] = from as u8;
                        
                        queue2.push(next);
                    }
                }
            }
            std::mem::swap(&mut queue1, &mut queue2);
            queue2.clear();
            dist += 1;
        }
    }

    Input { valves, distance, size }
}

fn branch_and_bound(input: &Input, state: State, update: &mut impl FnMut(u32, u64) -> u32) {
    let State{ position, minutes, remaining_valves, pressure } = state;
    
    let score = update(pressure, remaining_valves);

    for valve in remaining_valves.bits() {
        let duration = input.distance[position * input.size + valve];
        if duration >= minutes {
            continue;
        }
        let minutes = minutes - duration;
        let remaining_valves = remaining_valves ^ (1 << valve);
        let pressure = pressure + minutes * input.valves[valve].rate;

        let upper_bound = {
            let mut minutes = minutes;
            let mut pressure = pressure;

            for valve in remaining_valves.bits() {
                if minutes <= 3 {
                    break;
                }
                minutes -= 3;
                pressure += minutes * input.valves[valve].rate;
            }
            pressure
        };

        if upper_bound > score {   
            let next = State { position: valve, remaining_valves, minutes, pressure };
            branch_and_bound(input, next, update);
        }
    }
}