use crate::gates::combinatorial::{half_adder, full_adder};
use crate::gates::Bit;
use std::convert::TryInto;

pub enum Opcode {
    Add,
    Sub,
    And,
    Or,
    Not,
    Neg,
}

pub fn add16(a: [Bit;16], b:[Bit;16]) -> [Bit;16] {
    let mut results = [Bit::Default();16];
    let (sum, mut carry) = half_adder(a[0], b[0]);
    results[0] = sum;
    for i in 1..16 {
        let (sum, new_carry) = full_adder(a[i], b[i], carry);
        results[i] = sum;
        carry = new_carry;
    }
    results
}

pub fn alu(a:[Bit;16], b:[Bit;16], opcode: Opcode) -> ([Bit;16], Bit, Bit) {
    match opcode {
        Opcode::Add => add16(a, b),
        Opcode::Sub => 
        Opcode::And => 
        Opcode::Or => 
        Opcode::Not => 
        Opcode::Neg => 
    }
}