use std::io::{self, Read, Write};

const MEMSIZE: usize = 32768;
const REGS: usize = 8;
const INTWRAP: u16 = 32768;

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

    fn execute(&mut self, trace: &mut bool) -> bool {
        let opcode = &self.memory[self.pc];
        let operands = &self.memory[self.pc + 1..]; // problem if last one has no operands?

        if *trace {
            println!(
                "Executing {}, regs: {:?} stack top: {:?}",
                self.pc,
                self.registers,
                self.stack.last()
            );
        }
        if self.pc == 5491 {
            println!("Completed");
        }
        match opcode {
            0 => {
                println!("\nHalting!");
                return false;
            }
            1 => {
                if operands[0] < 9 {
                    panic!("Too small register set! {}", operands[0]);
                }
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
                let sum = (self.read(operands[1]) as u32 + self.read(operands[2]) as u32)
                    % INTWRAP as u32;
                self.set(operands[0], sum as u16);
                self.pc += 4;
            }
            10 => {
                let prod = (self.read(operands[1]) as u32 * self.read(operands[2]) as u32)
                    % INTWRAP as u32;
                self.set(operands[0], prod as u16);
                self.pc += 4;
            }
            11 => {
                let mask = self.read(operands[1]) % self.read(operands[2]);
                self.set(operands[0], mask);
                self.pc += 4;
            }
            12 => {
                let mask = self.read(operands[1]) & self.read(operands[2]);
                self.set(operands[0], mask);
                self.pc += 4;
            }
            13 => {
                let mask = self.read(operands[1]) | self.read(operands[2]);
                self.set(operands[0], mask);
                self.pc += 4;
            }
            14 => {
                let mask = (INTWRAP - 1) & !self.read(operands[1]);
                self.set(operands[0], mask);
                self.pc += 3;
            }
            15 => {
                let read = self.read_mem(self.read(operands[1]));
                self.set(operands[0], read);
                self.pc += 3;
            }
            16 => {
                let read = self.read(operands[1]);
                self.set(self.read(operands[0]), read);
                self.pc += 3;
            }
            17 => {
                if self.read(operands[0]) == 6027 {
                    println!("Skipping: {:?}", self.registers);
                    // self.pc = 5564;
                    // self.pc = 5514;
                    self.pc += 2;
                    self.registers[0] = 6
                } else {
                    self.stack.push(self.pc as u16 + 2);
                    self.pc = self.read(operands[0]) as usize;
                }
            }
            18 => {
                if let Some(addr) = self.stack.pop() {
                    self.pc = addr as usize;
                } else {
                    println!("Halting due to returning on empty stack");
                    return false;
                }
            }
            19 => {
                //*trace = *trace; //&& operands[0] == 10;
                print!("{}", self.read(operands[0]) as u8 as char);
                io::stdout().flush().unwrap();
                self.pc += 2;
            }
            20 => {
                let ascii = io::stdin().bytes().next().unwrap().unwrap();
                if ascii == b'$' {
                    self.set(INTWRAP + 7, 25734);
                    println!("Now {:?}", self.registers);
                    io::stdin().bytes().next().unwrap().unwrap();
                } else if ascii == b'?' {
                    io::stdin().bytes().next().unwrap().unwrap();
                    *trace = !*trace;
                    println!("At {}, with regs: {:?}", self.pc, self.registers);
                } else {
                    // Just for debugging
                    print!("{}", ascii as char);
                    io::stdout().flush().unwrap();
                    self.set(operands[0], ascii as u16);
                    self.pc += 2;
                }
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
            if location == INTWRAP + 7 {
                println!("reading 8th reg, {:?}", self.pc);
            }
            self.registers[to_reg(location)]
        } else {
            panic!("Cannot parse {location}")
        }
    }

    fn read_mem(&self, location: u16) -> u16 {
        if is_literal(location) {
            self.memory[location as usize]
        // } else if is_reg(location) {
        //     self.registers[to_reg(location)]
        } else {
            panic!("Cannot read from {location}")
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
    for (i, num) in code.iter().enumerate() {
        state.memory[i] = *num;
    }

    let mut trace = false;
    while state.execute(&mut trace) {}
}

fn to_reg(literal: u16) -> usize {
    literal as usize - MEMSIZE
}

fn is_literal(literal: u16) -> bool {
    literal < INTWRAP
}

fn is_reg(literal: u16) -> bool {
    INTWRAP <= literal && literal < INTWRAP + 8
}
