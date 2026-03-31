use crate::gates::combinatorial::{half_adder, full_adder};
use super::*
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

fn zn_extractor(a:&[Bit;16]) -> (Bit, Bit) {
    let z_flag = not(a.iter().fold(false, |acc, &bit| or(acc, bit)));
    let n_flag = a[15];
    (z_flag, n_flag)
}

fn apply_base_gates(f: fn(Bit, Bit) -> Bit, a:[Bit;16], b:[Bit;16] ) -> ([Bit;16], Bit, Bit) {
    let mut res = [Bit::default();16];
    for i in 0..16 {
        res[i] = f(a[i], b[i]);
    }
    let (z_flag, n_flag) = zn_extractor(&res);
    (res, z_flag, n_flag)
}

fn apply_ornot(f: fn(Bit) -> Bit, a:[Bit;16]) -> ([Bit;16], Bit, Bit) {
    let mut res = [Bit::default();16];
    for i in 0..16 {
        res[i] = f(a[i]);
    }
    let (z_flag, n_flag) = zn_extractor(&res);
    (res, z_flag, n_flag)
}

pub fn alu(a:[Bit;16], b:[Bit;16], opcode: Opcode) -> ([Bit;16], Bit, Bit) {
    match opcode {
        Opcode::Add => {
            let add_result = add16(a, b);
            let (z_flag, n_flag) = zn_extractor(&add_result);
            (add_result, z_flag, n_flag)
        },
        Opcode::Sub => {
            let not_res = neg16(b);
            let mut one : [Bit;16] = [false;16];
            one[0] = true;
            let res = add16(a, add16(not_res, one));
            let (z_flag, n_flag) = zn_extractor(&res);
            (res, z_flag, n_flag)
        },
        Opcode::And => {
            apply_base_gates(and, a, b)
        },
        Opcode::Or => {
            apply_base_gates(or, a, b)
        },
        Opcode::Not => {
            apply_ornot(not, a)
        },
        Opcode::Neg => {
            let res = neg16(a);
            let (z_flag, n_flag) = zn_extractor(&res);
            (res, z_flag, n_flag)
        }
    }
}