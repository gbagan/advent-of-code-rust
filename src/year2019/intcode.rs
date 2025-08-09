use std::collections::VecDeque;
use crate::util::parser::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Status { Run, Halt, Input, Output(i32) }

#[derive(Clone)]
pub struct IntCode {
    data: Vec<i32>,
    ip: usize,
    input: VecDeque<i32>,
}

impl IntCode {
    pub fn new(input: &str) -> Self {
        let data = input.iter_signed().collect();
        Self { data, ip: 0, input: VecDeque::new() }
    }

    pub fn set(&mut self, index: usize, val: i32) {
        self.data[index] = val;
    }

    pub fn get(&self, index: usize) -> i32 {
        self.data[index]
    }

    pub fn run(&mut self) -> Status {
        loop {
            let instr = self.data[self.ip];
            match instr % 100 {
                1 => {
                    let a = self.address(1, (instr / 100) % 10);
                    let b = self.address(2, (instr / 1000) % 10);
                    let c = self.address(3, instr / 10000);
                    self.data[c] = self.data[a] + self.data[b];
                    self.ip += 4;
                }
                2 => {
                    let a = self.address(1, (instr / 100) % 10);
                    let b = self.address(2, (instr / 1000) % 10);
                    let c = self.address(3, instr / 10000);
                    self.data[c] = self.data[a] * self.data[b];
                    self.ip += 4;
                }
                3 => {
                    if let Some(value) = self.input.pop_front() {
                        let a = self.address(1, instr / 100);
                        self.data[a] = value;
                        self.ip += 2;
                    } else {
                        return Status::Input;
                    }
                }
                4 => {
                    let a = self.address(1, instr / 100);
                    self.ip += 2;
                    return Status::Output(self.data[a]);
                }
                5 => {
                    let a = self.address(1, (instr / 100) % 10);
                    if self.data[a] != 0 {
                        let b = self.address(2, instr / 1000);
                        self.ip = self.data[b] as usize;
                    } else {    
                        self.ip += 3;
                    }
                }
                6 => {
                    let a = self.address(1, (instr / 100) % 10);
                    if self.data[a] == 0 {
                        let b = self.address(2, instr / 1000);
                        self.ip = self.data[b] as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let a = self.address(1, (instr / 100) % 10);
                    let b = self.address(2, (instr / 1000) % 10);
                    let c = self.address(3, instr / 10000);
                    self.data[c] = (self.data[a] < self.data[b]) as i32;
                    self.ip += 4;
                }
                8 => {
                    let a = self.address(1, (instr / 100) % 10);
                    let b = self.address(2, (instr / 1000) % 10);
                    let c = self.address(3, instr / 10000);
                    self.data[c] = (self.data[a] == self.data[b]) as i32;
                    self.ip += 4;
                }
                99 => return Status::Halt,
                c => panic!("invalid instruction {c}")
            }
        }
    }

    fn address(&self, offset: usize, mode: i32) -> usize {
        match mode {
            0 => self.data[self.ip + offset] as usize,
            1 => self.ip + offset,
            _ => panic!("invalid mode: {mode}")
        }   
    }

    pub fn input(&mut self, val: i32) {
        self.input.push_back(val);
    }

    pub fn reset(&mut self) {
        self.ip = 0;
        self.input.clear();
    }
}

/*
*/