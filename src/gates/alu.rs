use crate::gates::combinatorial::{half_adder, full_adder};
use crate::gates::Bit;
use std::convert::TryInto;

pub fn add16(a: [Bit;16], b:[Bit;16]) -> [Bit;16] {
    let mut results = Vec::new();
    let resu = half_adder(a[0], b[0]);
    results.insert(0, resu.0);
    let mut carry = resu.1;
    for i in 1..16 {
        let  resu = full_adder(a[i], b[i], carry);
        carry = resu.1;
        results.insert(i, resu.0);
    }
    let arr: [Bit; 16] = results.try_into().expect("taille incorrecte");
    arr
}