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
    let mut results = [Bit::default();16];
    let (sum, mut carry) = half_adder(a[0], b[0]);
    results[0] = sum;
    for i in 1..16 {
        let (sum, new_carry) = full_adder(a[i], b[i], carry);
        results[i] = sum;
        carry = new_carry;
    }
    results
}

pub fn neg16(tab: [Bit;16]) -> [Bit;16] {
    let mut res = [Bit::default();16];
    for i in 0..16 {
        res[i] = not(tab[i])
    }
    res
}

pub fn alu(a:[Bit;16], b:[Bit;16], opcode: Opcode) -> ([Bit;16], Bit, Bit) {
    match opcode {
        Opcode::Add => {
            let add_result = add16(a, b);
            let z_flag = not(add_result.iter().fold(false, |acc, &bit| or(acc, bit)));
            let n_flag = add_result[15];
            (add_result, z_flag, n_flag)
        },
        Opcode::Sub => {
            let not_res = neg16(b);
            let mut one : [Bit;16] = [false;16];
            one[0] = true;
            let res = add16(a, add16(not_res, one));
            let z_flag = not(res.iter().fold(false, |acc, &bit| or(acc, bit)));
            let n_flag = res[15];
            (res, z_flag, n_flag)
        },
        Opcode::And => {
            let mut res = 
        }
        Opcode::Or => 
        Opcode::Not => 
        Opcode::Neg => 
    }
}