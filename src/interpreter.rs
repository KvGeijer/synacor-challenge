use std::io::{self, Write};

const MEMSIZE: usize = 32768;
const REGS: usize = 8;
const MAXINT: u16 = 32768;

struct State {
    memory: [u16; MEMSIZE],
    registers: [u16; REGS],
    stack: Vec<u16>,
    pc: usize, // Should probably not be here
}

impl State {
    fn new() -> Self {
        Self {
            memory: [0; MEMSIZE],
            registers: [0; REGS],
            stack: vec![],
            pc: 0,
        }
    }

    fn execute(&mut self, code: &[u16]) -> bool {
        let opcode = &code[self.pc];
        let operands = &code[self.pc + 1..]; // problem if last one has no operands?
                                             // println!("Executing {opcode}");
        match opcode {
            0 => {
                println!("\nHalting!");
                return false;
            }
            1 => {
                self.set(operands[0], self.read(operands[1]));
                self.pc += 3;
            }
            2 => {
                self.stack.push(self.read(operands[0]));
                self.pc += 2;
            }
            3 => {
                let popped = self.stack.pop().expect("Popping empty stack");
                self.set(operands[0], popped);
                self.pc += 2;
            }
            4 => {
                let cond = self.read(operands[1]) == self.read(operands[2]);
                self.set(operands[0], cond as u16);
                self.pc += 4;
            }
            5 => {
                let cond = self.read(operands[1]) > self.read(operands[2]);
                self.set(operands[0], cond as u16);
                self.pc += 4;
            }
            6 => {
                self.pc = self.read(operands[0]) as usize;
            }
            7 => {
                if self.read(operands[0]) != 0 {
                    self.pc = self.read(operands[1]) as usize;
                } else {
                    self.pc += 3;
                }
            }
            8 => {
                if self.read(operands[0]) == 0 {
                    self.pc = self.read(operands[1]) as usize;
                } else {
                    self.pc += 3;
                }
            }
            9 => {
                let sum = (self.read(operands[1]) + self.read(operands[2])) % MAXINT;
                self.set(operands[0], sum);
                self.pc += 4;
            }
            19 => {
                print!("{}", self.read(operands[0]) as u8 as char);
                io::stdout().flush().unwrap();
                self.pc += 2;
            }
            21 => {
                self.pc += 1;
            }
            x => {
                println!("encountered {x}, so quitting");
                return false;
            }
        };

        true
    }

    fn read(&self, location: u16) -> u16 {
        if is_literal(location) {
            location
        } else if is_reg(location) {
            self.registers[to_reg(location)]
        } else {
            panic!("Cannot parse {location}")
        }
    }

    fn set(&mut self, location: u16, value: u16) {
        if is_literal(location) {
            self.memory[location as usize] = value;
        } else if is_reg(location) {
            self.registers[to_reg(location)] = value;
        } else {
            panic!("Cannot parse {location}")
        }
    }
}

pub fn interpret(code: &[u16]) {
    let mut state = State::new();

    while state.execute(code) {}
}

fn to_reg(literal: u16) -> usize {
    literal as usize - 32768
}

fn is_literal(literal: u16) -> bool {
    literal < MAXINT
}

fn is_reg(literal: u16) -> bool {
    MAXINT <= literal && literal < MAXINT + 8
}
