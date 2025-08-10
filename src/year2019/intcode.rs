use std::collections::VecDeque;
use crate::util::parser::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status { Halt, Input, Output(i64) }

#[derive(Clone)]
pub struct IntCode {
    data: Vec<usize>,
    ip: usize,
    base: usize,
    input: VecDeque<usize>,
}

impl IntCode {
    pub fn new(input: &str) -> Self {
        let mut data: Vec<_> = input.iter_signed::<i64>().map(|x| x as usize).collect();
        let n = data.len();
        data.resize(n + 2000, 0);
        Self { data, ip: 0, base: 0, input: VecDeque::new() }
    }

    pub fn set(&mut self, index: usize, val: usize) {
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> usize {
        self.data[index]
    }


    pub fn run(&mut self) -> Status {
        loop {
            // avoid modulo and division
            let mut instr = self.data[self.ip];
            let mut mode1 = 0;
            let mut mode2 = 0;
            let mut mode3 = 0;
            if instr >= 20000 {
                instr -= 20000;
                mode3 = 2;
            }
            if instr >= 2000 {
                instr -= 2000;
                mode2 = 2;
            } else if instr >= 1000 {
                instr -= 1000;
                mode2 = 1;
            }
            if instr >= 200 {
                instr -= 200;
                mode1 = 2;
            } else if instr >= 100 {
                instr -= 100;
                mode1 = 1;
            }

            match instr {
                1 => {
                    let a = self.address(1, mode1);
                    let b = self.address(2, mode2);
                    let c = self.address(3, mode3);
                    self.data[c] = self.data[a] + self.data[b];
                    self.ip += 4;
                },
                2 => {
                    let a = self.address(1, mode1);
                    let b = self.address(2, mode2);
                    let c = self.address(3, mode3);
                    self.data[c] = self.data[a] * self.data[b];
                    self.ip += 4;
                }
                3 => {
                    if let Some(value) = self.input.pop_front() {
                        let a = self.address(1, mode1);
                        self.data[a] = value;
                        self.ip += 2;
                    } else {
                        return Status::Input;
                    }
                }
                4 => {
                    let a = self.address(1, mode1);
                    self.ip += 2;
                    return Status::Output(self.data[a] as i64);
                }
                5 => {
                    let a = self.address(1, mode1);
                    if self.data[a] != 0 {
                        let b = self.address(2, mode2);
                        self.ip = self.data[b] as usize;
                    } else {    
                        self.ip += 3;
                    }
                }
                6 => {
                    let a = self.address(1, mode1);
                    if self.data[a] == 0 {
                        let b = self.address(2, mode2);
                        self.ip = self.data[b] as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let a = self.address(1, mode1);
                    let b = self.address(2, mode2);
                    let c = self.address(3, mode3);
                    self.data[c] = ((self.data[a] as i64) < (self.data[b] as i64)) as usize;
                    self.ip += 4;
                }
                8 => {
                    let a = self.address(1, mode1);
                    let b = self.address(2, mode2);
                    let c = self.address(3,  mode3);
                    self.data[c] = (self.data[a] == self.data[b]) as usize;
                    self.ip += 4;
                }
                9 => {
                    let a = self.address(1, mode1);
                    self.base += self.data[a] as usize;
                    self.ip += 2;
                }
                99 => return Status::Halt,
                c => panic!("invalid instruction {c}")
            }
        }
    }

    #[inline]
    fn address(&self, offset: usize, mode: usize) -> usize {
        match mode {
            0 => self.data[self.ip + offset],
            1 => self.ip + offset,
            2 => self.base + self.data[self.ip + offset],
            _ => panic!("invalid mode: {mode}")
        }   
    }

    pub fn input(&mut self, val: i64) {
        self.input.push_back(val as usize);
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.base = 0;
        self.input.clear();
    }
}

/*
macro_rules! a0 {
            ($x: expr) => { self.data[$x] }
        }

        macro_rules! a1 {
            ($x: expr) => { $x }
        }

        macro_rules! a2 {
            ($x: expr) => { self.data[self.base+$x] }
        }
        
        loop {
            let instr = self.data[self.ip];
            let a = self.data[self.ip+1];
            let b = self.data[self.ip+2];
            let c= self.data[self.ip+3];

            match instr % 100 {
                1 =>     { a0!(c) = a0!(a) + a0!(b); self.ip += 4 },
                101 =>   { a0!(c) = a1!(a) + a0!(b); self.ip += 4 },
                201 =>   { a0!(c) = a2!(a) + a0!(b); self.ip += 4 },
                1001 =>  { a0!(c) = a0!(a) + a1!(b); self.ip += 4 },
                1101 =>  { a0!(c) = a1!(a) + a1!(b); self.ip += 4 },
                1201 =>  { a0!(c) = a2!(a) + a1!(b); self.ip += 4 },
                2001 =>  { a0!(c) = a0!(a) + a2!(b); self.ip += 4 },
                2101 =>  { a0!(c) = a1!(a) + a2!(b); self.ip += 4 },
                2201 =>  { a0!(c) = a2!(a) + a2!(b); self.ip += 4 },
                20001 => { a2!(c) = a0!(a) + a0!(b); self.ip += 4 },
                20101 => { a2!(c) = a1!(a) + a0!(b); self.ip += 4 },
                20201 => { a2!(c) = a2!(a) + a0!(b); self.ip += 4 },
                21001 => { a2!(c) = a0!(a) + a1!(b); self.ip += 4 },
                21101 => { a2!(c) = a1!(a) + a1!(b); self.ip += 4 },
                21201 => { a2!(c) = a2!(a) + a1!(b); self.ip += 4 },
                22001 => { a2!(c) = a0!(a) + a2!(b); self.ip += 4 },
                22101 => { a2!(c) = a1!(a) + a2!(b); self.ip += 4 },
                22201 => { a2!(c) = a2!(a) + a2!(b); self.ip += 4 },

                2 =>     { a0!(c) = a0!(a) * a0!(b); self.ip += 4 },
                102 =>   { a0!(c) = a1!(a) * a0!(b); self.ip += 4 },
                202 =>   { a0!(c) = a2!(a) * a0!(b); self.ip += 4 },
                1002 =>  { a0!(c) = a0!(a) * a1!(b); self.ip += 4 },
                1102 =>  { a0!(c) = a1!(a) * a1!(b); self.ip += 4 },
                1202 =>  { a0!(c) = a2!(a) * a1!(b); self.ip += 4 },
                2002 =>  { a0!(c) = a0!(a) * a2!(b); self.ip += 4 },
                2102 =>  { a0!(c) = a1!(a) * a2!(b); self.ip += 4 },
                2202 =>  { a0!(c) = a2!(a) * a2!(b); self.ip += 4 },
                20002 => { a2!(c) = a0!(a) * a0!(b); self.ip += 4 },
                20102 => { a2!(c) = a1!(a) * a0!(b); self.ip += 4 },
                20202 => { a2!(c) = a2!(a) * a0!(b); self.ip += 4 },
                21002 => { a2!(c) = a0!(a) * a1!(b); self.ip += 4 },
                21102 => { a2!(c) = a1!(a) * a1!(b); self.ip += 4 },
                21202 => { a2!(c) = a2!(a) * a1!(b); self.ip += 4 },
                22002 => { a2!(c) = a0!(a) * a2!(b); self.ip += 4 },
                22102 => { a2!(c) = a1!(a) * a2!(b); self.ip += 4 },
                22202 => { a2!(c) = a2!(a) * a2!(b); self.ip += 4 },

                3 => {
                    if let Some(value) = self.input.pop_front() {
                        a0!(a) = value;
                        self.ip += 2;
                    } else {
                        return Status::Input;
                    }
                },
                203 => {
                    if let Some(value) = self.input.pop_front() {
                        a2!(a) = value;
                        self.ip += 2;
                    } else {
                        return Status::Input;
                    }
                },
                4   => { self.ip += 2; return Status::Output(a0!(a) as i64); }
                104 => { self.ip += 2; return Status::Output(a1!(a) as i64); }
                204 => { self.ip += 2; return Status::Output(a2!(a) as i64); }
*/