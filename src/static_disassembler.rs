use std::fmt;

pub enum Instr {
    Halt,
    Set(u16, u16),
    Push(u16),
    Pop(u16),
    Eq(u16, u16, u16),
    Gt(u16, u16, u16),
    Jmp(u16),
    Jt(u16, u16),
    Jf(u16, u16),
    Add(u16, u16, u16),
    Mult(u16, u16, u16),
    Mod(u16, u16, u16),
    And(u16, u16, u16),
    Or(u16, u16, u16),
    Not(u16, u16),
    Rmem(u16, u16),
    Wmem(u16, u16),
    Call(u16),
    Ret,
    Out(u16),
    In(u16),
    Noop,
}

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instr::Halt => write!(f, "Halt"),
            Instr::Set(a, b) => write!(f, "Set({}, {}) ", a, b),
            Instr::Push(a) => write!(f, "Push({}) ", a),
            Instr::Pop(a) => write!(f, "Pop({}) ", a),
            Instr::Eq(a, b, c) => write!(f, "Eq({}, {}, {}) ", a, b, c),
            Instr::Gt(a, b, c) => write!(f, "Gt({}, {}, {}) ", a, b, c),
            Instr::Jmp(a) => write!(f, "Jmp({}) ", a),
            Instr::Jt(a, b) => write!(f, "Jt({}, {}) ", a, b),
            Instr::Jf(a, b) => write!(f, "Jf({}, {}) ", a, b),
            Instr::Add(a, b, c) => write!(f, "Add({}, {}, {}) ", a, b, c),
            Instr::Mult(a, b, c) => write!(f, "Mult({}, {}, {}) ", a, b, c),
            Instr::Mod(a, b, c) => write!(f, "Mod({}, {}, {}) ", a, b, c),
            Instr::And(a, b, c) => write!(f, "And({}, {}, {}) ", a, b, c),
            Instr::Or(a, b, c) => write!(f, "Or({}, {}, {}) ", a, b, c),
            Instr::Not(a, b) => write!(f, "Not({}, {}) ", a, b),
            Instr::Rmem(a, b) => write!(f, "Rmem({}, {}) ", a, b),
            Instr::Wmem(a, b) => write!(f, "Wmem({}, {}) ", a, b),
            Instr::Call(a) => write!(f, "Call({}) ", a),
            Instr::Ret => write!(f, "Ret "),
            Instr::Out(a) => write!(f, "Out({}) ", a),
            Instr::In(a) => write!(f, "In({}) ", a),
            Instr::Noop => write!(f, "Noop "),
        }
    }
}

fn parse(pc: &mut usize, code: &[u16]) -> Option<Instr> {
    let opcode = &code[*pc];
    let operands = &code[*pc + 1..];

    match opcode {
        0 => {
            *pc += 1;
            Some(Instr::Halt)
        }
        1 => {
            *pc += 3;
            Some(Instr::Set(operands[0], operands[1]))
        }
        2 => {
            *pc += 2;
            Some(Instr::Push(operands[0]))
        }
        3 => {
            *pc += 2;
            Some(Instr::Pop(operands[0]))
        }
        4 => {
            *pc += 4;
            Some(Instr::Eq(operands[0], operands[1], operands[2]))
        }
        5 => {
            *pc += 4;
            Some(Instr::Gt(operands[0], operands[1], operands[2]))
        }
        6 => {
            *pc += 2;
            Some(Instr::Jmp(operands[0]))
        }
        7 => {
            *pc += 3;
            Some(Instr::Jt(operands[0], operands[1]))
        }
        8 => {
            *pc += 3;
            Some(Instr::Jf(operands[0], operands[1]))
        }
        9 => {
            *pc += 4;
            Some(Instr::Add(operands[0], operands[1], operands[2]))
        }
        10 => {
            *pc += 4;
            Some(Instr::Mult(operands[0], operands[1], operands[2]))
        }
        11 => {
            *pc += 4;
            Some(Instr::Mod(operands[0], operands[1], operands[2]))
        }
        12 => {
            *pc += 4;
            Some(Instr::And(operands[0], operands[1], operands[2]))
        }
        13 => {
            *pc += 4;
            Some(Instr::Or(operands[0], operands[1], operands[2]))
        }
        14 => {
            *pc += 3;
            Some(Instr::Not(operands[0], operands[1]))
        }
        15 => {
            *pc += 3;
            Some(Instr::Rmem(operands[0], operands[1]))
        }
        16 => {
            *pc += 3;
            Some(Instr::Wmem(operands[0], operands[1]))
        }
        17 => {
            *pc += 2;
            Some(Instr::Call(operands[0]))
        }
        18 => {
            *pc += 1;
            Some(Instr::Ret)
        }
        19 => {
            *pc += 2;
            Some(Instr::Out(operands[0]))
        }
        20 => {
            *pc += 2;
            Some(Instr::In(operands[0]))
        }
        21 => {
            *pc += 1;
            Some(Instr::Noop)
        }
        _ => {
            // panic!(
            //     "encountered {x}, so quitting.\ncont:\n{:?}",
            //     &operands[0..10]
            // );
            *pc += 1;
            None
        }
    }
}

fn disassemble(code: &[u16]) -> Vec<(usize, Instr)> {
    let mut pc = 0;
    let mut instrs = vec![];
    while pc < code.len() {
        let ind = pc.clone();
        if let Some(instr) = parse(&mut pc, code) {
            // println!("Parsed: {ind}:{instr}");
            instrs.push((ind, instr));
        }
    }

    instrs
}

pub fn print_disassembly(code: &[u16]) {
    for (ind, instr) in disassemble(code) {
        println!("{ind}: {instr}");
    }
}
