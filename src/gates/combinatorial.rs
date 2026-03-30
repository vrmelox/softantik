use super::{and,xor, or, not, Bit};

pub fn mux(a: Bit, b: Bit, s:Bit) -> Bit {
    or(and(a, not(s)), and(b, s))
}

pub fn dmux(ain: Bit, s:Bit) -> (Bit, Bit) {
    (and(ain, not(s)), and(ain, s))
}

pub fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    (xor(a, b), and(a, b))
}

pub fn full_adder(a: Bit, b: Bit, cin: Bit) -> (Bit, Bit) {
    (xor(xor(a, b), cin), or(and(a, b), and(xor(a, b), cin)))
}