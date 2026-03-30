use super::*

pub fn add16(a: [Bit;16], b:[Bit;16]) -> [Bit;16] {
    let mut results = Vec::new();
    results.push(half_adder(a[0], b[0]))
    for i in 1..14 {
        results.push()
    }
}